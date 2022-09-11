use std::{ops::Add, path::Path};

use crate::{
    class::{
        AlphaCover, AlphaCoverJson, AlphaResponse, FileNode, HelloResponse, HubExtendedJson,
        HubLatest, HubNotice, HubPackages, HubResponse, HubUpdate, LazyDeleteNode, PluginsResponse,
        ServiceNodeConfig, ServiceNodePublic, VentoyResponse,
    },
    config::{Config, ExtendedConfig},
    constant::{
        ALPHA_COVER, HUB_EXTENDED_UPDATE_PACK, HUB_UPDATE_DIR, HUB_UPDATE_PACK, PROTOCOL,
        VENTOY_PLUGIN,
    },
    scanner::Scanner,
    utils::{get_json, get_service},
};

pub fn get_general_response(
    scanner: &mut Scanner,
    config: &Config,
) -> Result<(HelloResponse, AlphaResponse, Vec<LazyDeleteNode>), anyhow::Error> {
    let pub_services: Vec<ServiceNodePublic> = config
        .mirror
        .services
        .clone()
        .into_iter()
        .map(|node| ServiceNodePublic {
            name: node.name,
            path: node.path,
        })
        .collect();
    let (hub_extended, notices, extended_alpha_config) =
        parse_extended_jsons(config.config.clone()).unwrap();
    let mut root = config.mirror.root.clone();
    let services = &config.mirror.services;

    let kernel_service = get_service(services, String::from("kernel")).unwrap();
    let kernel_response = get_kernel_response(scanner, &root, kernel_service)?;

    let ventoy_service = get_service(services, String::from("ventoy")).unwrap();
    let ventoy_response = get_ventoy_response(scanner, &root, ventoy_service)?;

    let hub_service = get_service(services, String::from("hub")).unwrap();
    let hub_response = get_hub_response(scanner, &root, hub_service, hub_extended, notices)?;

    let alpha_service = get_service(services, String::from("alpha")).unwrap();
    let alpha_response = get_alpha_response(scanner, &root, alpha_service, extended_alpha_config)?;

    //因为配置校验可能无法检查子目录的固定名称文件，因此最后再扫描插件包以便前期发现panic
    let plugins_service = get_service(services, String::from("plugins")).unwrap();
    let (plugins_response, lazy_delete_list) =
        get_plugins_response(scanner, &mut root, plugins_service)?;
    //保存hash map bin
    scanner.save_hash_map();

    Ok((
        HelloResponse {
            name: config.mirror.name.clone(),
            description: config.mirror.description.clone(),
            protocol: String::from(PROTOCOL),
            root: root.to_owned(),
            property: config.property.clone(),
            services: pub_services,
            plugins: plugins_response,
            kernel: kernel_response,
            ventoy: ventoy_response,
            hub: hub_response,
        },
        alpha_response,
        lazy_delete_list,
    ))
}

fn parse_extended_jsons(
    extended_config: ExtendedConfig,
) -> Result<(HubExtendedJson, Vec<HubNotice>, AlphaCoverJson), String> {
    let hub: HubExtendedJson = get_json(extended_config.hub)?;
    let notice: Vec<HubNotice> = get_json(extended_config.hub_notices)?;
    let extended_alpha_config: AlphaCoverJson = get_json(extended_config.alpha_cover)?;

    Ok((hub, notice, extended_alpha_config))
}

fn get_plugins_response(
    scanner: &mut Scanner,
    root: &mut String,
    plugins_service: ServiceNodeConfig,
) -> Result<(PluginsResponse, Vec<LazyDeleteNode>), anyhow::Error> {
    let (tree, lazy_delete_list) = scanner.scan_packages(plugins_service.local.clone())?;

    Ok((
        PluginsResponse {
            tree,
            path: root.clone() + (&plugins_service.path),
        },
        lazy_delete_list,
    ))
}

fn get_kernel_response(
    scanner: &mut Scanner,
    root: &String,
    kernel_service: ServiceNodeConfig,
) -> Result<FileNode, anyhow::Error> {
    let file_node = scanner.scan_file_node(
        kernel_service.local,
        root.clone().add(&kernel_service.path),
        String::from("^Edgeless.*iso$"),
        2,
    )?;

    Ok(file_node)
}

fn get_alpha_response(
    scanner: &mut Scanner,
    root: &String,
    alpha_service: ServiceNodeConfig,
    extended_alpha_config: AlphaCoverJson,
) -> Result<AlphaResponse, anyhow::Error> {
    let path_local = String::from(Path::new(&alpha_service.local).to_string_lossy());
    let cover_file_node = scanner.get_file_node(
        String::from(ALPHA_COVER),
        path_local.clone(),
        root.clone().add(&alpha_service.path),
    )?;
    let kernel_wim_file_node = scanner.scan_file_node(
        path_local,
        alpha_service.path,
        String::from("^Edgeless.*wim$"),
        2,
    )?;

    Ok(AlphaResponse {
        kernel_wim: kernel_wim_file_node,
        cover: AlphaCover {
            lower_than: extended_alpha_config.lower_than,
            file: cover_file_node,
        },
    })
}

fn get_ventoy_response(
    scanner: &mut Scanner,
    root: &String,
    ventoy_service: ServiceNodeConfig,
) -> Result<VentoyResponse, anyhow::Error> {
    let windows = scanner.scan_file_node(
        ventoy_service.local.clone(),
        root.clone().add(&ventoy_service.path),
        String::from("^ventoy-.*-windows\\.zip$"),
        1,
    )?;
    let linux = scanner.scan_file_node(
        ventoy_service.local.clone(),
        root.clone().add(&ventoy_service.path),
        String::from("^ventoy-.*-linux\\.tar\\.gz$"),
        1,
    )?;
    let plugin = scanner.get_file_node(
        String::from(VENTOY_PLUGIN),
        ventoy_service.local.clone(),
        root.clone().add(&ventoy_service.path),
    )?;

    Ok(VentoyResponse {
        windows,
        linux,
        plugin,
    })
}

fn get_hub_response(
    scanner: &mut Scanner,
    root: &String,
    hub_service: ServiceNodeConfig,
    hub_extended: HubExtendedJson,
    notices: Vec<HubNotice>,
) -> Result<HubResponse, anyhow::Error> {
    let path_local = hub_service.local;
    let path_url = root.clone().add(&hub_service.path);
    let path_update = String::from(
        Path::new(&path_local)
            .join(HUB_UPDATE_DIR)
            .to_string_lossy(),
    );

    let file_node_hub = scanner.scan_file_node(
        path_local,
        path_url.clone(),
        String::from("^Edgeless Hub.*7z$"),
        2,
    )?;
    let file_node_update = scanner.get_file_node(
        String::from(HUB_UPDATE_PACK),
        path_update.clone(),
        path_url.clone(),
    )?;
    let file_node_extended_update = scanner.get_file_node(
        String::from(HUB_EXTENDED_UPDATE_PACK),
        path_update.clone(),
        path_url.clone(),
    )?;

    let latest = HubLatest {
        version: file_node_hub.version.clone(),
        page: hub_extended.download_page,
    };
    let update = HubUpdate {
        allow_normal_since: hub_extended.allow_normal_since,
        force_update_until: hub_extended.force_update_until,
        wide_gaps: hub_extended.wide_gaps,
    };
    let packages = HubPackages {
        update: file_node_update,
        extended_update: file_node_extended_update,
        full: file_node_hub,
    };

    Ok(HubResponse {
        latest,
        update,
        notices,
        packages,
    })
}
