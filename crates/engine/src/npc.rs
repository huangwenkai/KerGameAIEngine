//! NPC system: AI behaviors, pathing, spawning

use serde::{Deserialize, Serialize};

/// NPC behavior state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Behavior {
    Idle,
    Patrol,
    Chase,
    Flee,
    Attack,
}

/// Simple NPC entity
#[derive(Debug, Clone)]
pub struct NPC {
    pub id: u32,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub behavior: Behavior,
    pub patrol_points: Vec<(f32, f32)>,
    pub current_patrol: usize,
    pub speed: f32,
    pub detection_range: f32,
}

impl NPC {
    pub fn new(id: u32, name: &str, x: f32, y: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            x,
            y,
            behavior: Behavior::Idle,
            patrol_points: Vec::new(),
            current_patrol: 0,
            speed: 50.0,
            detection_range: 100.0,
        }
    }

    pub fn set_patrol(&mut self, points: Vec<(f32, f32)>) {
        self.patrol_points = points;
        self.behavior = Behavior::Patrol;
    }

    pub fn update(&mut self, dt: f32, player_x: f32, player_y: f32) {
        let dx = player_x - self.x;
        let dy = player_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        // Behavior transitions
        if dist < self.detection_range {
            self.behavior = Behavior::Chase;
        } else if self.behavior == Behavior::Chase {
            self.behavior = Behavior::Patrol;
        }

        // Execute behavior
        match self.behavior {
            Behavior::Idle => {}
            Behavior::Patrol => self.do_patrol(dt),
            Behavior::Chase => self.do_chase(dt, player_x, player_y),
            Behavior::Flee => self.do_flee(dt, player_x, player_y),
            Behavior::Attack => {}
        }
    }

    fn do_patrol(&mut self, dt: f32) {
        if self.patrol_points.is_empty() {
            return;
        }

        let (target_x, target_y) = self.patrol_points[self.current_patrol];
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < 5.0 {
            self.current_patrol = (self.current_patrol + 1) % self.patrol_points.len();
        } else {
            let nx = dx / dist;
            let ny = dy / dist;
            self.x += nx * self.speed * dt;
            self.y += ny * self.speed * dt;
        }
    }

    fn do_chase(&mut self, dt: f32, player_x: f32, player_y: f32) {
        let dx = player_x - self.x;
        let dy = player_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 0.1 {
            let nx = dx / dist;
            let ny = dy / dist;
            self.x += nx * self.speed * dt;
            self.y += ny * self.speed * dt;
        }
    }

    fn do_flee(&mut self, dt: f32, player_x: f32, player_y: f32) {
        let dx = self.x - player_x; // Opposite direction
        let dy = self.y - player_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 0.1 {
            let nx = dx / dist;
            let ny = dy / dist;
            self.x += nx * self.speed * dt;
            self.y += ny * self.speed * dt;
        }
    }
}

/// NPC manager
pub struct NPCSystem {
    npcs: Vec<NPC>,
    next_id: u32,
}

impl NPCSystem {
    pub fn new() -> Self {
        Self {
            npcs: Vec::new(),
            next_id: 1,
        }
    }

    pub fn spawn(&mut self, name: &str, x: f32, y: f32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.npcs.push(NPC::new(id, name, x, y));
        id
    }

    pub fn get_npc_mut(&mut self, id: u32) -> Option<&mut NPC> {
        self.npcs.iter_mut().find(|n| n.id == id)
    }

    pub fn update_all(&mut self, dt: f32, player_x: f32, player_y: f32) {
        for npc in &mut self.npcs {
            npc.update(dt, player_x, player_y);
        }
    }

    pub fn count(&self) -> usize {
        self.npcs.len()
    }

    pub fn count_by_behavior(&self, behavior: Behavior) -> usize {
        self.npcs.iter().filter(|n| n.behavior == behavior).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npc_creation() {
        let npc = NPC::new(1, "Guard", 10.0, 20.0);
        assert_eq!(npc.behavior, Behavior::Idle);
    }

    #[test]
    fn npc_patrol() {
        let mut npc = NPC::new(1, "Guard", 0.0, 0.0);
        npc.set_patrol(vec![(100.0, 0.0), (100.0, 100.0)]);
        
        assert_eq!(npc.behavior, Behavior::Patrol);
        npc.update(1.0, 1000.0, 1000.0); // Player far away
        assert!(npc.x > 0.0); // Should move toward patrol point
    }

    #[test]
    fn npc_chase() {
        let mut npc = NPC::new(1, "Enemy", 0.0, 0.0);
        npc.detection_range = 100.0;
        
        npc.update(1.0, 50.0, 0.0); // Player within range
        assert_eq!(npc.behavior, Behavior::Chase);
        assert!(npc.x > 0.0); // Should move toward player
    }

    #[test]
    fn npc_system() {
        let mut system = NPCSystem::new();
        let id = system.spawn("Guard", 0.0, 0.0);
        
        assert_eq!(system.count(), 1);
        assert!(system.get_npc_mut(id).is_some());
    }
}
