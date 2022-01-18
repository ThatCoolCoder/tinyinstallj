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