use crate::commands::utils::stringify;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config<T> {
    pub config_path: PathBuf,
    pub data: T,
}

pub trait ConfigTrait<T>: std::marker::Sized {
    fn try_new(config_path: PathBuf) -> Result<Self, String>;
    fn set(&mut self, new: T) -> Result<(), String>;
    fn write_file(&self) -> Result<(), String>;
    fn read_file(&mut self) -> Result<(), String>;
}

impl<T> ConfigTrait<T> for Config<T>
where
    T: std::default::Default + serde::Serialize + serde::de::DeserializeOwned,
{
    fn try_new(config_path: PathBuf) -> Result<Self, String> {
        let mut config = Config {
            config_path,
            data: T::default(),
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

    fn set(&mut self, new: T) -> Result<(), String> {
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
