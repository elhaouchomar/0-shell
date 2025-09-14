use std::io::{self, Write};
use std::path::PathBuf;

pub fn display_prompt(multi_line: bool, current_path: &PathBuf) {
    if multi_line {
        print!("> ");
        io::stdout().flush().unwrap();
        return;
    }

    let mut home_dir = PathBuf::new();
    
    match get_home_dir() {
        Some(dir) => home_dir = dir,
        None => {
            print!("err getting home path");
            io::stdout().flush().unwrap();
            return;
        }
    };

    let mut prompt_path = current_path.clone();
    if current_path.starts_with(&home_dir) {
        if let Ok(path_suffix) = current_path.strip_prefix(&home_dir) {
            prompt_path = PathBuf::from("~");
            prompt_path.push(path_suffix);
        }
    }

    // Path = bold blue, $ = bold red
    print!(
        "\x1b[1;38;5;33m{}\x1b[0m\x1b[1;31m$\x1b[0m ",
        prompt_path.display()
    );
    io::stdout().flush().unwrap();
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
