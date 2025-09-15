use std::fs;
use std::path::Path;
use std::io::ErrorKind;

pub fn cmd_mkdir(args: &Vec<String>){

    if args.len() == 0 {
        println!("mkdir: missing operand");
        return
    }
    for dir in args.iter() {
        let dr_path = Path::new(dir);
            match fs::create_dir(dr_path){
                Ok(_)=> () ,
                Err(e) if e.kind() == ErrorKind::AlreadyExists =>  println!("mkdir: cannot create directory {:},  {:}", dr_path.display() , "Directory already exists."),
                Err(e) if e.kind() == ErrorKind::NotFound =>  println!("mkdir: cannot create directory {:},  {:}", dr_path.display() , "Parent directory not found."),
                Err(e) => println!("Failed to create directory: {}", e)
            };
    }
}
