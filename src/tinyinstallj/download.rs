use bytes::Bytes;
use reqwest::blocking;

use super::config;

pub fn download_jar() -> Option<Bytes>{
    let result = blocking::get(config::JAR_FILE_URL);
    let response = match result {
        Ok(response) => response,
        Err(_e) => return None
    };
    let file_bytes = match response.bytes() {
        Ok(bytes) => bytes,
        Err(_e) => return None
    };
    return Some(file_bytes)
}