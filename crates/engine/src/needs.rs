//! NPC needs simulator for utility AI
//! 
//! Implements basic needs (thirst, hunger, sleep) that drive goal-directed behavior.
//! NPCs seek to satisfy needs through world interaction (find water, hunt food, find bed).

use anyhow::Result;

/// Basic need types for NPCs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NeedType {
    Thirst,
    Hunger,
    Sleep,
}

/// Need state for a single need
#[derive(Debug, Clone)]
pub struct Need {
    pub need_type: NeedType,
    /// Current value (0.0 = fully satisfied, 1.0 = critical)
    pub value: f32,
    /// Rate of decay per second
    pub decay_rate: f32,
}

impl Need {
    pub fn new(need_type: NeedType) -> Self {
        let decay_rate = match need_type {
            NeedType::Thirst => 0.01,  // 100s to critical
            NeedType::Hunger => 0.005, // 200s to critical
            NeedType::Sleep => 0.003,  // 333s to critical
        };
        
        Self {
            need_type,
            value: 0.0,
            decay_rate,
        }
    }
    
    /// Update need over time (increases toward 1.0)
    pub fn update(&mut self, dt: f32) {
        self.value = (self.value + self.decay_rate * dt).min(1.0);
    }
    
    /// Satisfy need (reduce value)
    pub fn satisfy(&mut self, amount: f32) {
        self.value = (self.value - amount).max(0.0);
    }
    
    /// Check if need is critical (requires immediate attention)
    pub fn is_critical(&self) -> bool {
        self.value > 0.8
    }
    
    /// Get urgency score (0.0 = no urgency, 1.0 = critical)
    pub fn urgency(&self) -> f32 {
        self.value
    }
}

/// NPC needs collection
#[derive(Debug, Clone)]
pub struct Needs {
    pub thirst: Need,
    pub hunger: Need,
    pub sleep: Need,
}

impl Needs {
    pub fn new() -> Self {
        Self {
            thirst: Need::new(NeedType::Thirst),
            hunger: Need::new(NeedType::Hunger),
            sleep: Need::new(NeedType::Sleep),
        }
    }
    
    /// Update all needs
    pub fn update(&mut self, dt: f32) {
        self.thirst.update(dt);
        self.hunger.update(dt);
        self.sleep.update(dt);
    }
    
    /// Get most urgent need (highest value)
    pub fn most_urgent(&self) -> &Need {
        let mut max_need = &self.thirst;
        
        if self.hunger.value > max_need.value {
            max_need = &self.hunger;
        }
        if self.sleep.value > max_need.value {
            max_need = &self.sleep;
        }
        
        max_need
    }
    
    /// Check if any need is critical
    pub fn has_critical_need(&self) -> bool {
        self.thirst.is_critical() || self.hunger.is_critical() || self.sleep.is_critical()
    }
}

impl Default for Needs {
    fn default() -> Self {
        Self::new()
    }
}

/// Goal type driven by needs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoalType {
    FindWater,
    FindFood,
    FindBed,
    Idle,
}

/// Goal selector for utility AI
pub struct GoalSelector {
    /// Thresholds for goal activation
    pub thirst_threshold: f32,
    pub hunger_threshold: f32,
    pub sleep_threshold: f32,
}

impl GoalSelector {
    pub fn new() -> Self {
        Self {
            thirst_threshold: 0.6,
            hunger_threshold: 0.5,
            sleep_threshold: 0.7,
        }
    }
    
    /// Select highest-priority goal based on needs
    pub fn select_goal(&self, needs: &Needs) -> GoalType {
        // Critical needs always take priority
        if needs.thirst.is_critical() {
            return GoalType::FindWater;
        }
        if needs.hunger.is_critical() {
            return GoalType::FindFood;
        }
        if needs.sleep.is_critical() {
            return GoalType::FindBed;
        }
        
        // Check thresholds
        if needs.thirst.value > self.thirst_threshold {
            return GoalType::FindWater;
        }
        if needs.hunger.value > self.hunger_threshold {
            return GoalType::FindFood;
        }
        if needs.sleep.value > self.sleep_threshold {
            return GoalType::FindBed;
        }
        
        GoalType::Idle
    }
}

impl Default for GoalSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn need_decay() {
        let mut need = Need::new(NeedType::Thirst);
        assert_eq!(need.value, 0.0);
        
        // Update for 50 seconds
        for _ in 0..50 {
            need.update(1.0);
        }
        
        // Thirst decays at 0.01/s, so 50s → 0.5
        assert!((need.value - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn need_satisfaction() {
        let mut need = Need::new(NeedType::Hunger);
        need.value = 0.8;
        
        need.satisfy(0.5);
        assert_eq!(need.value, 0.3);
        
        need.satisfy(1.0); // Should clamp to 0.0
        assert_eq!(need.value, 0.0);
    }
    
    #[test]
    fn most_urgent_need() {
        let mut needs = Needs::new();
        needs.thirst.value = 0.3;
        needs.hunger.value = 0.7;
        needs.sleep.value = 0.2;
        
        let urgent = needs.most_urgent();
        assert_eq!(urgent.need_type, NeedType::Hunger);
    }
    
    #[test]
    fn goal_selection() {
        let selector = GoalSelector::new();
        let mut needs = Needs::new();
        
        // All satisfied → Idle
        assert_eq!(selector.select_goal(&needs), GoalType::Idle);
        
        // Thirst critical → FindWater
        needs.thirst.value = 0.85;
        assert_eq!(selector.select_goal(&needs), GoalType::FindWater);
        
        // Reset thirst, make hunger threshold → FindFood
        needs.thirst.value = 0.3;
        needs.hunger.value = 0.6;
        assert_eq!(selector.select_goal(&needs), GoalType::FindFood);
    }
}
