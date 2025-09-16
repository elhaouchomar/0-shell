use crate::utils::parser::Arg;
use crate::utils::prompt::get_home_dir;
use std::env::{ set_current_dir};
use std::path::{PathBuf, Path};

pub fn cmd_cd(args: &[Arg], previous: &Path) {
    let processed_args = if args.len() > 0 && args[0].value == "--" {
        &args[1..]
    } else {
        args
    };

    if processed_args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    match processed_args.get(0) {
        None => {
            match get_home_dir() {
                Some(dir) => set_dir(&dir, false),
                None => {
                    set_dir( &PathBuf::from("/"), false)
                }
            }
        }
        Some(arg) => {
            match arg.value.as_str() {
                "-" => {
                    set_dir(previous, true);
                }
                "~" if !arg.quoted => {
                    match get_home_dir() {
                        Some(dir) => set_dir(&dir, false),
                        None => {
                            set_dir( &PathBuf::from("/"), false)
                        }
                    }
                }
                path if path.starts_with("~/") && !arg.quoted => {
                    match get_home_dir() {
                        Some(home) => set_dir(&home.join(&path[2..]), false),
                        None => {
                            eprintln!("cd: can't cd to {}", path)
                        }
                    }
                }
                path => {
                   set_dir(&PathBuf::from(path), false)
                }
            }
        }
    }
}

pub fn set_dir(p: &Path, print: bool) {
    match set_current_dir(&p) {
        Ok(_) => {
            if print {println!("{}", p.display())}
        }
        Err(_) => {
            eprintln!("cd: can't cd to {}", p.display());
        }
    }
}