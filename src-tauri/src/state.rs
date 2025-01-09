use crate::{
    disaster_info::DisasterInfo,
    settings::{Config, Settings},
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct AppState {
    pub settings: Config<Settings>,
    pub disaster_info: Option<DisasterInfo>,
}

impl AppState {
    pub fn try_new(config_path: PathBuf) -> Result<Self, String> {
        Ok(Self {
            settings: Config::<Settings>::try_new(config_path)?,
            disaster_info: None,
        })
    }
}
