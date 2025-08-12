use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::error::Error;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn read_into_hashmap() -> Result<sled::Db, Box<dyn Error>> {
    let hashmap: sled::Db = sled::open("storage")?;
    Ok(hashmap)
}

pub fn write(content: &String) -> Result<String, Box<dyn Error>> {
    let db: sled::Db = sled::open("storage")?;

    let salt = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let hash = blake3::hash(format!("{}{}", content, salt).as_bytes());
    let encoded = STANDARD.encode(hash.to_hex().to_string());
    db.insert(&encoded, content.as_bytes())?;

    Ok(encoded)
}
