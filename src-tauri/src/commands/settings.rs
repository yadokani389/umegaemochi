use tauri::Manager;

// https://qiita.com/takavfx/items/5c27d22df50be45a8968
use crate::commands::utils::stringify;
use crate::state::AppState;
use std::fs::File;
use std::io::Write;

const SETTINGS_FILE_PATH: &str = "umegaemochi/settings.toml";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    weather_city_id: String,
    atcoder_id: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            weather_city_id: "130010".into(),
            atcoder_id: "1step621".into(),
        }
    }
}

impl Settings {
    pub fn try_new() -> Result<Self, String> {
        let config_path = if let Some(config_path) = tauri_api::path::config_dir() {
            config_path.join(SETTINGS_FILE_PATH)
        } else {
            return Err("Failed to get config directory".into());
        };

        if !config_path.exists() {
            Settings::default().write_file()?;
            Ok(Settings::default())
        } else {
            let mut settings = Settings::default();
            settings.read_file()?;
            Ok(settings)
        }
    }

    pub fn set_weather_city_id(&mut self, city_id: String) -> Result<(), String> {
        self.weather_city_id = city_id;
        self.write_file()
    }

    pub fn set_atcoder_id(&mut self, atcoder_id: String) -> Result<(), String> {
        self.atcoder_id = atcoder_id;
        self.write_file()
    }
}

trait Config {
    fn write_file(&self) -> Result<(), String>;
    fn read_file(&mut self) -> Result<(), String>;
}

impl Config for Settings {
    fn write_file(&self) -> Result<(), String> {
        let Some(mut config_path) = tauri_api::path::config_dir() else {
            return Err("Failed to get config directory".into());
        };

        config_path.push(SETTINGS_FILE_PATH);
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).map_err(stringify)?;
        }
        let mut file = File::create(config_path).map_err(stringify)?;
        let toml = toml::to_string(self).map_err(stringify)?;

        file.write_all(toml.as_bytes()).map_err(stringify)?;
        file.flush().map_err(stringify)?;

        Ok(())
    }

    fn read_file(&mut self) -> Result<(), String> {
        let Some(mut config_path) = tauri_api::path::config_dir() else {
            return Err("Failed to get config directory".into());
        };

        config_path.push(SETTINGS_FILE_PATH);
        let s = std::fs::read_to_string(&config_path).map_err(stringify)?;
        *self = toml::from_str(&s).map_err(stringify)?;

        Ok(())
    }
}

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
    app.get_webview_window("main")
        .unwrap()
        .eval("window.location.reload()")
        .unwrap();
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
    app.get_webview_window("main")
        .unwrap()
        .eval("window.location.reload()")
        .unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_settings(app_state: tauri::State<AppState>) -> Result<Settings, String> {
    Ok(app_state.settings.lock().map_err(stringify)?.clone())
}
