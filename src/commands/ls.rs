use std::path::{Path};
use std::fs;
use std::os::unix::fs::{PermissionsExt};
use std::path::PathBuf;

struct LsOptions {
    show_hidden: bool,
    show_long: bool,
    show_suffix: bool,
}

#[derive(PartialEq, Eq)]
pub enum FileType {
    Regular(u8),
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
    Unknown,
}

use FileType::*;
impl FileType {
    pub fn preffix(&self) -> char {
        match self {
            Regular(_) => '-',
            Directory => 'd',
            Symlink => 'l',
            BlockDevice => 'b',
            CharDevice => 'c',
            Fifo => 'p',
            Socket => 's',
            Unknown => '?'
        }
    }

    pub fn suffix(&self) -> char {
        match self {
            Regular(x) => if *x == 1 {'*'} else {' '}
            Directory => '/',
            Fifo => '|',
            Socket => '=',
            BlockDevice | CharDevice => ' ',
            Unknown => '?',
            Symlink => '@'
        }
    }
}

pub struct FileInfo {
    pub name: String,
    pub premession: String,
    pub typee: FileType,
    pub size: u64,
    pub nlink: u64,
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub mod_date: String,
    pub user: String,
    pub group: String,
}

pub fn cmd_ls(args: &Vec<String>) {
    let mut options = LsOptions {
        show_hidden: false,
        show_long: false,
        show_suffix: false,
    };

    let mut paths = Vec::new();
    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'l' => {
                        options.show_long = true;
                    },
                    'a' => {
                        options.show_hidden = true;
                    },
                    'F' => {
                        options.show_suffix = true;
                    },
                    _ => {
                        println!("\x1b[33mls\x1b[0m: Not a valid option {}", arg);
                        return
                    }
                }
            }
        } else {
            paths.push(arg.to_string())
        }
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    let mut files = Vec::new();

    for path in paths {
        let path = Path::new(&path);
        if path.is_file() {
            get_file_info(path, &options);
        } else if path.is_dir() {
            files.push(get_file_info(&PathBuf::from("."), &options));
            files.push(get_file_info(&PathBuf::from(".."), &options));

            match fs::read_dir(path) {
                Ok(entries) => {
                    entries.for_each(|entry| {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                files.push(get_file_info(&path, &options));
                            },
                            Err(e) => println!("{e}")
                        }
                    });
                },
                Err(e) => println!("{e}")
            };
        } else {
            println!("ls: cannot access {}: No such file or directory", path.to_string_lossy())
        }
    }

    if !options.show_long {
        simple_print(&mut files, &options);
    } else {
        files.iter_mut().for_each(|f| {
            println!("{}  ", f.name);
        });
    }
}

fn simple_print(files: &mut Vec<FileInfo>, options: &LsOptions) {
    files.sort_by(|a, b| {
        if a.name == "." { return std::cmp::Ordering::Less; }
        if b.name == "." { return std::cmp::Ordering::Greater; }
        if a.name == ".." { return std::cmp::Ordering::Less; }
        if b.name == ".." { return std::cmp::Ordering::Greater; }
        a.name.trim_matches('.').to_lowercase().cmp(&b.name.trim_matches('.').to_lowercase())
    });

    files.iter_mut().for_each(|f| {
        if options.show_hidden {
            print!("{}  ", f.name.trim())
        } else if !f.name.starts_with('.') {
            print!("{}  ", f.name.trim())
        }
    });
    println!()
}

fn get_file_info(path: &Path, options: &LsOptions) -> FileInfo {
    let mut file_info = FileInfo {
        name: "".to_string(),
        typee: Unknown,
        premession: "".to_string(),
        size: 0,
        nlink: 0,
        major: None,
        minor: None,
        mod_date: "".to_string(),
        user: "".to_string(),
        group: "".to_string()
    };
    
    match fs::symlink_metadata(path) {
        
        Ok(meta) => {
            let mode = meta.permissions().mode();
            let mut name  = get_file_name(&path);
            // add suffix to display name depend on type and flags
            let f_type = get_file_type(mode);
            classify_name(&mut name, options, &f_type, path);

            if !options.show_long {
                file_info.name = name;
                return file_info;
            }

            file_info.premession = get_permissions(mode, &f_type);
            
            // println!("{}",premession);
            file_info
        },
        Err(_) => file_info
    }
    
     
   

    // FileInfo {
    //     name: display_name,
    //     premession: "".to_string(),
    //     size: 0,
    //     nlink: 0,
    //     major: None,
    //     minor: None,
    //     mod_date: "".to_string(),
    //     user: "".to_string(),
    //     group: "".to_string()
    // }
}

fn get_file_name(path: &Path) -> String {
    if let Some(name) = path.file_name() {
        name.to_string_lossy().to_string()
    } else {
        path.to_string_lossy().to_string()
    }
}


fn get_permissions(mode: u32, f_type: &FileType) -> String {
    let mut  perm = String::new();
    perm.push(f_type.preffix());
    let masks = [
        ((0o400, 0o200, 0o100, 0o4000)),
        ((0o40, 0o20, 0o10, 0o2000)),
        ((0o4, 0o2, 0o1, 0o1000))
    ];

    for mask in masks {
        if mode & mask.0 != 0 { perm.push('r') } else { perm.push('-') }
        if mode & mask.1 != 0 { perm.push('w') } else { perm.push('-') }

        let mut x = if mode & mask.2 != 0 {'x'} else {'-'};
        let sp = if mask.3 == 0o1000 {('t', 'T')} else {('s', 'S')};
        if mode & mask.3 != 0 {
            if  x == 'x' {x = sp.0} else {x = sp.1}
        }

        perm.push(x);
    }

    perm
}

// sp 100 000 000 000 // 000 100 000 000 // 000 010 000 000 // 000 001 000 000
// sp 010 000 000 000 // 000 000 100 000 // 000 000 010 000 // 000 000 001 000
// sp 001 000 000 000 // 000 000 000 100 // 000 000 000 010 // 000 000 000 001

fn get_file_type(mode: u32) -> FileType {
    match mode & 0b1111_0000_0000_0000 {
        0b0001_0000_0000_0000 => Fifo,
        0b0010_0000_0000_0000 => CharDevice,
        0b0100_0000_0000_0000 => Directory,
        0b0110_0000_0000_0000 => BlockDevice,
        0b1000_0000_0000_0000 => if mode & 0b001_001_001 != 0 {Regular(1)} else {Regular(0)},
        0b1010_0000_0000_0000 => Symlink,
        0b1100_0000_0000_0000 => Socket,
        _                     => Unknown
    }
}

fn classify_name(name: &mut String, options: &LsOptions, f_type: &FileType, path: &Path) {
    if options.show_suffix {
        if f_type == &Symlink && options.show_long {
            let final_target = fs::canonicalize(path).unwrap_or( PathBuf::from(""));
            let next = fs::read_link(path).unwrap_or( PathBuf::from(""));
            let t_name = get_file_name(&next);
            let mut s = String::from(" -> ");
            s.push_str(&t_name);
            match fs::metadata(final_target) {
                Ok(meta) => {
                    s.push(get_file_type(meta.permissions().mode()).suffix());
                },
                Err(_) => ()
            }
            name.push_str(&s);
        } else {
            name.push(f_type.suffix());
        }
    }
}