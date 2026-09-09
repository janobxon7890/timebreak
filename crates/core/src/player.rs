use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerStance {
    Standing,
    Walking,
    Running,
    Crouching,
    Crouched,
    Jumping,
    Airborne,
    Landing,
    CounterStrafing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub max_speed: f32,
    pub stance: PlayerStance,
    pub is_crouching: bool,
    pub is_airborne: bool,
    pub time_since_landing_ms: f32,
    pub move_forward: bool,
    pub move_back: bool,
    pub move_left: bool,
    pub move_right: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new(250.0) // 250 units/s standard CS knife/pistol max speed
    }
}

impl PlayerState {
    pub fn new(max_speed: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            max_speed,
            stance: PlayerStance::Standing,
            is_crouching: false,
            is_airborne: false,
            time_since_landing_ms: 1000.0,
            move_forward: false,
            move_back: false,
            move_left: false,
            move_right: false,
        }
    }

    pub fn current_speed(&self) -> f32 {
        (self.vx * self.vx + self.vy * self.vy).sqrt()
    }

    pub fn update(&mut self, dt_secs: f32) {
        // Effective max speed depends on crouch
        let effective_max_speed = if self.is_crouching {
            self.max_speed * 0.34 // CS2 crouch speed ratio
        } else {
            self.max_speed
        };

        // Determine input direction
        let mut wish_x = 0.0f32;
        let mut wish_y = 0.0f32;

        if self.move_right {
            wish_x += 1.0;
        }
        if self.move_left {
            wish_x -= 1.0;
        }
        if self.move_back {
            wish_y += 1.0;
        }
        if self.move_forward {
            wish_y -= 1.0;
        }

        let wish_len = (wish_x * wish_x + wish_y * wish_y).sqrt();
        if wish_len > 0.001 {
            wish_x /= wish_len;
            wish_y /= wish_len;
        }

        // Acceleration and counter-strafing
        let acceleration = 5.5 * effective_max_speed; // CS2 sv_accelerate style
        let friction = 5.2;

        if wish_len > 0.001 {
            // Check for counter-strafing: opposite direction braking
            let dot = self.vx * wish_x + self.vy * wish_y;
            if dot < 0.0 {
                self.stance = PlayerStance::CounterStrafing;
                // Braking is much faster when counter-pressing
                self.vx += wish_x * acceleration * 2.5 * dt_secs;
                self.vy += wish_y * acceleration * 2.5 * dt_secs;
            } else {
                self.vx += wish_x * acceleration * dt_secs;
                self.vy += wish_y * acceleration * dt_secs;
            }
        } else {
            // Apply friction deceleration when no keys are held
            let speed = self.current_speed();
            if speed > 0.01 {
                let drop = speed * friction * dt_secs;
                let new_speed = (speed - drop).max(0.0);
                self.vx = (self.vx / speed) * new_speed;
                self.vy = (self.vy / speed) * new_speed;
            } else {
                self.vx = 0.0;
                self.vy = 0.0;
            }
        }

        // Clamp to max speed
        let speed = self.current_speed();
        if speed > effective_max_speed && effective_max_speed > 0.0 {
            self.vx = (self.vx / speed) * effective_max_speed;
            self.vy = (self.vy / speed) * effective_max_speed;
        }

        // Integrate position (virtual parallax space)
        self.x += self.vx * dt_secs;
        self.y += self.vy * dt_secs;

        // Update stance
        let final_speed = self.current_speed();
        if self.is_airborne {
            self.stance = PlayerStance::Airborne;
            self.time_since_landing_ms = 0.0;
        } else if self.is_crouching {
            if final_speed > 10.0 {
                self.stance = PlayerStance::Crouching;
            } else {
                self.stance = PlayerStance::Crouched;
            }
        } else if final_speed < 10.0 {
            self.stance = PlayerStance::Standing;
        } else if final_speed < self.max_speed * 0.52 {
            self.stance = PlayerStance::Walking;
        } else {
            self.stance = PlayerStance::Running;
        }

        if !self.is_airborne {
            self.time_since_landing_ms += dt_secs * 1000.0;
            if self.time_since_landing_ms < 180.0 {
                self.stance = PlayerStance::Landing;
            }
        }
    }

    pub fn jump(&mut self) {
        if !self.is_airborne {
            self.is_airborne = true;
            self.stance = PlayerStance::Jumping;
            self.time_since_landing_ms = 0.0;
        }
    }

    pub fn land(&mut self) {
        if self.is_airborne {
            self.is_airborne = false;
            self.stance = PlayerStance::Landing;
            self.time_since_landing_ms = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_strafing_settles_velocity_faster() {
        let mut player = PlayerState::new(250.0);
        player.move_right = true;

        // Run right for 0.5 seconds
        for _ in 0..30 {
            player.update(1.0 / 60.0);
        }
        let speed_moving_right = player.current_speed();
        assert!(speed_moving_right > 200.0);

        // Counter-strafe: release right, press left
        player.move_right = false;
        player.move_left = true;

        // Update 3 ticks (50ms)
        for _ in 0..3 {
            player.update(1.0 / 60.0);
        }

        // Velocity should rapidly plummet towards 0
        assert!(player.current_speed() < speed_moving_right * 0.5);
    }
}
