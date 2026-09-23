//! Story/Quest/Event system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// World flags for quest/story state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldFlags {
    flags: HashMap<String, bool>,
    counters: HashMap<String, i32>,
}

impl WorldFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, flag: &str, value: bool) {
        self.flags.insert(flag.to_string(), value);
    }

    pub fn get(&self, flag: &str) -> bool {
        self.flags.get(flag).copied().unwrap_or(false)
    }

    pub fn increment(&mut self, counter: &str) {
        *self.counters.entry(counter.to_string()).or_insert(0) += 1;
    }

    pub fn get_counter(&self, counter: &str) -> i32 {
        self.counters.get(counter).copied().unwrap_or(0)
    }

    pub fn set_counter(&mut self, counter: &str, value: i32) {
        self.counters.insert(counter.to_string(), value);
    }
}

/// Quest state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestState {
    NotStarted,
    Active,
    Completed,
    Failed,
}

/// Simple quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub state: QuestState,
    pub required_flags: Vec<String>,
    pub reward_gold: i32,
}

impl Quest {
    pub fn new(id: &str, name: &str, description: &str, reward_gold: i32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            state: QuestState::NotStarted,
            required_flags: Vec::new(),
            reward_gold,
        }
    }

    pub fn can_start(&self, flags: &WorldFlags) -> bool {
        self.required_flags.iter().all(|f| flags.get(f))
    }

    pub fn start(&mut self) {
        if self.state == QuestState::NotStarted {
            self.state = QuestState::Active;
        }
    }

    pub fn complete(&mut self) {
        if self.state == QuestState::Active {
            self.state = QuestState::Completed;
        }
    }
}

/// Quest manager
pub struct QuestSystem {
    quests: Vec<Quest>,
}

impl QuestSystem {
    pub fn new() -> Self {
        Self { quests: Vec::new() }
    }

    pub fn add_quest(&mut self, quest: Quest) {
        self.quests.push(quest);
    }

    pub fn get_quest_mut(&mut self, id: &str) -> Option<&mut Quest> {
        self.quests.iter_mut().find(|q| q.id == id)
    }

    pub fn active_count(&self) -> usize {
        self.quests.iter().filter(|q| q.state == QuestState::Active).count()
    }

    pub fn completed_count(&self) -> usize {
        self.quests.iter().filter(|q| q.state == QuestState::Completed).count()
    }

    pub fn count(&self) -> usize {
        self.quests.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_flags() {
        let mut flags = WorldFlags::new();
        flags.set("tutorial_done", true);
        assert!(flags.get("tutorial_done"));
        assert!(!flags.get("boss_defeated"));
    }

    #[test]
    fn world_counters() {
        let mut flags = WorldFlags::new();
        flags.increment("enemies_killed");
        flags.increment("enemies_killed");
        assert_eq!(flags.get_counter("enemies_killed"), 2);
    }

    #[test]
    fn quest_lifecycle() {
        let mut quest = Quest::new("q1", "Kill 10 Rats", "Defeat 10 rats", 100);
        assert_eq!(quest.state, QuestState::NotStarted);
        
        quest.start();
        assert_eq!(quest.state, QuestState::Active);
        
        quest.complete();
        assert_eq!(quest.state, QuestState::Completed);
    }

    #[test]
    fn quest_requirements() {
        let mut flags = WorldFlags::new();
        let mut quest = Quest::new("q2", "Advanced Quest", "Requires flag", 200);
        quest.required_flags.push("tutorial_done".to_string());
        
        assert!(!quest.can_start(&flags));
        
        flags.set("tutorial_done", true);
        assert!(quest.can_start(&flags));
    }

    #[test]
    fn quest_system() {
        let mut system = QuestSystem::new();
        system.add_quest(Quest::new("q1", "Quest 1", "First quest", 100));
        
        assert_eq!(system.count(), 1);
        
        if let Some(quest) = system.get_quest_mut("q1") {
            quest.start();
        }
        
        assert_eq!(system.active_count(), 1);
    }
}
