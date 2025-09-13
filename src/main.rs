mod commands;
mod utils;

use std::env;
use std::path::PathBuf;

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
            CURRENT_TRACKED_DIR
                .clone()
                .unwrap_or_else(|| PathBuf::from("/"))
        };
        utils::prompt::display_prompt(multi_line, &current_path);

        input += &utils::input::get_input();

        let ends_with_unescaped_backslash =
            input.chars().rev().take_while(|&c| c == '\\').count() % 2 == 1;
        if utils::input::input_has_unclosed_quotes(&input) || ends_with_unescaped_backslash   {
            multi_line = true;
            if !ends_with_unescaped_backslash {
                input.push('\n');
            }
            continue;
        }
        multi_line = false;

        let escaped_input = commands::echo::handle_escapes(&input);
        let command = utils::parser::parse_input(&escaped_input);

        match command.name.as_str() {
            "cat" => commands::cat::cmd_cat(&command.args),
            "cd" => {
                commands::cd::cmd_cd(&command.args, &mut previous_dir);
                if let Ok(current_dir) = env::current_dir() {
                    unsafe {
                        CURRENT_TRACKED_DIR = Some(current_dir.clone());
                    }
                    commands::pwd::update_last_known_dir(current_dir);
                }
            },
            "clear" => commands::clear::cmd_clear(),
            "echo" => commands::echo::cmd_echo(&command.args),
            "exit" => commands::exit::cmd_exit(),
            // "ls" => commands::ls::cmd_ls
            "pwd" => commands::pwd::cmd_pwd(),
            _ => println!("\x1b[33m{}\x1b[0m: Command not found", command.name),
        }
    }
}
