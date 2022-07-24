use std::{fs, io, path::Path};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::SystemTime;
use regex::Regex;
use crate::class::EptFileNode;
use crate::hash::get_hash;

#[derive(PartialEq)]
pub enum FileType {
    Dir,
    File,
}

fn read_dir(path: String, filter: FileType) -> Result<Vec<String>, io::Error> {
    let category_dir = fs::read_dir(path)?;

    let mut collection = Vec::new();
    for entry_res in category_dir {
        let entry = entry_res?;
        if (filter == FileType::Dir && entry.file_type().unwrap().is_dir()) || (filter == FileType::File && entry.file_type().unwrap().is_file()) {
            collection.push(String::from(entry.file_name().to_string_lossy()));
        }
    }

    Ok(collection)
}

fn get_file_node(sub_path: String, name: String) -> Result<EptFileNode, io::Error> {
    let p = Path::new(&sub_path).join(name.clone());
    let meta = fs::metadata(p)?;

    Ok(EptFileNode {
        hash: get_hash(sub_path + "/" + &name)?,
        name,
        size: meta.len(),
        timestamp: meta.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
    })
}

pub fn scan_plugins(path: String) -> Result<HashMap<String, Vec<EptFileNode>>, io::Error> {
    let mut map: HashMap<String, Vec<EptFileNode>> = HashMap::new();
    //读取分类目录
    let categories = read_dir(path.clone(), FileType::Dir)?;
    //读取一层子目录
    for category in categories {
        let sub_path = String::from(Path::new(&path.clone()).join(category.clone()).to_string_lossy());
        let file_list = read_dir(sub_path.clone(), FileType::File)?;
        let mut collection = Vec::new();
        for name in file_list {
            collection.push(get_file_node(sub_path.clone(), name.to_owned()).unwrap());
        }
        map.insert(category, collection);
    }

    Ok(map)
}

//文件选择器函数
pub fn file_selector(path: String, exp: String) -> Result<String, String> {
    //校验路径是否存在
    if !Path::new(&path).exists() {
        return Err(String::from("file_selector:Can't find ") + &path);
    }

    //校验正则表达式是否有效
    let expression = Regex::new(&exp);
    if let Err(_) = expression {
        return Err(String::from("file_selector:Invalid expression: ") + &exp);
    }
    let reg = expression.unwrap();

    //列出文件列表
    let file_list = fs::read_dir(&path);
    if let Err(_) = file_list {
        return Err(String::from("file_selector:Can't read as directory: ") + &path);
    }

    //遍历匹配文件名
    let mut valid_data = false;
    let mut result = String::from("Null");
    for entry in file_list.unwrap() {
        let true_name = String::from(entry.unwrap().file_name().to_string_lossy());
        if reg.is_match(&true_name) {
            if valid_data {
                //对比字符串判断是否需要更新
                if true_name.cmp(&result) == Ordering::Greater {
                    result = true_name;
                }
            } else {
                valid_data = true;
                result = true_name;
            }
        }
    }

    return if valid_data {
        Ok(result)
    } else {
        Err(String::from("file_selector:Matched nothing when looking into ") + &path + " for " + &exp)
    };
}
