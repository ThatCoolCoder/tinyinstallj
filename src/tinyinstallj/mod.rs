pub mod check_is_admin;
pub mod check_java_installation;
pub mod get_jar;
pub mod config;
pub mod utils;

pub fn install() {
    show_introduction();
    println!("");

    if ! check_is_admin::is_admin() {
        on_user_not_admin();
    }

    match utils::ask_yn("Do you want to proceed with installation?", false) {
        true => println!(""),
        false => cancel_installation()
    }

    println!("Searching for JRE...");
    let java_path = match check_java_installation::get_java_path() {
        Some(path) => {
            println!("Found JRE: {}\n", path);
            path
        },
        None => {
            on_java_not_found();
            return ();
        }
    };

    println!("Checking JRE version...");
    match check_java_installation::java_version_sufficient(java_path) {
        true => println!("Java version sufficient\n"),
        false => on_insufficient_java()
    }

    println!("Downloading {}...", config::JAR_FILE_URL);
    let bytes = match get_jar::download_jar() {
        Some(bytes) => bytes,
        None => {
            cancel_installation();
            return ();
        }
    };
    println!("Finished download\n");

    println!("Saving file to disk...");
    match get_jar::save_jar(bytes) {
        true => println!("Saved file\n"),
        false => {
            println!("Failed to save file (check permissions?)");
            cancel_installation();
        }
    }

    println!("That's the end of the installer. There's probably a .jar file in your downloads folder that you can run now.");
}

fn on_user_not_admin() {
    // todo: uncomment this for prod
    // println!("You must be an administrator to run this installer.");
    // cancel_installation();
}

fn show_introduction() {
    println!("\n\nWelcome to the installer for {}", config::FULL_PROGRAM_NAME);
    println!("(Created with tinyinstallj - https://github.com/ThatCoolCoder/tinyinstallj)\n");
    println!("For those unfamiliar with using terminal-based programs, you enter values by typing something in and pressing enter/return");
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

fn cancel_installation() {
    println!("Cancelling installation...");
    std::process::exit(0);
}