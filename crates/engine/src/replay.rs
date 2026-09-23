//! Replay hash verification for determinism testing

use sha2::{Sha256, Digest};
use crate::commands::Command;

pub struct ReplayHasher {
    hasher: Sha256,
    seed: u64,
}

impl ReplayHasher {
    pub fn new(seed: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(seed.to_le_bytes());
        
        Self { hasher, seed }
    }

    pub fn hash_command(&mut self, cmd: &Command) {
        let serialized = serde_json::to_vec(cmd).unwrap();
        self.hasher.update(&serialized);
    }

    pub fn finalize(&self) -> String {
        format!("{:x}", self.hasher.clone().finalize())
    }
}
