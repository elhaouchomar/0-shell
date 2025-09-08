use crate::utils::parser::Arg;
use crate::utils::prompt::get_home_dir;
use std::env::{current_dir, set_current_dir};
use std::path::PathBuf;

pub fn cmd_cd(args: &Vec<Arg>, previous: &mut Option<PathBuf>) {
    if args.len() > 1 {
        println!("\x1b[33mcd\x1b[0m: too many arguments");
        return;
    }

    let current = current_dir().ok();

    let path: String = match args.get(0) {
        None => match get_home_dir() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            None => "~".to_string(),
        },
        Some(arg) => match (arg.value.as_str(), arg.quoted) {
            ("-", _) => {
                match previous {
                    Some(prev) => {
                        let p = prev.to_str().unwrap().to_string();
                        println!("{}", p); 
                        p
                    }
                    None => {
                        println!("{}", current_dir().unwrap().to_str().unwrap()); 
                        return; 
                    }
                }
            }

            ("~", false) => match get_home_dir() {
                Some(dir) => dir.to_str().unwrap().to_string(),
                None => "~".to_string(),
            },
            (val, false) if val.starts_with("~/") => match get_home_dir() {
                Some(dir) => format!("{}/{}", dir.to_str().unwrap(), &val[2..]),
                None => val.to_string(),
            },
            (val, _) => val.to_string(),
        },
    };

    match set_current_dir(&path) {
        Ok(_) => {
            *previous = current;
        }
        Err(e) => println!(
            "\x1b[33mcd\x1b[0m: {}: \"{}\"",
            e.to_string().split(" (os error").next().unwrap(),
            path
        ),
    }
}
