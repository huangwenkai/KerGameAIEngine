//! Item system: definitions, inventory, drop/pickup

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Item ID
pub type ItemId = u32;

/// Item type categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Material,
    Tool,
}

/// Item rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn color_rgba(self) -> [u8; 4] {
        match self {
            Rarity::Common => [200, 200, 200, 255],
            Rarity::Uncommon => [100, 255, 100, 255],
            Rarity::Rare => [100, 100, 255, 255],
            Rarity::Epic => [200, 100, 255, 255],
            Rarity::Legendary => [255, 165, 0, 255],
        }
    }
}

/// Item definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDef {
    pub id: ItemId,
    pub name: String,
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub max_stack: u32,
    pub damage: Option<i32>,
    pub armor: Option<i32>,
    pub description: String,
}

impl ItemDef {
    pub fn new_weapon(id: ItemId, name: &str, damage: i32, rarity: Rarity) -> Self {
        Self {
            id,
            name: name.to_string(),
            item_type: ItemType::Weapon,
            rarity,
            max_stack: 1,
            damage: Some(damage),
            armor: None,
            description: format!("A weapon that deals {} damage", damage),
        }
    }

    pub fn new_armor(id: ItemId, name: &str, armor: i32, rarity: Rarity) -> Self {
        Self {
            id,
            name: name.to_string(),
            item_type: ItemType::Armor,
            rarity,
            max_stack: 1,
            damage: None,
            armor: Some(armor),
            description: format!("Armor providing {} defense", armor),
        }
    }

    pub fn new_consumable(id: ItemId, name: &str, rarity: Rarity, max_stack: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            item_type: ItemType::Consumable,
            rarity,
            max_stack,
            damage: None,
            armor: None,
            description: "A consumable item".to_string(),
        }
    }

    pub fn new_material(id: ItemId, name: &str, max_stack: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            item_type: ItemType::Material,
            rarity: Rarity::Common,
            max_stack,
            damage: None,
            armor: None,
            description: "Crafting material".to_string(),
        }
    }
}

/// Item instance (with count)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub def_id: ItemId,
    pub count: u32,
}

impl ItemStack {
    pub fn new(def_id: ItemId, count: u32) -> Self {
        Self { def_id, count }
    }
}

/// Dropped item in world
#[derive(Debug, Clone)]
pub struct DroppedItem {
    pub x: f32,
    pub y: f32,
    pub stack: ItemStack,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

/// Inventory system
pub struct Inventory {
    slots: Vec<Option<ItemStack>>,
    capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
            capacity,
        }
    }

    pub fn add_item(&mut self, stack: ItemStack, registry: &ItemRegistry) -> Result<(), String> {
        let def = registry.get(stack.def_id).ok_or("Item not found")?;

        // Try to stack with existing items
        if def.max_stack > 1 {
            for slot in &mut self.slots {
                if let Some(existing) = slot {
                    if existing.def_id == stack.def_id {
                        let space = def.max_stack.saturating_sub(existing.count);
                        if space > 0 {
                            let to_add = space.min(stack.count);
                            existing.count += to_add;
                            if to_add >= stack.count {
                                return Ok(());
                            }
                            // Continue with remainder
                        }
                    }
                }
            }
        }

        // Find empty slot
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(stack);
                return Ok(());
            }
        }

        Err("Inventory full".to_string())
    }

    pub fn remove_item(&mut self, slot_index: usize, count: u32) -> Option<ItemStack> {
        if slot_index >= self.capacity {
            return None;
        }

        if let Some(stack) = &mut self.slots[slot_index] {
            if stack.count <= count {
                self.slots[slot_index].take()
            } else {
                stack.count -= count;
                Some(ItemStack::new(stack.def_id, count))
            }
        } else {
            None
        }
    }

    pub fn get_slot(&self, index: usize) -> Option<&ItemStack> {
        if index < self.capacity {
            self.slots[index].as_ref()
        } else {
            None
        }
    }

    pub fn slot_count(&self) -> usize {
        self.capacity
    }

    pub fn item_count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }
}

/// Item registry (database of all item definitions)
pub struct ItemRegistry {
    items: HashMap<ItemId, ItemDef>,
    next_id: ItemId,
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn register(&mut self, def: ItemDef) -> ItemId {
        let id = def.id;
        self.items.insert(id, def);
        id
    }

    pub fn generate_id(&mut self) -> ItemId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn get(&self, id: ItemId) -> Option<&ItemDef> {
        self.items.get(&id)
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

/// World item manager (dropped items in world)
pub struct WorldItems {
    items: Vec<DroppedItem>,
}

impl WorldItems {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn drop_item(&mut self, x: f32, y: f32, stack: ItemStack) {
        self.items.push(DroppedItem {
            x,
            y,
            stack,
            velocity_x: 0.0,
            velocity_y: 0.0,
        });
    }

    pub fn update(&mut self, dt: f32) {
        for item in &mut self.items {
            // Simple physics
            item.velocity_y += 400.0 * dt; // Gravity
            item.x += item.velocity_x * dt;
            item.y += item.velocity_y * dt;

            // Ground at y=100 (simplified)
            if item.y > 400.0 {
                item.y = 400.0;
                item.velocity_y = 0.0;
                item.velocity_x *= 0.9; // Friction
            }
        }
    }

    pub fn pickup_near(&mut self, x: f32, y: f32, radius: f32) -> Option<ItemStack> {
        let radius_sq = radius * radius;
        if let Some(index) = self.items.iter().position(|item| {
            let dx = item.x - x;
            let dy = item.y - y;
            dx * dx + dy * dy < radius_sq
        }) {
            Some(self.items.remove(index).stack)
        } else {
            None
        }
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_def_creation() {
        let weapon = ItemDef::new_weapon(1, "Sword", 10, Rarity::Common);
        assert_eq!(weapon.item_type, ItemType::Weapon);
        assert_eq!(weapon.damage, Some(10));
    }

    #[test]
    fn inventory_add_item() {
        let mut inv = Inventory::new(10);
        let mut registry = ItemRegistry::new();
        
        let sword_id = registry.generate_id();
        registry.register(ItemDef::new_weapon(sword_id, "Sword", 10, Rarity::Common));

        let stack = ItemStack::new(sword_id, 1);
        assert!(inv.add_item(stack, &registry).is_ok());
        assert_eq!(inv.item_count(), 1);
    }

    #[test]
    fn inventory_stacking() {
        let mut inv = Inventory::new(10);
        let mut registry = ItemRegistry::new();
        
        let potion_id = registry.generate_id();
        registry.register(ItemDef::new_consumable(potion_id, "Potion", Rarity::Common, 99));

        inv.add_item(ItemStack::new(potion_id, 10), &registry).unwrap();
        inv.add_item(ItemStack::new(potion_id, 20), &registry).unwrap();

        // Should stack into one slot
        assert_eq!(inv.item_count(), 1);
        assert_eq!(inv.get_slot(0).unwrap().count, 30);
    }

    #[test]
    fn inventory_full() {
        let mut inv = Inventory::new(2);
        let mut registry = ItemRegistry::new();
        
        let sword1 = registry.generate_id();
        let sword2 = registry.generate_id();
        let sword3 = registry.generate_id();
        
        registry.register(ItemDef::new_weapon(sword1, "Sword1", 10, Rarity::Common));
        registry.register(ItemDef::new_weapon(sword2, "Sword2", 12, Rarity::Common));
        registry.register(ItemDef::new_weapon(sword3, "Sword3", 14, Rarity::Common));

        assert!(inv.add_item(ItemStack::new(sword1, 1), &registry).is_ok());
        assert!(inv.add_item(ItemStack::new(sword2, 1), &registry).is_ok());
        assert!(inv.add_item(ItemStack::new(sword3, 1), &registry).is_err());
    }

    #[test]
    fn world_items_drop_and_pickup() {
        let mut world = WorldItems::new();
        
        world.drop_item(10.0, 20.0, ItemStack::new(1, 5));
        assert_eq!(world.item_count(), 1);

        let picked = world.pickup_near(12.0, 22.0, 5.0);
        assert!(picked.is_some());
        assert_eq!(picked.unwrap().count, 5);
        assert_eq!(world.item_count(), 0);
    }

    #[test]
    fn world_items_physics() {
        let mut world = WorldItems::new();
        world.drop_item(10.0, 0.0, ItemStack::new(1, 1));

        world.update(0.1);
        assert!(world.items[0].y > 0.0, "Item should fall");
        assert!(world.items[0].velocity_y > 0.0, "Should have downward velocity");
    }

    #[test]
    fn rarity_colors() {
        assert_eq!(Rarity::Common.color_rgba()[0], 200);
        assert_eq!(Rarity::Legendary.color_rgba()[0], 255);
    }
}
