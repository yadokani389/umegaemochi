mod commands;
mod periodic;
mod server;
mod state;

use commands::utils::{
    complete_todo, get_server_address, get_settings, get_sports_news, get_todos, get_version,
    get_yahoo_news, get_exchange_rate,
};

use std::sync::Mutex;
use tauri::Manager;

const CONFIG_PATH: &str = "umegaemochi/";
const WIDGET_LIST: [&str; 9] = [
    "WidgetWeather",
    "WidgetNews",
    "WidgetAtCoder",
    "WidgetCalendar",
    "WidgetClock",
    "WidgetTodo",
    "WidgetSportsNews",
    "WidgetWeeklyWeather",
    "WidgetExchangeRate",
];
const VERSION: &str = match option_env!("CARGO_PKG_VERSION") {
    Some(version) => version,
    None => "unknown",
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
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
                handle.path().config_dir()?.join(CONFIG_PATH),
            )?);
            app.manage(app_state);
            tauri::async_runtime::spawn(async move { server::start_server(handle).await });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move { periodic::start_daily_reload(handle).await });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move { periodic::start_control_nightmode(handle).await });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                state::disaster_info::check_disaster_updates(handle).await
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_yahoo_news,
            get_server_address,
            get_settings,
            get_version,
            get_todos,
            complete_todo,
            get_sports_news,
            get_exchange_rate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
