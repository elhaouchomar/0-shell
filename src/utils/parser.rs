#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>
}
#[derive(Debug)]
pub struct Arg {
    pub value: String,
    pub quoted: bool,
}


pub fn pasre_input(input: &str) -> Command {
    let mut command = Command {
        name : String::new(),
        args : Vec::new(),
    };
    
    let mut word = String::new();
    let mut single_quote = false;
    let mut double_quote = false;

    for c in input.chars() {
        match c {
            '\"' if !single_quote => {
                double_quote = !double_quote;
                if !double_quote {
                    push_word(&mut word, &mut command);
                }
            }

            '\''  if !double_quote => {
                single_quote = !single_quote;
                if !single_quote {
                    push_word(&mut word, &mut command);
                }
            }

            ' '  if !single_quote && !double_quote => {
                push_word(&mut word, &mut command);
            }
            _ => word.push(c),
        }
    }

    push_word(&mut word, &mut command);

    command
}


fn push_word(word: &mut String, command: &mut Command) {
    if !word.is_empty() {
        if command.name.is_empty() {
            command.name = word.clone();
        } else {
            command.args.push(word.clone());
        }
        word.clear();
    }
}