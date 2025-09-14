use std::path::{Path};
use std::time::SystemTime;
use chrono::{DateTime, Local};
use libc::{major, minor};
use std::fs::{self, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::PathBuf;

// struct LsOptions {
//     show_hidden: bool,
//     show_long: bool,
//     show_suffix: bool,
// }

#[derive(PartialEq, Eq, Debug)]
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

#[derive(Debug)]
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
pub struct LFormat {
    pub nlink_len: usize,
    pub user_len: usize,
    pub group_len: usize,
    pub size_len: usize,
    pub major_len: usize,
    pub minor_len: usize,
    pub date_len: usize
}

impl LFormat {
    pub fn new() -> Self {
        Self { nlink_len: 0, user_len: 0, group_len: 0, size_len: 0, major_len: 0, minor_len: 0, date_len: 0 }
    }
}

impl FileInfo {
    pub fn print_long(&self, w: &LFormat) {
        let size = if self.typee == CharDevice || self.typee == BlockDevice {
            format!("{:>maj_w$}, {:>min_w$}", self.major.unwrap().to_string(), self.minor.unwrap().to_string(), min_w = w.minor_len, maj_w = w.major_len)
        } else {
            format!("{:>w$}", self.size, w = w.size_len)
        };

        let max_s = w.size_len.max(w.major_len + w.minor_len + 2);

        println!("{} {:>lin_w$} {:<u_w$} {:<g_w$} {:>max_s$} {:<d_w$} {}", self.premession, self.nlink, self.user, self.group, size, self.mod_date, self.name, lin_w = w.nlink_len, u_w = w.user_len, g_w = w.group_len, max_s = max_s, d_w = w.date_len)
    }
}

pub fn cmd_ls(args: &Vec<String>) {
    let mut options = LsOptions {
        show_hidden: false,
        show_long: false,
        show_suffix: false,
    };

// pub fn cmd_ls(args: &Vec<Arg>) {
//     let mut options = LsOptions {
//         show_hidden: false,
//         show_long: false,
//         show_suffix: false,
//     };

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    let mut folders = Vec::new();
    let mut simple_files = Vec::new();

    sort_paths(&mut paths);
    for str_path in paths {
        let mut files = Vec::new();
        let path = Path::new(&str_path);
        if path.is_file() {
            if let Some(file_info) = get_file_info(&path, &options).0 {
            files.push(file_info);
            }
            simple_files.push((files, "".to_string(), 0));
        } else if path.is_dir() {
            let curr = get_file_info(&&PathBuf::from("."), &options);
            let parent = get_file_info(&&PathBuf::from(".."), &options);
            let mut blocks = 0;

            if let Some(file_info) = curr.0 {
                files.push(file_info);
                if options.show_hidden {blocks += curr.1;}
            }
            if let Some(file_info) = parent.0 {
                files.push(file_info);
                if options.show_hidden {blocks += parent.1};
            }

             
            match fs::read_dir(path) {
                Ok(entries) => {
                    entries.for_each(|entry| {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                let file_infos = get_file_info(&path, &options);
                                if let Some(file_info) = file_infos.0 {
                                    files.push(file_info);
                                    blocks += file_infos.1;
                                }
                            },
                            Err(e) => println!("{e}")
                        }
                    });
                },
                Err(e) => println!("{e}")
            };

            folders.push((files, str_path, blocks));
        } else {
            println!("ls: cannot access '{}': No such file or directory", path.to_string_lossy())
        }
    }
    // println!("{:#?}\n-----------------------------------------------------\n {:?}", paths_files_info, simple_files)
    if !options.show_long {
        simple_print(&simple_files, &mut folders, &options);
    } else {
        long_print(&simple_files, &mut folders, options.show_hidden);
    } 
}

fn get_file_info(path: &Path, options: &LsOptions) -> (Option<FileInfo>, u64) {
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
            let blocks = meta.blocks();

            file_info.name = name;
            if !options.show_long {
                return (Some(file_info), blocks);
            }

            file_info.premession = get_permissions(mode, &f_type);
            file_info.nlink = meta.nlink();
            file_info.user = get_user_name(meta.uid());
            file_info.group = get_group_name(meta.gid());
            
            if f_type == CharDevice || f_type == BlockDevice {
                let rdev = meta.rdev();
                file_info.major = Some(major(rdev));
                file_info.minor = Some(minor(rdev));
            } else {
                file_info.size = meta.size()
            }
            file_info.typee = f_type;
            file_info.mod_date = get_mod_time(&meta);

            
            // println!("{}",premession);
            (Some(file_info), blocks)
        },
        Err(_) => (None, 0)
    }
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
            let t_name = next.to_string_lossy().to_string();
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


use users::{get_user_by_uid, get_group_by_gid};
fn get_user_name(uid: u32) -> String {
    match get_user_by_uid(uid) {
        Some(user) => user.name().to_string_lossy().to_string(),
        None => uid.to_string()
    }
}

fn get_group_name(gid: u32) -> String {
    match get_group_by_gid(gid) {
        Some(group) => group.name().to_string_lossy().to_string(),
        None => gid.to_string()
    }
}

fn get_mod_time(meta: &Metadata) -> String {
    let last_mod_time = meta.modified().unwrap_or(SystemTime::now());
    let last_mod_time: DateTime<Local> = DateTime::from(last_mod_time);
    let now: DateTime<Local> = SystemTime::now().into();
    let six_months = chrono::Duration::days(180);

    if now - last_mod_time > six_months {
        last_mod_time.format("%b %e  %Y").to_string()
    } else {
        last_mod_time.format("%b %e %H:%M").to_string()
    }
}

fn simple_print(files:  &Vec<(Vec<FileInfo>, String, u64)>, folders: &mut Vec<(Vec<FileInfo>, String, u64)>, options: &LsOptions) {
    files.iter().for_each(|f| {
        f.0.iter().for_each(|info| {
            print!("{}  ", info.name.trim())
        });
    });

    if files.len() != 0 {
        println!()
    }

    if folders.len() > 0 && files.len() > 0 {
        println!()
    }

    let folders_len = folders.len();
    let print_name = folders_len > 1 || files.len() > 0;
    folders.iter_mut().enumerate().for_each(|(i, f)| {
        if print_name {
            println!("{}:", f.1)
        }
        sort_file_infos(&mut f.0);
        let mut c = 0;
        f.0.iter().for_each(|f| {
            if options.show_hidden {
                c += 1;
                print!("{}  ", f.name.trim())
            } else if !f.name.starts_with('.') {
                c += 1;
                print!("{}  ", f.name.trim())
            }
        });
        if c != 0 {
            println!();
        }

        if folders_len > 1 && i != folders_len -1 {
            println!()
        }
    });
    
}

fn long_print(files: &Vec<(Vec<FileInfo>, String, u64)>, folders: &mut Vec<(Vec<FileInfo>, String, u64)>, show_hidden: bool) {
    let mut format_r = LFormat::new();
    get_columns_maxes(&files, &mut format_r);
    get_columns_maxes(&folders, &mut format_r);

    files.iter().for_each(|v| {
        v.0.iter().for_each(|v| {
            v.print_long(&format_r);
        });
    });

    if folders.len() > 0 && files.len() > 0 {
        println!()
    }

    let folders_len = folders.len();
    let print_name = folders_len > 1 || files.len() > 0;
    folders.iter_mut().enumerate().for_each(|(i, f)| {
        if print_name {
            println!("{}:", f.1)
        }
        sort_file_infos(&mut f.0);
        println!("total {}",f.2 / 2);
        f.0.iter().for_each(|f| {
            if show_hidden {
                f.print_long(&format_r);
            } else if !f.name.starts_with('.') {
                f.print_long(&format_r);
            }
        });

        if folders_len > 1 && i != folders_len -1 {
            println!()
        }
    });
}

fn sort_paths(paths: &mut Vec<String>) {
    paths.sort_by(|a, b| {
        if a == "." { return std::cmp::Ordering::Less; }
        if b == "." { return std::cmp::Ordering::Greater; }
        if a == ".." { return std::cmp::Ordering::Less; }
        if b == ".." { return std::cmp::Ordering::Greater; }
        a.trim_matches('.').to_lowercase().cmp(&b.trim_matches('.').to_lowercase())
    });
}

fn sort_file_infos(files: &mut Vec<FileInfo>) {
    files.sort_by(|a, b| {
        if a.name == "." { return std::cmp::Ordering::Less; }
        if b.name == "." { return std::cmp::Ordering::Greater; }
        if a.name == ".." { return std::cmp::Ordering::Less; }
        if b.name == ".." { return std::cmp::Ordering::Greater; }
        a.name.trim_matches('.').to_lowercase().cmp(&b.name.trim_matches('.').to_lowercase())
    });
}

fn get_columns_maxes(files: &Vec<(Vec<FileInfo>, String, u64)>, format: &mut LFormat) {
    files.iter().for_each(|v| {
        v.0.iter().for_each(|v| {
            let nlink_str = v.nlink.to_string();
            let size_str = v.size.to_string();

            if nlink_str.len() > format.nlink_len {
                format.nlink_len = nlink_str.len()
            }

            if v.user.len() > format.user_len {
                format.user_len = v.user.len()
            }

            if v.group.len() > format.group_len {
                format.group_len = v.group.len()
            }

            if size_str.len() > format.size_len {
                format.size_len = size_str.len()
            }

            if v.mod_date.len() > format.date_len {
                format.date_len = v.mod_date.len();
            }

            match v.major {
                Some(v) => {
                    let v = v.to_string();
                    if v.len() > format.major_len {
                        format.major_len = v.len()
                    }
                },
                None => ()
            }

            match v.minor {
                Some(v) => {
                    let v = v.to_string();
                    if v.len() > format.minor_len {
                        format.minor_len = v.len()
                    }
                },
                None => ()
            }
        });
    });
}
