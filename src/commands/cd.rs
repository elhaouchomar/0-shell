use crate::utils::parser::Arg;
use crate::utils::prompt::get_home_dir;
use std::env::{current_dir, set_current_dir, var};
use std::path::{Path, PathBuf};
use std::io;

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
                    eprintln!("cd: HOME not set");
                    return;
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
                            eprintln!("cd: OLDPWD not set");
                            return;
                        }
                    }
                }
                "~" if !arg.quoted => {
                    match get_home_dir() {
                        Some(dir) => dir,
                        None => {
                            eprintln!("cd: HOME not set");
                            return;
                        }
                    }
                }
                path if path.starts_with("~/") && !arg.quoted => {
                    match get_home_dir() {
                        Some(home) => home.join(&path[2..]),
                        None => {
                            eprintln!("cd: HOME not set");
                            return;
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
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("cd: {}: No such file or directory", target_path.display());
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("cd: {}: Permission denied", target_path.display());
                }
                _ => {
                    if target_path.exists() {
                        if target_path.is_file() {
                            eprintln!("cd: {}: Not a directory", target_path.display());
                        } else {
                            eprintln!("cd: {}: {}", target_path.display(), e);
                        }
                    } else {
                        eprintln!("cd: {}: No such file or directory", target_path.display());
                    }
                }
            }
        }
    }
}