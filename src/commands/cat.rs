use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use crate::utils::parser::Arg;

pub fn cmd_cat(args: &[Arg]) {
    let mut files_to_read = Vec::new();
    let mut stdin_requests = Vec::new();
    let mut options_ended = false;

    for (index, arg) in args.iter().enumerate() {
        let value = &arg.value;
        
        if !options_ended && value.starts_with('-') && !value.is_empty() && value != "-" {
            if value == "--" {
                options_ended = true;
                continue;
            }
            
            if value.starts_with("---") {
                eprintln!("cat: unrecognized option '{}'", value);
                return;
            }
            
            if value.starts_with("--") {
                eprintln!("cat: unrecognized option '{}'", value);
                return;
            }
            
            if value.len() > 1 {
                for ch in value.chars().skip(1) {
                    eprintln!("cat: invalid option -- '{}'", ch);
                    return;
                }
            }
        } else if value == "-" {
            stdin_requests.push(index);
        } else {
            files_to_read.push((index, value));
        }
    }

    if files_to_read.is_empty() && stdin_requests.is_empty() {
        read_from_stdin_with_counter();
        return;
    }

    let mut items: Vec<(usize, &str)> = Vec::new();
    items.extend(files_to_read.iter().map(|&(i, s)| (i, s.as_str())));
    items.extend(stdin_requests.iter().map(|&i| (i, "-")));
    items.sort_by_key(|&(index, _)| index);

    for (_, item) in items {
        if item == "-" {
            read_from_stdin_with_counter();
        } else {
            read_file(item);
        }
    }
}

fn read_from_stdin_with_counter() {
    static mut STDIN_COUNTER: i32 = 0;
    
    unsafe {
        STDIN_COUNTER += 1;
    }
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    
    loop {
        buffer.clear();
        match handle.read_line(&mut buffer) {
            Ok(0) => {
                unsafe {
                    STDIN_COUNTER -= 1;
                    if STDIN_COUNTER <= 0 {
                        break;
                    } else {
                        continue;
                    }
                }
            }
            Ok(_) => print!("{}", buffer),
            Err(e) => {
                eprintln!("cat: stdin: {}", e);
                unsafe {
                    STDIN_COUNTER -= 1;
                }
                break;
            }
        }
    }
}

fn read_file(filename: &str) {
    match File::open(filename) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();
            match reader.read_to_string(&mut buffer) {
                Ok(_) => print!("{}", buffer),
                Err(e) => eprintln!("cat: {}: {}", filename, e),
            }
        }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("cat: {}: No such file or directory", filename);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("cat: {}: Permission denied", filename);
                }
                _ => {
                    if let Ok(metadata) = std::fs::metadata(filename) {
                        if metadata.is_dir() {
                            eprintln!("cat: {}: Is a directory", filename);
                        } else {
                            eprintln!("cat: {}: {}", filename, e);
                        }
                    } else {
                        eprintln!("cat: {}: {}", filename, e);
                    }
                }
            }
        }
    }
}