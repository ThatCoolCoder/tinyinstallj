use std;
use std::io::{Write};

// For some reason rust is not picking up that we import these in other files,
// so stop it complaining
// #[allow(dead_code)]

pub fn ask_yn(prompt: &str, default_response: bool) -> bool {
    let full_prompt = match default_response {
        true => format!("{} (Y/n) ", prompt),
        false => format!("{} (y/N) ", prompt)
    };
    let input = get_input(full_prompt.as_str());
    
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

pub fn get_program_path(program_name: &String) -> Option<String> {
    // Try and find the full path of a program on the PATH env var
    let path_splitter = match std::env::consts::OS {
        "windows" => ';',
        _ => ':'
    };

    let system_path = match std::env::var("PATH")  {
        Ok(x) => x,
        Err(_e) => return None
    };
    for path_dir in system_path.split(path_splitter) {
        let path = std::path::Path::new(path_dir).join(&program_name);
        if path.exists() {
            return Some(path.into_os_string().into_string().unwrap());
        }
    }
    return None;
}