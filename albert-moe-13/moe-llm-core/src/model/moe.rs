// Sparse Top-3 MoE routing with asymmetric safety gate — whitepaper §11.1, §10.4
// Gate kept in F32 (candle_nn::Linear): routing needs fine-grained signal that
// ternary resolution can't provide at 256→12 scale. Expert MLPs remain ternary.
use candle_core::{Result, Tensor};
use candle_nn::{Module, VarBuilder};
use super::mlp::Mlp;
use std::cell::RefCell;

// Thread-local accumulators — one per forward pass, drained by train_bible.rs.
// No locks, no allocation on the hot path.
thread_local! {
    static ROUTING_ACC: RefCell<Vec<f32>> = RefCell::new(Vec::new());
    // Neg-entropy sum across all MoE layers — minimising maximises routing diversity.
    static ENTROPY_ACC: RefCell<Option<Tensor>> = RefCell::new(None);
    // Load-balancing loss (Switch Transformer §5): num_experts * Σ(f_i · P_i).
    // f_i = fraction of tokens routed to expert i; P_i = mean gate softmax prob for expert i.
    // Gradient flows through P_i, pushing over-used experts' gate probs down.
    static LB_ACC: RefCell<Option<Tensor>> = RefCell::new(None);
}

pub fn clear_routing_capture() {
    ROUTING_ACC.with(|r| r.borrow_mut().clear());
}

pub fn take_routing_capture(num_experts: usize) -> Vec<f32> {
    ROUTING_ACC.with(|r| {
        let acc = r.borrow();
        if acc.is_empty() { return vec![0.0; num_experts]; }
        let total: f32 = acc.iter().sum::<f32>().max(1e-9);
        acc.iter().map(|&w| w / total).collect()
    })
}

pub fn clear_entropy_capture() {
    ENTROPY_ACC.with(|e| *e.borrow_mut() = None);
}

pub fn take_entropy_capture() -> Option<Tensor> {
    ENTROPY_ACC.with(|e| e.borrow_mut().take())
}

pub fn clear_lb_capture() {
    LB_ACC.with(|lb| *lb.borrow_mut() = None);
}

pub fn take_lb_capture() -> Option<Tensor> {
    LB_ACC.with(|lb| lb.borrow_mut().take())
}

// Router temperature < 1.0 sharpens the gate softmax, amplifying small logit differences.
// After uniform-routing training the gate weights produce ~0.002 logit spread — softmax
// collapses to near-uniform regardless of CE signal. T=0.7 gives ~1.4× amplification,
// forcing clear expert preferences without the instability of very low T.
const ROUTER_TEMP: f64 = 0.7;

pub struct MoeBlock {
    gate: candle_nn::Linear,
    experts: Vec<Mlp>,
    num_experts: usize,
}

impl MoeBlock {
    pub fn new(hidden_size: usize, num_experts: usize, vb: VarBuilder, threshold: f32) -> Result<Self> {
        let gate = candle_nn::linear_no_bias(hidden_size, num_experts, vb.pp("gate"))?;
        let mut experts = Vec::new();
        let vb_experts = vb.pp("experts");
        for i in 0..num_experts {
            experts.push(Mlp::new(hidden_size, hidden_size * 4, vb_experts.pp(i), threshold)?);
        }
        Ok(Self { gate, experts, num_experts })
    }

    pub fn prepare_inference(&self) -> Result<()> {
        for expert in &self.experts { expert.prepare_inference()?; }
        Ok(())
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (b, s, h) = x.dims3()?;
        let dev = x.device();
        let x_flat = x.reshape((b * s, h))?;
        
        // 1. Gate logits — F32 linear for routing resolution (ternary too coarse at 256→12)
        let mut gate_logits = self.gate.forward(x)?; // [B, S, E]
        gate_logits = (gate_logits / ROUTER_TEMP)?;

        // Clean gate probabilities — computed pre-noise so gradients flow through the
        // unperturbed softmax. Used for both entropy telemetry and the LB loss.
        let full_probs = candle_nn::ops::softmax(&gate_logits, candle_core::D::Minus1)?;

        // Neg-entropy accumulation for entropy regularization telemetry.
        {
            let log_p = (&full_probs + 1e-9_f64)?.log()?;
            let neg_entropy = (&full_probs * log_p)?.sum(candle_core::D::Minus1)?.mean_all()?;
            ENTROPY_ACC.with(|e| {
                let mut cell = e.borrow_mut();
                *cell = match cell.take() {
                    None => Some(neg_entropy),
                    Some(prev) => Some((&prev + &neg_entropy).unwrap()),
                };
            });
        }

        // Add Gating Jitter (Noise) for exploration
        let noise = Tensor::rand(0.98f32, 1.02f32, gate_logits.shape(), dev)?;
        gate_logits = gate_logits.broadcast_mul(&noise)?;

        // 2. Top-3 Routing (v2.0 Evolution)
        let large_neg_val = Tensor::new(&[-1e9f32], dev)?;

        // Max 1
        let max1_indices = gate_logits.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let mask1 = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?
            .broadcast_eq(&max1_indices.unsqueeze(candle_core::D::Minus1)?)?;

        // Max 2
        let gate_logits_m1 = mask1.where_cond(&large_neg_val.broadcast_as(gate_logits.shape())?, &gate_logits)?;
        let max2_indices = gate_logits_m1.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let mask2 = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?
            .broadcast_eq(&max2_indices.unsqueeze(candle_core::D::Minus1)?)?;

        // Max 3
        let gate_logits_m2 = mask2.where_cond(&large_neg_val.broadcast_as(gate_logits.shape())?, &gate_logits_m1)?;
        let max3_indices = gate_logits_m2.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let mask3 = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?
            .broadcast_eq(&max3_indices.unsqueeze(candle_core::D::Minus1)?)?;

        // Load-balancing loss (Switch Transformer §5).
        // f_i from noisy argmax (non-differentiable), P_i from clean softmax (differentiable).
        // Gradient through P_i pushes over-used experts' gate probs down, independent of CE.
        {
            let sel = ((mask1.to_dtype(candle_core::DType::F32)?
                + mask2.to_dtype(candle_core::DType::F32)?)?
                + mask3.to_dtype(candle_core::DType::F32)?)?;
            let f_i = sel.reshape((b * s, self.num_experts))?.mean(0)?;
            let p_i = full_probs.reshape((b * s, self.num_experts))?.mean(0)?;
            let lb = ((&f_i * &p_i)?.sum_all()? * self.num_experts as f64)?;
            LB_ACC.with(|acc| {
                let mut cell = acc.borrow_mut();
                *cell = match cell.take() {
                    None => Some(lb),
                    Some(prev) => Some((&prev + &lb).unwrap()),
                };
            });
        }

        let max1_values = gate_logits.max(candle_core::D::Minus1)?;
        let max2_values = gate_logits_m1.max(candle_core::D::Minus1)?;
        let max3_values = gate_logits_m2.max(candle_core::D::Minus1)?;
        
        // 3. ASYMMETRIC SAFETY LOGIC (v2.0)
        // Threshold 0.0: only zero out experts with negative gate confidence.
        // Positive-signal experts are allowed through so routing can differentiate.
        let safety_threshold = 0.0f32;
        
        let apply_safety = |idx_tensor: &Tensor, val_tensor: &Tensor| -> Result<Tensor> {
            let is_safety = idx_tensor.lt(4u32)?.to_dtype(candle_core::DType::F32)?;
            let is_low_conf = val_tensor.lt(safety_threshold)?.to_dtype(candle_core::DType::F32)?;
            let should_hold = (is_safety * is_low_conf)?;
            let multiplier = (should_hold.neg()? + 1.0)?;
            val_tensor.broadcast_mul(&multiplier)
        };

        let max1_values = apply_safety(&max1_indices, &max1_values)?;
        let max2_values = apply_safety(&max2_indices, &max2_values)?;
        let max3_values = apply_safety(&max3_indices, &max3_values)?;

        // Softmax across Top-3
        let top3_logits = Tensor::stack(&[max1_values.flatten_all()?, max2_values.flatten_all()?, max3_values.flatten_all()?], 1)?;
        let top3_probs = candle_nn::ops::softmax(&top3_logits, 1)?;

        let mut final_output = Tensor::zeros((b * s, h), x.dtype(), dev)?;

        // 4. Sequential Expert Execution (Stable)
        let m1_flat = max1_indices.flatten_all()?;
        let m2_flat = max2_indices.flatten_all()?;
        let m3_flat = max3_indices.flatten_all()?;
        
        let p1 = top3_probs.narrow(1, 0, 1)?.flatten_all()?;
        let p2 = top3_probs.narrow(1, 1, 1)?.flatten_all()?;
        let p3 = top3_probs.narrow(1, 2, 1)?.flatten_all()?;

        for expert_idx in 0..self.num_experts {
            let mask1_bool = m1_flat.eq(expert_idx as u32)?;
            let mask2_bool = m2_flat.eq(expert_idx as u32)?;
            let mask3_bool = m3_flat.eq(expert_idx as u32)?;

            let w1 = (mask1_bool.to_dtype(x.dtype())? * &p1)?;
            let w2 = (mask2_bool.to_dtype(x.dtype())? * &p2)?;
            let w3 = (mask3_bool.to_dtype(x.dtype())? * &p3)?;
            let combined_weight = (w1 + w2 + w3)?.unsqueeze(1)?;

            // SparseSkip: if no token is routed to this expert, skip the MLP entirely — whitepaper §5.2.
            // For single-token inference (Top-3, 12 experts) this skips 9/12 experts — ~4× speedup.
            let max_w = combined_weight.max_all()?.to_scalar::<f32>()?;

            // Accumulate routing weight for telemetry (thread-local, zero overhead).
            ROUTING_ACC.with(|r| {
                let mut acc = r.borrow_mut();
                if acc.len() <= expert_idx { acc.resize(expert_idx + 1, 0.0); }
                acc[expert_idx] += max_w;
            });

            if max_w == 0.0 {
                continue;
            }

            let expert_out = self.experts[expert_idx].forward(&x_flat)?;
            final_output = (final_output + expert_out.broadcast_mul(&combined_weight)?)?;
        }
        
        final_output.reshape((b, s, h))
    }
}
