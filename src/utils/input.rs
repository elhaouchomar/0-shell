use std::io::{self};
use std::process::exit;

pub fn get_input() -> String {
    let mut input = String::new();
    let nb_of_chars = io::stdin().read_line(&mut input).unwrap();
    if nb_of_chars == 0 {
        println!();
        exit(0);
    }

    input.trim().to_string()
}

pub fn input_has_unclosed_quotes(input: &str) -> bool {
    let mut single = false;
    let mut double = false;

    for c in input.chars() {
        match c {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            _ => {}
        }
    }

    single || double
}