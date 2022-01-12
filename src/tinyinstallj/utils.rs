use std::io::{Write};

pub fn ask_yn(prompt: &str, default_response: bool) -> bool {

    let mut line = String::new();
    print!("{} (y/n) ", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    line = line.trim().to_string();

    if line == "y" {
        return true;
    }
    else if line == "n" {
        return false;
    }
    else {
        return default_response;
    }
}