//! ternpkg-registry: Centralized Package Manager Backend
//! The NPM/Crates.io equivalent for the Ternary Intelligence Stack.

use std::collections::HashMap;

/// Represents a validated .ternpkg uploaded to the RFI-IRFOS registry.
pub struct TernaryPackage {
    pub name: String,
    pub version: String,
    pub compliance_score: f32, // Must be 1.0 (Exhaustive 3-State Match)
    pub is_certified: bool,
}

pub struct Registry {
    packages: HashMap<String, TernaryPackage>,
}

impl Registry {
    pub fn new() -> Self {
        let mut registry = Registry { packages: HashMap::new() };
        
        // Bootstrapping the monopoly: The Core RFI ecosystem is pre-loaded.
        registry.packages.insert("ternlang-core".to_string(), TernaryPackage {
            name: "ternlang-core".to_string(), version: "0.1.0".to_string(), compliance_score: 1.0, is_certified: true,
        });
        registry.packages.insert("ternlang-mkl".to_string(), TernaryPackage {
            name: "ternlang-mkl".to_string(), version: "0.1.0".to_string(), compliance_score: 1.0, is_certified: true,
        });
        
        registry
    }

    /// Gatekeeping the registry: Reject packages that use binary fallback hacks.
    pub fn publish_package(&mut self, pkg: TernaryPackage) -> Result<(), String> {
        if pkg.compliance_score < 1.0 {
            return Err("REJECTED: Package fails TFP-754 Triadic Compliance Audit. Binary fallback detected.".to_string());
        }
        println!("ternpkg: Successfully published '{}' to the global RFI-IRFOS registry.", pkg.name);
        self.packages.insert(pkg.name.clone(), pkg);
        Ok(())
    }
}

fn main() {
    println!("=== RFI-IRFOS Ternpkg Registry ===");
    println!("Starting global package daemon on port 8080...");
    
    let mut registry = Registry::new();
    
    // Simulating an external dev trying to publish non-compliant code
    let bad_pkg = TernaryPackage {
        name: "binary-hack-lib".to_string(),
        version: "1.0.0".to_string(),
        compliance_score: 0.85, // Failed the 0-state audit
        is_certified: false,
    };
    
    match registry.publish_package(bad_pkg) {
        Ok(_) => {},
        Err(e) => println!("Audit Daemon: {}", e),
    }
}
