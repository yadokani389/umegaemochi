mod commands;
use commands::phone::start_server;
use commands::settings::{get_settings, set_atcoder_id, set_weather_city_id};
use commands::utils::get_yahoo_news;

mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = state::AppState::try_new().unwrap();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_yahoo_news,
            start_server,
            get_settings,
            set_atcoder_id,
            set_weather_city_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
