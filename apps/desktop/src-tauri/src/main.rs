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

#[tauri::command]
fn hide_overlay(window: tauri::WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
fn show_overlay(window: tauri::WebviewWindow) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    {
        use cocoa::base::id;
        if let Ok(ns_window_ptr) = window.ns_window() {
            let ns_window = ns_window_ptr as id;
            unsafe {
                timebreak_platform::macos::configure_macos_transparent_overlay(ns_window);
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn toggle_overlay_visibility(window: tauri::WebviewWindow) -> Result<bool, String> {
    let is_visible = window.is_visible().unwrap_or(true);
    if is_visible {
        window.hide().map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        #[cfg(target_os = "macos")]
        {
            use cocoa::base::id;
            if let Ok(ns_window_ptr) = window.ns_window() {
                let ns_window = ns_window_ptr as id;
                unsafe {
                    timebreak_platform::macos::configure_macos_transparent_overlay(ns_window);
                }
            }
        }
        Ok(true)
    }
}

#[tauri::command]
fn toggle_fullscreen(window: tauri::WebviewWindow) -> Result<bool, String> {
    let current_size = window.inner_size().unwrap_or_default();
    let monitor = window.primary_monitor().ok().flatten();
    let is_fullscreen_size = if let Some(m) = &monitor {
        current_size.width >= m.size().width - 20 && current_size.height >= m.size().height - 20
    } else {
        false
    };

    let target_fullscreen = !is_fullscreen_size;
    if target_fullscreen {
        if let Some(m) = &monitor {
            let _ = window.set_position(*m.position());
            let _ = window.set_size(*m.size());
        }
    } else {
        let _ = window.set_size(tauri::LogicalSize::new(880.0, 560.0));
        let _ = window.center();
    }
    let _ = window.set_always_on_top(true);
    #[cfg(target_os = "macos")]
    {
        use cocoa::base::id;
        if let Ok(ns_window_ptr) = window.ns_window() {
            let ns_window = ns_window_ptr as id;
            unsafe {
                timebreak_platform::macos::configure_macos_transparent_overlay(ns_window);
            }
        }
    }
    Ok(target_fullscreen)
}

#[tauri::command]
fn set_window_pip(window: tauri::WebviewWindow, is_pip: bool) -> Result<(), String> {
    if is_pip {
        let _ = window.set_size(tauri::LogicalSize::new(800.0, 520.0));
        let _ = window.center();
    } else {
        if let Ok(Some(m)) = window.primary_monitor() {
            let _ = window.set_position(*m.position());
            let _ = window.set_size(*m.size());
        }
    }
    let _ = window.set_always_on_top(true);
    #[cfg(target_os = "macos")]
    {
        use cocoa::base::id;
        if let Ok(ns_window_ptr) = window.ns_window() {
            let ns_window = ns_window_ptr as id;
            unsafe {
                timebreak_platform::macos::configure_macos_transparent_overlay(ns_window);
            }
        }
    }
    Ok(())
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
            // Configure main window directly as transparent overlay spanning the monitor
            if let Some(main_window) = app.get_webview_window("main") {
                if let Ok(Some(monitor)) = main_window.primary_monitor() {
                    let size = monitor.size();
                    let position = monitor.position();
                    let _ = main_window.set_position(*position);
                    let _ = main_window.set_size(*size);
                }
                let _ = main_window.set_always_on_top(true);

                #[cfg(target_os = "macos")]
                {
                    use cocoa::base::id;
                    if let Ok(ns_window_ptr) = main_window.ns_window() {
                        let ns_window = ns_window_ptr as id;
                        unsafe {
                            timebreak_platform::macos::configure_macos_transparent_overlay(
                                ns_window,
                            );
                        }
                    }
                }
            }

            // Create macOS tray icon for 1-click restore/hide
            let toggle_item = tauri::menu::MenuItem::with_id(
                app,
                "toggle",
                "TimeBreak: Ko'rsatish / Yashirish (H)",
                true,
                None::<&str>,
            )?;
            let quit_item = tauri::menu::MenuItem::with_id(
                app,
                "quit",
                "Chiqish (Quit)",
                true,
                None::<&str>,
            )?;
            let menu = tauri::menu::Menu::with_items(app, &[&toggle_item, &quit_item])?;

            let icon = app
                .default_window_icon()
                .cloned()
                .unwrap_or_else(|| {
                    let mut rgba = Vec::with_capacity(32 * 32 * 4);
                    for _ in 0..(32 * 32) {
                        rgba.extend_from_slice(&[56, 189, 248, 255]); // Sky-400
                    }
                    tauri::image::Image::new_owned(rgba, 32, 32)
                });

            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "toggle" {
                        if let Some(main_win) = app.get_webview_window("main") {
                            let is_vis = main_win.is_visible().unwrap_or(false);
                            if is_vis {
                                let _ = main_win.hide();
                            } else {
                                let _ = main_win.show();
                                let _ = main_win.set_focus();
                                #[cfg(target_os = "macos")]
                                {
                                    use cocoa::base::id;
                                    if let Ok(ns_window_ptr) = main_win.ns_window() {
                                        let ns_window = ns_window_ptr as id;
                                        unsafe {
                                            timebreak_platform::macos::configure_macos_transparent_overlay(ns_window);
                                        }
                                    }
                                }
                            }
                        }
                    } else if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;

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
            get_dev_config,
            toggle_fullscreen,
            set_window_pip,
            hide_overlay,
            show_overlay,
            toggle_overlay_visibility
        ])
        .run(tauri::generate_context!())
        .expect("error while running TimeBreak application");
}
