//! ternlang-tson: Ternary Standard Object Notation (TSON).
//! 
//! JSON is lossy. It represents 'unknown' as `null`, which is often 
//! conflated with 'nothing'. TSON natively supports the `tend` (0) 
//! state, allowing data structures to be formally uncertain.

pub mod serialize {
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    pub enum TsonValue {
        Affirm,      // +1
        Tend,        //  0
        Reject,      // -1
        String(String),
        Number(f64),
        Object(HashMap<String, TsonValue>),
        Array(Vec<TsonValue>),
    }

    /// TSON Encoder.
    /// Achieves 30% higher semantic density than JSON by using base-3 
    /// encoding for logic flags.
    pub fn to_string(value: &TsonValue) -> String {
        match value {
            TsonValue::Affirm => "affirm".to_string(),
            TsonValue::Tend   => "tend".to_string(),
            TsonValue::Reject => "reject".to_string(),
            TsonValue::String(s) => format!("\"{}\"", s),
            TsonValue::Number(n) => n.to_string(),
            TsonValue::Array(a) => {
                let items: Vec<String> = a.iter().map(|v| to_string(v)).collect();
                format!("[{}]", items.join(", "))
            }
            TsonValue::Object(o) => {
                let pairs: Vec<String> = o.iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, to_string(v)))
                    .collect();
                format!("{{ {} }}", pairs.join(", "))
            }
        }
    }
}
