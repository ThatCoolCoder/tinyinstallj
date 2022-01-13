use std::env;
use std::fs;
use std::path::Path;

use bytes::Bytes;

#[path = "./config.rs"]
mod config;

use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

pub fn install_jar(jar_bytes: Bytes) -> bool {
    let binary_directory = match env::consts::OS {
        "windows" => "C:\\Program Files\\",
        _ => "/usr/bin/"
    };
    let runner_script_name = match env::consts::OS {
        "windows" => config::SIMPLE_PROGRAM_NAME.to_owned() + ".bat",
        _ => config::SIMPLE_PROGRAM_NAME.to_string()
    };
    let installed_jar_path = Path::new(binary_directory).join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".jar")
        .into_os_string().into_string().unwrap();
    let runner_script_path = Path::new(binary_directory).join(runner_script_name.as_str())
        .into_os_string().into_string().unwrap();
    let runner_script_contents = match env::consts::OS {
        "windows" => format!("java -jar {}", installed_jar_path),
        _ => format!("#!/bin/sh
            java -jar {}", installed_jar_path)
    };

    match fs::write(installed_jar_path, jar_bytes) {
        Ok(_v) => (),
        Err(_e) => return false
    }

    match fs::write(runner_script_path, runner_script_contents) {
        Ok(_v) => (),
        Err(_e) => return false
    }

    if env::consts::OS != "linux" {
        let perms = Permissions::from_mode(0o644);
        match fs::set_permissions(runner_script_path, perms) {
            Ok(_v) => (),
            Err(_e) => return false
        }
    }

    return true;
}