use std::fs;

use bytes::Bytes;
use reqwest::blocking;

#[path = "./config.rs"]
mod config;

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

pub fn save_jar(jar_bytes: Bytes) -> bool {
    return fs::write(config::SIMPLE_PROGRAM_NAME.to_owned() + ".jar", jar_bytes).is_ok();
}