use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub source: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillRegistry {
    pub skills: HashMap<String, Skill>,
}

impl SkillRegistry {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("albert");
        path.push("skills.json");
        path
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(SkillRegistry::default());
        }
        let contents = fs::read_to_string(&path)?;
        let registry = serde_json::from_str(&contents)?;
        Ok(registry)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(&path, contents)?;
        Ok(())
    }

    pub fn teach_skill(
        &mut self,
        name: String,
        source_path: String,
        description: Option<String>,
    ) -> Result<(), String> {
        if name.is_empty() {
            return Err("skill name cannot be empty".to_string());
        }
        if source_path.is_empty() {
            return Err("source path cannot be empty".to_string());
        }

        let path = PathBuf::from(&source_path);
        if !path.exists() {
            return Err(format!("source file not found: {}", source_path));
        }

        let source = fs::read_to_string(&path)
            .map_err(|e| format!("failed to read source: {}", e))?;

        let skill = Skill {
            name: name.clone(),
            source,
            description,
            created_at: chrono::Local::now().to_rfc3339(),
        };
        self.skills.insert(name, skill);
        Ok(())
    }

    pub fn delete_skill(&mut self, name: &str) -> Result<(), String> {
        if self.skills.remove(name).is_none() {
            return Err(format!("skill '{}' not found", name));
        }
        Ok(())
    }

    pub fn get_skill(&self, name: &str) -> Option<&Skill> {
        self.skills.get(name)
    }

    pub fn list_skills(&self) -> Vec<&Skill> {
        let mut skills: Vec<_> = self.skills.values().collect();
        skills.sort_by_key(|s| &s.created_at);
        skills
    }
}
