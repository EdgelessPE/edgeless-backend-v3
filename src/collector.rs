use std::collections::HashMap;
use std::io;
use std::ops::Add;
use crate::hash_service::HashService;
use crate::{config, scanner::Scanner};
use crate::class::{EptResponse};

const PROTOCOL: &str = "3.0.0";

pub fn ept(cfg: &config::Config) -> Result<EptResponse, io::Error> {
    let c = cfg.to_owned();
    let hash_service=HashService::new(HashMap::new());
    let mut scanner=Scanner::new(hash_service);
    let tree = scanner.scan_packages(c.position.plugins)?;

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
