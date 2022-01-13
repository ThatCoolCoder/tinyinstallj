use std::env;
use std::fs;
use std::path::Path;

use bytes::Bytes;

#[path = "./config.rs"]
mod config;

pub fn install_jar(jar_bytes: Bytes) -> bool {
    let binary_directory = match env::consts::OS {
        "windows" => "C:\\Program Files\\",
        _ => "/usr/bin/"
    };
    let runner_script_name = match env::consts::OS {
        "windows" => config::SIMPLE_PROGRAM_NAME.to_owned() + ".bat",
        _ => config::SIMPLE_PROGRAM_NAME.to_string()
    };
    let installed_jar_path = Path::new(binary_directory).join(config::SIMPLE_PROGRAM_NAME)
        .into_os_string().into_string().unwrap();
    let runner_script_contents = match env::consts::OS {
        "windows" => "",
        _ => format!("#!/bin/sh
            .{}", installed_jar_path).as_str()
    };
    
    match fs::write(installed_jar_path, jar_bytes) {
        Ok(_v) => (),
        Err(_e) => return false
    }

    return true;
}