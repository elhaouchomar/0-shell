

pub fn cmd_echo(args: &Vec<String>) {
    println!("{}", handle_escapes(&args.join(" ")));
}


fn handle_escapes(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('\'') => result.push('\''),
                Some('"') => result.push('\"'),
                Some('\\') => result.push('\\'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                },
                None => result.push('\\')
            }
        } else {
            result.push(c)
        }
    }

    result
} 
