// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(deprecated)]

use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use timebreak_analytics::{
    MetricsCalculator, Recommendation, Recommender, SessionMetrics, ShotTelemetryEvent,
};
use timebreak_ballistics::WeaponDefinition;
use timebreak_core::{AppState, StateMachine};
use timebreak_platform::CS2Profile;
use timebreak_storage::StorageManager;

struct TimeBreakState {
    pub state_machine: Mutex<StateMachine>,
    pub storage: Mutex<StorageManager>,
    pub profile: Mutex<CS2Profile>,
    pub current_weapon: Mutex<WeaponDefinition>,
}

#[tauri::command]
fn get_app_state(state: State<TimeBreakState>) -> AppState {
    state.state_machine.lock().unwrap().current()
}

#[tauri::command]
fn get_cs2_profile(state: State<TimeBreakState>) -> CS2Profile {
    state.profile.lock().unwrap().clone()
}

#[tauri::command]
fn update_cs2_profile(state: State<TimeBreakState>, profile: CS2Profile) -> Result<(), String> {
    *state.profile.lock().unwrap() = profile;
    Ok(())
}

#[tauri::command]
fn get_all_weapons() -> Vec<WeaponDefinition> {
    WeaponDefinition::all_initial()
}

#[tauri::command]
fn set_active_weapon(
    state: State<TimeBreakState>,
    weapon_id: String,
) -> Result<WeaponDefinition, String> {
    if let Some(w) = WeaponDefinition::get_by_id(&weapon_id) {
        *state.current_weapon.lock().unwrap() = w.clone();
        Ok(w)
    } else {
        Err(format!("Weapon with id '{}' not found", weapon_id))
    }
}

#[tauri::command]
fn start_break(app: AppHandle, state: State<TimeBreakState>) -> Result<(), String> {
    let mut sm = state.state_machine.lock().unwrap();
    let _ = sm.transition(AppState::Playing);

    if let Some(overlay_window) = app.get_webview_window("overlay") {
        #[cfg(target_os = "macos")]
        {
            use cocoa::base::id;
            if let Ok(ns_window_ptr) = overlay_window.ns_window() {
                let ns_window = ns_window_ptr as id;
                unsafe {
                    timebreak_platform::macos::configure_macos_transparent_overlay(ns_window);
                }
            }
        }

        let _ = overlay_window.show();
        let _ = overlay_window.set_focus();
    }

    Ok(())
}

#[tauri::command]
fn end_break(app: AppHandle, state: State<TimeBreakState>) -> Result<(), String> {
    let mut sm = state.state_machine.lock().unwrap();
    let _ = sm.transition(AppState::Results);

    if let Some(overlay_window) = app.get_webview_window("overlay") {
        let _ = overlay_window.hide();
    }

    // Bring main window to front for results
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.show();
        let _ = main_window.set_focus();
    }

    Ok(())
}

#[tauri::command]
fn emergency_escape(app: AppHandle, state: State<TimeBreakState>) -> Result<(), String> {
    let mut sm = state.state_machine.lock().unwrap();
    let _ = sm.transition(AppState::ReturningToWork);
    let _ = sm.transition(AppState::Idle);

    if let Some(overlay_window) = app.get_webview_window("overlay") {
        let _ = overlay_window.hide();
    }

    Ok(())
}

#[tauri::command]
fn record_session_results(
    state: State<TimeBreakState>,
    session_id: String,
    weapon_id: String,
    start_time_ms: u64,
    end_time_ms: u64,
    shots: Vec<ShotTelemetryEvent>,
) -> Result<(SessionMetrics, Recommendation), String> {
    let metrics = MetricsCalculator::compute_session_metrics(
        session_id,
        weapon_id,
        start_time_ms,
        end_time_ms,
        &shots,
    );

    let rec = Recommender::generate_recommendation(&metrics);

    let mut storage = state.storage.lock().unwrap();
    let _ = storage.insert_session(&metrics, Some(&rec));
    let _ = storage.insert_shots_batch(&shots);

    Ok((metrics, rec))
}

#[tauri::command]
fn get_recent_sessions(
    state: State<TimeBreakState>,
    limit: usize,
) -> Result<Vec<SessionMetrics>, String> {
    let storage = state.storage.lock().unwrap();
    storage
        .get_recent_sessions(limit)
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
struct DevConfig {
    pub is_dev: bool,
    pub auto_break: bool,
    pub session_seconds: Option<u32>,
}

#[tauri::command]
fn get_dev_config() -> DevConfig {
    let is_dev = std::env::var("TIMEBREAK_DEV")
        .map(|v| v == "1")
        .unwrap_or(false);
    let auto_break = std::env::var("TIMEBREAK_AUTO_BREAK")
        .map(|v| v == "1")
        .unwrap_or(false);
    let session_seconds = std::env::var("TIMEBREAK_SESSION_SECONDS")
        .ok()
        .and_then(|v| v.parse().ok());
    DevConfig {
        is_dev,
        auto_break,
        session_seconds,
    }
}

#[tauri::command]
fn get_setting(state: State<TimeBreakState>, key: String) -> Result<Option<String>, String> {
    let storage = state.storage.lock().unwrap();
    storage.get_setting(&key).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_setting(state: State<TimeBreakState>, key: String, value: String) -> Result<(), String> {
    let storage = state.storage.lock().unwrap();
    storage.set_setting(&key, &value).map_err(|e| e.to_string())
}

fn main() {
    env_logger::init();

    // In-memory or local database
    let storage = StorageManager::open_in_memory().expect("Failed to initialize database");
    let initial_weapon = WeaponDefinition::ak47();

    let app_state = TimeBreakState {
        state_machine: Mutex::new(StateMachine::new()),
        storage: Mutex::new(storage),
        profile: Mutex::new(CS2Profile::default()),
        current_weapon: Mutex::new(initial_weapon),
    };

    tauri::Builder::default()
        .manage(app_state)
        .setup(|app| {
            // Configure overlay window on startup
            if let Some(overlay_window) = app.get_webview_window("overlay") {
                #[cfg(target_os = "macos")]
                {
                    use cocoa::base::id;
                    if let Ok(ns_window_ptr) = overlay_window.ns_window() {
                        let ns_window = ns_window_ptr as id;
                        unsafe {
                            timebreak_platform::macos::configure_macos_transparent_overlay(
                                ns_window,
                            );
                        }
                    }
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_state,
            get_cs2_profile,
            update_cs2_profile,
            get_all_weapons,
            set_active_weapon,
            start_break,
            end_break,
            emergency_escape,
            record_session_results,
            get_recent_sessions,
            get_setting,
            set_setting,
            get_dev_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running TimeBreak application");
}
