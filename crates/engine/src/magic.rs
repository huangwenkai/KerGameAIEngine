//! Magic system: spells, mana, combos

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Spell element type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Water,
    Earth,
    Air,
    Light,
    Dark,
}

impl Element {
    pub fn color_rgba(self) -> [u8; 4] {
        match self {
            Element::Fire => [255, 100, 0, 255],
            Element::Water => [0, 100, 255, 255],
            Element::Earth => [139, 90, 43, 255],
            Element::Air => [200, 200, 255, 255],
            Element::Light => [255, 255, 200, 255],
            Element::Dark => [100, 0, 100, 255],
        }
    }
}

/// Spell component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellComponent {
    pub element: Element,
    pub power: i32,
    pub mana_cost: i32,
}

/// Spell combo result
#[derive(Debug, Clone)]
pub struct SpellCombo {
    pub name: String,
    pub elements: Vec<Element>,
    pub total_power: i32,
    pub total_cost: i32,
    pub effect_type: SpellEffect,
}

/// Spell effect type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellEffect {
    Projectile,
    AoE,
    Buff,
    Debuff,
    Heal,
    Summon,
}

/// Mana pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mana {
    pub current: i32,
    pub max: i32,
    pub regen_rate: i32,
}

impl Mana {
    pub fn new(max: i32, regen_rate: i32) -> Self {
        Self {
            current: max,
            max,
            regen_rate,
        }
    }

    pub fn regenerate(&mut self) {
        self.current = (self.current + self.regen_rate).min(self.max);
    }

    pub fn spend(&mut self, amount: i32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    pub fn percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

/// Spell crafting system
pub struct SpellCrafter {
    /// Known combo recipes
    recipes: HashMap<Vec<Element>, String>,
}

impl SpellCrafter {
    pub fn new() -> Self {
        let mut recipes = HashMap::new();
        
        // Basic single-element spells
        recipes.insert(vec![Element::Fire], "Fireball".to_string());
        recipes.insert(vec![Element::Water], "Water Jet".to_string());
        recipes.insert(vec![Element::Earth], "Stone Spike".to_string());
        recipes.insert(vec![Element::Air], "Wind Blast".to_string());
        
        // Two-element combos
        recipes.insert(vec![Element::Fire, Element::Air], "Inferno".to_string());
        recipes.insert(vec![Element::Water, Element::Air], "Ice Storm".to_string());
        recipes.insert(vec![Element::Fire, Element::Earth], "Lava Burst".to_string());
        recipes.insert(vec![Element::Water, Element::Earth], "Mud Wall".to_string());
        recipes.insert(vec![Element::Light, Element::Fire], "Holy Fire".to_string());
        recipes.insert(vec![Element::Dark, Element::Water], "Poison Cloud".to_string());
        
        // Three-element combos
        recipes.insert(vec![Element::Fire, Element::Water, Element::Earth], "Steam Explosion".to_string());
        recipes.insert(vec![Element::Light, Element::Air, Element::Fire], "Meteor".to_string());
        
        Self { recipes }
    }

    pub fn craft_spell(&self, components: &[SpellComponent]) -> Option<SpellCombo> {
        if components.is_empty() {
            return None;
        }

        let elements: Vec<_> = components.iter().map(|c| c.element).collect();
        let total_power: i32 = components.iter().map(|c| c.power).sum();
        let total_cost: i32 = components.iter().map(|c| c.mana_cost).sum();

        // Try to find recipe (try all permutations for simplicity)
        let name = self.find_recipe(&elements)?;

        let effect_type = self.determine_effect(&elements, total_power);

        Some(SpellCombo {
            name,
            elements,
            total_power,
            total_cost,
            effect_type,
        })
    }

    fn find_recipe(&self, elements: &[Element]) -> Option<String> {
        // Try exact match
        if let Some(name) = self.recipes.get(elements) {
            return Some(name.clone());
        }

        // Try sorted match
        let mut sorted = elements.to_vec();
        sorted.sort_by_key(|e| format!("{:?}", e));
        self.recipes.get(&sorted).cloned()
    }

    fn determine_effect(&self, elements: &[Element], power: i32) -> SpellEffect {
        if elements.contains(&Element::Light) && elements.contains(&Element::Water) {
            SpellEffect::Heal
        } else if elements.len() >= 3 {
            SpellEffect::AoE
        } else if power > 50 {
            SpellEffect::AoE
        } else {
            SpellEffect::Projectile
        }
    }

    pub fn recipe_count(&self) -> usize {
        self.recipes.len()
    }
}

/// Spellcaster with mana
#[derive(Debug, Clone)]
pub struct Spellcaster {
    pub mana: Mana,
    pub spell_budget: usize, // Max spells per tick for safety
}

impl Spellcaster {
    pub fn new(max_mana: i32, regen_rate: i32, spell_budget: usize) -> Self {
        Self {
            mana: Mana::new(max_mana, regen_rate),
            spell_budget,
        }
    }

    pub fn can_cast(&self, cost: i32) -> bool {
        self.mana.current >= cost
    }

    pub fn cast_spell(&mut self, cost: i32) -> bool {
        self.mana.spend(cost)
    }

    pub fn update(&mut self) {
        self.mana.regenerate();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mana_regeneration() {
        let mut mana = Mana::new(100, 10);
        mana.current = 50;
        
        mana.regenerate();
        assert_eq!(mana.current, 60);
        
        for _ in 0..5 {
            mana.regenerate();
        }
        assert_eq!(mana.current, 100);
    }

    #[test]
    fn mana_spending() {
        let mut mana = Mana::new(100, 10);
        
        assert!(mana.spend(30));
        assert_eq!(mana.current, 70);
        
        assert!(!mana.spend(80));
        assert_eq!(mana.current, 70);
    }

    #[test]
    fn single_element_spell() {
        let crafter = SpellCrafter::new();
        let components = vec![
            SpellComponent { element: Element::Fire, power: 20, mana_cost: 10 }
        ];
        
        let spell = crafter.craft_spell(&components).unwrap();
        assert_eq!(spell.name, "Fireball");
        assert_eq!(spell.total_power, 20);
        assert_eq!(spell.total_cost, 10);
    }

    #[test]
    fn combo_spell() {
        let crafter = SpellCrafter::new();
        let components = vec![
            SpellComponent { element: Element::Fire, power: 15, mana_cost: 8 },
            SpellComponent { element: Element::Air, power: 12, mana_cost: 7 },
        ];
        
        let spell = crafter.craft_spell(&components).unwrap();
        assert_eq!(spell.name, "Inferno");
        assert_eq!(spell.total_power, 27);
        assert_eq!(spell.total_cost, 15);
    }

    #[test]
    fn element_colors() {
        assert_eq!(Element::Fire.color_rgba()[0], 255);
        assert_eq!(Element::Water.color_rgba()[2], 255);
    }

    #[test]
    fn spellcaster_casting() {
        let mut caster = Spellcaster::new(100, 5, 10);
        
        assert!(caster.can_cast(30));
        assert!(caster.cast_spell(30));
        assert_eq!(caster.mana.current, 70);
        
        assert!(!caster.can_cast(80));
    }

    #[test]
    fn spellcaster_update() {
        let mut caster = Spellcaster::new(100, 5, 10);
        caster.mana.current = 50;
        
        caster.update();
        assert_eq!(caster.mana.current, 55);
    }

    #[test]
    fn spell_effects() {
        let crafter = SpellCrafter::new();
        
        // High power = AoE
        let high_power = vec![
            SpellComponent { element: Element::Fire, power: 60, mana_cost: 30 }
        ];
        let spell = crafter.craft_spell(&high_power).unwrap();
        assert_eq!(spell.effect_type, SpellEffect::AoE);
    }
}
