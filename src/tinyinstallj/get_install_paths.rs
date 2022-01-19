use std::path::{Path, PathBuf};

use dirs;

use super::config;

pub struct InstallPaths {
    pub base_dir: PathBuf,
    pub runner_script: PathBuf,
    pub uninstall_script: PathBuf,
    pub desktop_link: PathBuf,
    pub jar: PathBuf
}

pub fn get_install_paths() -> Option<InstallPaths> {
    let base_dir = match std::env::consts::OS {
        "windows" => {
            match dirs::home_dir() {
                Some(v) => v.join("tinyinstallj-programs"),
                None => return None
            }
        }
        _ => Path::new("/usr/bin/").to_path_buf()
    };
    let runner_script_name = match std::env::consts::OS {
        "windows" => config::SIMPLE_PROGRAM_NAME.to_owned() + ".bat",
        _ => config::SIMPLE_PROGRAM_NAME.to_string()
    };
    let uninstall_script_name = match std::env::consts::OS {
        "windows" => format!("{}-uninstall.bat", config::SIMPLE_PROGRAM_NAME),
        _ => format!("{}-uninstall", config::SIMPLE_PROGRAM_NAME)
    };

    let jar_path = Path::new(&base_dir).join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".jar");
    let runner_script_path = Path::new(&base_dir).join(runner_script_name.as_str());
    let uninstall_script_path = Path::new(&base_dir).join(uninstall_script_name.as_str());
    let desktop_link_path = match std::env::consts::OS {
        "windows" => Path::new(&base_dir).join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".lnk"),
        _ => Path::new("/usr/share/applications").join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".desktop")
    };

    return Some(InstallPaths {
        base_dir,
        runner_script: runner_script_path,
        uninstall_script: uninstall_script_path,
        desktop_link: desktop_link_path,
        jar: jar_path
    });
}