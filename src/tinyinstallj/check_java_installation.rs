use std::env;

use super::config;
use super::utils;

pub fn get_java_path() -> Option<String> {
    let executable_name = match env::consts::OS {
        "windows" => "java.exe",
        _ => "java"
    };
    return utils::get_program_path(&executable_name.to_owned());
}

pub fn get_java_version(path: String) -> i32 {
    let output = match std::process::Command::new(path)
        .arg("-fullversion")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output() {
        // Weirdly openjdk (haven't tested Oracle Java) writes version to stderr
        Ok(output) => String::from_utf8_lossy(&output.stderr).to_string(),
        Err(_e) => {
            return get_java_version_from_user()
        }
    };

    let words: Vec<&str> = output.split(' ').collect();
    let index_of_version = match words.iter().position(|&x| x == "version"){
        Some(idx) => match idx < words.len() - 1 {
            true => idx + 1,
            false => return get_java_version_from_user()
        }
        None => return get_java_version_from_user()
    };
    let mut version_str = words[index_of_version].to_owned();
    version_str = version_str.replace('"', "");
    let version_numbers: Vec<&str> = version_str.split('.').collect();
    if version_numbers.len() < 2 {
        return get_java_version_from_user();
    }
    let mut version_number_idx = 0;
    // If java version is something like 1.7.505, then we use the '7'
    if version_numbers[0] == "1" {
        version_number_idx = 1;
    }
    let true_version = match version_numbers[version_number_idx].parse::<i32>() {
        Ok(v) => v,
        Err(_e) => return get_java_version_from_user()
    };

    return true_version;
}

fn get_java_version_from_user() -> i32 {
    println!("Failed to get Java version automatically.");
    return match utils::ask_yn(format!("Does your computer have JRE version {} or greater installed?", config::MIN_JAVA_VERSION).as_str(), false) {
        true => config::MIN_JAVA_VERSION,
        false => 0
    };
}