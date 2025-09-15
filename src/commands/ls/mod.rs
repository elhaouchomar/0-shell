mod structs;
mod helpers;
use helpers::{sort_paths, get_file_info, simple_print, long_print};
use structs::LsFlags;
use std::path::PathBuf;
use std::path::Path;
use std::fs;


pub fn cmd_ls(args: &Vec<String>) {
    let mut options = LsFlags {
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

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    let mut folders = Vec::new();
    let mut simple_files = Vec::new();

    sort_paths(&mut paths);
    for str_path in paths {
        let mut files = Vec::new();
        let path = Path::new(&str_path);
        if path.is_file() {
            if let Some(file_info) = get_file_info(&path, &options).0 {
            files.push(file_info);
            }
            simple_files.push((files, "".to_string(), 0));
        } else if path.is_dir() {
            let curr = get_file_info(&&PathBuf::from("."), &options);
            let parent = get_file_info(&&PathBuf::from(".."), &options);
            let mut blocks = 0;

            if let Some(file_info) = curr.0 {
                files.push(file_info);
                if options.show_hidden {blocks += curr.1;}
            }
            if let Some(file_info) = parent.0 {
                files.push(file_info);
                if options.show_hidden {blocks += parent.1};
            }

             
            match fs::read_dir(path) {
                Ok(entries) => {
                    entries.for_each(|entry| {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                let file_infos = get_file_info(&path, &options);
                                if let Some(file_info) = file_infos.0 {
                                    files.push(file_info);
                                    blocks += file_infos.1;
                                }
                            },
                            Err(e) => println!("{e}")
                        }
                    });
                },
                Err(e) => println!("{e}")
            };

            folders.push((files, str_path, blocks));
        } else {
            println!("ls: cannot access '{}': No such file or directory", path.to_string_lossy())
        }
    }
    

    if !options.show_long {
        simple_print(&simple_files, &mut folders, &options);
    } else {
        long_print(&simple_files, &mut folders, options.show_hidden);
    } 
}