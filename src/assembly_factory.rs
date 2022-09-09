use std::{io, ops::Add, path::Path};

use crate::{scanner::Scanner, class::{HubResponse, ServiceNodeConfig, HubExtendedJson, HubNotice, HubLatest, HubUpdate, HubPackages}, constant::{HUB_UPDATE_DIR, HUB_UPDATE_PACK,HUB_EXTENDED_UPDATE_PACK}};

pub fn get_hub_response(
    scanner:Scanner,
    hub_service: ServiceNodeConfig,
    hub_extended: HubExtendedJson,
    notices: Vec<HubNotice>,
    root: String,
)->Result<HubResponse, anyhow::Error> {
    let path_local=hub_service.local;
    let path_url=root.add(&hub_service.path);
    let path_update=String::from(Path::new(&path_local).join(HUB_UPDATE_DIR).to_string_lossy());

    let file_node_hub=scanner.scan_file_node(path_local, path_url, String::from("^Edgeless Hub.*7z$"), 2)?;
    let file_node_update=scanner.get_file_node(String::from(HUB_UPDATE_PACK),path_update.clone(),path_url.clone());
    let file_node_extended_update=scanner.get_file_node(String::from(HUB_EXTENDED_UPDATE_PACK),path_update.clone(),path_url.clone());

    let latest=HubLatest{
        version:file_node_hub.version.clone(),
        page:hub_extended.download_page,
    };
    let update=HubUpdate{
        allow_normal_since:hub_extended.allow_normal_since,
        force_update_until:hub_extended.force_update_until,
        wide_gaps:hub_extended.wide_gaps,
    };
    // let packages:HubPackages{
    //     update
    // }
}