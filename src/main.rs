mod utils;
mod commands;
use std::path::PathBuf;

fn main() {
    let mut input = String::new();
    let mut multi_line = false;
    let mut previous_dir: Option<PathBuf> = None;

    loop {
        if !multi_line {
            input.clear();
        }

        utils::prompt::display_prompt(multi_line);

        input += &utils::input::get_input();
        if utils::input::input_has_unclosed_quotes(&input) || input.ends_with('\\') {
            multi_line = true;
            if !input.ends_with('\\') {
                input.push('\n');
            }
            continue;
        } else {
            multi_line = false;
        }

        let command = utils::parser::pasre_input(&input);

        match command.name.as_str() {
            // "ls" => {
            //     commands::ls::cmd_ls(&command.args);
            // }
            "pwd" => {
                commands::pwd::cmd_pwd();
            }
            "echo" => {
                commands::echo::cmd_echo(&command.args);
            }
            "cd" => {
                commands::cd::cmd_cd(&command.args, &mut previous_dir);
            }
            "exit" => {
                commands::exit::cmd_exit();
            }
            "clear" => {
                commands::clear::cmd_clear();
            }
            "cat" => {
                commands::cat::cmd_cat(&command.args);
            }
            _ => println!("\x1b[33m{}\x1b[0m: Command not found", command.name),
        }
        // println!("{:?}", command);
    }
}
