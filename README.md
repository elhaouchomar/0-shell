# 🐚 0-shell

`0-shell` is a minimalist, Unix-like shell implemented in **Rust**, built entirely from scratch to support basic Unix commands without relying on external binaries or traditional shell interpreters like `bash` or `sh`.

Inspired by `BusyBox`, this shell is tailored for **embedded systems**, **learning environments**, and **low-resource deployments**, with full control over command behavior and system-level interaction.

---

## ✨ Features

✅ Built-in command execution  
✅ No use of `/bin/*` or other external binaries  
✅ Handles user input, errors, and edge cases robustly  
✅ Graceful `Ctrl+D` (EOF) exit  
✅ Minimal memory usage thanks to Rust safety  

---

## 🛠️ Build & Run

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended)
- `cargo` build tool (comes with Rust)

## 🧾 Supported Commands

These commands are implemented manually, without shelling out to system tools.

### 📂 Navigation & Filesystem

| Command          | Description                                                                 |
|------------------|-----------------------------------------------------------------------------|
| `cd [dir]`        | Change current directory (`cd` for home, `cd -` for previous)              |
| `pwd`             | Print working directory                                                    |
| `mkdir name`      | Create a new directory                                                     |
| `rm [-r] target`  | Delete a file or recursively delete a directory                            |
| `ls [-l -a -F]`   | List directory contents (long, all files, classify)                        |

### 📝 File Manipulation

| Command             | Description                                                              |
|---------------------|--------------------------------------------------------------------------|
| `cat [file ...]`     | Print file(s) to stdout or read from stdin (`-`)                        |
| `cp src dst`         | Copy a file or multiple files to a destination                          |
| `mv src dst`         | Move/rename file(s) or move into a directory                            |

### 📣 Output

| Command               | Description                                                             |
|------------------------|------------------------------------------------------------------------|
| `echo [text]`           | Print to stdout; supports `~` expansion and quoted strings            |

### ⛔ Exit

| Command | Description      |
|---------|------------------|
| `exit`  | Exit the shell   |



## 🔍 Example Usage

```bash
$ ./0-shell
$ cd dev
$ pwd
/dev
$ ls -l
total 0
crw-------  1 root   root     10,    58 Feb  5 09:21 acpi_thermal_rel
crw-r--r--  1 root   root     10,   235 Feb  5 09:21 autofs
drwxr-xr-x  2 root   root           540 Feb  5 09:21 block
...
$ something
Command 'something' not found
$ echo "Hello There"
Hello There
$ exit
$
