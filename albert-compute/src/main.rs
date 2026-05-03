use candle_core::{Device, Tensor, DType};
use candle_nn::{Optimizer, SGD, VarMap, Init};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev = &Device::Cpu;
    
    // Initialize parameter w=0.0 using VarMap (Scalar weight)
    let varmap = VarMap::new();
    let w = varmap.get((), "w", Init::Const(0.0), DType::F32, dev)?;
    let x = Tensor::new(&[2.0f32], dev)?;
    let target = Tensor::new(&[10.0f32], dev)?;
    
    let mut sgd = SGD::new(varmap.all_vars(), 0.1)?;
    
    println!("Initial w: {:.4}", w.to_scalar::<f32>()?);
    
    for i in 0..50 {
        // Forward pass: pred = w * x
        let pred = w.broadcast_mul(&x)?;
        let loss = pred.sub(&target)?.sqr()?.mean_all()?;
        
        // Backward pass
        let grads = loss.backward()?;
        
        // Optimizer step
        sgd.step(&grads)?;
        
        if i % 10 == 0 {
            println!("Step {}: w={:.4}, loss={:.4}", i, w.to_scalar::<f32>()?, loss.to_scalar::<f32>()?);
        }
    }
    
    println!("Final w: {:.4}", w.to_scalar::<f32>()?);
    Ok(())
}
