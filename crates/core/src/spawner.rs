use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::target::{TargetBehaviour, TargetEntity, TargetPose};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnerConfig {
    pub min_lifetime_ms: u64,
    pub max_lifetime_ms: u64,
    pub spawn_interval_ms: u64,
    pub max_concurrent_targets: usize,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl Default for SpawnerConfig {
    fn default() -> Self {
        Self {
            min_lifetime_ms: 1800,
            max_lifetime_ms: 3200,
            spawn_interval_ms: 1200,
            max_concurrent_targets: 3,
            screen_width: 1920.0,
            screen_height: 1080.0,
        }
    }
}

pub struct TargetSpawner {
    config: SpawnerConfig,
    last_spawn_ms: u64,
    target_counter: u64,
}

impl TargetSpawner {
    pub fn new(config: SpawnerConfig) -> Self {
        Self {
            config,
            last_spawn_ms: 0,
            target_counter: 0,
        }
    }

    pub fn maybe_spawn<R: Rng>(
        &mut self,
        current_time_ms: u64,
        active_target_count: usize,
        rng: &mut R,
    ) -> Option<TargetEntity> {
        if active_target_count >= self.config.max_concurrent_targets {
            return None;
        }

        if current_time_ms < self.last_spawn_ms + self.config.spawn_interval_ms {
            return None;
        }

        self.last_spawn_ms = current_time_ms;
        self.target_counter += 1;

        let id = format!("target_{}", self.target_counter);

        // Controlled random pose
        let pose_roll = rng.gen_range(0..100);
        let pose = if pose_roll < 50 {
            TargetPose::Standing
        } else if pose_roll < 75 {
            TargetPose::Crouched
        } else if pose_roll < 88 {
            TargetPose::HeadShoulder
        } else {
            TargetPose::HeadOnly
        };

        // Controlled random behaviour
        let beh_roll = rng.gen_range(0..100);
        let behaviour = if beh_roll < 55 {
            TargetBehaviour::Static
        } else if beh_roll < 80 {
            TargetBehaviour::Strafe
        } else {
            TargetBehaviour::PopUp
        };

        let lifetime_ms = rng.gen_range(self.config.min_lifetime_ms..=self.config.max_lifetime_ms);

        // Spawn within safe margins of screen bounds (avoid edges / menu bars)
        let margin_x = 120.0;
        let margin_top = 80.0;
        let margin_bottom = 160.0;

        let x = rng.gen_range(margin_x..(self.config.screen_width - margin_x - 60.0));
        let y = rng.gen_range(margin_top..(self.config.screen_height - margin_bottom - 140.0));

        Some(TargetEntity::new_standard(
            id,
            x,
            y,
            pose,
            behaviour,
            current_time_ms,
            lifetime_ms,
            "main".to_string(),
        ))
    }
}
