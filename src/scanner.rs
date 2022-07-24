use std::{fs, io, path::Path};
use std::collections::HashMap;
use std::time::SystemTime;
use crate::class::EptFileNode;

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
        name,
        size: meta.len(),
        timestamp: meta.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        hash: String::from("114514"),
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