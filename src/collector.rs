use std::io;
use std::ops::Add;
use crate::{config, scanner};
use crate::class::{EptResponse};

const PROTOCOL: &str = "3.0.0";

pub fn ept(cfg: &config::Config) -> Result<EptResponse, io::Error> {
    let c = cfg.to_owned();
    let tree = scanner::scan_plugins(c.position.plugins)?;

    Ok(EptResponse {
        name: c.mirror.name,
        description: c.mirror.description,
        native_server: c.mirror.native_server,
        upload_bandwidth: c.mirror.upload_bandwidth,
        protocol: String::from(PROTOCOL),
        root: c.url.domain.add(&c.url.plugins),
        sync_interval: c.mirror.sync_interval,
        official_maintained: c.mirror.official_maintained,
        services: c.mirror.services,
        tree,
    })
}
