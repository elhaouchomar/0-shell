mod structs;
mod helpers;
use helpers::{sort_paths, get_file_info, simple_print, long_print};
use structs::LsFlags;
use std::path::PathBuf;
use std::fs;
use crate::utils::parser::Arg;


pub fn cmd_ls(args: &Vec<Arg>) {
    let mut options = LsFlags {
        show_hidden: false,
        show_long: false,
        show_suffix: false,
    };

    let mut paths = Vec::new();
    for arg in args {
        if arg.value.starts_with('-') {
            for c in arg.value.chars().skip(1) {
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
                        println!("\x1b[33mls\x1b[0m: Not a valid option {}", arg.value);
                        return
                    }
                }
            }
        } else {
            paths.push(arg.value.to_string())
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
        let path = PathBuf::from(&str_path);

        let metadata = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(e) => {
                println!("ls: cannot access '{}': {}", path.to_string_lossy(), e);
                continue;
            }
        };

        if path.is_dir() && (!metadata.file_type().is_symlink() || !options.show_long) {
            let mut curr = path.clone();
            let mut parrent = path.clone();
            curr.push(".");
            parrent.push("..");

            let curr = get_file_info(&curr, &options, ".");
            let parent = get_file_info(&parrent, &options, "..");
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
                                let file_infos = get_file_info(&path, &options, "");
                                if let Some(file_info) = file_infos.0 {
                                    if options.show_hidden || !file_info.name.starts_with(".") {
                                        blocks += file_infos.1;
                                    }
                                    files.push(file_info);
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
            if let Some(file_info) = get_file_info(&path, &options, "").0 {
                files.push(file_info);
                simple_files.push((files, "".to_string(), 0));
            } else {
                println!("ls: cannot access '{}': No such file or directory", path.to_string_lossy())
            }
            
        } 
    }
    

    if !options.show_long {
        simple_print(&simple_files, &mut folders, &options);
    } else {
        long_print(&simple_files, &mut folders, options.show_hidden);
    } 
}