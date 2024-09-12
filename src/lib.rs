use log::warn;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub path: PathBuf,
}

impl Config {
    /// save current config to a toml file
    pub fn save(self) {

        // if dir not exists, create one
        match &self.path.parent() {
            None => {}
            Some(dir) => { fs::create_dir_all(dir).expect("creating config dir failed"); }
        }

        // create file
        fs::File::create(&self.path).expect("creating config file failed");

        // stringify and write
        let toml_string = toml::to_string_pretty(&self).expect("toml to string failed");
        fs::write(&self.path, toml_string).expect("writing config file failed");
    }

    /// Read config from a toml file
    pub fn read<T>(self) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let config_file = &self.path;

        let f = fs::read_to_string(config_file);

        match f {
            Ok(content) => {
                let config: T = toml::from_str(&content).expect("parsing toml error");
                Some(config)
            }
            Err(e) => {
                warn!("{}", e);
                None
            }
        }
    }
}
