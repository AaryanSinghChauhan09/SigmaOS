# SigmaOS Built-in Command Contracts

## Overview
Every built-in command in SigmaOS Shell must adhere to a strict specification contract including synopsis, side effects, exit status policies, and required test cases.

---

## `cd`

### Synopsis
`cd [-L|-P] [directory]`

### Behavior
- Changes the shell's current working directory.
- Updates `PWD` and `OLDPWD` environment variables.
- Default directory (no arguments): expands to `$HOME`.
- Argument `-`: changes working directory to `$OLDPWD` and prints the new directory to stdout.
- Supports logical (`-L`) and physical (`-P`) path resolution.

### Errors & Exit Codes
- Success: exit status 0
- Nonexistent directory: exit status 1
- Permission denied: exit status 1
- `HOME` not set (when invoked without arguments): exit status 1

---

## `pwd`

### Synopsis
`pwd [-L|-P]`

### Behavior
- Prints the absolute pathname of the current working directory.
- `-L`: Output logical working directory (honors symlinks from `PWD`).
- `-P`: Output physical working directory (resolves all symlinks).

### Errors & Exit Codes
- Success: exit status 0
- Invalid option: exit status 2

---

## `echo`

### Synopsis
`echo [-n] [-e] [string ...]`

### Behavior
- Outputs string arguments separated by spaces, followed by newline.
- `-n`: Suppress trailing newline.
- `-e`: Enable interpretation of backslash escapes (`\n`, `\t`, `\\`).

### Errors & Exit Codes
- Success: exit status 0

---

## `export`

### Synopsis
`export [-p] [name[=value] ...]`

### Behavior
- Sets export attribute for shell variables, making them accessible to child process environments.
- `-p`: Displays all exported variables in reusable `export name="value"` format.

### Errors & Exit Codes
- Success: exit status 0
- Invalid variable name: exit status 1

---

## `exit`

### Synopsis
`exit [n]`

### Behavior
- Terminates shell process with exit status `n`.
- If `n` is omitted, uses exit status of the previous executed command.

### Errors & Exit Codes
- Success / Specified exit status `n` (0..255).
- Non-numeric argument: exit status 2 with error message to stderr.
