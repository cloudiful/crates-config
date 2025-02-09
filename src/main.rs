use std::collections::HashMap;
use config::Config;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn main() {
    let config = Config{
        path: PathBuf::from("Cargo.toml"),
    };
    let res = config.read().unwrap();
    info!("{:#?}", res);
}
