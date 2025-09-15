pub struct LsFlags {
    pub show_hidden: bool,
    pub show_long: bool,
    pub show_suffix: bool,
}

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

pub struct LongFormatWidths {
    pub perm_len: usize,
    pub nlink_len: usize,
    pub user_len: usize,
    pub group_len: usize,
    pub size_len: usize,
    pub major_len: usize,
    pub minor_len: usize,
    pub date_len: usize
}

impl LongFormatWidths {
    pub fn new() -> Self {
        Self { perm_len: 0,nlink_len: 0, user_len: 0, group_len: 0, size_len: 0, major_len: 0, minor_len: 0, date_len: 0 }
    }
}

impl FileInfo {
    pub fn print_long(&self, w: &LongFormatWidths) {
        let size = if self.typee == CharDevice || self.typee == BlockDevice {
            format!("{:>maj_w$}, {:>min_w$}", self.major.unwrap().to_string(), self.minor.unwrap().to_string(), min_w = w.minor_len, maj_w = w.major_len)
        } else {
            format!("{:>w$}", self.size, w = w.size_len)
        };

        let max_s = w.size_len.max(w.major_len + w.minor_len + 2);

        println!("{:<perm_w$} {:>lin_w$} {:<u_w$} {:<g_w$} {:>max_s$} {:<d_w$} {}",
            self.premession, self.nlink, self.user, self.group, size, self.mod_date, self.name,
            lin_w = w.nlink_len, u_w = w.user_len, g_w = w.group_len, max_s = max_s, d_w = w.date_len, perm_w = w.perm_len)
    }
}