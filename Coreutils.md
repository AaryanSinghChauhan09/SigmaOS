# SigmaOS Coreutils

`userland/coreutils/` provides BusyBox-style multi-call binary with 20+ essential utilities.

---

## Available Commands

| Command | Description | Status |
|---------|-------------|--------|
| `ls` | List directory contents (`-l`, `-a`, `-la`) | ✅ |
| `cat` | Concatenate and print files | ✅ |
| `echo` | Print arguments (`-n` no-newline) | ✅ |
| `pwd` | Print working directory | ✅ |
| `mkdir` | Create directory (`-p` parents) | ✅ |
| `rm` | Remove files/dirs (`-r`, `-f`) | ✅ |
| `cp` | Copy files | ✅ |
| `mv` | Move/rename files | ✅ |
| `touch` | Create empty file / update timestamp | ✅ |
| `stat` | Display file metadata | ✅ |
| `grep` | Search patterns in files (`-i` case-insensitive) | ✅ |
| `head` | Print first N lines (`-n`) | ✅ |
| `tail` | Print last N lines (`-n`) | ✅ |
| `wc` | Word/line/byte count (`-l`, `-w`, `-c`) | ✅ |
| `uname` | System information (`-a`, `-s`, `-r`, etc.) | ✅ |
| `sleep` | Sleep for N seconds | ✅ |
| `env` | Print environment / run command with env | ✅ |
| `id` | Print user/group IDs | ✅ |
| `whoami` | Print current username | ✅ |
| `yes` | Repeatedly print string | ✅ |
| `true` | Exit 0 | ✅ |
| `false` | Exit 1 | ✅ |

---

## Usage

```bash
# As multi-call binary
sigma-coreutils ls /tmp
sigma-coreutils grep "error" /var/log/sigma.log
sigma-coreutils wc -l /etc/passwd

# Via symlinks (BusyBox style)
ln -s sigma-coreutils /usr/bin/ls
ln -s sigma-coreutils /usr/bin/grep
ls /tmp           # works directly
```

---

## Building

```bash
cd userland/coreutils
cargo build --release
# Output: target/release/sigma-coreutils

# Create symlinks
for cmd in ls cat echo pwd mkdir rm cp mv touch stat grep head tail wc uname sleep env id; do
    ln -sf sigma-coreutils /usr/bin/$cmd
done
```

---

## Missing (Phase D)

- `sed` — stream editor (regex replacement)
- `awk` — text processing
- `find` — file search
- `tar` — archive utility
- `gzip` / `xz` — compression
- `dd` — disk imager
- `mount` / `umount` — filesystem mounting
- `ps` — process list
- `kill` — send signals

---

*Source: `userland/coreutils/src/main.rs` — Rust std, ~400 lines*
