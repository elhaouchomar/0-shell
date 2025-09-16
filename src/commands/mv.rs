use crate::utils::parser::Arg;
use std::fs;
use std::path::{Path, PathBuf};

pub fn cmd_mv(args: &[Arg]) {
    if args.is_empty() {
        println!("\x1b[33mmv\x1b[0m: missing file operand");
        return;
    }
    if args.len() == 1 {
        println!(
            "\x1b[33mmv\x1b[0m: missing destination file operand after '{}'",
            args[0].value
        );
        return;
    }

    let dest = Path::new(&args[args.len() - 1].value);
    let sources = &args[..args.len() - 1];

    // had case is just for: mv file1 file2
    if sources.len() == 1 {
        let src = Path::new(&sources[0].value);

        if !src.exists() {
            println!(
                "mv: cannot stat '{}': No such file or directory",
                src.display()
            );
            return;
        }

        let final_dest = if dest.exists() && dest.is_dir() {
            let mut new_path = PathBuf::from(dest);
            new_path.push(src.file_name().unwrap());
            new_path
        } else {
            dest.to_path_buf()
        };

        if let Err(_) = fs::rename(&src, &final_dest) {
            
            if let Err(copy_err) = fs::copy(&src, &final_dest) {
                println!(
                    "mv: cannot move '{}' to '{}': {}",
                    src.display(),
                    final_dest.display(),
                    copy_err
                );
                return;
            }
            if src.is_file() {
                let _ = fs::remove_file(&src);
            } else if src.is_dir() {
                let _ = fs::remove_dir_all(&src);
            }
        }
        return;
    }

    // o hadi for multiple sources: mv file1 file2 file3 dir/
    if !dest.exists() || !dest.is_dir() {
        println!("mv: target '{}' is not a directory", dest.display());
        return;
    }

    for src_arg in sources {
        let src = Path::new(&src_arg.value);

        if !src.exists() {
            println!(
                "mv: cannot stat '{}': No such file or directory",
                src.display()
            );
            continue;
        }

        let mut new_path = PathBuf::from(dest);
        new_path.push(src.file_name().unwrap());

        if let Err(_) = fs::rename(&src, &new_path) {

            if let Err(copy_err) = fs::copy(&src, &new_path) {
                println!(
                    "mv: cannot move '{}' to '{}': {}",
                    src.display(),
                    new_path.display(),
                    copy_err
                );
                continue;
            }
            if src.is_file() {
                let _ = fs::remove_file(&src);
            } else if src.is_dir() {
                let _ = fs::remove_dir_all(&src);
            }
        }
    }
}
