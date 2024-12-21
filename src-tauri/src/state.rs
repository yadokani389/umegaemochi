use crate::commands::settings::Settings;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub settings: Mutex<Settings>,
}

impl AppState {
    pub fn try_new() -> Result<Self, String> {
        Ok(Self {
            settings: Mutex::from(Settings::try_new()?),
        })
    }
}
