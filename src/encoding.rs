use std::error::Error;
use std::io::Write;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use flate2::Compression;
use flate2::write::ZlibEncoder;

pub fn encode(content: &String) -> Result<String, Box<dyn Error>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::fast());
    e.write(&content.as_bytes())?;
    let compressed = e.finish()?;

    let encoded = STANDARD.encode(compressed);

    Ok(encoded)
}