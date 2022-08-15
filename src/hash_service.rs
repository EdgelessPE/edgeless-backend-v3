use sha256::digest_file;
use std::path::Path;
use std::{collections::HashMap, io};

pub struct HashService {
    map: HashMap<String, String>,
}

impl HashService {
    pub fn new(map: HashMap<String, String>) -> Self {
        HashService { map }
    }

    pub fn query(&mut self, path: String, key: String) -> io::Result<String> {
        Ok(self
            .map
            .entry(key)
            .or_insert(digest_file(Path::new(&path))?)
            .to_owned())
    }

    pub fn delete_cache(&mut self,key: String){
        self.map.remove(&key);
    }
}
