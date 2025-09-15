use crate::utils::parser::Arg;
use std::fs::create_dir;
use std::path::Path;

pub fn cmd_mkdir(args: &[Arg]) {
    if args.is_empty() {
        println!("\x1b[33mmkdir\x1b[0m: missing operand");
        return;
    }


    for arg in args {
        let path = Path::new(arg.value.as_str());

        if path.exists() {
            if path.is_dir() {
                println!("mkdir: cannot create directory '{}': File exists", path.display());
            } else {
                println!(
                    "mkdir: cannot create directory '{}': File exists and is not a directory",
                    path.display()
                );
            }
            continue;
        }

        match create_dir(path) {
            Ok(_) => {}
            Err(e) => {
                println!("mkdir: cannot create directory '{}': {}", path.display(), e);
            }
        }
    }
}
