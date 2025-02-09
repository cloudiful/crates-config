use log::info;
use config::Config;

fn main() {
    let config = Config::read("Cargo.toml").unwrap();
    info!("{:?}", config);
}
