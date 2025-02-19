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
    T: serde::de::DeserializeOwned + Default + serde::Serialize,
{
    let config_file = &path;

    let f = fs::read_to_string(config_file);

    match f {
        Ok(content) => {
            let config: T = toml::from_str(&content).expect("parsing toml error");
            Some(config)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default_config = T::default();
            save(path.clone(), &default_config);
            Some(default_config)
        }
        Err(e) => {
            warn!("{}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct Conf {
        hello: i32,
        name: String,
    }

    impl Default for Conf {
        fn default() -> Self {
            Conf{
                hello: 32,
                name: "hello".to_string(),
            }
        }
    }

    #[test]
    fn it_works() {
        let conf: Conf = read(PathBuf::from("config.toml")).unwrap();
        assert_eq!(conf.name, "hello".to_string());

        save(PathBuf::from("config.toml"), Conf::default());
        let conf2: Conf = read(PathBuf::from("config.toml")).unwrap();

        assert_eq!(conf2.hello, 32)
    }
}