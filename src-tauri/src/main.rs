// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod daily_reload;
mod server;
mod settings;
mod state;

use commands::settings::{get_settings, set_atcoder_id, set_weather_city_id};
use commands::utils::{get_server_address, get_yahoo_news};
use std::sync::Mutex;
use tauri::Manager;

const SETTINGS_FILE_PATH: &str = "umegaemochi/settings.toml";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            let app_state = Mutex::new(state::AppState::try_new(
                handle.path().config_dir()?.join(SETTINGS_FILE_PATH),
            )?);
            app.manage(app_state);
            tauri::async_runtime::spawn(async move { server::start_server(handle).await });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move { daily_reload::start_job(handle).await });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_yahoo_news,
            get_server_address,
            get_settings,
            set_atcoder_id,
            set_weather_city_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
