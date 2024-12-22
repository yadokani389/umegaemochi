use crate::settings::Settings;
use crate::commands::utils::stringify;
use crate::state::AppState;
use tauri::Emitter;

#[tauri::command]
pub fn set_weather_city_id(
    app: tauri::AppHandle,
    app_state: tauri::State<AppState>,
    city_id: String,
) -> Result<(), String> {
    app_state
        .settings
        .lock()
        .map_err(stringify)?
        .set_weather_city_id(city_id)?;
    let _ = app.emit("settings_changed", ());
    Ok(())
}

#[tauri::command]
pub fn set_atcoder_id(
    app: tauri::AppHandle,
    app_state: tauri::State<AppState>,
    atcoder_id: String,
) -> Result<(), String> {
    app_state
        .settings
        .lock()
        .map_err(stringify)?
        .set_atcoder_id(atcoder_id)?;
    let _ = app.emit("settings_changed", ());
    Ok(())
}

#[tauri::command]
pub fn get_settings(app_state: tauri::State<AppState>) -> Result<Settings, String> {
    Ok(app_state.settings.lock().map_err(stringify)?.clone())
}
