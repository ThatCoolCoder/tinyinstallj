use std::path::{Path, PathBuf};

use dirs;
use platform_dirs::AppDirs;

use super::config;

pub struct InstallPaths {
    pub base_dir: PathBuf,
    pub runner_script: PathBuf,
    pub uninstall_script: PathBuf,
    pub app_link: PathBuf,
    pub jar: PathBuf,
    pub icon: PathBuf
}

pub fn get_install_paths() -> Option<InstallPaths> {
    let is_windows: bool = std::env::consts::OS == "windows";
    let app_dirs = AppDirs::new(None, true).unwrap();

    let base_dir = match is_windows {
        true => {
            match dirs::home_dir() {
                Some(v) => v.join("tinyinstallj-programs"),
                None => return None
            }
        }
        _ => Path::new("/usr/bin/").to_path_buf()
    };
    let runner_script_name = match is_windows {
        true => config::SIMPLE_PROGRAM_NAME.to_owned() + ".bat",
        _ => config::SIMPLE_PROGRAM_NAME.to_string()
    };
    let uninstall_script_name = match is_windows {
        true => format!("{}-uninstall.bat", config::SIMPLE_PROGRAM_NAME),
        _ => format!("{}-uninstall", config::SIMPLE_PROGRAM_NAME)
    };

    let jar_path = Path::new(&base_dir).join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".jar");
    let icon_path = match is_windows {
        true => Path::new(&base_dir).join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".ico"),
        _ => Path::new("/usr/share/icons/").join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".ico")
    };
    let runner_script_path = Path::new(&base_dir).join(runner_script_name.as_str());
    let uninstall_script_path = Path::new(&base_dir).join(uninstall_script_name.as_str());
    let app_link_path = match is_windows {
        true => app_dirs.config_dir
            .join(r#"Microsoft\Windows\Start Menu\Programs"#)
            .join(config::FULL_PROGRAM_NAME.to_owned() + ".lnk"),
        _ => Path::new("/usr/share/applications").join(config::SIMPLE_PROGRAM_NAME.to_owned() + ".desktop")
    };

    return Some(InstallPaths {
        base_dir,
        runner_script: runner_script_path,
        uninstall_script: uninstall_script_path,
        app_link: app_link_path,
        jar: jar_path,
        icon: icon_path,
    });
}