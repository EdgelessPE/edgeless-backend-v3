use crate::class::{FileType, ServiceNodeConfig};
use crate::constant::SPLITER;
use casual_logger::Log;
use regex::Regex;
use std::io;
use std::{
    cmp::{self, Ordering},
    fs,
    path::Path,
};
use anyhow::anyhow;

pub fn get_json<T>(path: String) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let text_res = fs::read_to_string(&path);
    if let Err(e) = text_res {
        return Err(anyhow!("Can't read file {} to string : {}", &path, e));
    }
    let text = &text_res.unwrap();
    let parse_res: serde_json::Result<T> = serde_json::from_str(text);
    if let Err(e) = parse_res {
        return Err(anyhow!("Can't parse file {} as json : {}", &path, e));
    }
    Ok(parse_res.unwrap())
}


pub fn get_service(services: &Vec<ServiceNodeConfig>, name: String) -> Option<ServiceNodeConfig> {
    for service in services.clone().into_iter() {
        if service.name == name {
            return Some(service);
        }
    }
    None
}
//文件选择器函数
pub fn file_selector(path: String, exp: String, version_index: usize) -> anyhow::Result<String> {
    //校验路径是否存在
    if !Path::new(&path).exists() {
        return Err(anyhow!("file_selector:Can't find {}",&path));
    }

    //校验正则表达式是否有效
    let expression = Regex::new(&exp);
    if let Err(_) = expression {
        return Err(anyhow!("file_selector:Invalid expression: {}",&exp));
    }
    let regex = expression.unwrap();

    //列出文件列表
    let file_list = fs::read_dir(&path);
    if let Err(_) = file_list {
        return Err(anyhow!("file_selector:Can't read as directory: {}",&path));
    }

    //遍历匹配文件名
    let mut valid_data = false;
    let mut result = String::from("Null");
    let mut result_version: Vec<u32> = vec![0];
    for entry in file_list.unwrap() {
        let file_name = entry.unwrap().file_name().clone();
        let true_name = file_name.to_str().unwrap().clone();
        if regex.is_match(true_name) {
            if valid_data {
                //对比字符串判断是否需要更新
                let cur_version = &version_extractor(String::from(true_name), version_index)?;
                let cur_version_split: Vec<u32> = cur_version
                    .split(".")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
                if version_cmp(&cur_version_split, &result_version) == Ordering::Greater {
                    result = String::from(true_name);
                    result_version = cur_version_split;
                }
            } else {
                valid_data = true;
                result = String::from(true_name);
                let cur_version = &version_extractor(String::from(true_name), version_index)?;
                result_version = cur_version
                    .split(".")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
            }
        }
    }

    return if valid_data {
        Ok(result)
    } else {
        return Err(anyhow!("file_selector:Matched nothing when looking into {} for {}",&path,&exp))
    };
}

//版本号提取器函数
pub fn version_extractor(name: String, index: usize) -> anyhow::Result<String> {
    //首次切割，获取拓展名的值及其长度
    let mut ext_name = "";
    let mut ext_len = 0;
    let result_ext: Vec<&str> = name.split(".").collect();
    if result_ext.len() > 1 {
        ext_name = result_ext[result_ext.len() - 1];
        ext_len = ext_name.len();
    }

    //再次切割（去拓展名切割），获取字段，将拓展名叠加到最后
    let mut result: Vec<&str> = name[0..name.len() - ext_len - 1].split(SPLITER).collect();
    result.push(ext_name);

    if index > result.len() {
        return Err(anyhow!("version_extractor:Index out of range when split {},got {}",&name,&index.to_string()));
    }
    //println!("{:?}",result);
    return Ok(result[index].to_string());
}

pub fn version_cmp(a: &Vec<u32>, b: &Vec<u32>) -> Ordering {
    for i in 0..cmp::min(a.len(), b.len()) {
        if a[i] < b[i] {
            return Ordering::Less;
        } else if a[i] > b[i] {
            return Ordering::Greater;
        }
    }

    //处理前缀版本号相同但是长度不一致的情况
    if a.len() != b.len() {
        //找出较长的那一个
        let t = if a.len() < b.len() { b } else { a };
        //读取剩余位
        for i in cmp::min(a.len(), b.len())..cmp::max(a.len(), b.len()) {
            if t[i] != 0 {
                return if a.len() < b.len() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
        }
    }

    Ordering::Equal
}

pub fn read_dir(path: String, filter: FileType) -> io::Result<Vec<String>> {
    let p = Path::new(&path);
    if !p.exists() {
        Log::error(&format!("Path {} not exist!", &path));
    }
    let category_dir = fs::read_dir(path)?;

    let mut collection = Vec::new();
    for entry_res in category_dir {
        let entry = entry_res?;
        if (filter == FileType::Dir && entry.file_type().unwrap().is_dir())
            || (filter == FileType::File && entry.file_type().unwrap().is_file())
        {
            collection.push(String::from(entry.file_name().to_string_lossy()));
        }
    }

    Ok(collection)
}
