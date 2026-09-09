use crate::telemetry::{DirectionBias, SessionMetrics};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub session_id: String,
    pub title: String,
    pub observation: String,
    pub evidence: String,
    pub likely_cause: String,
    pub suggested_drill: String,
    pub suggested_duration_minutes: u32,
}

pub struct Recommender;

impl Recommender {
    pub fn generate_recommendation(metrics: &SessionMetrics) -> Recommendation {
        // Condition 1: High moving shots percentage -> counter-strafe recommendation
        if metrics.moving_shots_percentage > 25.0 {
            return Recommendation {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: metrics.session_id.clone(),
                title: "Haraktda otish / Counter-Strafe kamchiligi".to_string(),
                observation: format!(
                    "{:.1}% o'qlaringiz to'liq to'xtashdan oldin otilgan.",
                    metrics.moving_shots_percentage
                ),
                evidence: format!(
                    "Harakat intizomi bali: {:.0}/100. Tezlik pasaymasdan otilgan o'qlar aniqlikni keskin tushirgan.",
                    metrics.movement_score
                ),
                likely_cause: "Qarama-qarshi tugmani bosib tormozlash (counter-strafe) kechikishi.".to_string(),
                suggested_drill: "counter_strafe".to_string(),
                suggested_duration_minutes: 3,
            };
        }

        // Condition 2: Directional vertical bias (shooting too low)
        if metrics.direction_bias == DirectionBias::Low
            || metrics.direction_bias == DirectionBias::LowLeft
            || metrics.direction_bias == DirectionBias::LowRight
        {
            return Recommendation {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: metrics.session_id.clone(),
                title: "Nishon balandligi past / Crosshair Placement".to_string(),
                observation: "O'qlaringiz nishon markazidan pastga og'moqda.".to_string(),
                evidence: format!(
                    "O'rtacha vertikal og'ish: +{:.1} px (pastga). Boshga tegish: {:.1}%.",
                    metrics.mean_vertical_error, metrics.headshot_percentage
                ),
                likely_cause: "Krossxayrni bosh balandligidan pastroqda ushlab turish odati."
                    .to_string(),
                suggested_drill: "headshot".to_string(),
                suggested_duration_minutes: 3,
            };
        }

        // Condition 3: Directional vertical bias (shooting too high)
        if metrics.direction_bias == DirectionBias::High
            || metrics.direction_bias == DirectionBias::HighLeft
            || metrics.direction_bias == DirectionBias::HighRight
        {
            return Recommendation {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: metrics.session_id.clone(),
                title: "Otish paytida qaytish (recoil) kompensatsiyasi kam".to_string(),
                observation: "O'qlaringiz yuqoriga uchib ketmoqda.".to_string(),
                evidence: format!(
                    "O'rtacha vertikal og'ish: {:.1} px (tepaga).",
                    metrics.mean_vertical_error
                ),
                likely_cause: "Sprey paytida sichqonchani yetarli darajada pastga tortmaslik."
                    .to_string(),
                suggested_drill: "spray_control".to_string(),
                suggested_duration_minutes: 4,
            };
        }

        // Condition 4: High reaction time but good accuracy -> speed up
        if metrics.avg_reaction_time_ms > 450.0 && metrics.accuracy > 65.0 {
            return Recommendation {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: metrics.session_id.clone(),
                title: "Reaksiya tezligini oshirish mashqi".to_string(),
                observation: "Aniqlik yuqori, lekin birinchi o'qqa bo'lgan reaksiya sekinroq."
                    .to_string(),
                evidence: format!(
                    "O'rtacha reaksiya: {:.0} ms. P90: {:.0} ms.",
                    metrics.avg_reaction_time_ms, metrics.p90_reaction_time_ms
                ),
                likely_cause: "Ortiqcha nishonga olish vaqti sarflanmoqda.".to_string(),
                suggested_drill: "reaction".to_string(),
                suggested_duration_minutes: 3,
            };
        }

        // Default: balanced flick & quick break
        Recommendation {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: metrics.session_id.clone(),
            title: "Ajoyib natija! Quick Break bilan davom eting".to_string(),
            observation: format!(
                "Aniqlik {:.1}%, umumiy ball {:.0}/100.",
                metrics.accuracy, metrics.overall_score
            ),
            evidence: format!(
                "O'rtacha xatolik: {:.1} px. Barqaror nishon nazorati.",
                metrics.rms_error
            ),
            likely_cause: "Barqaror mushak xotirasi va to'g'ri krossxayr joylashuvi.".to_string(),
            suggested_drill: "quick_break".to_string(),
            suggested_duration_minutes: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_strafe_recommendation() {
        let metrics = SessionMetrics {
            session_id: "s_test".to_string(),
            start_time_ms: 0,
            end_time_ms: 10000,
            duration_seconds: 10,
            weapon_id: "weapon_ak47".to_string(),
            shots_fired: 20,
            hits: 10,
            misses: 10,
            accuracy: 50.0,
            headshots: 2,
            headshot_percentage: 20.0,
            kills: 2,
            avg_reaction_time_ms: 300.0,
            median_reaction_time_ms: 290.0,
            p90_reaction_time_ms: 350.0,
            mean_horizontal_error: 0.0,
            mean_vertical_error: 0.0,
            rms_error: 12.0,
            direction_bias: DirectionBias::Center,
            movement_score: 60.0,
            spray_score: 75.0,
            overall_score: 60.0,
            moving_shots_percentage: 40.0, // 40% moving shots
        };

        let rec = Recommender::generate_recommendation(&metrics);
        assert_eq!(rec.suggested_drill, "counter_strafe");
    }
}
