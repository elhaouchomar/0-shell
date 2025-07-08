use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn display_prompt() {
    let path = env::current_dir().unwrap_or(PathBuf::new());
    let last_folder = path.file_name()
    .and_then(|os_str| os_str.to_str())
    .unwrap_or("");
    // match dirs::home_dir() {
    //     Some(dir) => println!("hooooooom: {}", dir.display()),
    //     None => print!("nothiiiiiiiiiiiiiiiing ")
    // };

    print!("{}$ ", last_folder);
    io::stdout().flush().unwrap();
}