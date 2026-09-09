use rusqlite::{params, Connection, Result};
use std::path::Path;
use timebreak_analytics::{Recommendation, SessionMetrics, ShotTelemetryEvent};

pub struct StorageManager {
    conn: Connection,
}

impl StorageManager {
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let sm = Self { conn };
        sm.run_migrations()?;
        Ok(sm)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let sm = Self { conn };
        sm.run_migrations()?;
        Ok(sm)
    }

    fn run_migrations(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;

            CREATE TABLE IF NOT EXISTS sessions (
                session_id TEXT PRIMARY KEY,
                start_time_ms INTEGER NOT NULL,
                end_time_ms INTEGER NOT NULL,
                duration_seconds INTEGER NOT NULL,
                weapon_id TEXT NOT NULL,
                shots_fired INTEGER NOT NULL,
                hits INTEGER NOT NULL,
                misses INTEGER NOT NULL,
                accuracy REAL NOT NULL,
                headshots INTEGER NOT NULL,
                headshot_percentage REAL NOT NULL,
                kills INTEGER NOT NULL,
                avg_reaction_time_ms REAL NOT NULL,
                rms_error REAL NOT NULL,
                direction_bias TEXT NOT NULL,
                movement_score REAL NOT NULL,
                spray_score REAL NOT NULL,
                overall_score REAL NOT NULL,
                moving_shots_percentage REAL NOT NULL
            );

            CREATE TABLE IF NOT EXISTS shots (
                shot_id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                timestamp_ms INTEGER NOT NULL,
                weapon_id TEXT NOT NULL,
                shot_index INTEGER NOT NULL,
                aim_x REAL NOT NULL,
                aim_y REAL NOT NULL,
                final_shot_x REAL NOT NULL,
                final_shot_y REAL NOT NULL,
                hit INTEGER NOT NULL,
                headshot INTEGER NOT NULL,
                player_speed REAL NOT NULL,
                stance TEXT NOT NULL,
                reaction_time_ms INTEGER,
                FOREIGN KEY (session_id) REFERENCES sessions(session_id)
            );

            CREATE TABLE IF NOT EXISTS recommendations (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                title TEXT NOT NULL,
                observation TEXT NOT NULL,
                evidence TEXT NOT NULL,
                likely_cause TEXT NOT NULL,
                suggested_drill TEXT NOT NULL,
                suggested_duration_minutes INTEGER NOT NULL,
                FOREIGN KEY (session_id) REFERENCES sessions(session_id)
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_start_time ON sessions(start_time_ms);
            CREATE INDEX IF NOT EXISTS idx_shots_session ON shots(session_id);
            "#,
        )?;
        Ok(())
    }

    pub fn insert_session(
        &mut self,
        metrics: &SessionMetrics,
        rec: Option<&Recommendation>,
    ) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
            r#"
            INSERT INTO sessions (
                session_id, start_time_ms, end_time_ms, duration_seconds, weapon_id,
                shots_fired, hits, misses, accuracy, headshots, headshot_percentage,
                kills, avg_reaction_time_ms, rms_error, direction_bias,
                movement_score, spray_score, overall_score, moving_shots_percentage
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)
            "#,
            params![
                metrics.session_id,
                metrics.start_time_ms,
                metrics.end_time_ms,
                metrics.duration_seconds,
                metrics.weapon_id,
                metrics.shots_fired,
                metrics.hits,
                metrics.misses,
                metrics.accuracy,
                metrics.headshots,
                metrics.headshot_percentage,
                metrics.kills,
                metrics.avg_reaction_time_ms,
                metrics.rms_error,
                format!("{:?}", metrics.direction_bias),
                metrics.movement_score,
                metrics.spray_score,
                metrics.overall_score,
                metrics.moving_shots_percentage,
            ],
        )?;

        if let Some(r) = rec {
            tx.execute(
                r#"
                INSERT INTO recommendations (
                    id, session_id, title, observation, evidence, likely_cause,
                    suggested_drill, suggested_duration_minutes
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
                params![
                    r.id,
                    r.session_id,
                    r.title,
                    r.observation,
                    r.evidence,
                    r.likely_cause,
                    r.suggested_drill,
                    r.suggested_duration_minutes,
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn insert_shots_batch(&mut self, shots: &[ShotTelemetryEvent]) -> Result<()> {
        if shots.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare_cached(
                r#"
                INSERT INTO shots (
                    shot_id, session_id, timestamp_ms, weapon_id, shot_index,
                    aim_x, aim_y, final_shot_x, final_shot_y, hit, headshot,
                    player_speed, stance, reaction_time_ms
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
                "#,
            )?;

            for s in shots {
                stmt.execute(params![
                    s.shot_id,
                    s.session_id,
                    s.timestamp_ms,
                    s.weapon_id,
                    s.shot_index,
                    s.aim_x,
                    s.aim_y,
                    s.final_shot_x,
                    s.final_shot_y,
                    if s.hit { 1 } else { 0 },
                    if s.headshot { 1 } else { 0 },
                    s.player_speed,
                    format!("{:?}", s.stance),
                    s.reaction_time_ms,
                ])?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    pub fn get_recent_sessions(&self, limit: usize) -> Result<Vec<SessionMetrics>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT session_id, start_time_ms, end_time_ms, duration_seconds, weapon_id,
                   shots_fired, hits, misses, accuracy, headshots, headshot_percentage,
                   kills, avg_reaction_time_ms, rms_error, direction_bias,
                   movement_score, spray_score, overall_score, moving_shots_percentage
            FROM sessions
            ORDER BY start_time_ms DESC
            LIMIT ?1
            "#,
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            let bias_str: String = row.get(14)?;
            let direction_bias = match bias_str.as_str() {
                "High" => timebreak_analytics::DirectionBias::High,
                "Low" => timebreak_analytics::DirectionBias::Low,
                "Left" => timebreak_analytics::DirectionBias::Left,
                "Right" => timebreak_analytics::DirectionBias::Right,
                "HighLeft" => timebreak_analytics::DirectionBias::HighLeft,
                "HighRight" => timebreak_analytics::DirectionBias::HighRight,
                "LowLeft" => timebreak_analytics::DirectionBias::LowLeft,
                "LowRight" => timebreak_analytics::DirectionBias::LowRight,
                _ => timebreak_analytics::DirectionBias::Center,
            };

            Ok(SessionMetrics {
                session_id: row.get(0)?,
                start_time_ms: row.get(1)?,
                end_time_ms: row.get(2)?,
                duration_seconds: row.get(3)?,
                weapon_id: row.get(4)?,
                shots_fired: row.get(5)?,
                hits: row.get(6)?,
                misses: row.get(7)?,
                accuracy: row.get(8)?,
                headshots: row.get(9)?,
                headshot_percentage: row.get(10)?,
                kills: row.get(11)?,
                avg_reaction_time_ms: row.get(12)?,
                median_reaction_time_ms: row.get(12)?,
                p90_reaction_time_ms: row.get(12)?,
                mean_horizontal_error: 0.0,
                mean_vertical_error: 0.0,
                rms_error: row.get(13)?,
                direction_bias,
                movement_score: row.get(15)?,
                spray_score: row.get(16)?,
                overall_score: row.get(17)?,
                moving_shots_percentage: row.get(18)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_lifecycle() {
        let mut sm = StorageManager::open_in_memory().unwrap();

        let metrics = SessionMetrics {
            session_id: "s_test_1".to_string(),
            start_time_ms: 1000,
            end_time_ms: 6000,
            duration_seconds: 5,
            weapon_id: "weapon_ak47".to_string(),
            shots_fired: 10,
            hits: 8,
            misses: 2,
            accuracy: 80.0,
            headshots: 4,
            headshot_percentage: 50.0,
            kills: 4,
            avg_reaction_time_ms: 220.0,
            median_reaction_time_ms: 215.0,
            p90_reaction_time_ms: 260.0,
            mean_horizontal_error: 0.0,
            mean_vertical_error: 0.0,
            rms_error: 5.0,
            direction_bias: timebreak_analytics::DirectionBias::Center,
            movement_score: 95.0,
            spray_score: 88.0,
            overall_score: 85.0,
            moving_shots_percentage: 5.0,
        };

        sm.insert_session(&metrics, None).unwrap();
        let recents = sm.get_recent_sessions(10).unwrap();
        assert_eq!(recents.len(), 1);
        assert_eq!(recents[0].session_id, "s_test_1");
        assert_eq!(recents[0].accuracy, 80.0);
    }
}
