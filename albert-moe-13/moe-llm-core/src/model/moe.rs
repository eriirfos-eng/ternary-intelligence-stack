// Sparse Top-3 MoE routing with asymmetric safety gate — whitepaper §11.1, §10.4
// Gate kept in F32 (candle_nn::Linear): routing needs fine-grained signal that
// ternary resolution can't provide at 256→12 scale. Expert MLPs remain ternary.
//
// Ternary Traffic Light (TTL) routing layer:
// Each expert is assigned a trit state (Green/Orange/Red) from rolling EMA utilization.
// Green experts get a gate logit boost; Red are suppressed; Orange pass unmodified
// but contribute at ORANGE_SCALE to the output mix. Derived from Lisa Scharler's
// triunity routing sketch — the trit is the execution budget, not just the routing signal.
use candle_core::{Result, Tensor};
use candle_nn::{Module, VarBuilder};
use super::mlp::Mlp;
use super::traffic_light::TrafficLight;
use std::cell::RefCell;

thread_local! {
    static ROUTING_ACC: RefCell<Vec<f32>> = RefCell::new(Vec::new());
    static ENTROPY_ACC: RefCell<Option<Tensor>> = RefCell::new(None);
    static LB_ACC:      RefCell<Option<Tensor>> = RefCell::new(None);
    // Traffic light telemetry: accumulated (green, orange, red, state_string) per step.
    // Each MoeBlock appends its layer's counts; train_bible drains once per batch.
    static TLIGHT_ACC:  RefCell<Vec<(usize, usize, usize, String)>> = RefCell::new(Vec::new());
    // Training mode flag: Gumbel noise is injected during training only.
    // prepare_inference() sets this to false so eval_perplexity gets clean deterministic routing.
    static TRAIN_MODE: RefCell<bool> = RefCell::new(true);
}

pub fn clear_routing_capture() { ROUTING_ACC.with(|r| r.borrow_mut().clear()); }
pub fn clear_entropy_capture() { ENTROPY_ACC.with(|e| *e.borrow_mut() = None); }
pub fn clear_lb_capture()      { LB_ACC.with(|lb| *lb.borrow_mut() = None); }
pub fn clear_tlight_capture()  { TLIGHT_ACC.with(|t| t.borrow_mut().clear()); }
pub fn enter_eval_mode()       { TRAIN_MODE.with(|m| *m.borrow_mut() = false); }

pub fn take_routing_capture(num_experts: usize) -> Vec<f32> {
    ROUTING_ACC.with(|r| {
        let acc = r.borrow();
        if acc.is_empty() { return vec![0.0; num_experts]; }
        let total: f32 = acc.iter().sum::<f32>().max(1e-9);
        acc.iter().map(|&w| w / total).collect()
    })
}

pub fn take_entropy_capture() -> Option<Tensor> {
    ENTROPY_ACC.with(|e| e.borrow_mut().take())
}

pub fn take_lb_capture() -> Option<Tensor> {
    LB_ACC.with(|lb| lb.borrow_mut().take())
}

/// Returns per-layer traffic light snapshots: (green, orange, red, state_string).
/// One entry per MoeBlock that fired this step.
pub fn take_tlight_capture() -> Vec<(usize, usize, usize, String)> {
    TLIGHT_ACC.with(|t| std::mem::take(&mut *t.borrow_mut()))
}

// Router temperature < 1.0 sharpens the gate softmax, amplifying small logit differences.
const ROUTER_TEMP: f64 = 0.7;

// Gumbel noise scale for training-time routing exploration.
// Additive noise of O(1) regardless of logit magnitude — the ±2% multiplicative
// noise it replaces was a no-op when logits ≈ 0 (Universal Nash state).
// Disabled in eval mode via TRAIN_MODE flag.
const GUMBEL_NOISE_SCALE: f64 = 1.0;

pub struct MoeBlock {
    gate: candle_nn::Linear,
    experts: Vec<Mlp>,
    num_experts: usize,
    // Ternary Traffic Light: per-expert execution budget controller.
    // Interior mutability: forward() takes &self but needs to update EMA each step.
    traffic_light: RefCell<TrafficLight>,
}

impl MoeBlock {
    pub fn new(hidden_size: usize, num_experts: usize, vb: VarBuilder, threshold: f32) -> Result<Self> {
        let gate = candle_nn::linear_no_bias(hidden_size, num_experts, vb.pp("gate"))?;
        let mut experts = Vec::new();
        let vb_experts = vb.pp("experts");
        for i in 0..num_experts {
            experts.push(Mlp::new(hidden_size, hidden_size * 4, vb_experts.pp(i), threshold)?);
        }
        Ok(Self {
            gate,
            experts,
            num_experts,
            traffic_light: RefCell::new(TrafficLight::new(num_experts)),
        })
    }

    pub fn prepare_inference(&self) -> Result<()> {
        enter_eval_mode();
        for expert in &self.experts { expert.prepare_inference()?; }
        Ok(())
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (b, s, h) = x.dims3()?;
        let dev = x.device();
        let x_flat = x.reshape((b * s, h))?;

        // 1. Gate logits — F32 linear for routing resolution
        let mut gate_logits = self.gate.forward(x)?; // [B, S, E]
        gate_logits = (gate_logits / ROUTER_TEMP)?;

        // 2. Clean probabilities for LB loss and entropy (computed on unmodified logits
        //    so LB gradient is orthogonal to the traffic light hard correction).
        let full_probs = candle_nn::ops::softmax(&gate_logits, candle_core::D::Minus1)?;

        // Neg-entropy accumulation.
        {
            let log_p = (&full_probs + 1e-9_f64)?.log()?;
            let neg_entropy = (&full_probs * log_p)?.sum(candle_core::D::Minus1)?.mean_all()?;
            ENTROPY_ACC.with(|e| {
                let mut cell = e.borrow_mut();
                *cell = match cell.take() {
                    None       => Some(neg_entropy),
                    Some(prev) => Some((&prev + &neg_entropy).unwrap()),
                };
            });
        }

        // 3. Ternary Traffic Light logit modifier — applied AFTER full_probs (keeps LB
        //    gradient clean) and BEFORE noise (consistent per-expert pressure each step).
        //    During warmup (first 50 steps) all modifiers are 0.0 — no routing change.
        {
            let tl = self.traffic_light.borrow();
            if !tl.is_warmup() {
                let modifiers = tl.logit_modifiers();
                let mod_tensor = Tensor::from_vec(modifiers, (1usize, 1usize, self.num_experts), dev)?;
                gate_logits = gate_logits.broadcast_add(&mod_tensor)?;
            }
        }

        // 4. Gumbel noise for exploration (training only).
        // g = -log(-log(u)), u ~ Uniform(ε, 1-ε) → standard Gumbel(0,1).
        // Additive: meaningful even when logits ≈ 0, directly breaks Universal Nash.
        if TRAIN_MODE.with(|m| *m.borrow()) {
            let u = Tensor::rand(1e-6_f32, 1.0_f32 - 1e-6_f32, gate_logits.shape(), dev)?;
            let gumbel = u.log()?.neg()?.log()?.neg()?;
            gate_logits = (gate_logits + (gumbel * GUMBEL_NOISE_SCALE)?)?;
        }

        // 5. Top-3 Routing.
        let large_neg_val = Tensor::new(&[-1e9f32], dev)?;

        let max1_indices = gate_logits.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let mask1 = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?
            .broadcast_eq(&max1_indices.unsqueeze(candle_core::D::Minus1)?)?;

        let gate_logits_m1 = mask1.where_cond(&large_neg_val.broadcast_as(gate_logits.shape())?, &gate_logits)?;
        let max2_indices = gate_logits_m1.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let mask2 = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?
            .broadcast_eq(&max2_indices.unsqueeze(candle_core::D::Minus1)?)?;

        let gate_logits_m2 = mask2.where_cond(&large_neg_val.broadcast_as(gate_logits.shape())?, &gate_logits_m1)?;
        let max3_indices = gate_logits_m2.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let mask3 = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?
            .broadcast_eq(&max3_indices.unsqueeze(candle_core::D::Minus1)?)?;

        // 6. Load-balancing loss (Switch Transformer §5).
        let f_i_tensor;
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
                    None       => Some(lb),
                    Some(prev) => Some((&prev + &lb).unwrap()),
                };
            });
            f_i_tensor = f_i;
        }

        // 7. Update traffic light EMA from observed routing, then capture telemetry.
        {
            let f_i_vec = f_i_tensor.to_vec1::<f32>().unwrap_or_else(|_| vec![0.0; self.num_experts]);
            let mut tl = self.traffic_light.borrow_mut();
            tl.update(&f_i_vec);
            let (g, o, r) = tl.counts();
            let states = tl.state_string();
            TLIGHT_ACC.with(|t| t.borrow_mut().push((g, o, r, states)));
        }

        // 8. Asymmetric safety gate (v2.0): zero out low-confidence safety experts.
        let max1_values = gate_logits.max(candle_core::D::Minus1)?;
        let max2_values = gate_logits_m1.max(candle_core::D::Minus1)?;
        let max3_values = gate_logits_m2.max(candle_core::D::Minus1)?;

        let safety_threshold = 0.0f32;
        let apply_safety = |idx_tensor: &Tensor, val_tensor: &Tensor| -> Result<Tensor> {
            let is_safety  = idx_tensor.lt(4u32)?.to_dtype(candle_core::DType::F32)?;
            let is_low_conf = val_tensor.lt(safety_threshold)?.to_dtype(candle_core::DType::F32)?;
            let should_hold = (is_safety * is_low_conf)?;
            let multiplier  = (should_hold.neg()? + 1.0)?;
            val_tensor.broadcast_mul(&multiplier)
        };

        let max1_values = apply_safety(&max1_indices, &max1_values)?;
        let max2_values = apply_safety(&max2_indices, &max2_values)?;
        let max3_values = apply_safety(&max3_indices, &max3_values)?;

        // Top-3 softmax weights.
        let top3_logits = Tensor::stack(
            &[max1_values.flatten_all()?, max2_values.flatten_all()?, max3_values.flatten_all()?],
            1,
        )?;
        let top3_probs = candle_nn::ops::softmax(&top3_logits, 1)?;

        let mut final_output = Tensor::zeros((b * s, h), x.dtype(), dev)?;

        let m1_flat = max1_indices.flatten_all()?;
        let m2_flat = max2_indices.flatten_all()?;
        let m3_flat = max3_indices.flatten_all()?;

        let p1 = top3_probs.narrow(1, 0, 1)?.flatten_all()?;
        let p2 = top3_probs.narrow(1, 1, 1)?.flatten_all()?;
        let p3 = top3_probs.narrow(1, 2, 1)?.flatten_all()?;

        // 9. Sequential expert execution with ternary execution budget.
        let output_scales = self.traffic_light.borrow().output_scales();

        for expert_idx in 0..self.num_experts {
            let mask1_bool = m1_flat.eq(expert_idx as u32)?;
            let mask2_bool = m2_flat.eq(expert_idx as u32)?;
            let mask3_bool = m3_flat.eq(expert_idx as u32)?;

            let w1 = (mask1_bool.to_dtype(x.dtype())? * &p1)?;
            let w2 = (mask2_bool.to_dtype(x.dtype())? * &p2)?;
            let w3 = (mask3_bool.to_dtype(x.dtype())? * &p3)?;
            let combined_weight = (w1 + w2 + w3)?.unsqueeze(1)?;

            let max_w = combined_weight.max_all()?.to_scalar::<f32>()?;

            ROUTING_ACC.with(|r| {
                let mut acc = r.borrow_mut();
                if acc.len() <= expert_idx { acc.resize(expert_idx + 1, 0.0); }
                acc[expert_idx] += max_w;
            });

            // @sparseskip: skip non-routed experts entirely — whitepaper §5.2.
            if max_w == 0.0 { continue; }

            let expert_out = self.experts[expert_idx].forward(&x_flat)?;

            // Ternary execution budget: scale output by traffic light state.
            // Green = 1.0 (full), Orange = ORANGE_SCALE (partial), Red = 1.0 (but skipped above).
            let scale = output_scales[expert_idx] as f64;
            let scaled_weight = if (scale - 1.0).abs() > 1e-6 {
                (combined_weight * scale)?
            } else {
                combined_weight
            };

            final_output = (final_output + expert_out.broadcast_mul(&scaled_weight)?)?;
        }

        final_output.reshape((b, s, h))
    }
}
