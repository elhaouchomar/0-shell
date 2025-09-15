use std::fs;
use std::path::Path;
use std::io::ErrorKind;
use crate::utils::parser::Arg;

pub fn cmd_rm(args: &Vec<Arg>){
 
   let (recursive_flag, bad_opt)= catch_flag(args);
   if bad_opt  == true{
        println!("rm: invalid option");
        return

   }
      if args.len() == 0 || (recursive_flag == true && args.len() == 1){
        println!("rm: missing operand");
        return
    }

    for arg in args.iter().filter(|x| x.value != "-r") {
        let path = Path::new(&arg.value);
        if !path.exists(){
             println!("rm: cannot remove directory {:},  {:}", path.display() , "No such file or directory"  );
             return;
        }

        if let Ok(metadata) = fs::metadata(path){
            if metadata.is_dir(){
                if recursive_flag {
                    match fs::remove_dir(path){
                Ok(_)=> () ,
                Err(e) if e.kind() == ErrorKind::NotFound =>  println!("rm: cannot remove directory {:},  {:}", path.display() , "No such file or directory"  ),
                Err(e) => println!("Failed to remove directory: {}", e)   
                    }      
                }else {
                    println!("rm: cannot remove {:} :  Is a directory" , path.display())
                }
            }else if metadata.is_file(){

                match fs::remove_file(path) {
                Ok(_)=> () ,
                Err(e) if e.kind() == ErrorKind::NotFound =>  println!("rm: cannot remove file {:},  {:}", path.display() , "No such file or directory"  ),
                Err(e) => println!("Failed to remove file: {}", e)
         }
            }
        } 
     

    }
}


pub fn catch_flag(args : &Vec<Arg>)->(bool , bool){
    let mut tple = (false , false);
    for arg in args {
    
        if arg.value.starts_with("-") && arg.value != "-r"{
            return (false , true);
        }
        if  arg.value == "-r" {
                tple.0 = true;
        }

    }
    tple
}