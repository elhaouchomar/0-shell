use super::structs::*;  
use std::path::Path;
use std::time::{SystemTime, Duration};
use chrono::{DateTime, Local};
use libc::{major, minor};
use std::fs::{self, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::PathBuf;
use users::{get_user_by_uid, get_group_by_gid};
use terminal_size::{terminal_size, Width};

pub fn get_file_info(path: &Path, options: &LsFlags, name: &str) -> (Option<FileInfo>, u64) {
    let mut file_info = FileInfo {
        name: "".to_string(),
        typee: FileType::Unknown,
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
            let mut name  = if name.is_empty() {get_file_name(&path)} else {name.to_string()};
            let f_type = get_file_type(mode);
            if has_special_chars(&name) {name = format!("'{name}'")};
            classify_name(&mut name, options, &f_type, path);
            name = name.trim().to_string();
            let blocks = meta.blocks();
            file_info.name = name;
            if !options.show_long {
                return (Some(file_info), blocks);
            }

            let mut perm = get_permissions(mode, &f_type);
            if has_acl(&path) {perm.push('+')} 
            file_info.premession = perm;
            file_info.nlink = meta.nlink();
            file_info.user = get_user_name(meta.uid());
            file_info.group = get_group_name(meta.gid());
            
            if f_type == FileType::CharDevice || f_type == FileType::BlockDevice {
                let rdev = meta.rdev();
                file_info.major = Some(major(rdev));
                file_info.minor = Some(minor(rdev));
            } else {
                file_info.size = meta.size()
            }
            file_info.typee = f_type;
            file_info.mod_date = get_mod_time(&meta);

            (Some(file_info), blocks)
        },
        Err(_) => (None, 0)
    }
}

fn has_acl(path: &Path) -> bool {
    if let Ok(attrs) = xattr::list(path) {
        for attr in attrs {
            if let Ok(name) = attr.into_string() {
                // On Linux, ACLs often appear as "system.posix_acl_access"
                if name.contains("posix_acl") {
                    return true;
                }
                // println!("name => {}", name)
            }
        }
    }
    false
}

fn get_file_name(path: &Path) -> String {
    if let Some(name) = path.file_name() {
        name.to_string_lossy().to_string()
    } else {
        path.to_string_lossy().to_string()
    }
}

// /usr/bin/passwd
//0000 101 101 001 100
//0001 000 001 000 000 


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
        0b0001_0000_0000_0000 => FileType::Fifo,
        0b0010_0000_0000_0000 => FileType::CharDevice,
        0b0100_0000_0000_0000 => FileType::Directory,
        0b0110_0000_0000_0000 => FileType::BlockDevice,
        0b1000_0000_0000_0000 => if mode & 0b001_001_001 != 0 {FileType::Regular(1)} else {FileType::Regular(0)},
        0b1010_0000_0000_0000 => FileType::Symlink,
        0b1100_0000_0000_0000 => FileType::Socket,
        _                     => FileType::Unknown
    }
}

fn classify_name(name: &mut String, options: &LsFlags, f_type: &FileType, path: &Path) {
    if f_type == &FileType::Symlink && options.show_long {
        let final_target = fs::canonicalize(path).unwrap_or( PathBuf::from(""));
        let next = fs::read_link(path).unwrap_or( PathBuf::from(""));
        let t_name = next.to_string_lossy().to_string();
        let mut s = String::from(" -> ");
        s.push_str(&t_name);
        if options.show_suffix {
            match fs::metadata(final_target) {
                Ok(meta) => {
                    s.push(get_file_type(meta.permissions().mode()).suffix());
                },
                Err(_) => ()
            }
        }
        name.push_str(&s);
    } else if options.show_suffix{
        name.push(f_type.suffix());
    }
}

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
    let mut last_mod_time = meta.modified().unwrap_or(SystemTime::now());
    last_mod_time += Duration::from_secs(60 * 60);
    let last_mod_time: DateTime<Local> = DateTime::from(last_mod_time);
    let now: DateTime<Local> = SystemTime::now().into();
    let six_months = chrono::Duration::days(180);

    if now - last_mod_time > six_months {
        last_mod_time.format("%b %e  %Y").to_string()
    } else {
        last_mod_time.format("%b %e %H:%M").to_string()
    }
}

pub fn simple_print(files:  &Vec<(Vec<FileInfo>, String, u64)>, folders: &mut Vec<(Vec<FileInfo>, String, u64)>, options: &LsFlags) {
    let mut terminal_w = 0;
    if let  Some((Width(w), _)) = terminal_size() {
        terminal_w = w
    }

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
        let mut total_len = 0;
        let mut col_max_w = 0;
        let names: Vec<&str> = f.0.iter()
        .filter(|f| options.show_hidden || !f.name.starts_with('.'))
        .map(|f| {
            total_len += f.name.trim().len() + 2;

            if f.name.trim().len() > col_max_w {col_max_w = f.name.trim().len()};
            f.name.trim()
        }).collect();

        // println!("{}, {}, {}", col_max_w, terminal_w, total_len);
        
        
        if total_len.saturating_sub(2) > terminal_w as usize {
            // println!("{:#?}", names);
            let mut cols = terminal_w as usize / (col_max_w + 2);
            let rows = (names.len() + cols - 1) / cols;
            if cols == 0 {
                cols = 1;
            }


            for row in 0..rows {
                for col in 0..cols {
                    let idx = col * rows + row;
                    if idx < names.len() {
                        print!("{:<width$}", names[idx], width = (col_max_w + 2));
                    }
                }
                println!();
            }

        } else {
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
        }
        
        

        if folders_len > 1 && i != folders_len -1 {
            println!()
        }
    });
    
}

pub fn long_print(files: &Vec<(Vec<FileInfo>, String, u64)>, folders: &mut Vec<(Vec<FileInfo>, String, u64)>, show_hidden: bool) {
    let mut format_r = LongFormatWidths::new();
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

pub fn sort_paths(paths: &mut Vec<String>) {
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

fn get_columns_maxes(files: &Vec<(Vec<FileInfo>, String, u64)>, format: &mut LongFormatWidths) {
    files.iter().for_each(|v| {
        v.0.iter().for_each(|v| {
            let nlink_str = v.nlink.to_string();
            let size_str = v.size.to_string();

            if v.premession.len() > format.perm_len {
                format.perm_len = v.premession.len();
            }

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

fn has_special_chars(s: &str) -> bool {
    s.chars().any(|c| {
        !(c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' || c == '/')
    })
}
