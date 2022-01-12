use std::env;
use std::path::Path;

#[path = "./config.rs"]
mod config;
#[path = "./utils.rs"]
mod utils;

pub fn get_java_path() -> Option<String> {
    let path_splitter = match env::consts::OS {
        "windows" => ';',
        _ => ':'
    };
    let executable_name = match env::consts::OS {
        "windows" => "java.exe",
        _ => "java"
    };

    let system_path = match env::var("PATH")  {
        Ok(x) => x,
        Err(_e) => return None
    };
    for path_dir in system_path.split(path_splitter) {
        let path = Path::new(path_dir).join(executable_name);
        if path.exists() {
            return Some(path.into_os_string().into_string().unwrap());
        }
    }
    return None;
}

pub fn java_version_sufficient(path: String) -> bool {
    println!("Automated JRE version checking has not been implemented yet");
    return utils::ask_yn(format!("Does your computer have JRE version {} or greater installed?", config::MIN_JAVA_VERSION).as_str(), false)
}