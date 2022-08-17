mod config_class;

use crate::constant::CONFIG_FILE;
pub use config_class::Config;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn read_config() -> Result<Config, io::Error> {
    let config_path = Path::new(CONFIG_FILE);
    if !config_path.exists() {
        panic!("Error:Can't find config file : {}", CONFIG_FILE)
    }

    let mut text = String::new();
    File::open(CONFIG_FILE)?.read_to_string(&mut text).unwrap();

    Ok(toml::from_str(&text).unwrap())
}
