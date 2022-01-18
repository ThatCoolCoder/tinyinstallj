use std;

pub mod check_is_admin;
pub mod check_java_installation;
pub mod download;
pub mod install;
pub mod config;
pub mod utils;
pub mod get_install_paths;

use get_install_paths::InstallPaths;

pub fn install() {
    show_introduction();
    println!("");

    if ! check_is_admin::is_admin() {
        on_user_not_admin();
    }

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

    println!("Searching for JRE...");
    let java_path = match check_java_installation::get_java_path() {
        Some(path) => {
            println!("Found JRE: {}\n", path);
            path
        },
        None => {
            on_java_not_found();
            return;
        }
    };

    println!("Checking JRE version...");
    match check_java_installation::java_version_sufficient(java_path) {
        true => println!("Java version sufficient\n"),
        false => on_insufficient_java()
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
    if utils::ask_yn("Do you want to create a desktop shortcut?", true) {
        output_result("Creating desktop shortcut...", install::create_desktop_link(&install_paths));
    }
    println!("");

    println!("Installation complete.\n");
    show_uninstall_instructions(&install_paths);
    println!("");
    if std::env::consts::OS == "windows" {
        println!("You will need to sign out and sign back in for changes to take effect");
        println!("(This will be fixed in a later version of tinyinstallj)\n");
    }
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

fn on_java_not_found() {
    println!("No Java installation was found on your system\n");
    on_general_java_issue();
}

fn on_insufficient_java() {
    println!("The Java version installed on your system is not sufficient\n");
    on_general_java_issue();
}

fn on_general_java_issue() {
    println!(concat!("It is recommended that you quit the installation and install a suitable Java version, ",
        "but those who know what they're doing can opt to continue."));
    let result = utils::ask_yn("Do you wish to quit?", true);
    println!("");
    if result {
        cancel_installation();
    }
}

fn on_failed_to_determine_path() {
    println!("Failed to determine installation paths.");
    cancel_installation();
}

fn cancel_installation() {
    utils::get_input("Cancelled installation. Press enter to exit.");
    std::process::exit(0);
}