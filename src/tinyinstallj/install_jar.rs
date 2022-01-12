use std;

use bytes::Bytes;

#[path = "./utils.rs"]
mod utils;
#[path = "./config.rs"]
mod config;

pub fn install_jar(jar_bytes: Bytes) -> Option<Vec<String>> {
    // Figure out some paths and file contents
    let binary_directory = match std::env::consts::OS {
        "windows" => "C:\\Program Files\\",
        _ => "/usr/bin/"
    };
    let runner_script_name = match std::env::consts::OS {
        "windows" => config::SIMPLE_PROGRAM_NAME.to_owned() + ".bat",
        _ => config::SIMPLE_PROGRAM_NAME.to_string()
    };
    let installed_jar_path = std::path::Path::new(binary_directory).join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".jar")
        .into_os_string().into_string().unwrap();
    let runner_script_path = std::path::Path::new(binary_directory).join(runner_script_name.as_str())
        .into_os_string().into_string().unwrap();
    let runner_script_contents = match std::env::consts::OS {
        "windows" => format!("java -jar \"{}\"", installed_jar_path),
        _ => format!("#!/bin/sh
            java -jar \"{}\"", installed_jar_path)
    };

    match std::fs::write(&installed_jar_path, jar_bytes) {
        Ok(_v) => (),
        Err(_e) => return None
    }

    match std::fs::write(&runner_script_path, runner_script_contents) {
        Ok(_v) => (),
        Err(_e) => return None
    }

    // Try chmoding
    if std::env::consts::OS != "windows" {
        match std::process::Command::new("chmod")
            .arg("+x")
            .arg(&runner_script_path)
            .status() {

            Ok(status) => {
                match status.code() {
                    Some(code) => {
                        if code > 0 {
                            return None;
                        }
                    }
                    None => return None
                }
            },
            Err(_e) => return None
        };
    }

    // Add dir to path
    if std::env::consts::OS == "windows" {
        utils::append_to_path("\"C:\\Program Files\"");
    }

    return Some(vec![runner_script_path, installed_jar_path]);
}