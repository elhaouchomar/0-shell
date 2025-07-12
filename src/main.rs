mod utils;
mod commands;


fn main() {
    let mut input = String::new();
    let mut multi_line = false;
    loop {
        if ! multi_line {
            input.clear();
        }

        utils::prompt::display_prompt(multi_line);
        
        input += &utils::input::get_input();
        if utils::input::input_has_unclosed_quotes(&input) {
            multi_line = true;
            input.push('\n');
            continue;
        } else {
            multi_line = false;
        }


        let command = utils::parser::pasre_input(&input);
        match command.name.as_str() {
            "pwd" => {
                commands::pwd::cmd_pwd();
            },
            "echo" => {
                commands::echo::cmd_echo(&command.args);
            },
            _ => println!("ksjdsd")
        }
        // println!("{:?}", utils::parser::pasre_input(&input));
    }

}
