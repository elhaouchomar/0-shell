use std::env::current_dir;

pub fn cmd_pwd() {
    match current_dir() {
        Ok(path) => {
            println!("{}",path.display());
        },
        Err(_) => {
            println!("\x1b[31mpwd\x1b[0m: Error getting current directory");
        }
    }
}