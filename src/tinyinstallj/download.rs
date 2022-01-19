use bytes::Bytes;
use reqwest::blocking;

use super::config;

pub fn download_file(file_url: String) -> Result<Bytes, String> {
    let error_message = format!("Failed to download {}", file_url);
    let result = blocking::get(file_url);
    let response = match result {
        Ok(response) => response,
        Err(_e) => return Err(error_message)
    };
    let file_bytes = match response.bytes() {
        Ok(bytes) => bytes,
        Err(_e) => return Err(error_message)
    };
    return Ok(file_bytes)
}