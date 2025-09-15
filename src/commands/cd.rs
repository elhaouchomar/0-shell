use crate::utils::parser::Arg;
use crate::utils::prompt::get_home_dir;
use std::env::{current_dir, set_current_dir};
use std::path::PathBuf;

pub fn cmd_cd(args: &[Arg], previous: &mut Option<PathBuf>) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    let current = current_dir().ok();

    let target_path = match args.get(0) {
        None => {
            match get_home_dir() {
                Some(dir) => dir,
                None => {
                    current.clone().unwrap_or_else(|| PathBuf::from("/"))
                }
            }
        }
        Some(arg) => {
            match arg.value.as_str() {
                "-" => {
                    match previous {
                        Some(prev) => {
                            println!("{}", prev.display());
                            prev.clone()
                        }
                        None => {
                            if let Some(ref cur) = current {
                                println!("{}", cur.display());
                                cur.clone()
                            } else {
                                PathBuf::from("/")
                            }
                        }
                    }
                }
                "~" if !arg.quoted => {
                    match get_home_dir() {
                        Some(dir) => dir,
                        None => {
                            current.clone().unwrap_or_else(|| PathBuf::from("/"))
                        }
                    }
                }
                path if path.starts_with("~/") && !arg.quoted => {
                    match get_home_dir() {
                        Some(home) => home.join(&path[2..]),
                        None => {
                            current.clone().unwrap_or_else(|| PathBuf::from("/"))
                        }
                    }
                }
                ".." => {
                    match current_dir() {
                        Ok(current_path) => {
                            match current_path.parent() {
                                Some(parent) => parent.to_path_buf(),
                                None => PathBuf::from("/")
                            }
                        }
                        Err(_) => {
                            unsafe {
                                if let Some(ref tracked) = crate::CURRENT_TRACKED_DIR {
                                    match tracked.parent() {
                                        Some(parent) => parent.to_path_buf(),
                                        None => PathBuf::from("/")
                                    }
                                } else {
                                    PathBuf::from("/")
                                }
                            }
                        }
                    }
                }
                path => {
                    PathBuf::from(path)
                }
            }
        }
    };

    match set_current_dir(&target_path) {
        Ok(_) => {
            *previous = current;
            
            if let Ok(new_dir) = current_dir() {
                unsafe {
                    crate::CURRENT_TRACKED_DIR = Some(new_dir);
                }
            }
        }
        Err(_) => {
            eprintln!("cd: can't cd to {}", target_path.display());
        }
    }
}
