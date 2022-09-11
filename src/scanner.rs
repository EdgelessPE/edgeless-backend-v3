use casual_logger::Log;

use crate::class::{EptFileNode, FileNode, FileType, LazyDeleteNode};
use crate::constant::HASH_MAP_FILE;
use crate::hash2::IntegrityCache;
use crate::utils::{file_selector, read_dir, version_cmp, version_extractor};
use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::Add;
use std::time::SystemTime;
use std::{fs, io, path::Path};

//获取用于哈希服务索引的key
fn get_key(file_name: String, timestamp: u64) -> String {
    file_name.add("_").add(&timestamp.to_string())
}

//获取元信息，返回元组（时间戳，大小）
fn get_meta(path: String) -> io::Result<(u64, u64)> {
    let file_path = Path::new(&path);
    if !file_path.exists() {
        println!("Error:File not exist : {}", &path);
        Log::error(&format!("Error:File not exist : {}", &path));
    }
    let meta = fs::metadata(file_path)?;
    let timestamp = meta
        .modified()
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok((timestamp, meta.len()))
}

//选取多个插件包中的最高版本并淘汰其他
pub fn dulp_selector(names: Vec<String>) -> (String, Vec<String>) {
    //生成带版本号的输入数组
    let mut names_with_version: Vec<(String, Vec<u32>)> = names
        .into_iter()
        .map(|name| {
            let s: Vec<&str> = (&name).split("_").collect();
            let version: Vec<u32> = s[1]
                .split(".")
                .map(|s| {
                    let try_parse = s.parse::<u32>();
                    if let Err(_) = try_parse {
                        Log::warn(&format!("Can't parse version for {}", name));
                        0
                    } else {
                        try_parse.unwrap()
                    }
                })
                .collect();
            (name, version)
        })
        .collect();
    //按照版本号升序排列
    names_with_version.sort_by(|a, b| version_cmp(&a.1, &b.1));
    let mut sorted: Vec<String> = names_with_version.into_iter().map(|node| node.0).collect();
    //弹出
    let reserve = sorted.pop().unwrap();
    Log::info(&format!("Reserve {}, lazy delete {:?}", &reserve, &sorted));
    //返回
    (reserve, sorted)
}

pub struct Scanner {
    integrity: Box<IntegrityCache>,
}

impl Scanner {
    pub fn new(integrity: IntegrityCache) -> Self {
        Scanner {
            integrity: Box::new(integrity),
        }
    }

    pub fn scan_packages(
        &mut self,
        path: String,
    ) -> Result<(HashMap<String, Vec<EptFileNode>>, Vec<LazyDeleteNode>), io::Error> {
        let mut result: HashMap<String, Vec<EptFileNode>> = HashMap::new();
        let mut lazy_delete: Vec<LazyDeleteNode> = vec![];
        let initial_calc_hash = !Path::new(HASH_MAP_FILE).exists();

        //读取分类目录
        Log::info(&format!("Read packages on {}", &path));
        let categories = read_dir(path.clone(), FileType::Dir)?;
        //读取一层子目录
        for category in categories {
            if initial_calc_hash {
                println!("Scanning category {}", &category);
            }
            //分类目录路径
            let sub_path = String::from(
                Path::new(&path.clone())
                    .join(category.clone())
                    .to_string_lossy(),
            );
            //扫描获取分类目录下的所有插件包
            let file_list = read_dir(sub_path.clone(), FileType::File)?;

            //插件包去重
            let mut dulp_map: HashMap<String, Vec<String>> = HashMap::new();
            let mut collection = Vec::new();

            for name in file_list {
                let s: Vec<&str> = (&name).split("_").collect();
                let package_name = s[0].to_string();
                dulp_map.entry(package_name).or_insert(vec![]).push(name);
            }

            //迭代map生成collection
            for (_, file_names) in dulp_map.into_iter() {
                if file_names.len() == 1 {
                    collection.push(file_names[0].clone());
                } else {
                    let (reserve, delete_list_string) = dulp_selector(file_names);
                    collection.push(reserve);
                    let delete_list: Vec<LazyDeleteNode> = delete_list_string
                        .into_iter()
                        .map(|file| {
                            let file_path =
                                String::from(Path::new(&sub_path).join(&file).to_string_lossy());
                            let (timestamp, _) = get_meta(file_path.clone()).unwrap();
                            LazyDeleteNode {
                                path: file_path,
                                key: get_key(file, timestamp),
                            }
                        })
                        .collect();
                    lazy_delete = [lazy_delete, delete_list].concat();
                }
            }

            //由字符串collection生成文件节点collection
            let file_node_collection: Vec<EptFileNode> = collection
                .into_iter()
                .par_bridge()
                .map_with(self.integrity.to_owned(), |box_integrity_cache, file| {
                    let file_path =
                        String::from(Path::new(&sub_path).join(&file).to_string_lossy());
                    let (timestamp, size) = get_meta(file_path.clone()).unwrap();

                    let key = get_key(file.clone(), timestamp);
                    let integrity_cache = box_integrity_cache.as_mut();
                    let integrity = integrity_cache.query(&key, file_path).unwrap().clone();

                    EptFileNode {
                        name: file,
                        size,
                        timestamp,
                        integrity,
                    }
                })
                .collect();

            result.insert(category, file_node_collection);
        }

        Ok((result, lazy_delete))
    }

    pub fn scan_file_node(
        &mut self,
        path_local: String,
        path_url: String,
        regex: String,
        version_index: usize,
    ) -> anyhow::Result<FileNode> {
        let name = file_selector(path_local.clone(), regex, version_index).unwrap();
        let version = version_extractor(name.clone(), version_index).unwrap();

        let file_path = String::from(Path::new(&path_local).join(&name).to_string_lossy());
        let (timestamp, size) = get_meta(file_path.clone())?;

        let key = get_key(name.clone(), timestamp);
        let integrity = self.integrity.query(key, file_path.clone())?;

        let url = path_url.add("/").add(&name);
        Ok(FileNode {
            name,
            version,
            url,
            size,
            timestamp,
            integrity,
        })
    }

    pub fn get_file_node(
        &mut self,
        file_name: String,
        path_local: String,
        path_url: String,
    ) -> anyhow::Result<FileNode> {
        let file_path = String::from(Path::new(&path_local).join(&file_name).to_string_lossy());
        let (timestamp, size) = get_meta(file_path.clone())?;
        let url = String::from(Path::new(&path_url).join(&file_name).to_string_lossy());

        let key = get_key(file_name.clone(), timestamp);
        let integrity = self.integrity.query(key, file_path.clone())?;
        Ok(FileNode {
            name: file_name,
            version: String::from("0.0.0"),
            url,
            size,
            timestamp,
            integrity,
        })
    }

    pub fn delete_file(&mut self, path: String, key: String) {
        Log::info(&format!("Delete file {}", &path));
        let file_path = Path::new(&path);
        if file_path.exists() {
            if let Err(err) = fs::remove_file(&file_path) {
                Log::error(&format!(
                    "Can't delete {}, io error : {}",
                    file_path.to_string_lossy(),
                    err
                ));
            }
        } else {
            Log::warn(&format!(
                "Can't delete {}, file not exist",
                file_path.to_string_lossy()
            ));
        }
        self.integrity.remove(&key);
    }

    pub fn save_hash_map(&self) {
        match self.integrity.save(HASH_MAP_FILE) {
            Ok(_) => {
                Log::info("Hash map cache saved");
                println!("Info:Hash cache bin saved");
            }
            Err(err) => {
                Log::error(&format!("Can't save hash map {}", err));
                println!("Error:Can't save hash map {}", err);
            }
        }
    }
}
