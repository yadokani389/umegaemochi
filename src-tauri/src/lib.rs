mod commands;
mod daily_reload;
mod disaster_info;
mod server;
mod settings;
mod state;

use commands::utils::{get_server_address, get_settings, get_version, get_yahoo_news};
use std::sync::Mutex;
use tauri::Manager;

const SETTINGS_FILE_PATH: &str = "umegaemochi/settings.toml";
const WIDGET_LIST: [&str; 6] = [
    "WidgetWeather",
    "WidgetNews",
    "WidgetAtCoder",
    "WidgetCalendar",
    "WidgetClock",
    "WidgetSportsNews",
];
const VERSION: &str = match option_env!("CARGO_PKG_VERSION") {
    Some(version) => version,
    None => "unknown",
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_autostart::init(
                        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                        None,
                    ))
                    .unwrap();
            }
            let handle = app.handle().clone();
            let app_state = Mutex::new(state::AppState::try_new(
                handle.path().config_dir()?.join(SETTINGS_FILE_PATH),
            )?);
            app.manage(app_state);
            tauri::async_runtime::spawn(async move { server::start_server(handle).await });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move { daily_reload::start_job(handle).await });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                disaster_info::check_disaster_updates(handle).await
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_yahoo_news,
            get_server_address,
            get_settings,
            get_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
