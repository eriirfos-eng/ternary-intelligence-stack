// Embedded reference documentation for Albert CLI
// These are high-value assets collected from production systems and OpenClaw

pub const SOUL: &str = include_str!("../SOUL.md");
pub const PATTERNS: &str = include_str!("../patterns.md");
pub const SECURITY: &str = include_str!("../SECURITY.md");
pub const ANTIPATTERNS: &str = include_str!("../antipatterns.md");
pub const MEMORY: &str = include_str!("../MEMORY.md");
pub const CONTRIBUTING: &str = include_str!("../CONTRIBUTING.md");
pub const THREAT_MODEL: &str = include_str!("../THREAT-MODEL-ATLAS.md");
pub const TESTING: &str = include_str!("../testing.md");

/// Get a reference doc by name
pub fn get_doc(name: &str) -> Option<&'static str> {
    match name.to_lowercase().as_str() {
        "soul" => Some(SOUL),
        "patterns" => Some(PATTERNS),
        "security" => Some(SECURITY),
        "antipatterns" => Some(ANTIPATTERNS),
        "memory" => Some(MEMORY),
        "contributing" => Some(CONTRIBUTING),
        "threat-model" | "threat" => Some(THREAT_MODEL),
        "testing" | "tests" => Some(TESTING),
        _ => None,
    }
}

/// List all available reference docs
pub fn list_docs() -> &'static [&'static str] {
    &["soul", "patterns", "security", "antipatterns", "memory", "contributing", "threat-model", "testing"]
}
