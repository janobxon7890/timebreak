use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitboxType {
    Head,
    UpperChest,
    LowerChest,
    Stomach,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitboxRect {
    pub hitbox_type: HitboxType,
    pub rel_x: f32,
    pub rel_y: f32,
    pub width: f32,
    pub height: f32,
    pub damage_multiplier: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetPose {
    Standing,
    Crouched,
    HeadOnly,
    HeadShoulder,
    HalfBodyLeft,
    HalfBodyRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetBehaviour {
    Static,
    PopUp,
    Strafe,
    PeekLeft,
    PeekRight,
    Reaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetEntity {
    pub id: String,
    pub spawn_time_ms: u64,
    pub lifetime_ms: u64,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub width: f32,
    pub height: f32,
    pub pose: TargetPose,
    pub behaviour: TargetBehaviour,
    pub hitboxes: Vec<HitboxRect>,
    pub visible_fraction: f32,
    pub is_alive: bool,
    pub screen_id: String,
}

impl TargetEntity {
    #[allow(clippy::too_many_arguments)]
    pub fn new_standard(
        id: String,
        x: f32,
        y: f32,
        pose: TargetPose,
        behaviour: TargetBehaviour,
        spawn_time_ms: u64,
        lifetime_ms: u64,
        screen_id: String,
    ) -> Self {
        let width = 60.0;
        let height = match pose {
            TargetPose::Standing => 140.0,
            TargetPose::Crouched => 95.0,
            TargetPose::HeadOnly => 35.0,
            TargetPose::HeadShoulder => 55.0,
            TargetPose::HalfBodyLeft | TargetPose::HalfBodyRight => 110.0,
        };

        let hitboxes = Self::generate_hitboxes(pose, width, height);

        let (vx, vy) = match behaviour {
            TargetBehaviour::Strafe => (80.0, 0.0),
            TargetBehaviour::PeekLeft => (-120.0, 0.0),
            TargetBehaviour::PeekRight => (120.0, 0.0),
            _ => (0.0, 0.0),
        };

        Self {
            id,
            spawn_time_ms,
            lifetime_ms,
            x,
            y,
            vx,
            vy,
            width,
            height,
            pose,
            behaviour,
            hitboxes,
            visible_fraction: 1.0,
            is_alive: true,
            screen_id,
        }
    }

    fn generate_hitboxes(pose: TargetPose, width: f32, height: f32) -> Vec<HitboxRect> {
        match pose {
            TargetPose::HeadOnly => {
                vec![HitboxRect {
                    hitbox_type: HitboxType::Head,
                    rel_x: 0.0,
                    rel_y: 0.0,
                    width,
                    height,
                    damage_multiplier: 4.0,
                }]
            }
            TargetPose::HeadShoulder => {
                vec![
                    HitboxRect {
                        hitbox_type: HitboxType::Head,
                        rel_x: width * 0.2,
                        rel_y: 0.0,
                        width: width * 0.6,
                        height: height * 0.5,
                        damage_multiplier: 4.0,
                    },
                    HitboxRect {
                        hitbox_type: HitboxType::UpperChest,
                        rel_x: 0.0,
                        rel_y: height * 0.5,
                        width,
                        height: height * 0.5,
                        damage_multiplier: 1.0,
                    },
                ]
            }
            TargetPose::Standing | TargetPose::HalfBodyLeft | TargetPose::HalfBodyRight => {
                vec![
                    // Head
                    HitboxRect {
                        hitbox_type: HitboxType::Head,
                        rel_x: width * 0.25,
                        rel_y: 0.0,
                        width: width * 0.5,
                        height: height * 0.22,
                        damage_multiplier: 4.0,
                    },
                    // Upper chest
                    HitboxRect {
                        hitbox_type: HitboxType::UpperChest,
                        rel_x: width * 0.15,
                        rel_y: height * 0.22,
                        width: width * 0.7,
                        height: height * 0.25,
                        damage_multiplier: 1.0,
                    },
                    // Stomach
                    HitboxRect {
                        hitbox_type: HitboxType::Stomach,
                        rel_x: width * 0.2,
                        rel_y: height * 0.47,
                        width: width * 0.6,
                        height: height * 0.2,
                        damage_multiplier: 1.25,
                    },
                    // Legs
                    HitboxRect {
                        hitbox_type: HitboxType::LeftLeg,
                        rel_x: width * 0.15,
                        rel_y: height * 0.67,
                        width: width * 0.3,
                        height: height * 0.33,
                        damage_multiplier: 0.75,
                    },
                    HitboxRect {
                        hitbox_type: HitboxType::RightLeg,
                        rel_x: width * 0.55,
                        rel_y: height * 0.67,
                        width: width * 0.3,
                        height: height * 0.33,
                        damage_multiplier: 0.75,
                    },
                ]
            }
            TargetPose::Crouched => {
                vec![
                    // Head
                    HitboxRect {
                        hitbox_type: HitboxType::Head,
                        rel_x: width * 0.22,
                        rel_y: 0.0,
                        width: width * 0.56,
                        height: height * 0.3,
                        damage_multiplier: 4.0,
                    },
                    // Chest / Stomach compressed
                    HitboxRect {
                        hitbox_type: HitboxType::UpperChest,
                        rel_x: width * 0.1,
                        rel_y: height * 0.3,
                        width: width * 0.8,
                        height: height * 0.4,
                        damage_multiplier: 1.0,
                    },
                    HitboxRect {
                        hitbox_type: HitboxType::LeftLeg,
                        rel_x: width * 0.1,
                        rel_y: height * 0.7,
                        width: width * 0.8,
                        height: height * 0.3,
                        damage_multiplier: 0.75,
                    },
                ]
            }
        }
    }

    pub fn update(&mut self, dt_secs: f32, bounds_width: f32) {
        self.x += self.vx * dt_secs;
        self.y += self.vy * dt_secs;

        // Bounce back inside screen boundaries if strafing
        if (self.x < 50.0 && self.vx < 0.0)
            || (self.x + self.width > bounds_width - 50.0 && self.vx > 0.0)
        {
            self.vx = -self.vx;
        }
    }

    pub fn check_hit(&self, shot_x: f32, shot_y: f32) -> Option<(HitboxType, f32)> {
        if !self.is_alive {
            return None;
        }

        // Test hitboxes in order of priority (head first)
        for hb in &self.hitboxes {
            let abs_x = self.x + hb.rel_x;
            let abs_y = self.y + hb.rel_y;

            if shot_x >= abs_x
                && shot_x <= abs_x + hb.width
                && shot_y >= abs_y
                && shot_y <= abs_y + hb.height
            {
                return Some((hb.hitbox_type, hb.damage_multiplier));
            }
        }

        None
    }

    pub fn head_center(&self) -> (f32, f32) {
        for hb in &self.hitboxes {
            if hb.hitbox_type == HitboxType::Head {
                return (
                    self.x + hb.rel_x + hb.width * 0.5,
                    self.y + hb.rel_y + hb.height * 0.5,
                );
            }
        }
        (self.x + self.width * 0.5, self.y + self.height * 0.2)
    }

    pub fn target_center(&self) -> (f32, f32) {
        (self.x + self.width * 0.5, self.y + self.height * 0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headshot_registration() {
        let target = TargetEntity::new_standard(
            "t1".into(),
            100.0,
            100.0,
            TargetPose::Standing,
            TargetBehaviour::Static,
            0,
            5000,
            "main".into(),
        );

        let (hx, hy) = target.head_center();
        let hit = target.check_hit(hx, hy);
        assert!(hit.is_some());
        let (ht, multiplier) = hit.unwrap();
        assert_eq!(ht, HitboxType::Head);
        assert_eq!(multiplier, 4.0);
    }

    #[test]
    fn test_body_and_miss_registration() {
        let target = TargetEntity::new_standard(
            "t1".into(),
            100.0,
            100.0,
            TargetPose::Standing,
            TargetBehaviour::Static,
            0,
            5000,
            "main".into(),
        );

        // Body hit
        let hit_body = target.check_hit(100.0 + 30.0, 100.0 + 50.0);
        assert!(hit_body.is_some());
        assert_ne!(hit_body.unwrap().0, HitboxType::Head);

        // Complete miss
        let hit_miss = target.check_hit(10.0, 10.0);
        assert!(hit_miss.is_none());
    }
}
