use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn display_prompt(multi_line: bool) {
    if multi_line {
        print!("> ");
        io::stdout().flush().unwrap();
        return
    }

    let path = env::current_dir().unwrap_or(PathBuf::new());
    let mut home_dir = PathBuf::new();
    
    match get_home_dir() {
        Some(dir) => home_dir = dir,
        None => print!("err geting home path")
    };

    let mut prompt_path = PathBuf::new();
    if path.starts_with(&home_dir) {
        let path_sufix = path.strip_prefix(&home_dir).unwrap();
        prompt_path = PathBuf::from("~");
        prompt_path.push(path_sufix);
    }

    print!("\x1b[38;5;33m{}\x1b[0m$ ", prompt_path.display());
    io::stdout().flush().unwrap();
}

fn get_home_dir() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        env::var_os("HOME").map(PathBuf::from)
    }

    #[cfg(windows)]
    {
        env::var_os("USERPROFILE").map(PathBuf::from)
    }
}