use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CronTask {
    pub name: String,
    pub schedule: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CronRegistry {
    pub tasks: HashMap<String, CronTask>,
}

impl CronRegistry {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("albert");
        path.push("cron.json");
        path
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(CronRegistry::default());
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

    pub fn add_task(&mut self, name: String, schedule: String) -> Result<(), String> {
        if name.is_empty() {
            return Err("task name cannot be empty".to_string());
        }
        if schedule.is_empty() {
            return Err("schedule cannot be empty".to_string());
        }
        if !is_valid_cron(&schedule) {
            return Err(format!("invalid cron expression: {}", schedule));
        }

        let task = CronTask {
            name: name.clone(),
            schedule,
            created_at: chrono::Local::now().to_rfc3339(),
        };
        self.tasks.insert(name, task);
        Ok(())
    }

    pub fn remove_task(&mut self, name: &str) -> Result<(), String> {
        if self.tasks.remove(name).is_none() {
            return Err(format!("task '{}' not found", name));
        }
        Ok(())
    }

    pub fn list_tasks(&self) -> Vec<&CronTask> {
        let mut tasks: Vec<_> = self.tasks.values().collect();
        tasks.sort_by_key(|t| &t.created_at);
        tasks
    }
}

fn is_valid_cron(expr: &str) -> bool {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 5 {
        return false;
    }

    let field_ranges = [(0, 59), (0, 23), (1, 31), (1, 12), (0, 6)];
    for (i, &(min, max)) in field_ranges.iter().enumerate() {
        if !is_valid_cron_field(parts[i], min, max) {
            return false;
        }
    }
    true
}

fn is_valid_cron_field(field: &str, min: i32, max: i32) -> bool {
    if field == "*" {
        return true;
    }

    if field.contains(',') {
        return field.split(',').all(|f| is_valid_cron_field(f.trim(), min, max));
    }

    if field.contains('-') {
        let parts: Vec<&str> = field.split('-').collect();
        if parts.len() != 2 {
            return false;
        }
        if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            return start >= min && end <= max && start <= end;
        }
        return false;
    }

    if field.contains('/') {
        let parts: Vec<&str> = field.split('/').collect();
        if parts.len() != 2 {
            return false;
        }
        if parts[0] != "*" && parts[0].parse::<i32>().is_err() {
            return false;
        }
        return parts[1].parse::<i32>().is_ok();
    }

    if let Ok(num) = field.parse::<i32>() {
        num >= min && num <= max
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cron() {
        assert!(is_valid_cron("0 2 * * *"));
        assert!(is_valid_cron("*/5 * * * *"));
        assert!(is_valid_cron("0 0 1 1 0"));
        assert!(!is_valid_cron("0 0 32 * *"));
        assert!(!is_valid_cron("60 * * * *"));
        assert!(!is_valid_cron("0 24 * * *"));
    }
}
