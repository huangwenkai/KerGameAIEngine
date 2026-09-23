//! Replay determinism tests

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn replay_determinism_same_seed() {
        // Run 1
        let config = EngineConfig {
            seed: 42,
            headless: true,
            ..Default::default()
        };
        let mut engine1 = Engine::new(config);

        // Execute command sequence
        for i in 0..100 {
            let x = engine1.rng.gen_range(0.0..1000.0);
            let y = engine1.rng.gen_range(0.0..1000.0);
            engine1.queue_command(commands::Command::SpawnEntity { x, y });
        }
        
        for _ in 0..10 {
            engine1.tick().unwrap();
        }

        let hash1 = engine1.replay_hash();

        // Run 2 - identical seed, should produce identical result
        let config = EngineConfig {
            seed: 42,
            headless: true,
            ..Default::default()
        };
        let mut engine2 = Engine::new(config);

        for i in 0..100 {
            let x = engine2.rng.gen_range(0.0..1000.0);
            let y = engine2.rng.gen_range(0.0..1000.0);
            engine2.queue_command(commands::Command::SpawnEntity { x, y });
        }
        
        for _ in 0..10 {
            engine2.tick().unwrap();
        }

        let hash2 = engine2.replay_hash();

        assert_eq!(hash1, hash2, "Same seed should produce identical replay hash");
    }

    #[test]
    fn replay_determinism_different_seed() {
        let config1 = EngineConfig {
            seed: 42,
            headless: true,
            ..Default::default()
        };
        let mut engine1 = Engine::new(config1);

        for _ in 0..50 {
            let x = engine1.rng.gen_range(0.0..1000.0);
            let y = engine1.rng.gen_range(0.0..1000.0);
            engine1.queue_command(commands::Command::SpawnEntity { x, y });
        }
        
        for _ in 0..10 {
            engine1.tick().unwrap();
        }

        let hash1 = engine1.replay_hash();

        let config2 = EngineConfig {
            seed: 999,
            headless: true,
            ..Default::default()
        };
        let mut engine2 = Engine::new(config2);

        for _ in 0..50 {
            let x = engine2.rng.gen_range(0.0..1000.0);
            let y = engine2.rng.gen_range(0.0..1000.0);
            engine2.queue_command(commands::Command::SpawnEntity { x, y });
        }
        
        for _ in 0..10 {
            engine2.tick().unwrap();
        }

        let hash2 = engine2.replay_hash();

        assert_ne!(hash1, hash2, "Different seeds should produce different replay hashes");
    }

    #[test]
    fn command_execution_order() {
        let config = EngineConfig {
            seed: 0,
            headless: true,
            ..Default::default()
        };
        let mut engine = Engine::new(config);

        // Spawn entity
        engine.queue_command(commands::Command::SpawnEntity { x: 0.0, y: 0.0 });
        engine.tick().unwrap();

        assert_eq!(engine.world.entity_count(), 1);

        // Set velocity
        engine.queue_command(commands::Command::SetVelocity {
            entity_id: 1,
            vx: 10.0,
            vy: 5.0,
        });
        engine.tick().unwrap();

        let entity = engine.world.get_entity(1).unwrap();
        assert_eq!(entity.vx, 10.0);
        assert_eq!(entity.vy, 5.0);

        // After physics update, entity should have moved
        let dt = engine.config.fixed_timestep.as_secs_f32();
        let expected_x = 10.0 * dt;
        let expected_y = 5.0 * dt;
        
        assert!((entity.x - expected_x).abs() < 0.001);
        assert!((entity.y - expected_y).abs() < 0.001);
    }
}
