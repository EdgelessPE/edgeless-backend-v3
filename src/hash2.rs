use casual_logger::Log;
use serde::{Serialize, Deserialize};
use dashmap::DashMap;
use std::fs::File;
use std::path::Path;
use std::io;
use sha256::digest_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntegrityMethod {
  #[serde(rename = "sha256")]
  Sha256,

  #[serde(rename = "blake3")]
  Blake3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integrity {
  pub method: IntegrityMethod,
  pub value: String
}

pub type IntegrityCacheInner = DashMap<String, Integrity>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityCache {
  inner: IntegrityCacheInner
}

impl IntegrityCache {
  pub fn new() -> Self {
    IntegrityCache { inner: DashMap::new() }
  }

  pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
    let mut file = File::open(&path)?;
    if let Ok(inner) = bincode::deserialize_from::<&mut File, IntegrityCacheInner>(&mut file) {
      Log::info("Use Integrity Cache File");
      Ok(Self { inner })
    } else {
      Log::warn("Integrity Cache File corrupted, use empty one");
      Ok(Self::new())
    }
  }

  pub fn compute<P: AsRef<Path>>(method: IntegrityMethod, path: P) -> anyhow::Result<Integrity> {
    match method {
      IntegrityMethod::Blake3 => compute_hash_blake3(path),
      IntegrityMethod::Sha256=> compute_hash_sha256(path),
    }
  }

  pub fn query<P: AsRef<Path>>(&mut self, key: String, path: P) -> anyhow::Result<&Integrity> {
    //TODO: 在配置中设置默认哈希算法，这里暂时写死罢。  
    Ok(
      self.inner.entry(key)
      .or_try_insert_with(
        || Self::compute(IntegrityMethod::Blake3, path)
      )?.value()
    )
  }

  pub fn remove(&mut self, key: String) -> Option<Integrity> {
    self.inner.remove(&key).map(|v| v.1)
  }

  pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    bincode::serialize_into(&mut file, &self)?;

    Ok(())
  }

}

pub fn try_into_memmap_file(file: &File) -> anyhow::Result<Option<io::Cursor<memmap2::Mmap>>> {
  let metadata = file.metadata()?;
  let file_size = metadata.len();

  Ok(
    if !metadata.is_file() {
      None
    } else if file_size > isize::max_value() as u64 {
      None
    } else if file_size == 0 {
      None
    } else if file_size < 16 * 1024 {
      None
    } else {
      let mmap = unsafe {
        memmap2::MmapOptions::new()
          .len(file_size as usize)
          .map(file)?
      };

      Some(io::Cursor::new(mmap))
    }
  )
}

fn copy_wide(mut reader: impl io::Read, hasher: &mut blake3::Hasher) -> io::Result<u64> {
  let mut buffer = [0; 65536];
  let mut total = 0;
  loop {
      match reader.read(&mut buffer) {
          Ok(0) => return Ok(total),
          Ok(n) => {
              hasher.update(&buffer[..n]);
              total += n as u64;
          }
          Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
          Err(e) => return Err(e),
      }
  }
}

pub fn compute_hash_blake3<P: AsRef<Path>>(path: P) -> anyhow::Result<Integrity> {
  let mut file = File::open(&path)?;
  let mut hasher = blake3::Hasher::new();
  if let Some(mmap) = try_into_memmap_file(&file)? {
    hasher.update_rayon(mmap.get_ref());
  } else {
    copy_wide(file, &mut hasher)?;
  }

  let hash = hasher.finalize();
  
  Ok(Integrity { method: IntegrityMethod::Blake3, value: hash.to_hex().to_string() })
}

pub fn compute_hash_sha256<P: AsRef<Path>>(path: P) -> anyhow::Result<Integrity> {
  let value=digest_file(path)?;
  Ok(Integrity{
    method:IntegrityMethod::Sha256,
    value
  })
}


#[cfg(test)]
mod tests {
  
}