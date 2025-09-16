use crate::utils::parser::Arg;
use std::fs::{remove_file, remove_dir_all};
use std::path::Path;

pub fn cmd_rm(args: &[Arg]) {
    if args.is_empty() {
        println!("\x1b[33mrm\x1b[0m: missing operand");
        return;
    }

    let mut recursive = false;
    let mut targets: Vec<&Arg> = Vec::new();

    for arg in args {
        if arg.value == "-r" {
            recursive = true;
        } else {
            targets.push(arg);
        }
    }

    if targets.is_empty() {
        println!("\x1b[33mrm\x1b[0m: missing operand");
        return;
    }

    for arg in targets {
        let path = Path::new(arg.value.as_str());

        if !path.exists() {
            println!("rm: cannot remove '{}': No such file or directory", path.display());
            continue;
        }

        if path.is_file() {
            match remove_file(path) {
                Ok(_) => {}
                Err(e) => println!("rm: cannot remove '{}': {}", path.display(), e),
            }
        } else if path.is_dir() {
            if recursive {
                match remove_dir_all(path) {
                    Ok(_) => {}
                    Err(e) => println!("rm: cannot remove directory '{}': {}", path.display(), e),
                }
            } else {
                println!("rm: cannot remove '{}': Is a directory", path.display());
            }
        }
    }
}
