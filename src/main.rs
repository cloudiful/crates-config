use log::info;
use crate::config::Config;

mod config;

fn main() {
    let config = Config::read("Cargo.toml").unwrap();
    info!("{:?}", config);
}
