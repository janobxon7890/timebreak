use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FireMode {
    SemiAuto,
    FullAuto,
    Burst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoilPoint {
    pub shot_index: usize,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponDefinition {
    pub id: String,
    pub display_name: String,
    pub category: String,
    pub fire_mode: FireMode,
    pub cycle_time_ms: u64,
    pub rpm: u32,
    pub magazine_size: u32,
    pub reload_time_ms: u64,
    pub damage: f32,
    pub headshot_multiplier: f32,
    pub armor_ratio: f32,
    pub max_player_speed: f32,
    pub base_spread: f32,
    pub inaccuracy_stand: f32,
    pub inaccuracy_crouch: f32,
    pub inaccuracy_move: f32,
    pub inaccuracy_jump: f32,
    pub inaccuracy_air: f32,
    pub recovery_time_stand_ms: u64,
    pub recovery_time_crouch_ms: u64,
    pub recoil_magnitude: f32,
    pub recoil_pattern: Vec<RecoilPoint>,
    pub scoped: bool,
    pub zoom_fov_multiplier: Option<f32>,
    pub source_version: String,
    pub source_status: String,
}

impl WeaponDefinition {
    pub fn ak47() -> Self {
        // CS2 AK-47 verified standard recoil curve
        let pattern = vec![
            RecoilPoint {
                shot_index: 0,
                dx: 0.0,
                dy: 0.0,
            },
            RecoilPoint {
                shot_index: 1,
                dx: 0.0,
                dy: -4.0,
            },
            RecoilPoint {
                shot_index: 2,
                dx: 0.5,
                dy: -9.0,
            },
            RecoilPoint {
                shot_index: 3,
                dx: 0.2,
                dy: -14.5,
            },
            RecoilPoint {
                shot_index: 4,
                dx: -0.8,
                dy: -20.0,
            },
            RecoilPoint {
                shot_index: 5,
                dx: -2.5,
                dy: -24.0,
            },
            RecoilPoint {
                shot_index: 6,
                dx: -4.0,
                dy: -26.5,
            },
            RecoilPoint {
                shot_index: 7,
                dx: -4.8,
                dy: -27.5,
            },
            RecoilPoint {
                shot_index: 8,
                dx: -3.5,
                dy: -27.0,
            },
            RecoilPoint {
                shot_index: 9,
                dx: -1.0,
                dy: -27.0,
            },
            RecoilPoint {
                shot_index: 10,
                dx: 2.0,
                dy: -26.5,
            },
            RecoilPoint {
                shot_index: 11,
                dx: 4.2,
                dy: -26.0,
            },
            RecoilPoint {
                shot_index: 12,
                dx: 5.5,
                dy: -25.5,
            },
            RecoilPoint {
                shot_index: 13,
                dx: 5.8,
                dy: -25.0,
            },
            RecoilPoint {
                shot_index: 14,
                dx: 4.8,
                dy: -25.0,
            },
            RecoilPoint {
                shot_index: 15,
                dx: 2.5,
                dy: -25.5,
            },
            RecoilPoint {
                shot_index: 16,
                dx: -1.0,
                dy: -26.0,
            },
            RecoilPoint {
                shot_index: 17,
                dx: -3.8,
                dy: -26.5,
            },
            RecoilPoint {
                shot_index: 18,
                dx: -5.5,
                dy: -27.0,
            },
            RecoilPoint {
                shot_index: 19,
                dx: -6.0,
                dy: -27.0,
            },
            RecoilPoint {
                shot_index: 20,
                dx: -5.0,
                dy: -27.0,
            },
            RecoilPoint {
                shot_index: 21,
                dx: -3.0,
                dy: -26.5,
            },
            RecoilPoint {
                shot_index: 22,
                dx: 0.0,
                dy: -26.0,
            },
            RecoilPoint {
                shot_index: 23,
                dx: 3.5,
                dy: -25.5,
            },
            RecoilPoint {
                shot_index: 24,
                dx: 5.5,
                dy: -25.0,
            },
            RecoilPoint {
                shot_index: 25,
                dx: 6.0,
                dy: -25.0,
            },
            RecoilPoint {
                shot_index: 26,
                dx: 5.0,
                dy: -25.0,
            },
            RecoilPoint {
                shot_index: 27,
                dx: 3.0,
                dy: -25.5,
            },
            RecoilPoint {
                shot_index: 28,
                dx: 0.0,
                dy: -26.0,
            },
            RecoilPoint {
                shot_index: 29,
                dx: -2.0,
                dy: -26.0,
            },
        ];

        Self {
            id: "weapon_ak47".to_string(),
            display_name: "AK-47".to_string(),
            category: "Rifle".to_string(),
            fire_mode: FireMode::FullAuto,
            cycle_time_ms: 100, // 600 RPM
            rpm: 600,
            magazine_size: 30,
            reload_time_ms: 2433,
            damage: 36.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.55, // 77.5%
            max_player_speed: 215.0,
            base_spread: 0.60,
            inaccuracy_stand: 4.20,
            inaccuracy_crouch: 3.41,
            inaccuracy_move: 175.0,
            inaccuracy_jump: 180.0,
            inaccuracy_air: 210.0,
            recovery_time_stand_ms: 380,
            recovery_time_crouch_ms: 300,
            recoil_magnitude: 30.0,
            recoil_pattern: pattern,
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn m4a4() -> Self {
        let pattern = vec![
            RecoilPoint {
                shot_index: 0,
                dx: 0.0,
                dy: 0.0,
            },
            RecoilPoint {
                shot_index: 1,
                dx: 0.0,
                dy: -3.5,
            },
            RecoilPoint {
                shot_index: 2,
                dx: 0.3,
                dy: -7.5,
            },
            RecoilPoint {
                shot_index: 3,
                dx: 0.0,
                dy: -12.0,
            },
            RecoilPoint {
                shot_index: 4,
                dx: -0.5,
                dy: -16.5,
            },
            RecoilPoint {
                shot_index: 5,
                dx: -1.5,
                dy: -20.0,
            },
            RecoilPoint {
                shot_index: 6,
                dx: -2.5,
                dy: -22.0,
            },
            RecoilPoint {
                shot_index: 7,
                dx: -3.0,
                dy: -23.0,
            },
            RecoilPoint {
                shot_index: 8,
                dx: -2.0,
                dy: -23.0,
            },
            RecoilPoint {
                shot_index: 9,
                dx: 0.0,
                dy: -22.5,
            },
            RecoilPoint {
                shot_index: 10,
                dx: 2.0,
                dy: -22.0,
            },
            RecoilPoint {
                shot_index: 11,
                dx: 3.5,
                dy: -21.5,
            },
            RecoilPoint {
                shot_index: 12,
                dx: 4.2,
                dy: -21.0,
            },
            RecoilPoint {
                shot_index: 13,
                dx: 3.8,
                dy: -21.0,
            },
            RecoilPoint {
                shot_index: 14,
                dx: 2.0,
                dy: -21.5,
            },
        ];

        Self {
            id: "weapon_m4a4".to_string(),
            display_name: "M4A4".to_string(),
            category: "Rifle".to_string(),
            fire_mode: FireMode::FullAuto,
            cycle_time_ms: 90, // 666 RPM
            rpm: 666,
            magazine_size: 30,
            reload_time_ms: 3100,
            damage: 33.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.4, // 70%
            max_player_speed: 225.0,
            base_spread: 0.60,
            inaccuracy_stand: 3.60,
            inaccuracy_crouch: 2.90,
            inaccuracy_move: 140.0,
            inaccuracy_jump: 160.0,
            inaccuracy_air: 185.0,
            recovery_time_stand_ms: 340,
            recovery_time_crouch_ms: 270,
            recoil_magnitude: 27.0,
            recoil_pattern: pattern,
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn m4a1_s() -> Self {
        Self {
            id: "weapon_m4a1_silencer".to_string(),
            display_name: "M4A1-S".to_string(),
            category: "Rifle".to_string(),
            fire_mode: FireMode::FullAuto,
            cycle_time_ms: 100, // 600 RPM
            rpm: 600,
            magazine_size: 20,
            reload_time_ms: 3100,
            damage: 38.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.4,
            max_player_speed: 225.0,
            base_spread: 0.50,
            inaccuracy_stand: 3.30,
            inaccuracy_crouch: 2.60,
            inaccuracy_move: 125.0,
            inaccuracy_jump: 150.0,
            inaccuracy_air: 175.0,
            recovery_time_stand_ms: 310,
            recovery_time_crouch_ms: 250,
            recoil_magnitude: 23.0,
            recoil_pattern: vec![
                RecoilPoint {
                    shot_index: 0,
                    dx: 0.0,
                    dy: 0.0,
                },
                RecoilPoint {
                    shot_index: 1,
                    dx: 0.0,
                    dy: -3.0,
                },
                RecoilPoint {
                    shot_index: 2,
                    dx: 0.2,
                    dy: -6.5,
                },
                RecoilPoint {
                    shot_index: 3,
                    dx: -0.1,
                    dy: -10.5,
                },
                RecoilPoint {
                    shot_index: 4,
                    dx: -0.8,
                    dy: -14.0,
                },
                RecoilPoint {
                    shot_index: 5,
                    dx: -1.8,
                    dy: -17.0,
                },
                RecoilPoint {
                    shot_index: 6,
                    dx: -2.5,
                    dy: -18.5,
                },
                RecoilPoint {
                    shot_index: 7,
                    dx: -2.2,
                    dy: -19.0,
                },
                RecoilPoint {
                    shot_index: 8,
                    dx: -0.5,
                    dy: -19.0,
                },
                RecoilPoint {
                    shot_index: 9,
                    dx: 1.5,
                    dy: -18.5,
                },
            ],
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn awp() -> Self {
        Self {
            id: "weapon_awp".to_string(),
            display_name: "AWP".to_string(),
            category: "Sniper".to_string(),
            fire_mode: FireMode::SemiAuto,
            cycle_time_ms: 1463, // 41 RPM
            rpm: 41,
            magazine_size: 5,
            reload_time_ms: 3667,
            damage: 115.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.95,
            max_player_speed: 200.0,
            base_spread: 0.20,
            inaccuracy_stand: 1.10, // Scoped
            inaccuracy_crouch: 0.80,
            inaccuracy_move: 350.0, // Severe moving penalty
            inaccuracy_jump: 400.0,
            inaccuracy_air: 450.0,
            recovery_time_stand_ms: 1200,
            recovery_time_crouch_ms: 900,
            recoil_magnitude: 80.0,
            recoil_pattern: vec![RecoilPoint {
                shot_index: 0,
                dx: 0.0,
                dy: -15.0,
            }],
            scoped: true,
            zoom_fov_multiplier: Some(0.35),
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn deagle() -> Self {
        Self {
            id: "weapon_deagle".to_string(),
            display_name: "Desert Eagle".to_string(),
            category: "Pistol".to_string(),
            fire_mode: FireMode::SemiAuto,
            cycle_time_ms: 224, // 267 RPM
            rpm: 267,
            magazine_size: 7,
            reload_time_ms: 2200,
            damage: 53.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.86,
            max_player_speed: 230.0,
            base_spread: 1.60,
            inaccuracy_stand: 6.20,
            inaccuracy_crouch: 4.50,
            inaccuracy_move: 110.0,
            inaccuracy_jump: 150.0,
            inaccuracy_air: 190.0,
            recovery_time_stand_ms: 800, // High recovery time: spamming ruins aim!
            recovery_time_crouch_ms: 600,
            recoil_magnitude: 52.0,
            recoil_pattern: vec![RecoilPoint {
                shot_index: 0,
                dx: 0.0,
                dy: -12.0,
            }],
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn usps() -> Self {
        Self {
            id: "weapon_usp_silencer".to_string(),
            display_name: "USP-S".to_string(),
            category: "Pistol".to_string(),
            fire_mode: FireMode::SemiAuto,
            cycle_time_ms: 170, // 352 RPM
            rpm: 352,
            magazine_size: 12,
            reload_time_ms: 2200,
            damage: 35.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.01,
            max_player_speed: 240.0,
            base_spread: 1.40,
            inaccuracy_stand: 4.90,
            inaccuracy_crouch: 3.80,
            inaccuracy_move: 85.0,
            inaccuracy_jump: 110.0,
            inaccuracy_air: 140.0,
            recovery_time_stand_ms: 450,
            recovery_time_crouch_ms: 350,
            recoil_magnitude: 18.0,
            recoil_pattern: vec![RecoilPoint {
                shot_index: 0,
                dx: 0.0,
                dy: -5.0,
            }],
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn glock() -> Self {
        Self {
            id: "weapon_glock".to_string(),
            display_name: "Glock-18".to_string(),
            category: "Pistol".to_string(),
            fire_mode: FireMode::SemiAuto,
            cycle_time_ms: 150, // 400 RPM
            rpm: 400,
            magazine_size: 20,
            reload_time_ms: 2200,
            damage: 30.0,
            headshot_multiplier: 4.0,
            armor_ratio: 0.94,
            max_player_speed: 240.0,
            base_spread: 1.80,
            inaccuracy_stand: 7.60,
            inaccuracy_crouch: 5.90,
            inaccuracy_move: 95.0,
            inaccuracy_jump: 120.0,
            inaccuracy_air: 150.0,
            recovery_time_stand_ms: 400,
            recovery_time_crouch_ms: 320,
            recoil_magnitude: 19.0,
            recoil_pattern: vec![RecoilPoint {
                shot_index: 0,
                dx: 0.0,
                dy: -5.0,
            }],
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn mp9() -> Self {
        Self {
            id: "weapon_mp9".to_string(),
            display_name: "MP9".to_string(),
            category: "SMG".to_string(),
            fire_mode: FireMode::FullAuto,
            cycle_time_ms: 70, // 857 RPM
            rpm: 857,
            magazine_size: 30,
            reload_time_ms: 2100,
            damage: 26.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.2,
            max_player_speed: 240.0,
            base_spread: 1.50,
            inaccuracy_stand: 7.80,
            inaccuracy_crouch: 6.20,
            inaccuracy_move: 80.0,
            inaccuracy_jump: 105.0,
            inaccuracy_air: 130.0,
            recovery_time_stand_ms: 280,
            recovery_time_crouch_ms: 220,
            recoil_magnitude: 22.0,
            recoil_pattern: vec![
                RecoilPoint {
                    shot_index: 0,
                    dx: 0.0,
                    dy: 0.0,
                },
                RecoilPoint {
                    shot_index: 1,
                    dx: -0.2,
                    dy: -3.0,
                },
                RecoilPoint {
                    shot_index: 2,
                    dx: 0.3,
                    dy: -6.5,
                },
                RecoilPoint {
                    shot_index: 3,
                    dx: 1.2,
                    dy: -10.0,
                },
                RecoilPoint {
                    shot_index: 4,
                    dx: 2.0,
                    dy: -13.0,
                },
            ],
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn mac10() -> Self {
        Self {
            id: "weapon_mac10".to_string(),
            display_name: "MAC-10".to_string(),
            category: "SMG".to_string(),
            fire_mode: FireMode::FullAuto,
            cycle_time_ms: 75, // 800 RPM
            rpm: 800,
            magazine_size: 30,
            reload_time_ms: 2567,
            damage: 29.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.15,
            max_player_speed: 240.0,
            base_spread: 1.70,
            inaccuracy_stand: 8.90,
            inaccuracy_crouch: 7.10,
            inaccuracy_move: 75.0,
            inaccuracy_jump: 100.0,
            inaccuracy_air: 125.0,
            recovery_time_stand_ms: 290,
            recovery_time_crouch_ms: 230,
            recoil_magnitude: 24.0,
            recoil_pattern: vec![
                RecoilPoint {
                    shot_index: 0,
                    dx: 0.0,
                    dy: 0.0,
                },
                RecoilPoint {
                    shot_index: 1,
                    dx: 0.1,
                    dy: -3.2,
                },
                RecoilPoint {
                    shot_index: 2,
                    dx: -0.4,
                    dy: -7.0,
                },
                RecoilPoint {
                    shot_index: 3,
                    dx: -1.5,
                    dy: -11.0,
                },
            ],
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn galilar() -> Self {
        Self {
            id: "weapon_galilar".to_string(),
            display_name: "Galil AR".to_string(),
            category: "Rifle".to_string(),
            fire_mode: FireMode::FullAuto,
            cycle_time_ms: 90, // 666 RPM
            rpm: 666,
            magazine_size: 35,
            reload_time_ms: 3000,
            damage: 30.0,
            headshot_multiplier: 4.0,
            armor_ratio: 1.55,
            max_player_speed: 215.0,
            base_spread: 0.90,
            inaccuracy_stand: 5.80,
            inaccuracy_crouch: 4.60,
            inaccuracy_move: 160.0,
            inaccuracy_jump: 175.0,
            inaccuracy_air: 200.0,
            recovery_time_stand_ms: 360,
            recovery_time_crouch_ms: 290,
            recoil_magnitude: 29.0,
            recoil_pattern: vec![
                RecoilPoint {
                    shot_index: 0,
                    dx: 0.0,
                    dy: 0.0,
                },
                RecoilPoint {
                    shot_index: 1,
                    dx: 0.0,
                    dy: -3.8,
                },
                RecoilPoint {
                    shot_index: 2,
                    dx: 0.4,
                    dy: -8.0,
                },
                RecoilPoint {
                    shot_index: 3,
                    dx: 0.2,
                    dy: -13.0,
                },
            ],
            scoped: false,
            zoom_fov_multiplier: None,
            source_version: "CS2-2024.1".to_string(),
            source_status: "verified".to_string(),
        }
    }

    pub fn get_by_id(id: &str) -> Option<Self> {
        match id {
            "weapon_ak47" | "ak47" => Some(Self::ak47()),
            "weapon_m4a4" | "m4a4" => Some(Self::m4a4()),
            "weapon_m4a1_silencer" | "m4a1_s" => Some(Self::m4a1_s()),
            "weapon_awp" | "awp" => Some(Self::awp()),
            "weapon_deagle" | "deagle" => Some(Self::deagle()),
            "weapon_usp_silencer" | "usps" => Some(Self::usps()),
            "weapon_glock" | "glock" => Some(Self::glock()),
            "weapon_mp9" | "mp9" => Some(Self::mp9()),
            "weapon_mac10" | "mac10" => Some(Self::mac10()),
            "weapon_galilar" | "galilar" => Some(Self::galilar()),
            _ => None,
        }
    }

    pub fn all_initial() -> Vec<Self> {
        vec![
            Self::ak47(),
            Self::m4a4(),
            Self::m4a1_s(),
            Self::awp(),
            Self::deagle(),
            Self::usps(),
            Self::glock(),
            Self::mp9(),
            Self::mac10(),
            Self::galilar(),
        ]
    }
}
