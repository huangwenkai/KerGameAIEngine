//! Audio system (headless-safe)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sound effect ID
pub type SoundId = u32;

/// Audio event (for logging/tracking)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEvent {
    pub sound_id: SoundId,
    pub volume: f32,
    pub timestamp_ms: u64,
}

/// Headless audio backend (no actual audio playback)
pub struct AudioSystem {
    events: Vec<AudioEvent>,
    sound_registry: HashMap<SoundId, String>,
    next_id: SoundId,
    tick_counter: u64,
}

impl AudioSystem {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            sound_registry: HashMap::new(),
            next_id: 1,
            tick_counter: 0,
        }
    }

    pub fn register_sound(&mut self, name: &str) -> SoundId {
        let id = self.next_id;
        self.next_id += 1;
        self.sound_registry.insert(id, name.to_string());
        id
    }

    pub fn play(&mut self, sound_id: SoundId, volume: f32) {
        self.events.push(AudioEvent {
            sound_id,
            volume,
            timestamp_ms: self.tick_counter * 16, // ~60fps
        });
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn sound_count(&self) -> usize {
        self.sound_registry.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_registration() {
        let mut audio = AudioSystem::new();
        let id = audio.register_sound("explosion");
        assert!(id > 0);
        assert_eq!(audio.sound_count(), 1);
    }

    #[test]
    fn audio_playback() {
        let mut audio = AudioSystem::new();
        let id = audio.register_sound("jump");
        
        audio.play(id, 0.8);
        assert_eq!(audio.event_count(), 1);
    }

    #[test]
    fn audio_tick() {
        let mut audio = AudioSystem::new();
        audio.tick();
        audio.tick();
        assert_eq!(audio.tick_counter, 2);
    }
}
