//! Save/Load system

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

/// Minimal save data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub player_x: f32,
    pub player_y: f32,
    pub player_health: i32,
    pub seed: u64,
    pub tick_count: u64,
    pub flags: Vec<(String, bool)>,
}

impl SaveData {
    pub fn new(player_x: f32, player_y: f32, player_health: i32, seed: u64, tick_count: u64) -> Self {
        Self {
            player_x,
            player_y,
            player_health,
            seed,
            tick_count,
            flags: Vec::new(),
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let json = fs::read_to_string(path)?;
        let data = serde_json::from_str(&json)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_data_creation() {
        let save = SaveData::new(100.0, 200.0, 500, 42, 1000);
        assert_eq!(save.player_x, 100.0);
        assert_eq!(save.seed, 42);
    }

    #[test]
    fn save_load_roundtrip() {
        let save = SaveData::new(100.0, 200.0, 500, 42, 1000);
        let path = "/tmp/test_save.json";
        
        save.save_to_file(path).unwrap();
        let loaded = SaveData::load_from_file(path).unwrap();
        
        assert_eq!(loaded.player_x, save.player_x);
        assert_eq!(loaded.seed, save.seed);
        
        let _ = std::fs::remove_file(path);
    }
}
