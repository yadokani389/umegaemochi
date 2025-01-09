use crate::settings::Settings;
use crate::state::AppState;
use std::sync::Mutex;
use tauri::Emitter;

#[tauri::command]
pub fn set_weather_city_id(
    app: tauri::AppHandle,
    app_state: tauri::State<Mutex<AppState>>,
    city_id: String,
) -> Result<(), String> {
    app_state
        .lock()
        .unwrap()
        .settings
        .set_weather_city_id(city_id)?;
    let _ = app.emit("settings_changed", ());
    Ok(())
}

#[tauri::command]
pub fn set_atcoder_id(
    app: tauri::AppHandle,
    app_state: tauri::State<Mutex<AppState>>,
    atcoder_id: String,
) -> Result<(), String> {
    app_state
        .lock()
        .unwrap()
        .settings
        .set_atcoder_id(atcoder_id)?;
    let _ = app.emit("settings_changed", ());
    Ok(())
}

#[tauri::command]
pub fn get_settings(app_state: tauri::State<Mutex<AppState>>) -> Result<Settings, String> {
    Ok(app_state.lock().unwrap().settings.data.clone())
}
