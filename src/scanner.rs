use crate::class::{EptFileNode, LazyDeleteNode};
use crate::hash_service::HashService;
use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::ops::Add;
use std::time::SystemTime;
use std::{fs, io, path::Path};

#[derive(PartialEq)]
pub enum FileType {
    Dir,
    File,
}

//获取用于哈希服务索引的key
fn get_key(file_name: String, timestamp: u64) -> String {
    file_name.add("_").add(&timestamp.to_string())
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

//获取元信息，返回元组（时间戳，大小）
fn get_meta(path: String) -> io::Result<(u64, u64)> {
    let file_path = Path::new(&path);
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
                        println!("Warning:Can't parse version for {}", name);
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
    //返回
    (reserve, sorted)
}

pub struct Scanner {
    hash_service: HashService,
}

impl Scanner {
    pub fn new(hash_service: HashService) -> Self {
        Scanner { hash_service }
    }

    fn read_dir(&mut self, path: String, filter: FileType) -> Result<Vec<String>, io::Error> {
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

    fn get_file_node(&mut self, sub_path: String, name: String) -> Result<EptFileNode, io::Error> {
        let file_path = sub_path.add(&name);
        let (timestamp, size) = get_meta(file_path.clone())?;

        Ok(EptFileNode {
            hash: self
                .hash_service
                .query(file_path, get_key(name.clone(), timestamp))?,
            name,
            size,
            timestamp,
        })
    }

    pub fn scan_packages(
        &mut self,
        path: String,
    ) -> Result<(HashMap<String, Vec<EptFileNode>>, Vec<LazyDeleteNode>), io::Error> {
        let mut result: HashMap<String, Vec<EptFileNode>> = HashMap::new();
        let mut lazy_delete: Vec<LazyDeleteNode> = vec![];

        //读取分类目录
        let categories = self.read_dir(path.clone(), FileType::Dir)?;
        //读取一层子目录
        for category in categories {
            //分类目录路径
            let sub_path = String::from(
                Path::new(&path.clone())
                    .join(category.clone())
                    .to_string_lossy(),
            );
            //扫描获取分类目录下的所有插件包
            let file_list = self.read_dir(sub_path.clone(), FileType::File)?;

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
                .map(|file| {
                    let file_path =
                        String::from(Path::new(&sub_path).join(&file).to_string_lossy());
                    let (timestamp, size) = get_meta(file_path.clone()).unwrap();
                    let key = get_key(file.clone(), timestamp);
                    let hash = self.hash_service.query(file_path, key).unwrap();

                    EptFileNode {
                        name: file,
                        size,
                        timestamp,
                        hash,
                    }
                })
                .collect();

            result.insert(category, file_node_collection);
        }

        Ok((result, lazy_delete))
    }

    pub fn delete_file(&mut self, path: String, key: String) {
        let file_path = Path::new(&path);
        if file_path.exists() {
            if let Err(err) = fs::remove_file(&file_path) {
                println!(
                    "Fatal:Can't delete {}, io error : {}",
                    file_path.to_string_lossy(),
                    err
                );
            }
        } else {
            println!(
                "Warning:Can't delete {}, file not exist",
                file_path.to_string_lossy()
            );
        }
        self.hash_service.delete_cache(key);
    }
}
