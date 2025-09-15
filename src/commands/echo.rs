use crate::utils::parser::Arg;
use crate::utils::prompt::get_home_dir;

pub fn cmd_echo(args: &Vec<Arg>) {
    let mut output_parts = Vec::new();

    for arg in args {
        if arg.value == "~" && !arg.quoted {
            if let Some(home) = get_home_dir() {
                output_parts.push(home.display().to_string());
            } else {
                output_parts.push("~".to_string());
            }
        } else if arg.value.starts_with("~/") && !arg.quoted {
            if let Some(home) = get_home_dir() {
                let expanded = format!("{}/{}", home.display(), &arg.value[2..]);
                output_parts.push(expanded);
            } else {
                output_parts.push(arg.value.clone());
            }
        } else {
            output_parts.push(arg.value.clone());
        }
    }
    println!("{}", output_parts.join(" "));
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