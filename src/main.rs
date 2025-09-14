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

        let raw_command = utils::parser::parse_input(&input);

        let ends_with_unescaped_backslash =
            input.chars().rev().take_while(|&c| c == '\\').count() % 2 == 1;

        let has_unfinished_arg = raw_command.args.iter().any(|arg| arg.quoted);

        if has_unfinished_arg || ends_with_unescaped_backslash {
            multi_line = true;
            if !ends_with_unescaped_backslash {
                input.push('\n');
            }
            continue;
        }
        multi_line = false;

        let mut command = raw_command;
        
        for arg in &mut command.args {
            arg.value = handle_escapes(&arg.value);
        }

        match command.name.as_str() {
            "cat" => commands::cat::cmd_cat(&command.args),
            "ls" => {
                commands::ls::cmd_ls(&command.args);
            },
            "pwd" => {
                commands::pwd::cmd_pwd();
            },
            "echo" => {
                commands::echo::cmd_echo(&command.args);
            },
            "cd" => {
                commands::cd::cmd_cd(&command.args, &mut previous_dir);
                if let Ok(current_dir) = env::current_dir() {
                    unsafe { CURRENT_TRACKED_DIR = Some(current_dir.clone()) }
                    commands::pwd::update_last_known_dir(current_dir);
                }
            }
            "clear" => commands::clear::cmd_clear(),
            "echo" => commands::echo::cmd_echo(&command.args),
            "exit" => commands::exit::cmd_exit(),
            "pwd" => commands::pwd::cmd_pwd(),
            _ if !command.name.is_empty() => {
                println!("\x1b[33m{}\x1b[0m: Command not found", command.name)
            }
            _ => {}
        }
    }
}