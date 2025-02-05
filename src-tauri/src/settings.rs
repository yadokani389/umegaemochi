// https://qiita.com/takavfx/items/5c27d22df50be45a8968
use crate::commands::utils::stringify;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub weather_city_id: String,
    pub atcoder_id: String,
    pub widget_interval: u64,
    pub using_widgets: Vec<String>,
    pub auto_fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            weather_city_id: "130010".into(),
            atcoder_id: "1step621".into(),
            widget_interval: 10000,
            using_widgets: crate::WIDGET_LIST.iter().map(|x| x.to_string()).collect(),
            auto_fullscreen: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config<T> {
    config_path: PathBuf,
    pub data: T,
}

impl Config<Settings> {
    pub fn try_new(config_path: PathBuf) -> Result<Self, String> {
        let mut config = Config {
            config_path,
            data: Settings::default(),
        };
        if config.config_path.exists() {
            if let Err(e) = config.read_file() {
                println!("Error reading config file: {}", e);
                println!("Creating new config file");
            }
        }
        config.write_file()?;
        Ok(config)
    }

    pub fn set(&mut self, new: Settings) -> Result<(), String> {
        self.data = new;
        self.write_file()
    }

    fn write_file(&self) -> Result<(), String> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent).map_err(stringify)?;
        }
        let mut file = File::create(&self.config_path).map_err(stringify)?;
        let toml = toml::to_string(&self.data).map_err(stringify)?;

        file.write_all(toml.as_bytes()).map_err(stringify)?;
        file.flush().map_err(stringify)?;

        Ok(())
    }

    fn read_file(&mut self) -> Result<(), String> {
        let s = std::fs::read_to_string(&self.config_path).map_err(stringify)?;
        self.data = toml::from_str(&s).map_err(stringify)?;

        Ok(())
    }
}
