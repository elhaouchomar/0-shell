use std::io::{self, Write};
use std::path::PathBuf;

pub fn display_prompt(multi_line: bool, current_path: &PathBuf) {
    if multi_line {
        print!("> ");
        if io::stdout().flush().is_err(){
            eprintln!("cannot read line");
        };
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
   if io::stdout().flush().is_err(){
        eprintln!("cannot read line")
    };
}

pub fn get_home_dir() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        std::env::var_os("HOME").map(PathBuf::from)
    }

    #[cfg(windows)]
    {
        std::env::var_os("USERPROFILE").map(PathBuf::from)
    }
}