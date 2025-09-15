use crate::commands::echo::handle_escapes;
use std::path::PathBuf;
mod commands;
use std::env;
mod utils;

static mut CURRENT_TRACKED_DIR: Option<PathBuf> = None;

fn main() {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    unsafe { CURRENT_TRACKED_DIR = Some(current_dir.clone()) }
    commands::pwd::update_last_known_dir(current_dir);

    let mut input = String::new();
    let mut multi_line = false;
    let mut previous_dir: Option<PathBuf> = None;

    loop {
        if !multi_line {
            input.clear();
        }
        let current_path = unsafe {
            CURRENT_TRACKED_DIR.clone().unwrap_or_else(|| PathBuf::from("/"))
        };

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
        } else {
            multi_line = false;
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
                commands::cd::cmd_cd(&command.args, &mut previous_dir);
                if let Ok(current_dir) = env::current_dir() {
                    unsafe { CURRENT_TRACKED_DIR = Some(current_dir.clone()) }
                    commands::pwd::update_last_known_dir(current_dir);
                }
            },
            "clear" => commands::clear::cmd_clear(),
            "cp" => commands::cp::cmd_cp(&command.args),
            "echo" => {
                commands::echo::cmd_echo(&command.args);
                // println!("{:#?}", command.args);
            }
            "exit" => commands::exit::cmd_exit(),
            "ls" => commands::ls::cmd_ls(&command.args),
            "mkdir" => commands::mkdir::cmd_mkdir(&command.args),
            // "rm" => commands::rm::cmd_rm(&command.args),
            // "mv" => commands::mv::cmd_mv(&command.args),
            "pwd" => commands::pwd::cmd_pwd(&command.args),
            _ => println!("\x1b[33m{}\x1b[0m: Command not found", command.name),
        }
    }
}