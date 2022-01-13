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