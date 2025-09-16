use crate::commands::echo::handle_escapes;
use std::path::{PathBuf};
mod commands;
use std::env;
mod utils;

fn main() {
    if !atty::is(atty::Stream::Stdout) {
        return;
    }

    let mut current_path = get_current_dir();

    let mut input = String::new();
    let mut multi_line = false;
    let mut previous_dir= current_path.clone();

    loop {
        if !multi_line {
            input.clear();
        }

        utils::prompt::display_prompt(multi_line, &current_path);

        input += &utils::parser::get_input();
        
        if utils::parser::input_has_unclosed_quotes(&input) || input.ends_with('\\') {
            multi_line = true;
            if !input.ends_with('\\') {
                input.push('\n');
            } else {
                input.pop();
            }
            continue;
        }

        let raw_command = utils::parser::parse_input(&input);

        let has_unfinished_quotes = utils::parser::input_has_unclosed_quotes(&input);

        if has_unfinished_quotes {
            multi_line = true;
            input.push('\n');
            continue;
        }
        
        multi_line = false;

        let mut command = raw_command;

        for arg in &mut command.args {
            arg.value = handle_escapes(&arg.value);
        }

        match command.name.as_str() {
            "cat" => commands::cat::cmd_cat(&command.args),
            "cd" => {
                commands::cd::cmd_cd(&command.args, &previous_dir);
                previous_dir = current_path.clone();
                current_path = get_current_dir();
                commands::cd::set_dir(&current_path, false);
            },
            "clear" => commands::clear::cmd_clear(),
            "cp" => commands::cp::cmd_cp(&command.args),
            "echo" => {
                commands::echo::cmd_echo(&command.args);
            }
            "exit" => commands::exit::cmd_exit(),
            "ls" => commands::ls::cmd_ls(&command.args),
            "mkdir" => commands::mkdir::cmd_mkdir(&command.args),
            "rm" => commands::rm::cmd_rm(&command.args),
            "mv" => commands::mv::cmd_mv(&command.args),
            "pwd" => commands::pwd::cmd_pwd(&command.args, &current_path),
            "" => {},
            _ => println!("\x1b[33m{}\x1b[0m: Command not found", command.name),
        }
    }
}

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))
}