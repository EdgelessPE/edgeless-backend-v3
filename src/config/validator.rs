use crate::{
    class::{AlphaCoverJson, FileType, HubExtendedJson, HubNotice},
    utils::{file_selector, get_json, read_dir},
};

use super::Config;

pub fn valid(config: &Config) -> Result<(), i32> {
    let mut errors_count = 0;

    //检查服务给出的目录是否存在相应文件
    for service in config.mirror.services.clone() {
        let service_name: &str = &service.name[..];
        let (test_regex, version_index): (&str, i32) = match service_name {
            "plugins" => ("[^\\w]+", 0),
            "kernel" => ("^Edgeless.*iso$", 2),
            "alpha" => ("^Edgeless.*wim$", 2),
            "ventoy" => ("^ventoy-.*-windows.zip$", 1),
            "hub" => ("^Edgeless Hub.*7z$", 2),
            _ => ("\\S+", -1),
        };
        if version_index > 0 {
            let file_selector_res = file_selector(
                service.local.clone(),
                String::from(test_regex),
                version_index.try_into().unwrap(),
            );
            if let Err(e) = file_selector_res {
                errors_count += 1;
                println!(
                    "Error:Invalid service {} : can't find valid items in given local path : {}",
                    service_name, e
                );
            }
        } else if version_index == 0 {
            //检查插件包目录是否存在文件夹
            let list = read_dir(service.local.clone(), FileType::Dir);
            if let Err(e) = list {
                errors_count += 1;
                println!("Error:Invalid service {} : can't find sub directories in given local path : {}",service_name,e);
            }
        } else {
            errors_count += 1;
            println!("Error:Unknown service {}", service_name);
        }
    }

    //检查外部 json 是否有效
    let hub: Result<HubExtendedJson, String> = get_json(config.config.hub.to_owned());
    let notice: Result<Vec<HubNotice>, String> = get_json(config.config.hub_notices.to_owned());
    let extended_alpha_config: Result<AlphaCoverJson, String> =
        get_json(config.config.alpha_cover.clone());

    if let Err(e) = hub {
        errors_count += 1;
        println!("Error:Can't parse extended json config hub : {}", e);
    }

    if let Err(e) = notice {
        errors_count += 1;
        println!("Error:Can't parse extended json config notice : {}", e);
    }

    if let Err(e) = extended_alpha_config {
        errors_count += 1;
        println!(
            "Error:Can't parse extended json config extended_alpha_config : {}",
            e
        );
    }

    if errors_count == 0 {
        println!("Info:Config validation passed");
        Ok(())
    } else {
        Err(errors_count)
    }
}
