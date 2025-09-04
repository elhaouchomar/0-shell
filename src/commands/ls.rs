use std::path::Path;
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::path::PathBuf;

struct LsOptions {
    show_hidden: bool,
    show_long: bool,
    show_suffix: bool,
}

pub enum FileType {
    Regular,
    Directory,
    Symlink(PathBuf),
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
    Unknown,
}

impl FileType {
    pub fn classify_suffix(&self, show_long: bool) -> String {
        match self {
            FileType::Directory => "/".to_string(),
            FileType::Symlink(target) => if show_long {
                let t_name = get_file_name(target);
                // let f_type = get_file_type(&target, show_long);
                let mut s = String::from(" -> ");
                // if show_long {
                s.push_str(&t_name);
                // if soh
                    
                // s.push_str(&f_type.classify_suffix(show_long));
                // }
                s
            } else {
                "@".to_string()
            },
            FileType::Fifo => "|".to_string(),
            FileType::Socket => "=".to_string(),
            FileType::Regular | FileType::BlockDevice | FileType::CharDevice => "".to_string(),
            FileType::Unknown => "?".to_string(),
        }
    }
}

pub struct FileInfo {
    pub name: String,
    pub premession: String,
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
            // f.name.push_str(&f.f_type.classify_suffix(options.show_long));
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

    if options.show_hidden {
        files.iter_mut().for_each(|f| {
            print!("{}  ", f.name)
        });
        println!()
    } else {
        files.iter_mut().filter(|f| !f.name.starts_with('.')).for_each(|f| {
            print!("{}  ", f.name)
        });
        println!()
    }
}

fn get_file_info(path: &Path, options: &LsOptions) -> FileInfo{
    let mut name  = get_file_name(&path);
    // let f_type = get_file_type(path, options.show_long);
    if options.show_suffix || options.show_long {
        name.push_str(&suffix(path, options))
    }

    if !options.show_long {
        return FileInfo {
            name,
            premession: "".to_string(),
            size: 0,
            nlink: 0,
            major: None,
            minor: None,
            mod_date: "".to_string(),
            user: "".to_string(),
            group: "".to_string()
        }
    }

    FileInfo {
        name,
        premession: "".to_string(),
        size: 0,
        nlink: 0,
        major: None,
        minor: None,
        mod_date: "".to_string(),
        user: "".to_string(),
        group: "".to_string()
    }
}

fn get_file_name(path: &Path) -> String {
    if let Some(name) = path.file_name() {
        name.to_string_lossy().to_string()
    } else {
        path.to_string_lossy().to_string()
    }
}

fn suffix(path: &Path, options: &LsOptions) -> String {
    match fs::symlink_metadata(path) {
        Ok(meta) => {
            let file_type = meta.file_type();
            if file_type.is_file() || file_type.is_block_device() || file_type.is_char_device() {
                "".to_string()
            } else if file_type.is_dir() {
                if options.show_suffix {"/".to_string()} else {"".to_string()}
            } else if file_type.is_symlink() {
                if options.show_long {
                    let final_target = fs::canonicalize(path).unwrap_or( PathBuf::from(""));
                    let next = fs::read_link(path).unwrap_or( PathBuf::from(""));
                    let t_name = get_file_name(&next);
                    let mut s = String::from(" -> ");
                    s.push_str(&t_name);
                    if options.show_suffix {
                        s.push_str(&suffix(&final_target, options));
                    }
                    
                    s
                } else if options.show_suffix {
                    "@".to_string()
                } else {
                    "".to_string()
                }
            }else if file_type.is_fifo() {
                if options.show_suffix {"|".to_string()} else {"".to_string()}
            } else if file_type.is_socket() {
                if options.show_suffix {"=".to_string()} else {"".to_string()}
            } else {
                if options.show_suffix {"?".to_string()} else {"".to_string()}
            }
        }
        Err(_) => if options.show_suffix {"?".to_string()} else {"".to_string()}
    }
}