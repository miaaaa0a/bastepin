use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{Read, Write};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use base64::{engine::general_purpose::STANDARD, Engine as _};

fn check_for_storage() -> Result<String, Box<dyn Error>> {
    let path = std::env::var("STORAGE_PATH")?;
    if !std::fs::exists(&path).unwrap() {
        std::fs::write(&path, "")?;
    }
    Ok(path)
}

pub fn read_into_hashmap() -> Result<HashMap<String, String>, Box<dyn Error>>{
    let storage = check_for_storage()?;

    let buf = BufReader::new(File::open(storage)?);
    let mut key_buf = Vec::with_capacity(64);
    let mut content_buf = Vec::with_capacity(10240);
    let mut reading_content = false;
    let mut hashmap = HashMap::new();

    for x in buf.bytes() {
        match x {
            Ok(byte) => {
                match byte {
                    // colon :::
                    58 => reading_content = true,
                    // comma ,,,
                    44 => {
                        reading_content = false;
                        let k = String::from_utf16_lossy(&key_buf);
                        let v = String::from_utf16_lossy(&content_buf);
                        hashmap.insert(k, v);
                        key_buf.clear();
                        content_buf.clear();
                    },
                    _ => {
                        if reading_content { content_buf.push(byte.into()) } else { key_buf.push(byte.into()) };
                    },
                }
            },
            Err(e) => panic!("error while reading storage: {:?}", e)
        }
    }

    Ok(hashmap)
}

pub fn write(content: &String) -> Result<String, Box<dyn Error>> {
    let storage = check_for_storage()?;
    
    let salt = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let hash = blake3::hash(
        format!("{}{}", content, salt).as_bytes()
    );
    let encoded = STANDARD.encode(hash.to_hex().to_string());

    let mut storage = OpenOptions::new()
        .write(true)
        .append(true)
        .open(storage)
        .unwrap();

    write!(storage, "{}:{},", encoded, content)?;
    Ok(encoded)
}