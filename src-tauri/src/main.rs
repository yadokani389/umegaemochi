// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod server;
mod settings;
mod state;

use commands::settings::{get_settings, set_atcoder_id, set_weather_city_id};
use commands::utils::{get_server_address, get_yahoo_news};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let app_state = state::AppState::try_new().unwrap();

    tauri::Builder::default()
        .manage(app_state)
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move { server::start_server(handle).await });
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
