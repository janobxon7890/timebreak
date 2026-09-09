use crate::telemetry::{DirectionBias, SessionMetrics, ShotTelemetryEvent};

pub struct MetricsCalculator;

impl MetricsCalculator {
    pub fn compute_session_metrics(
        session_id: String,
        weapon_id: String,
        start_time_ms: u64,
        end_time_ms: u64,
        shots: &[ShotTelemetryEvent],
    ) -> SessionMetrics {
        let shots_fired = shots.len() as u32;
        if shots_fired == 0 {
            return SessionMetrics {
                session_id,
                start_time_ms,
                end_time_ms,
                duration_seconds: ((end_time_ms - start_time_ms) / 1000) as u32,
                weapon_id,
                shots_fired: 0,
                hits: 0,
                misses: 0,
                accuracy: 0.0,
                headshots: 0,
                headshot_percentage: 0.0,
                kills: 0,
                avg_reaction_time_ms: 0.0,
                median_reaction_time_ms: 0.0,
                p90_reaction_time_ms: 0.0,
                mean_horizontal_error: 0.0,
                mean_vertical_error: 0.0,
                rms_error: 0.0,
                direction_bias: DirectionBias::Center,
                movement_score: 100.0,
                spray_score: 100.0,
                overall_score: 0.0,
                moving_shots_percentage: 0.0,
            };
        }

        let mut hits = 0u32;
        let mut headshots = 0u32;
        let mut kills = 0u32;
        let mut reaction_times: Vec<f32> = Vec::new();

        let mut sum_err_x = 0.0f32;
        let mut sum_err_y = 0.0f32;
        let mut sum_squared_radial_err = 0.0f32;
        let mut valid_error_count = 0u32;

        let mut moving_shots = 0u32;

        for shot in shots {
            if shot.hit {
                hits += 1;
            }
            if shot.headshot {
                headshots += 1;
                kills += 1; // standard headshot kill in trainer
            } else if shot.hit && shot.ammo_before % 3 == 0 {
                kills += 1; // simulate multishot kill
            }

            if let Some(rt) = shot.reaction_time_ms {
                reaction_times.push(rt as f32);
            }

            if let (Some(ex), Some(ey)) = (shot.aim_error_x, shot.aim_error_y) {
                sum_err_x += ex;
                sum_err_y += ey;
                sum_squared_radial_err += ex * ex + ey * ey;
                valid_error_count += 1;
            }

            if shot.player_speed > 50.0 {
                moving_shots += 1;
            }
        }

        let misses = shots_fired - hits;
        let accuracy = (hits as f32 / shots_fired as f32) * 100.0;
        let headshot_percentage = if hits > 0 {
            (headshots as f32 / hits as f32) * 100.0
        } else {
            0.0
        };

        // Reaction times
        reaction_times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let avg_reaction_time_ms = if !reaction_times.is_empty() {
            reaction_times.iter().sum::<f32>() / reaction_times.len() as f32
        } else {
            0.0
        };

        let median_reaction_time_ms = if !reaction_times.is_empty() {
            let mid = reaction_times.len() / 2;
            reaction_times[mid]
        } else {
            0.0
        };

        let p90_reaction_time_ms = if !reaction_times.is_empty() {
            let idx = ((reaction_times.len() as f32 * 0.90).floor() as usize)
                .min(reaction_times.len() - 1);
            reaction_times[idx]
        } else {
            0.0
        };

        // Error calculations
        let mean_horizontal_error = if valid_error_count > 0 {
            sum_err_x / valid_error_count as f32
        } else {
            0.0
        };

        let mean_vertical_error = if valid_error_count > 0 {
            sum_err_y / valid_error_count as f32
        } else {
            0.0
        };

        let rms_error = if valid_error_count > 0 {
            (sum_squared_radial_err / valid_error_count as f32).sqrt()
        } else {
            0.0
        };

        // Direction bias determination (threshold of 8.0 units)
        let tau = 8.0f32;
        let is_left = mean_horizontal_error < -tau;
        let is_right = mean_horizontal_error > tau;
        let is_high = mean_vertical_error < -tau; // Inverted screen Y: -y is upwards
        let is_low = mean_vertical_error > tau;

        let direction_bias = match (is_high, is_low, is_left, is_right) {
            (true, false, true, false) => DirectionBias::HighLeft,
            (true, false, false, true) => DirectionBias::HighRight,
            (true, false, false, false) => DirectionBias::High,
            (false, true, true, false) => DirectionBias::LowLeft,
            (false, true, false, true) => DirectionBias::LowRight,
            (false, true, false, false) => DirectionBias::Low,
            (false, false, true, false) => DirectionBias::Left,
            (false, false, false, true) => DirectionBias::Right,
            _ => DirectionBias::Center,
        };

        let moving_shots_percentage = (moving_shots as f32 / shots_fired as f32) * 100.0;
        let movement_score = (100.0 - moving_shots_percentage).clamp(0.0, 100.0);

        // Spray score: penalties for high RMS error and misses
        let spray_score = (100.0 - (rms_error * 0.8)).clamp(10.0, 100.0);

        let overall_score =
            (accuracy * 0.4 + headshot_percentage * 0.2 + movement_score * 0.2 + spray_score * 0.2)
                .clamp(0.0, 100.0);

        SessionMetrics {
            session_id,
            start_time_ms,
            end_time_ms,
            duration_seconds: ((end_time_ms - start_time_ms) / 1000) as u32,
            weapon_id,
            shots_fired,
            hits,
            misses,
            accuracy,
            headshots,
            headshot_percentage,
            kills,
            avg_reaction_time_ms,
            median_reaction_time_ms,
            p90_reaction_time_ms,
            mean_horizontal_error,
            mean_vertical_error,
            rms_error,
            direction_bias,
            movement_score,
            spray_score,
            overall_score,
            moving_shots_percentage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use timebreak_core::PlayerStance;

    #[test]
    fn test_synthetic_right_bias_detection() {
        let mut shots = Vec::new();
        for i in 0..100 {
            shots.push(ShotTelemetryEvent {
                shot_id: format!("shot_{}", i),
                session_id: "s1".to_string(),
                timestamp_ms: 1000 + i * 100,
                weapon_id: "weapon_ak47".to_string(),
                shot_index: 0,
                ammo_before: 30,
                target_id: Some("t1".to_string()),
                target_pose: None,
                target_visible_fraction: None,
                aim_x: 120.0,
                aim_y: 100.0,
                target_center_x: Some(100.0),
                target_center_y: Some(100.0),
                head_center_x: Some(100.0),
                head_center_y: Some(80.0),
                aim_error_x: Some(20.0), // Intentionally 20 units to the right
                aim_error_y: Some(0.0),
                aim_error_distance: Some(20.0),
                recoil_x: 0.0,
                recoil_y: 0.0,
                spread_x: 0.0,
                spread_y: 0.0,
                final_shot_x: 120.0,
                final_shot_y: 100.0,
                player_velocity_x: 0.0,
                player_velocity_y: 0.0,
                player_speed: 0.0,
                stance: PlayerStance::Standing,
                airborne: false,
                scoped: false,
                hit: true,
                hitbox: None,
                headshot: false,
                reaction_time_ms: Some(240),
                screen_id: "main".to_string(),
            });
        }

        let metrics = MetricsCalculator::compute_session_metrics(
            "s1".to_string(),
            "weapon_ak47".to_string(),
            1000,
            11000,
            &shots,
        );

        assert_eq!(metrics.direction_bias, DirectionBias::Right);
        assert!((metrics.mean_horizontal_error - 20.0).abs() < 0.001);
    }
}
