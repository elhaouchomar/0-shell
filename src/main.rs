mod utils;
mod commands;
use crate::utils::parser::Arg;


fn main() {
    let mut input = String::new();
    let mut multi_line = false;
    loop {
        if ! multi_line {
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
                commands::cd::cmd_cd(&command.args);
            },
            "exit" => {
                commands::exit::cmd_exit()
            },
            "clear" => {
                commands::clear::cmd_clear()
            },
               "mkdir" => {
                commands::mkdir::cmd_mkdir(&command.args)
            },
                 "rm" => {
           let args =  &command.args.iter().map(|x| {
            let kk = Arg {
                value : x.to_string(),
                quoted : false
            };
            return  kk;
           } ).collect();
                commands::rm::cmd_rm(&args)
            },
                 "mv" => {
           let args =  &command.args.iter().map(|x| {
            let kk = Arg {
                value : x.to_string(),
                quoted : false
            };
            return  kk;
           } ).collect();
                commands::mv::cmd_mv(&args)
            }
            _ => println!("\x1b[33m{}\x1b[0m: Command not found", command.name)
        }
        // println!("{:?}", utils::parser::pasre_input(&input));
    }

}
