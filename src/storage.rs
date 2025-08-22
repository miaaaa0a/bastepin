use base64::{Engine as _, engine::general_purpose::STANDARD};
use sled::IVec;
use std::error::Error;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::io::Write;
use flate2::Compression;
use flate2::write::ZlibEncoder;

#[derive(Clone)]
pub struct Storage {
    pub db: sled::Db,
}
impl Storage {
    pub fn new(path: &str) -> Self {
        let db =
            sled::open(path).expect("sled should be able to open/create a database at this path");
        Self { db }
    }

    pub fn get(&self, k: String) -> Result<Option<IVec>, Box<dyn Error>> {
        let v = self.db.get(k)?;
        Ok(v)
    }

    fn encode(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::fast());
        e.write_all(content.as_bytes())?;
        let compressed = e.finish()?;

        let encoded = STANDARD.encode(compressed);

        Ok(encoded)
    }

    pub fn write(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let salt = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let hash = blake3::hash(format!("{}{}", content, salt).as_bytes());
        let k = STANDARD.encode(hash.to_hex().to_string());
        let v = self.encode(content)?;

        self.db.insert(&k, v.as_bytes())?;

        Ok(k)
    }
}
