use std::path::Path;
use std::fs;
use std::path::PathBuf;

struct LsOptions {
    show_hidden: bool,
    show_long: bool,
    show_suffix: bool,
}

pub fn cmd_ls(name: &String, args: &Vec<String>) {
    let mut options = LsOptions {
        show_hidden: false,
        show_long: false,
        show_suffix: false,
    };

    let mut paths = Vec::new();
    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'l' => {
                        options.show_long = true;
                    },
                    'a' => {
                        options.show_hidden = true;
                    },
                    'F' => {
                        options.show_suffix = true;
                    },
                    _ => {
                        println!("\x1b[33mls\x1b[0m: Not a valid option {}", arg);
                        return
                    }
                }
            }
        } else {
            paths.push(arg.to_string())
        }
    }

    match name.as_str() {
        "la" => {
            options.show_hidden = true;
        },
        "ll" => {
            options.show_hidden = true;
            options.show_long = true;
            options.show_suffix = true;
        },
        "l" => {
            options.show_suffix = true;
        },
        _ => ()
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    // println!("hidden: {}, long: {}, sufix: {}\n{:?}", options.show_hidden, options.show_long, options.show_suffix, paths);
    for path in paths {
        let path = Path::new(&path);
        if path.is_file() {
            // Show file only
            println!("file");
        } else if path.is_dir() {
            // list_dir(path, &flags);
            list_dir(path, &options);
            // println!("dir");
        } else {
            println!("\x1b[33mls\x1b[0m: Can not access \'{}\': No such file or directory", path.display());
        }
    }
}

fn list_dir(path: &Path, options: &LsOptions) {
    let mut entries = vec![];

    if options.show_hidden {
        entries.push(PathBuf::from("."));
        entries.push(PathBuf::from(".."));
    }

    // let entries = fs::read_dir(path).unwrap();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !options.show_hidden {
            let name = path.file_name().unwrap().to_string_lossy();
            if name.starts_with('.') {
                continue;
            }
        }

        entries.push(path);
    }
    // println!("{:?}", entries);
    for path in entries {
        let metadata = fs::metadata(&path).unwrap();
        
        let mut name: String = if path.ends_with(".") {
            ".".into()
        } else if path.ends_with("..") {
            "..".into()
        } else {
            match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => "".to_string(),
            }
        };

        // let mut display_name = name.to_string();
        if options.show_suffix {
            if metadata.is_dir() {
                name.push('/');
            }
        }

        print!("{}   ",name);
        // break;

    }
    println!();
}