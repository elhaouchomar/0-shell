use crate::utils::parser::Arg;
use std::path::{ Path, PathBuf };
use std::fs;

pub fn cmd_cp(args: &Vec<Arg>) {
    if args.is_empty() {
        println!("\x1b[33mcp\x1b[0m: missing file operand");
        return;
    }
    if args.len() == 1 {
        println!("\x1b[33mcp\x1b[0m: missing destination file operand after '{}'", args[0].value);
        return;
    }

    let dest = Path::new(&args[args.len() - 1].value);
    let sources = &args[..args.len() - 1];

    // in case of single source: cp file1 file2
    if sources.len() == 1 {
        let file_path = Path::new(&sources[0].value);

        if !file_path.exists() || !file_path.is_file() {
            println!("cp: cannot stat '{}': No such file or directory", file_path.display());
            return;
        }

        let final_dest = if dest.exists() && dest.is_dir() {
            let mut new_path = PathBuf::from(dest);
            new_path.push(file_path.file_name().unwrap());
            new_path
        } else {
            dest.to_path_buf()
        };

        if let Err(e) = fs::copy(&file_path, &final_dest) {
            println!(
                "cp: cannot copy '{}' to '{}': {}",
                file_path.display(),
                final_dest.display(),
                e
            );
        }
        return;
    }

    // in case of multiple sources: cp file1 file2 file3 dir/
    if (!dest.exists() || !dest.is_dir()) && args.len() > 2 {
        println!("cp: target '{}' is not a directory", dest.display());
        return;
    }

    for src_arg in sources {
        let file_path = Path::new(&src_arg.value);

        if !file_path.exists() || !file_path.is_file() {
            println!("cp: cannot stat '{}': No such file or directory", file_path.display());
            continue;
        }

        let mut new_path = PathBuf::from(dest);
        new_path.push(file_path.file_name().unwrap());

        if let Err(e) = fs::copy(&file_path, &new_path) {
            println!(
                "cp: cannot copy '{}' to '{}': {}",
                file_path.display(),
                new_path.display(),
                e
            );
        }
    }
}
