use std;

use webbrowser;

pub mod check_is_admin;
pub mod check_java_installation;
pub mod download;
pub mod install;
pub mod config;
pub mod utils;
pub mod get_install_paths;

use get_install_paths::InstallPaths;

pub fn install(force_install: bool) {
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

    println!("Downloading {}...", config::JAR_FILE_URL);
    let jar_bytes = match download::download_jar() {
        Some(bytes) => bytes,
        None => {
            cancel_installation();
            return;
        }
    };
    println!("Finished download\n");

    output_result("Setting up installation directory...", install::setup_install_dir(&install_paths));
    output_result("Saving jar file...", install::save_jar(&install_paths, jar_bytes));
    output_result("Creating runner script...", install::create_runner_script(&install_paths));
    output_result("Creating uninstaller...", install::create_uninstall_script(&install_paths));
    if ! config::IS_CONSOLE_APP {
        output_result("Creating application shortcut...", install::create_desktop_link(&install_paths));
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

fn on_user_not_admin() {
    match std::env::consts::OS {
        "windows" => println!("You must be an administrator to run this installer."),
        _ => println!("This installer must be run as root")
    }
    cancel_installation();
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
    println!("");
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