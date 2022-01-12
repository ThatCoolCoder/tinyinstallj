use std;
use std::io::{Write};

pub fn ask_yn(prompt: &str, default_response: bool) -> bool {
    let input = get_input(format!("{} (y/n) ", prompt).as_str());
    
    if input == "y" {
        return true;
    }
    else if input == "n" {
        return false;
    }
    else {
        return default_response;
    }
}

pub fn get_input(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    line = line.trim().to_string();
    return line;
}

pub fn append_to_path(new_path: &str) -> Result<(), std::env::JoinPathsError> {
    if let Some(path) = std::env::var_os("PATH") {
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        if ! paths.contains(&std::path::PathBuf::from(new_path)) {
            paths.push(std::path::PathBuf::from(new_path));
            let new_path = std::env::join_paths(paths)?;
            std::env::set_var("PATH", &new_path);
        }
    }

    Ok(())
}