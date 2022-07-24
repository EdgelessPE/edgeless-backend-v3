use std::io;
use sha256::digest_file;
use std::path::Path;

pub fn get_hash(path: String) -> Result<String, io::Error> {
    let p = Path::new(&path);
    digest_file(p)
}