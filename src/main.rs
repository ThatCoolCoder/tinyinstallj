use std::env;

mod tinyinstallj;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut force_install = false;
    if args.len() > 1 && args[1] == "-f" {
        force_install = true;
        println!("forcing!");
    }
    tinyinstallj::install(force_install);
}
