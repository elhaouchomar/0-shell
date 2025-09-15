use std::fs;
use std::path::Path;
use crate::utils::parser::Arg;
use std::io::ErrorKind;

pub fn cmd_mv(args: &Vec<Arg>){
    match args.len() {
            0 => {
                println!("mv: missing file operand")
            }
            1 => {
                println!("mv: missing destination file operand after {}" , &args[0].value);
                // println!("Try 'mv --help' for more information.")
            }
            2  => {
                let path = Path::new(&args[1].value);

                if path.exists() && path.is_dir(){
                        match fs::copy(&args[0].value, path){
                            Ok(_)=> (),
                            Err(e) if e.kind() == ErrorKind::NotFound => println!("errrorrr") ,
                            _=> todo!()

                        };
                }else if  path.exists() && !path.is_dir(){
                println!("mv: cannot overwrite non-directory {:} with directory {:}" , &args[0].value , path.display() );
                    return
                }else if !path.exists(){
                   match fs::rename(&args[0].value, &path){
                    Ok(_) => (),
                    Err(e) => println!("mv : error : {}" , e)
                   };    
                }
            },  
            _=> {
                let destin = Path::new(&args[args.len()].value);
                if destin.exists() && destin.is_dir(){
                    
                }
                for arg in args {

                }
            } 
    }
}