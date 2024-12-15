mod commands;
use commands::phone::start_server;
use commands::utils::get_yahoo_news;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_yahoo_news, start_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
