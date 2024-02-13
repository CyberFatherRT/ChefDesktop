use anyhow::Result;
use base64::{engine::general_purpose, Engine};

pub fn bytes_to_base64(data: &[u8]) -> String {
    base64::prelude::BASE64_STANDARD.encode(data)
}

pub fn from_base64(data: &str) -> Result<Vec<u8>> {
    Ok(general_purpose::STANDARD.decode(data.as_bytes())?)
}
