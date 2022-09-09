pub const CONFIG_FILE: &str = "./config.toml";
pub const HASH_MAP_FILE: &str = "./hash_map.bin";
pub const PROTOCOL: &str = "3.0.0";
pub const UPDATE_INTERVAL: u64 = 5 * 60;
pub const RESPONSE_VALID_INTERVAL: u64 = 60;

pub const CMD_REQUEST: &str = "cmd_request";
pub const SU_REQUEST: &str = "su_request";

pub const SPLITER: [char; 2] = ['-', '_'];
pub const HUB_UPDATE_DIR: &str = "Update";
pub const HUB_UPDATE_PACK: &str = "update.7z";
pub const HUB_EXTENDED_UPDATE_PACK: &str = "extended_update.7z";
pub const VENTOY_PLUGIN_PATH: &str = "plugin/ventoy_wimboot.img";
