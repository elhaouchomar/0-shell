use crate::utils::parser::Arg;
// use std::env::current_dir;
use std::path::PathBuf;

// static mut LAST_KNOWN_DIR: Option<PathBuf> = None;

pub fn cmd_pwd(args: &[Arg], path: &PathBuf) {
      if !args.is_empty() {
        eprintln!("pwd: too many arguments");
        return;
    }
    // match current_dir() {
    //     Ok(path) => {
    //         unsafe {
    //             LAST_KNOWN_DIR = Some(path.clone());
    //         }
    //         println!("{}", path.display());
    //     }
    //     Err(_) => {
    //         unsafe {
    //             if let Some(ref last_path) = LAST_KNOWN_DIR {
    //                 println!("{}", last_path.display());
    //             } else {
    //                 println!("/");
    //             }
    //         }
    //     }
    // }
    println!("{}", path.display());
}

// pub fn update_last_known_dir(path: PathBuf) {
//     unsafe {
//         LAST_KNOWN_DIR = Some(path);
//     }
// }
