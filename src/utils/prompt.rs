use std::io::{self, Write};
use std::path::PathBuf;

pub fn display_prompt(multi_line: bool, current_path: &PathBuf) {
    if multi_line {
        print!("> ");
        io::stdout().flush().unwrap();
        return;
    }

    let home_dir = get_home_dir().unwrap_or_else(|| PathBuf::from("/"));

    let mut prompt_path = current_path.clone();
    if current_path.starts_with(&home_dir) {
        if let Ok(path_suffix) = current_path.strip_prefix(&home_dir) {
            prompt_path = PathBuf::from("~");
            prompt_path.push(path_suffix);
        }
    }
    print!("\x1b[38;5;33m{}\x1b[0m$ ", prompt_path.display());
    io::stdout().flush().unwrap();
}

pub fn get_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}