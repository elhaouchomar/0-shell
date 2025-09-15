use std::process::exit;
use std::io::{self};

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Arg {
    pub value: String,
    pub quoted: bool,
}

pub fn get_input() -> String {
    let mut input = String::new();
    let nb_of_chars = io::stdin().read_line(&mut input).unwrap();
    if nb_of_chars == 0 {
        println!();
        exit(0);
    }
    input.trim().to_string()
}

pub fn parse_input(input: &str) -> Command {
    let mut command = Command {
        name: String::new(),
        args: Vec::new(),
    };

    let mut word = String::new();
    let mut word_quoted = false;    
    let mut single_quote = false;
    let mut double_quote = false;
    let mut escaped = false;

    for c in input.chars() {
        if escaped {
            word.push(c);
            escaped = false;
            continue;
        }

        match c {
            '\\' => {
                escaped = true;
            }
            '"' if !single_quote => {
                double_quote = !double_quote;
                word_quoted = true;
            }
            '\'' if !double_quote => {
                single_quote = !single_quote;
                word_quoted = true; 
            }
            ' ' if !single_quote && !double_quote => {
                push_word(&mut word, &mut command, word_quoted);
                word_quoted = false;
            }
            _ => word.push(c),
        }
    }

    push_word(&mut word, &mut command, word_quoted);

    command
}

fn push_word(word: &mut String, command: &mut Command, quoted: bool) {
    if !word.is_empty() {
        let value = word.clone();

        if command.name.is_empty() {
            command.name = value.clone();
        } else {
            command.args.push(Arg { value, quoted });
        }
        word.clear();
    }
}

pub fn input_has_unclosed_quotes(input: &str) -> bool {
    let mut single = false;
    let mut double = false;
    let mut escaped = false;

    for c in input.chars() {
        if escaped {
            escaped = false;
            continue;
        }

        match c {
            '\\' => escaped = true,
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            _ => {}
        }
    }

    single || double
}