use std;
use std::path::PathBuf;

use bytes::Bytes;
// use lnk::ShellLink;

#[cfg(target_family = "windows")]
use winreg::RegKey;

use super::config;
use super::get_install_paths::InstallPaths;

pub fn setup_install_dir(install_paths: &InstallPaths) -> Result<(), String> {
    // Make sure the target dir exists
    match std::fs::create_dir_all(&install_paths.base_dir) {
        Ok(_v) => (),
        Err(_e) => return Err(format!("Failed to create directory {}", &install_paths.base_dir.to_string_lossy()))
    };
    return match append_to_path_var(&install_paths.base_dir.to_string_lossy()) {
        Ok(_v) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn save_jar(install_paths: &InstallPaths, jar_bytes: Bytes) -> Result<(), String> {
    return match std::fs::write(&install_paths.jar, jar_bytes) {
        Ok(_v) => Ok(()),
        Err(_e) => Err(format!("Failed to write {}", &install_paths.jar.to_string_lossy()))
    };
}

pub fn create_runner_script(install_paths: &InstallPaths) -> Result<(), String> {
    let runner_script_contents = match std::env::consts::OS {
        "windows" => format!("@ECHO OFF
            START /B javaw -jar \"{}\"", &install_paths.jar.to_string_lossy()),
        _ => format!("#!/bin/sh
            java -jar \"{}\"", &install_paths.jar.to_string_lossy())
    };

    match std::fs::write(&install_paths.runner_script, runner_script_contents) {
        Ok(_v) => (),
        Err(_e) => return Err(format!("Failed to write {}", &install_paths.runner_script.to_string_lossy()))
    };

    // On linux, we need to chmod +x to make it executable
    if std::env::consts::OS != "windows" {
        match set_executable_bit(&install_paths.runner_script) {
            Ok(_v) => (),
            Err(e) => return Err(e)
        }
    }
    return Ok(());
}

pub fn create_uninstall_script(install_paths: &InstallPaths) -> Result<(), String> {
    // Windows doesn't like a batch file deleting itself.
    // (goto) 2>nul & del "%~f0" is a tricky way to get around this.

    let mut uninstall_script_contents = match std::env::consts::OS {
        "windows" => "@ECHO OFF",
        // On linux make sure that we are root
        _ => "#!/bin/sh
            if [ \"$UID\" != 0 ]
            then
                echo 'You must be root to run this script'
                exit
            fi"
    }.to_owned();
    let mut paths = vec![&install_paths.runner_script, &install_paths.jar, &install_paths.icon];
    if std::env::consts::OS != "windows" {
        paths.push(&install_paths.uninstall_script);
    }
    for path in paths {
        uninstall_script_contents += match std::env::consts::OS {
            "windows" => format!("\r\ndel \"{}\"", path.to_string_lossy()),
            _ => format!("\nrm '{}'", path.to_string_lossy())
        }.as_str();
    }

    if std::env::consts::OS == "windows" {
        uninstall_script_contents += &("\n".to_owned() + "(goto) 2>nul & del \"%~f0\"");
    }

    match std::fs::write(&install_paths.uninstall_script, uninstall_script_contents) {
        Ok(_v) => (),
        Err(_e) => return Err(format!("Failed to write {}", &install_paths.uninstall_script.to_string_lossy()))
    };

    // On linux, we need to chmod +x to make it executable
    if std::env::consts::OS != "windows" {
        match set_executable_bit(&install_paths.uninstall_script) {
            Ok(_v) => (),
            Err(e) => return Err(e)
        }
    }
    return Ok(()); 
}

#[cfg(target_family = "unix")]
pub fn create_desktop_link(install_paths: &InstallPaths) -> Result<(), String> {
    // With Rust 1.58 formatting can be done better & more easily,
    // but I want to support older versions
    let content = format!("
        [Desktop Entry]
        Encoding=UTF-8
        Version=1.0
        Type=Application
        Terminal={is_console_app}
        Exec=java -jar {jar_path}
        Name={full_program_name}
        Icon={icon_path}",
        is_console_app = config::IS_CONSOLE_APP,
        jar_path = install_paths.jar.to_string_lossy(),
        icon_path = install_paths.icon.to_string_lossy(),
        full_program_name = config::FULL_PROGRAM_NAME);

    match std::fs::write(&install_paths.desktop_link, content) {
        Ok(_v) => (),
        Err(_e) => return Err(format!("Failed to write {}", &install_paths.desktop_link.to_string_lossy()))
    };
    return Ok(());
}

#[cfg(target_family = "windows")]
pub fn create_desktop_link(_install_paths: &InstallPaths) -> Result<(), String> {
    println!("Creating links isn't supported on windows yet");
    return Ok(());
}

fn set_executable_bit(path: &PathBuf) -> Result<(), String> {
    let err_text = format!("Failed changing permissions of {}", &path.to_string_lossy());
    match std::process::Command::new("chmod")
        .arg("+x")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg(&*path.to_string_lossy())
        .status() {

        Ok(status) => {
            match status.code() {
                Some(code) => {
                    if code > 0 {
                        return Err(err_text);
                    }
                }
                None => return Err(err_text)
            }
        },
        Err(_e) => return Err(err_text)
    };
    return Ok(());
}

#[cfg(target_family = "unix")]
fn append_to_path_var(_new_path: &str) -> Result<(), String> {
    // Do nothing on unix since we only need it on windows
    return Ok(());
}

#[cfg(target_family = "windows")]
fn append_to_path_var(new_path: &str) -> Result<(), String> {
    let error_message = format!("Failed to add {} to path", new_path).to_owned();
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let (env, _) = match hkcu.create_subkey("Environment") {
        Ok(value) => value,
        Err(_e) => return Err(error_message)
    };

    let mut full_path: String = env.get_value("Path").unwrap();
    if ! full_path.contains((";".to_owned() + new_path + ";").as_str()) &&
        ! full_path.ends_with(&(";".to_owned() + new_path)) {

        full_path += (String::from(";") + new_path).as_str();
    }
    return match env.set_value("Path", &full_path) {
        Ok(_v) => Ok(()),
        Err(_e) => Err(error_message)
    };
}