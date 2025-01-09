use crate::settings::{Config, Settings};
use std::path::PathBuf;

#[derive(Debug)]
pub struct AppState {
    pub settings: Config<Settings>,
}

impl AppState {
    pub fn try_new(config_path: PathBuf) -> Result<Self, String> {
        Ok(Self {
            settings: Config::<Settings>::try_new(config_path)?,
        })
    }
}
