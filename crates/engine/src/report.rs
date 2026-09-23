//! Test report generation for demo runs

use serde::{Deserialize, Serialize};
use std::time::Duration;
use anyhow::Result;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct DemoReport {
    pub demo_id: String,
    pub seed: u64,
    pub success: bool,
    pub tick_count: u64,
    pub elapsed_real_ms: u64,
    pub elapsed_sim_ms: u64,
    pub replay_hash: String,
    pub message: String,
}

impl DemoReport {
    pub fn success(
        demo_id: String,
        seed: u64,
        tick_count: u64,
        elapsed_real: Duration,
        elapsed_sim: Duration,
        replay_hash: String,
    ) -> Self {
        Self {
            demo_id,
            seed,
            success: true,
            tick_count,
            elapsed_real_ms: elapsed_real.as_millis() as u64,
            elapsed_sim_ms: elapsed_sim.as_millis() as u64,
            replay_hash,
            message: "Demo completed successfully".to_string(),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        log::info!("Report saved to {}", path.display());
        Ok(())
    }
}
