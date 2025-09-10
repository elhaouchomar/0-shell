mod utils;
mod commands;
use std::path::PathBuf;
use std::env;

// Global variable to track current directory for prompt
static mut CURRENT_TRACKED_DIR: Option<PathBuf> = None;

fn main() {
    // Initialize current directory
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    unsafe {
        CURRENT_TRACKED_DIR = Some(current_dir.clone());
    }
    commands::pwd::update_last_known_dir(current_dir);
    
    let mut input = String::new();
    let mut multi_line = false;
    let mut previous_dir: Option<PathBuf> = None;

    loop {
        if !multi_line {
            input.clear();
        }

        // Get current tracked directory for prompt display
        let current_path = unsafe {
            CURRENT_TRACKED_DIR.clone().unwrap_or_else(|| PathBuf::from("/"))
        };
        
        utils::prompt::display_prompt(multi_line, &current_path);

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
            "pwd" => {
                commands::pwd::cmd_pwd();
            }
            "echo" => {
                commands::echo::cmd_echo(&command.args);
            }
            "cd" => {
                commands::cd::cmd_cd(&command.args, &mut previous_dir);
                // Update tracked directory after cd command
                if let Ok(current_dir) = env::current_dir() {
                    unsafe {
                        CURRENT_TRACKED_DIR = Some(current_dir.clone());
                    }
                    commands::pwd::update_last_known_dir(current_dir);
                }
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
    }
}