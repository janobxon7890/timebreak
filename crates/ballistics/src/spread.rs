use crate::weapon::WeaponDefinition;
use rand::Rng;
use timebreak_core::PlayerStance;

#[derive(Debug, Clone)]
pub struct SpreadSimulator;

impl SpreadSimulator {
    /// Calculate the spread/inaccuracy radius in virtual pixels/angles based on stance and velocity.
    pub fn calculate_inaccuracy_radius(
        weapon: &WeaponDefinition,
        stance: PlayerStance,
        current_speed: f32,
    ) -> f32 {
        let base_stance_inaccuracy = match stance {
            PlayerStance::Crouched | PlayerStance::Crouching => weapon.inaccuracy_crouch,
            PlayerStance::Airborne | PlayerStance::Jumping => weapon.inaccuracy_air,
            PlayerStance::Landing => weapon.inaccuracy_jump * 0.75,
            _ => weapon.inaccuracy_stand,
        };

        // Movement penalty increases with current speed / max speed
        let speed_ratio = if weapon.max_player_speed > 0.0 {
            (current_speed / weapon.max_player_speed).min(1.5)
        } else {
            0.0
        };

        let move_penalty = if speed_ratio > 0.34 {
            // CS2 threshold: above 34% velocity, moving inaccuracy applies aggressively
            weapon.inaccuracy_move * (speed_ratio - 0.34)
        } else {
            0.0
        };

        weapon.base_spread + base_stance_inaccuracy + move_penalty
    }

    /// Generates a deterministic (spread_x, spread_y) sample within the inaccuracy cone.
    pub fn sample_spread<R: Rng>(radius: f32, rng: &mut R) -> (f32, f32) {
        if radius <= 0.001 {
            return (0.0, 0.0);
        }

        // Uniform disk sampling
        let r = radius * rng.gen::<f32>().sqrt();
        let theta = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;

        (r * theta.cos(), r * theta.sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_spread_penalty_ordering() {
        let weapon = WeaponDefinition::ak47();

        let inacc_crouch =
            SpreadSimulator::calculate_inaccuracy_radius(&weapon, PlayerStance::Crouched, 0.0);
        let inacc_stand =
            SpreadSimulator::calculate_inaccuracy_radius(&weapon, PlayerStance::Standing, 0.0);
        let inacc_move =
            SpreadSimulator::calculate_inaccuracy_radius(&weapon, PlayerStance::Running, 215.0);
        let inacc_air =
            SpreadSimulator::calculate_inaccuracy_radius(&weapon, PlayerStance::Airborne, 100.0);

        assert!(inacc_crouch < inacc_stand);
        assert!(inacc_stand < inacc_move);
        assert!(inacc_stand < inacc_air);
    }

    #[test]
    fn test_seeded_spread_determinism() {
        let mut rng1 = ChaCha8Rng::seed_from_u64(42);
        let mut rng2 = ChaCha8Rng::seed_from_u64(42);

        let (x1, y1) = SpreadSimulator::sample_spread(10.0, &mut rng1);
        let (x2, y2) = SpreadSimulator::sample_spread(10.0, &mut rng2);

        assert_eq!(x1, x2);
        assert_eq!(y1, y2);
    }
}
