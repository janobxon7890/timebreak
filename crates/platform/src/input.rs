use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CS2Profile {
    pub dpi: f32,
    pub sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub resolution_width: u32,
    pub resolution_height: u32,
}

impl Default for CS2Profile {
    fn default() -> Self {
        Self {
            dpi: 800.0,
            sensitivity: 1.25, // eDPI = 1000
            zoom_sensitivity: 1.0,
            resolution_width: 1920,
            resolution_height: 1080,
        }
    }
}

impl CS2Profile {
    pub fn edpi(&self) -> f32 {
        self.dpi * self.sensitivity
    }

    /// Calculate cm/360 based on CS2 m_yaw (0.022 degrees per count)
    pub fn cm_360(&self) -> f32 {
        let m_yaw = 0.022; // CS2 standard
        let counts_per_360 = 360.0 / (self.sensitivity * m_yaw);
        let inches = counts_per_360 / self.dpi;
        inches * 2.54
    }

    /// Convert raw mouse deltas (dx, dy) into virtual crosshair screen offset in game coordinates
    pub fn delta_to_screen_offset(
        &self,
        delta_x: f32,
        delta_y: f32,
        is_scoped: bool,
    ) -> (f32, f32) {
        let effective_sens = if is_scoped {
            self.sensitivity * self.zoom_sensitivity
        } else {
            self.sensitivity
        };

        // CS2 1.0 sens standard: 1 mouse count = 0.022 degrees.
        // At 90 FOV on 1920 horizontal, 1 degree is roughly 1920 / 90 = 21.33 px.
        let fov_factor = 21.33 * 0.022;
        (
            delta_x * effective_sens * fov_factor,
            delta_y * effective_sens * fov_factor,
        )
    }
}

pub trait RawInputProvider {
    fn poll_mouse_delta(&mut self) -> (f32, f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cs2_edpi_and_cm360() {
        let profile = CS2Profile {
            dpi: 800.0,
            sensitivity: 1.0,
            zoom_sensitivity: 1.0,
            resolution_width: 1920,
            resolution_height: 1080,
        };

        assert_eq!(profile.edpi(), 800.0);
        let cm = profile.cm_360();
        // 800 dpi * 1.0 sens in CS2 is approx 51.95 cm/360
        assert!((cm - 51.95).abs() < 0.5);
    }
}
