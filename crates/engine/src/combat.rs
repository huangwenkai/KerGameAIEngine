//! Combat system: damage, health, death, loot drops

use crate::items::{ItemStack, Rarity};
use crate::rng::GameRng;
use serde::{Deserialize, Serialize};

/// Combat stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStats {
    pub max_health: i32,
    pub current_health: i32,
    pub base_damage: i32,
    pub armor: i32,
    pub is_alive: bool,
}

impl CombatStats {
    pub fn new(max_health: i32, base_damage: i32, armor: i32) -> Self {
        Self {
            max_health,
            current_health: max_health,
            base_damage,
            armor,
            is_alive: true,
        }
    }

    pub fn take_damage(&mut self, raw_damage: i32) -> i32 {
        let mitigated = (raw_damage as f32 * (1.0 - self.armor as f32 / (self.armor + 100) as f32)) as i32;
        let actual = mitigated.max(1); // Minimum 1 damage
        self.current_health -= actual;
        
        if self.current_health <= 0 {
            self.current_health = 0;
            self.is_alive = false;
        }
        
        actual
    }

    pub fn heal(&mut self, amount: i32) {
        self.current_health = (self.current_health + amount).min(self.max_health);
    }

    pub fn health_percent(&self) -> f32 {
        self.current_health as f32 / self.max_health as f32
    }
}

/// Loot table entry
#[derive(Debug, Clone)]
pub struct LootEntry {
    pub item_id: u32,
    pub min_count: u32,
    pub max_count: u32,
    pub drop_chance: f32, // 0.0 to 1.0
}

/// Loot table for entity
#[derive(Debug, Clone)]
pub struct LootTable {
    entries: Vec<LootEntry>,
}

impl LootTable {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, item_id: u32, min_count: u32, max_count: u32, drop_chance: f32) {
        self.entries.push(LootEntry {
            item_id,
            min_count,
            max_count,
            drop_chance,
        });
    }

    pub fn roll_loot(&self, rng: &mut GameRng) -> Vec<ItemStack> {
        let mut loot = Vec::new();
        
        for entry in &self.entries {
            if rng.gen_f32() < entry.drop_chance {
                let count = if entry.max_count > entry.min_count {
                    rng.gen_range(entry.min_count..=entry.max_count)
                } else {
                    entry.min_count
                };
                loot.push(ItemStack::new(entry.item_id, count));
            }
        }
        
        loot
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

/// Entity with combat capability
#[derive(Debug, Clone)]
pub struct CombatEntity {
    pub id: u32,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub stats: CombatStats,
    pub loot_table: LootTable,
}

impl CombatEntity {
    pub fn new(id: u32, name: &str, x: f32, y: f32, health: i32, damage: i32, armor: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            x,
            y,
            stats: CombatStats::new(health, damage, armor),
            loot_table: LootTable::new(),
        }
    }

    pub fn attack(&self, target: &mut CombatEntity) -> i32 {
        target.stats.take_damage(self.stats.base_damage)
    }

    pub fn is_alive(&self) -> bool {
        self.stats.is_alive
    }
}

/// Combat system manager
pub struct CombatSystem {
    entities: Vec<CombatEntity>,
    next_id: u32,
}

impl CombatSystem {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 1,
        }
    }

    pub fn spawn_entity(&mut self, name: &str, x: f32, y: f32, health: i32, damage: i32, armor: i32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.push(CombatEntity::new(id, name, x, y, health, damage, armor));
        id
    }

    pub fn get_entity(&self, id: u32) -> Option<&CombatEntity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut CombatEntity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn attack(&mut self, attacker_id: u32, target_id: u32) -> Option<i32> {
        if attacker_id == target_id {
            return None;
        }

        // Get attacker damage
        let damage = self.get_entity(attacker_id)?.stats.base_damage;

        // Apply to target
        if let Some(target) = self.get_entity_mut(target_id) {
            Some(target.stats.take_damage(damage))
        } else {
            None
        }
    }

    pub fn remove_dead(&mut self) -> Vec<CombatEntity> {
        let mut dead = Vec::new();
        self.entities.retain(|e| {
            if !e.is_alive() {
                dead.push(e.clone());
                false
            } else {
                true
            }
        });
        dead
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn alive_count(&self) -> usize {
        self.entities.iter().filter(|e| e.is_alive()).count()
    }
}

/// Generate standard loot table based on rarity
pub fn generate_loot_table(rarity: Rarity, item_ids: &[u32]) -> LootTable {
    let mut table = LootTable::new();
    
    let (base_chance, count_mult) = match rarity {
        Rarity::Common => (0.8, 1),
        Rarity::Uncommon => (0.6, 2),
        Rarity::Rare => (0.4, 3),
        Rarity::Epic => (0.2, 5),
        Rarity::Legendary => (0.1, 10),
    };

    for &item_id in item_ids {
        table.add_entry(item_id, count_mult, count_mult * 2, base_chance);
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combat_stats_damage() {
        let mut stats = CombatStats::new(100, 10, 0);
        assert_eq!(stats.current_health, 100);
        
        let actual = stats.take_damage(30);
        assert!(actual >= 29 && actual <= 30); // Minimum 1 damage rule
        assert_eq!(stats.current_health, 70);
    }

    #[test]
    fn combat_stats_death() {
        let mut stats = CombatStats::new(100, 10, 0);
        stats.take_damage(150);
        
        assert_eq!(stats.current_health, 0);
        assert!(!stats.is_alive);
    }

    #[test]
    fn combat_stats_armor() {
        let mut stats = CombatStats::new(100, 10, 50);
        let actual = stats.take_damage(100);
        
        // 50 armor should mitigate ~33% damage
        assert!(actual < 70);
    }

    #[test]
    fn loot_table_rolls() {
        let mut table = LootTable::new();
        table.add_entry(1, 1, 5, 1.0); // Always drops
        table.add_entry(2, 1, 1, 0.0); // Never drops
        
        let mut rng = GameRng::new(42);
        let loot = table.roll_loot(&mut rng);
        
        assert!(loot.iter().any(|s| s.def_id == 1));
        assert!(!loot.iter().any(|s| s.def_id == 2));
    }

    #[test]
    fn combat_system_attack() {
        let mut system = CombatSystem::new();
        let player = system.spawn_entity("Player", 0.0, 0.0, 100, 20, 0);
        let enemy = system.spawn_entity("Enemy", 10.0, 0.0, 50, 10, 0);
        
        system.attack(player, enemy);
        
        let enemy_entity = system.get_entity(enemy).unwrap();
        assert_eq!(enemy_entity.stats.current_health, 30);
    }

    #[test]
    fn combat_system_death() {
        let mut system = CombatSystem::new();
        let player = system.spawn_entity("Player", 0.0, 0.0, 100, 60, 0);
        let enemy = system.spawn_entity("Enemy", 10.0, 0.0, 50, 10, 0);
        
        // Attack twice to kill
        system.attack(player, enemy);
        system.attack(player, enemy);
        
        let dead = system.remove_dead();
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0].name, "Enemy");
        assert_eq!(system.entity_count(), 1);
    }

    #[test]
    fn loot_generation() {
        let table = generate_loot_table(Rarity::Rare, &[1, 2, 3]);
        assert_eq!(table.entry_count(), 3);
    }
}
