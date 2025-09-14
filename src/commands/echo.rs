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
    let mut chars = input.chars();
    let mut count = 0;

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('\'') => result.push('\''),
                Some('"') => result.push('\"'),
                Some('\\') => result.push('\\'),
                Some('f') => {
                    result.push('\n');
                    result.push_str(&" ".repeat(count));
                }
                Some('v') => result.push_str("\n\n\n\n"),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
        count += 1;
    }

    result
} 
