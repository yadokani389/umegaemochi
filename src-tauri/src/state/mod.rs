pub mod config;
pub mod disaster_info;
pub mod settings;
pub mod todo;

use config::Config;
use config::ConfigTrait;
use disaster_info::DisasterInfo;
use settings::Settings;
use std::collections::HashMap;
use std::path::PathBuf;
use todo::Todo;

#[derive(Debug)]
pub struct AppState {
    pub settings: Config<Settings>,
    pub disaster_info: Option<DisasterInfo>,
    pub todo: Config<HashMap<uuid::Uuid, Todo>>,
}

impl AppState {
    pub fn try_new(config_path: PathBuf) -> Result<Self, String> {
        Ok(Self {
            settings: Config::<Settings>::try_new(config_path.clone().join("settings.toml"))?,
            disaster_info: None,
            todo: Config::<HashMap<uuid::Uuid, Todo>>::try_new(config_path.join("todo.toml"))?,
        })
    }
}
