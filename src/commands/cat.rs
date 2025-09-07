use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub fn cmd_cat(args: &[String]) {
    if args.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => println!("{}", l),
                Err(e) => eprintln!("cat: {}", e),
            }
        }
    } else {
        for filename in args {
            match File::open(&filename) {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    let mut buffer = String::new();
                    if let Err(e) = reader.read_to_string(&mut buffer) {
                        eprintln!("cat: {}: {}", filename, e);
                        continue;
                    }
                    print!("{}", buffer);
                }
                Err(e) => eprintln!("cat: {}: {}", filename, e),
            }
        }
    }
}
