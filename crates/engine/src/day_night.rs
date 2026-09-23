//! Day/night cycle and time-of-day lighting

/// Day/night cycle state
#[derive(Debug, Clone)]
pub struct DayNightCycle {
    /// Current time in seconds (0 = midnight)
    pub time: f32,
    /// Day length in seconds
    pub day_length: f32,
    /// Enable cycle
    pub enabled: bool,
}

impl DayNightCycle {
    pub fn new(day_length: f32) -> Self {
        Self {
            time: day_length * 0.25, // Start at dawn
            day_length,
            enabled: true,
        }
    }
    
    /// Update time
    pub fn update(&mut self, dt: f32) {
        if self.enabled {
            self.time += dt;
            if self.time >= self.day_length {
                self.time -= self.day_length;
            }
        }
    }
    
    /// Get time of day [0.0, 1.0] where 0=midnight, 0.25=dawn, 0.5=noon, 0.75=dusk
    pub fn time_of_day(&self) -> f32 {
        (self.time / self.day_length).fract()
    }
    
    /// Get ambient light level [0, 255]
    pub fn ambient_light(&self) -> u8 {
        let tod = self.time_of_day();
        
        // Dawn (0.2-0.3): 50→255
        // Day (0.3-0.7): 255
        // Dusk (0.7-0.8): 255→50
        // Night (0.8-0.2): 50
        
        if tod < 0.2 {
            // Night
            50
        } else if tod < 0.3 {
            // Dawn
            let t = (tod - 0.2) / 0.1;
            (50.0 + t * 205.0) as u8
        } else if tod < 0.7 {
            // Day
            255
        } else if tod < 0.8 {
            // Dusk
            let t = (tod - 0.7) / 0.1;
            (255.0 - t * 205.0) as u8
        } else {
            // Night
            50
        }
    }
    
    /// Get clear color for sky rendering [r, g, b, a]
    pub fn sky_color(&self) -> [f32; 4] {
        let tod = self.time_of_day();
        
        if tod < 0.2 {
            // Night: dark blue
            [0.05, 0.05, 0.15, 1.0]
        } else if tod < 0.3 {
            // Dawn: dark blue → orange
            let t = (tod - 0.2) / 0.1;
            [
                0.05 + t * 0.75,
                0.05 + t * 0.35,
                0.15,
                1.0
            ]
        } else if tod < 0.5 {
            // Morning: orange → sky blue
            let t = (tod - 0.3) / 0.2;
            [
                0.8 - t * 0.3,
                0.4 + t * 0.3,
                0.15 + t * 0.65,
                1.0
            ]
        } else if tod < 0.7 {
            // Day: sky blue
            [0.5, 0.7, 0.8, 1.0]
        } else if tod < 0.8 {
            // Dusk: sky blue → orange
            let t = (tod - 0.7) / 0.1;
            [
                0.5 + t * 0.3,
                0.7 - t * 0.3,
                0.8 - t * 0.65,
                1.0
            ]
        } else {
            // Evening → night: orange → dark blue
            let t = (tod - 0.8) / 0.2;
            [
                0.8 - t * 0.75,
                0.4 - t * 0.35,
                0.15,
                1.0
            ]
        }
    }
    
    /// Get phase name
    pub fn phase_name(&self) -> &'static str {
        let tod = self.time_of_day();
        if tod < 0.2 {
            "Night"
        } else if tod < 0.3 {
            "Dawn"
        } else if tod < 0.5 {
            "Morning"
        } else if tod < 0.7 {
            "Day"
        } else if tod < 0.8 {
            "Dusk"
        } else {
            "Evening"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day_night_cycle_advances() {
        let mut cycle = DayNightCycle::new(120.0);
        cycle.time = 0.0;
        
        assert_eq!(cycle.ambient_light(), 50); // Midnight
        
        cycle.update(30.0); // Dawn
        assert!(cycle.ambient_light() > 50);
        
        cycle.update(30.0); // Day
        assert_eq!(cycle.ambient_light(), 255);
    }
    
    #[test]
    fn day_night_phases() {
        let mut cycle = DayNightCycle::new(100.0);
        
        cycle.time = 0.0;
        assert_eq!(cycle.phase_name(), "Night");
        
        cycle.time = 25.0;
        assert_eq!(cycle.phase_name(), "Dawn");
        
        cycle.time = 50.0;
        assert_eq!(cycle.phase_name(), "Day");
        
        cycle.time = 75.0;
        assert_eq!(cycle.phase_name(), "Dusk");
    }
}
