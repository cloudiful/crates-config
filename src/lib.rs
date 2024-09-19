use log::warn;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// save current config to a toml file
pub fn save<T>(path: PathBuf, config: T)
where
    T: serde::Serialize,
{

    // if dir not exists, create one
    match &path.parent() {
        None => {}
        Some(dir) => { fs::create_dir_all(dir).expect("creating config dir failed"); }
    }

    // create file
    fs::File::create(&path).expect("creating config file failed");

    // stringify and write
    let toml_string = toml::to_string_pretty(&config).expect("toml to string failed");
    fs::write(&path, toml_string).expect("writing config file failed");
}

/// Read config from a toml file
pub fn read<T>(path: PathBuf) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let config_file = &path;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct Conf {
        hello: i32,
    }

    #[test]
    fn it_works() {
        let conf: Conf = read(PathBuf::from("config.toml")).unwrap();
        assert_eq!(conf.hello, 1);
    }
}