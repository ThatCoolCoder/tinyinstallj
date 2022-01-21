use std;
use std::path::PathBuf;

use webbrowser;
use bytes::Bytes;

#[cfg(target_family = "unix")]
use isatty;

pub mod check_is_admin;
pub mod check_java_installation;
pub mod download;
pub mod install;
pub mod config;
pub mod utils;
pub mod get_install_paths;

use get_install_paths::InstallPaths;

pub fn install(force_install: bool) {
    check_is_in_terminal();
    if ! check_is_admin::is_admin() {
        on_user_not_admin();
    }

    show_introduction();
    println!("");

    match utils::ask_yn("Do you want to proceed with installation?", false) {
        true => println!(""),
        false => cancel_installation()
    };

    // We do this before everything else in case it fails 
    let install_paths = match get_install_paths::get_install_paths() {
        Some(v) => v,
        None => {
            on_failed_to_determine_path();
            return;
        }
    };
    
    if force_install {
        println!("Skipping Java checks because installer was run with the -f flag\n")
    }
    else {
        println!("Searching for JRE...");
        let java_path = match check_java_installation::get_java_path() {
            Some(path) => {
                println!("Found JRE: {}\n", path);
                path
            },
            None => {
                on_general_java_issue();
                return;
            }
        };

        println!("Checking JRE version...");
        let java_version = check_java_installation::get_java_version(java_path);
        println!("Found Java {}", java_version);
        if java_version >= config::MIN_JAVA_VERSION {
            println!("Java version sufficient\n");
        }
        else {
            on_general_java_issue();
        }
    }

    println!("Downloading {}...", config::JAR_URL);
    let jar_bytes = download_with_error_handling(config::JAR_URL.to_owned()).unwrap();
    println!("Downloading {}...", config::ICON_URL);
    let icon_bytes = download_with_error_handling(config::ICON_URL.to_owned()).unwrap();
    println!("Finished downloads\n");

    output_result("Setting up installation directory...", install::setup_install_dir(&install_paths));
    output_result("Saving jar file...", save_with_error_handling(&install_paths.jar, jar_bytes));
    output_result("Saving icon...", save_with_error_handling(&install_paths.icon, icon_bytes));
    output_result("Creating runner script...", install::create_runner_script(&install_paths));
    output_result("Creating uninstaller...", install::create_uninstall_script(&install_paths));
    if ! config::IS_CONSOLE_APP {
        output_result("Creating application shortcut...", install::create_app_link(&install_paths));
    }
    println!("");

    println!("Installation complete.\n");
    if std::env::consts::OS == "windows" {
        println!("You may need to sign out and sign back in for changes to take effect");
        println!("(This will be fixed in a later version of tinyinstallj)\n");
    }
    println!("{} has been installed as \"{}\"", config::FULL_PROGRAM_NAME, config::SIMPLE_PROGRAM_NAME);
    show_uninstall_instructions(&install_paths);
    println!("");
    utils::get_input("Press enter to exit");
}

// "Events" - main user interation, moved out of the main function to make it more concise

fn output_result(task_description: &str, result: Result<(), String>) {
    println!("{}", task_description);
    match result {
        Ok(_v) => (),
        Err(e) => {
            println!("{}\n", e);
            cancel_installation();
        }
    }
}

fn download_with_error_handling(file_url: String) -> Option<Bytes> {
    return match download::download_file(file_url) {
        Ok(bytes) => Some(bytes),
        Err(e) => {
            println!("{}", e);
            cancel_installation();
            return None;
        }
    };
}

fn save_with_error_handling(file_path: &PathBuf, bytes: Bytes) -> Result<(), String> {
    return match std::fs::write(file_path, &bytes) {
        Ok(v) => Ok(v),
        Err(_e) => Err(format!("Failed to write {}", file_path.to_string_lossy()))
    }
}

fn on_user_not_admin() {
    match std::env::consts::OS {
        "windows" => println!("You must be an administrator to run this installer."),
        _ => println!("This installer must be run as root")
    }
    cancel_installation();
}


#[cfg(target_family = "unix")]
fn check_is_in_terminal() {
    // If we're not being run in a tty (ex someone just double-clicked the program),
    // then fight to end up running in a terminal.
    // First tries launching a bunch of terminals and if none works tries to make a popup
    // on the user's desktop

    if ! isatty::stdout_isatty() {
        let executable_path = match std::env::current_exe() {
            Ok(v) => v,
            Err(_e) => {
                ask_user_to_run_in_terminal();
                return
            }
        }.to_string_lossy().to_string();
        


        // List of terminal emulators with best higher and most reliable lower.
        // It would be great to support gnome-terminal but it gives errors about
        // dbus not setting up correctly.
        // Weirdly, my system shows me any of these four randomly, so it's not hugely reliable

        // Get issues about using a temporary value when declaring this var inline the hashmap below,
        // so move it out
        let with_quotes = format!("sudo {}", executable_path);
        let terminal_emulators = std::collections::HashMap::from([
            ("konsole", vec!["-e", "sudo", &executable_path]),
            ("alacritty", vec!["-e", "sudo", &executable_path]),
            ("xfce4-terminal", vec!["-e",
                &with_quotes]),
            ("xterm", vec!["-e", "sudo", &executable_path])
        ]);

        for (emulator_name, args) in terminal_emulators {
            if utils::get_program_path(&emulator_name.to_owned()).is_some() {
                if std::process::Command::new(emulator_name)
                    .args(args)
                    .spawn()
                    .is_ok() {
                    
                    // If the terminal is totally successful, exit this and let it run
                    std::process::exit(0);
                }
                // Break if we found a terminal.
                // The terminal might not have worked but it's better to create no terminals
                // and use a fallback popup rather than create none
                break;
            }
        }

        // If none of those terminals worked, then give up and ask the user to do it
        ask_user_to_run_in_terminal();
    }
}

#[cfg(target_family = "unix")]
fn ask_user_to_run_in_terminal() {
    match std::process::Command::new("notify-send")
        .arg("-t")
        .arg("0")
        .arg(format!("Error installing {}", config::FULL_PROGRAM_NAME))
        .arg("This installer must be run in a terminal as root")
        .spawn() {
        Ok(_v) => (),
        Err(_e) => eprintln!("aaaahahdadflf")
    }
    std::process::exit(1);
}

#[cfg(target_family = "windows")]
fn check_is_in_terminal() {
    // Do nothing - rust programs open in CMD in windows by default
}

fn show_introduction() {
    println!("\n\nWelcome to the installer for {}", config::FULL_PROGRAM_NAME);
    println!("(Created with tinyinstallj - https://github.com/ThatCoolCoder/tinyinstallj)\n");
    println!("For those unfamiliar with using terminal-based programs, you enter values by typing something in and pressing enter/return");
}

fn show_uninstall_instructions(install_paths: &InstallPaths) {
    match std::env::consts::OS {
        "windows" => println!("To uninstall {}, run {}", config::FULL_PROGRAM_NAME, install_paths.uninstall_script.to_string_lossy()),
        _ => println!("To uninstall {}, run {} as root", config::FULL_PROGRAM_NAME, install_paths.uninstall_script.to_string_lossy())
    }
}

fn on_general_java_issue() {
    println!("A suitable Java installation was not found on your system");
    println!("This program requires a minimum Java version of {} to run", config::MIN_JAVA_VERSION);
    println!("    (Those who know what they're doing can force install with the -f flag)");
    if utils::ask_yn("Would you like this installer to direct you to download options for Java?", true) {
        if ! webbrowser::open("https://docs.microsoft.com/en-us/java/openjdk/download").is_ok() {
            println!("Failed to open web browser automatically, please navigate to https://docs.microsoft.com/en-us/java/openjdk/download");
        }
    }
    cancel_installation();
}

fn on_failed_to_determine_path() {
    println!("Failed to determine installation paths.");
    cancel_installation();
}

fn cancel_installation() {
    utils::get_input("Cancelled installation. Press enter to exit.");
    std::process::exit(0);
}