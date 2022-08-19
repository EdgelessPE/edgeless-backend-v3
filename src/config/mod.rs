mod config_class;
mod validator;

use crate::constant::CONFIG_FILE;
pub use config_class::Config;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use self::validator::valid;

pub fn read_config() -> Result<Config, io::Error> {
    let config_path = Path::new(CONFIG_FILE);
    if !config_path.exists() {
        panic!("Error:Can't find config file : {}", CONFIG_FILE)
    }

    let mut text = String::new();
    File::open(CONFIG_FILE)?.read_to_string(&mut text).unwrap();
    let cfg=toml::from_str(&text).unwrap();

    //运行配置校验
    let cfg_validation=valid(&cfg);
    if let Err(c)=cfg_validation {
        panic!("Found {} errors, please check before restart",c);
    }

    Ok(cfg)
}
