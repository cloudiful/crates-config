use std::fs;
use std::path::PathBuf;

use log::warn;
use serde::{Deserialize, Serialize};

// overall config
#[derive(Debug, Serialize, Deserialize)]
pub struct Config;

impl Config {
    pub fn save(c: &Config) {
        fs::create_dir_all("config").expect("creating config dir failed");
        fs::File::create("config/config.toml").expect("creating config file failed");
        let toml_string = toml::to_string_pretty(c).expect("toml to string failed");
        fs::write("config/config.toml", toml_string).expect("writing config file failed");
    }

    pub fn read(path: &str) -> Option<toml::Value> {
        let config_file = PathBuf::from(path);

        let f = fs::read_to_string(config_file);

        match f {
            Ok(content) => {
                let config: toml::Value = toml::from_str(&content).expect("parsing toml error");
                Some(config)
            }
            Err(e) => {
                warn!("{}", e);
                None
            }
        }
    }
}
