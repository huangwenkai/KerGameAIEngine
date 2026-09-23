//! Seeded RNG for deterministic gameplay

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Deterministic RNG wrapper
pub struct GameRng {
    rng: ChaCha8Rng,
}

impl GameRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Generate random f32 in [0.0, 1.0)
    pub fn gen_f32(&mut self) -> f32 {
        self.rng.gen()
    }

    /// Generate random u32
    pub fn gen_u32(&mut self) -> u32 {
        self.rng.gen()
    }

    /// Generate random value in range
    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
        R: rand::distributions::uniform::SampleRange<T>,
    {
        self.rng.gen_range(range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_rng() {
        let mut rng1 = GameRng::new(42);
        let mut rng2 = GameRng::new(42);

        for _ in 0..100 {
            assert_eq!(rng1.gen_u32(), rng2.gen_u32());
        }
    }

    #[test]
    fn different_seeds() {
        let mut rng1 = GameRng::new(42);
        let mut rng2 = GameRng::new(123);

        let val1 = rng1.gen_u32();
        let val2 = rng2.gen_u32();

        assert_ne!(val1, val2);
    }
}
