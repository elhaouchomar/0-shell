use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use crate::utils::parser::Arg;

struct LsOptions {
    show_hidden: bool,
    show_long: bool,
    show_suffix: bool,
}

// struct FileInfo {
//     name: String,
//     size: u32,
//     permissions: String,
//     links: u32,
//     user: String,
//     group: String,
//     time: String,
// }

pub fn cmd_ls(args: &Vec<Arg>) {
    let mut options = LsOptions {
        show_hidden: false,
        show_long: false,
        show_suffix: false,
    };

    let mut paths = Vec::new();
   /*  for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'l' => {
                        options.show_long = true;
                    }
                    'a' => {
                        options.show_hidden = true;
                    }
                    'F' => {
                        options.show_suffix = true;
                    }
                    _ => {
                        println!("\x1b[33mls\x1b[0m: Not a valid option {}", arg);
                        return;
                    }
                }
            }
        } else {
            paths.push(arg.to_string())
        }
    } */

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    // println!("hidden: {}, long: {}, sufix: {}\n{:?}", options.show_hidden, options.show_long, options.show_suffix, paths);
    for path in paths {
        let path = Path::new(&path);
        println!("{:?}", path);
        if path.is_file() {
            // Show file only
            // println!("file");
            // let metadata = fs::metadata(&path).unwrap();
            get_file_info(path);
            // println!("Permissions: {:?}", metadata.permissions().mode());
        } else if path.is_dir() {
            // list_dir(path, &flags);
            // list_dir(path, &options);
            // println!("dir");
        } else {
            println!(
                "\x1b[33mls\x1b[0m: Can not access \'{}\': No such file or directory",
                path.display()
            );
        }
    }
}

fn get_file_info(path: &Path) {
    let metadata = fs::symlink_metadata(path).unwrap();
    println!("{} {}", get_permissions(&path), metadata.nlink(), );
}

fn get_file_type_char(mode: u32) -> char {
    match mode & libc::S_IFMT {
        libc::S_IFREG => '-',  // regular file
        libc::S_IFDIR => 'd',  // directory
        libc::S_IFLNK => 'l',  // symlink
        libc::S_IFCHR => 'c',  // character device
        libc::S_IFBLK => 'b',  // block device
        libc::S_IFIFO => 'p',  // named pipe (FIFO)
        libc::S_IFSOCK => 's', // socket
        _ => '?',              // unknown
    }
}

fn get_permission_char(mode: u32, read: u32, write: u32, exec: u32) -> [char; 3] {
    [
        if mode & read != 0 { 'r' } else { '-' },
        if mode & write != 0 { 'w' } else { '-' },
        if mode & exec != 0 { 'x' } else { '-' },
    ]
}

fn get_permissions(path: &Path) -> String {
    let metadata = fs::symlink_metadata(path).unwrap();
    let mode = metadata.permissions().mode();

    let mut perms = String::new();

    // File type character
    perms.push(get_file_type_char(mode));

    // Owner (user)
    perms.extend(get_permission_char(
        mode,
        libc::S_IRUSR,
        libc::S_IWUSR,
        libc::S_IXUSR,
    ));

    // Group
    perms.extend(get_permission_char(
        mode,
        libc::S_IRGRP,
        libc::S_IWGRP,
        libc::S_IXGRP,
    ));

    // Others
    perms.extend(get_permission_char(
        mode,
        libc::S_IROTH,
        libc::S_IWOTH,
        libc::S_IXOTH,
    ));

    perms
}
