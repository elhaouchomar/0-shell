use crate::utils::parser::Arg;
use crate::utils::prompt::get_home_dir;
use std::fs::File;
use std::io::Write;

pub fn cmd_echo(args: &Vec<Arg>) {
    let mut output_parts = Vec::new();
    let mut redirect_to_file = None;
    let mut i = 0;

    // Find redirection operator
    while i < args.len() {
        if args[i].value == ">" && i + 1 < args.len() {
            redirect_to_file = Some(&args[i + 1]);
            break;
        }
        i += 1;
    }

    // Process arguments before redirection
    let args_to_process = if redirect_to_file.is_some() { &args[..i] } else { args };

    for arg in args_to_process {
        let processed_value = if arg.value == "~" && !arg.quoted {
            if let Some(home) = get_home_dir() {
                home.display().to_string()
            } else {
                "~".to_string()
            }
        } else if arg.value.starts_with("~/") && !arg.quoted {
            if let Some(home) = get_home_dir() {
                format!("{}/{}", home.display(), &arg.value[2..])
            } else {
                arg.value.clone()
            }
        } else {
            // Don't call handle_escapes here - it's already processed in main
            arg.value.clone()
        };
        output_parts.push(processed_value);
    }

    let output = output_parts.join(" ");

    // Handle output redirection
    if let Some(file_arg) = redirect_to_file {
        let filename = &file_arg.value;
        match File::create(filename) {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", output) {
                    eprintln!("echo: error writing to file '{}': {}", filename, e);
                }
            }
            Err(e) => {
                eprintln!("echo: cannot create file '{}': {}", filename, e);
            }
        }
    } else {
        println!("{}", output);
    }
}


pub fn handle_escapes(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('n') => { chars.next(); result.push('\n'); }
                Some('r') => { chars.next(); result.push('\r'); }
                Some('t') => { chars.next(); result.push('\t'); }
                Some('\'') => { chars.next(); result.push('\''); }
                Some('"') => { chars.next(); result.push('\"'); }
                Some('\\') => { chars.next(); result.push('\\'); }
                Some('a') => { chars.next(); result.push('\x07'); }
                Some('b') => { chars.next(); result.push('\x08'); }
                Some('f') => { chars.next(); result.push('\x0C'); }
                Some('v') => { chars.next(); result.push_str("\n\n\n\n");}

                Some(other) => {
                    result.push('\\');
                    result.push(*other);
                    chars.next();
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }

    result
}
