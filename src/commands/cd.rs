use std::env::{ set_current_dir };
use crate::utils::prompt::get_home_dir;

pub fn cmd_cd(args: &Vec<String>) {
    if args.len() > 1 {
        println!("\x1b[33mcd\x1b[0m: too many arguments");
        return
    }

    let mut path = String::new();
    if args.len() == 0 {
        if let Some(dir) = get_home_dir() {
            path = dir.to_str().unwrap().to_string();
        }
    } else {
        path = args[0].clone()
    }

    match set_current_dir(path) {
        Ok(_) => (),
        Err(e) => println!("\x1b[33mcd\x1b[0m: {}",e.to_string().split(" (os error").next().unwrap())
    }
}