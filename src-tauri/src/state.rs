use crate::settings::{Config, Settings};
use std::{path::PathBuf, sync::Mutex};

#[derive(Debug)]
pub struct AppState {
    pub settings: Mutex<Config<Settings>>,
}

impl AppState {
    pub fn try_new(config_path: PathBuf) -> Result<Self, String> {
        Ok(Self {
            settings: Mutex::from(Config::<Settings>::try_new(config_path)?),
        })
    }
}
