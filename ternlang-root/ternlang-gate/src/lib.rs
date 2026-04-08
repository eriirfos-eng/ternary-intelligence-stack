#![no_std]

pub enum GateType {
    SignalRouting,
    MemoryAccess,
    ComputeUnlock,
}

pub fn triadic_resolution(_gate: GateType, signals: &[f32]) -> i8 {
    if signals.is_empty() {
        return 0;
    }
    let mut sum = 0.0;
    for &s in signals {
        sum += s;
    }
    let avg = sum / (signals.len() as f32);

    if avg > 0.7 {
        1
    } else if avg < 0.3 {
        -1
    } else {
        0
    }
}
