use sha256::digest_file;
use std::fs;
use std::path::Path;
use std::{collections::HashMap, io};

use crate::constant::HASH_MAP_FILE;

pub struct HashService {
    map: HashMap<String, String>,
}

fn get_sha256(path: String) -> io::Result<String> {
    println!("Info:Computing sha256 for {}", &path);
    digest_file(Path::new(&path))
}

impl HashService {
    pub fn new() -> Self {
        let cache_path = Path::new(HASH_MAP_FILE);
        let map;
        if cache_path.exists() {
            let cache = fs::read(HASH_MAP_FILE).unwrap();
            let parse_res = bincode::deserialize(&cache);
            map = match parse_res {
                Ok(val) => {
                    println!("Info:Use hash cache");
                    val
                }
                Err(_) => {
                    println!("Warning:Can't parse hash map cache, use new one");
                    HashMap::new()
                }
            };
        } else {
            map = HashMap::new();
        }
        println!("Info:Get hash map : {:?}", map);
        HashService { map }
    }

    pub fn query(&mut self, path: String, key: String) -> io::Result<String> {
        println!(
            "Info:Query key {}, has entry : {}",
            &key,
            self.map.contains_key(&key)
        );
        Ok(self.map.entry(key).or_insert(get_sha256(path)?).to_owned())
    }

    pub fn update_map(&mut self, new_map: HashMap<String, String>) {
        self.map = new_map;
    }

    pub fn delete_record(&mut self, key: String) {
        self.map.remove(&key);
    }

    pub fn save_hash_map(&mut self) -> io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(&self.map).unwrap();
        fs::write(HASH_MAP_FILE, encoded)?;
        Ok(())
    }
}
