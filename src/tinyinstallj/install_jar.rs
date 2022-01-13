use std;

use bytes::Bytes;

#[path = "./config.rs"]
mod config;

use std::os::unix::fs::PermissionsExt;

pub fn install_jar(jar_bytes: Bytes) -> bool {
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
        "windows" => format!("java -jar {}", installed_jar_path),
        _ => format!("#!/bin/sh
            java -jar {}", installed_jar_path)
    };

    match std::fs::write(&installed_jar_path, jar_bytes) {
        Ok(_v) => (),
        Err(_e) => return false
    }

    match std::fs::write(&runner_script_path, runner_script_contents) {
        Ok(_v) => (),
        Err(_e) => return false
    }

    // Try chmoding
    if std::env::consts::OS != "windows" {
        match std::process::Command::new("chmod")
            .arg("+x")
            .arg(runner_script_path)
            .status() {

            Ok(status) => {
                match status.code() {
                    Some(code) => {
                        if code > 0 {
                            return false;
                        }
                    }
                    None => return false
                }
            },
            Err(_e) => return false
        };
    }

    return true;
}