use crate::weapon::WeaponDefinition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoilState {
    pub shot_index: usize,
    pub current_recoil_x: f32,
    pub current_recoil_y: f32,
    pub last_shot_time_ms: u64,
}

impl Default for RecoilState {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoilState {
    pub fn new() -> Self {
        Self {
            shot_index: 0,
            current_recoil_x: 0.0,
            current_recoil_y: 0.0,
            last_shot_time_ms: 0,
        }
    }

    pub fn advance_shot(&mut self, weapon: &WeaponDefinition, current_time_ms: u64) -> (f32, f32) {
        let pattern_len = weapon.recoil_pattern.len();
        if pattern_len == 0 {
            return (0.0, 0.0);
        }

        let idx = self.shot_index.min(pattern_len - 1);
        let point = &weapon.recoil_pattern[idx];

        self.current_recoil_x = point.dx;
        self.current_recoil_y = point.dy;
        self.shot_index += 1;
        self.last_shot_time_ms = current_time_ms;

        (self.current_recoil_x, self.current_recoil_y)
    }

    pub fn update_recovery(
        &mut self,
        weapon: &WeaponDefinition,
        current_time_ms: u64,
        is_crouching: bool,
    ) {
        if self.shot_index == 0 {
            return;
        }

        let recovery_time = if is_crouching {
            weapon.recovery_time_crouch_ms
        } else {
            weapon.recovery_time_stand_ms
        } as f32;

        let elapsed = current_time_ms.saturating_sub(self.last_shot_time_ms) as f32;
        if elapsed >= recovery_time {
            self.shot_index = 0;
            self.current_recoil_x = 0.0;
            self.current_recoil_y = 0.0;
        } else {
            // Decay recoil proportional to elapsed time
            let remaining_ratio = (1.0 - elapsed / recovery_time).clamp(0.0, 1.0);
            self.current_recoil_x *= remaining_ratio;
            self.current_recoil_y *= remaining_ratio;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ak47_recoil_progression_and_recovery() {
        let weapon = WeaponDefinition::ak47();
        let mut recoil = RecoilState::new();

        // Shot 1
        let (rx0, ry0) = recoil.advance_shot(&weapon, 1000);
        assert_eq!(rx0, 0.0);
        assert_eq!(ry0, 0.0);

        // Shot 2
        let (rx1, ry1) = recoil.advance_shot(&weapon, 1100);
        assert_eq!(rx1, 0.0);
        assert_eq!(ry1, -4.0);

        // Shot 3
        let (rx2, ry2) = recoil.advance_shot(&weapon, 1200);
        assert_eq!(rx2, 0.5);
        assert_eq!(ry2, -9.0);

        // Test recovery after delay
        recoil.update_recovery(&weapon, 1200 + 500, false);
        assert_eq!(recoil.shot_index, 0);
        assert_eq!(recoil.current_recoil_y, 0.0);
    }
}
