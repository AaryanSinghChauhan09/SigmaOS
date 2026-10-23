# SigmaOS Shell Built-in Command Contracts

Every built-in command in SigmaOS shell maintains an explicit contract defining synopsis, arguments, behavior, side effects, error modes, exit codes, and test obligations.

---

## `cd`

### Synopsis
`cd [-L|-P] [directory]`

### Behavior
- Changes the shell's current working directory.
- Updates `$PWD` and `$OLDPWD`.
- Defaults to `$HOME` if no directory argument is supplied.
- Operates as `cd $OLDPWD` if argument is `-`.

### Side Effects
- Updates process working directory (`chdir`) and shell environment variables `PWD` and `OLDPWD`.

### Errors & Exit Codes
- Success: Exit code `0`
- Nonexistent directory: Exit code `1`
- Permission denied: Exit code `1`
- `$HOME` or `$OLDPWD` not set when required: Exit code `1`

---

## `pwd`

### Synopsis
`pwd [-L|-P]`

### Behavior
- Writes the absolute path of the current working directory to standard output.
- `-L` prints logical path (including symlinks from `PWD`).
- `-P` resolves all symlinks to physical directory path.

### Errors & Exit Codes
- Success: Exit code `0`
- Invalid option or I/O error: Exit code `1`

---

## `echo`

### Synopsis
`echo [-n] [string...]`

### Behavior
- Prints positional arguments separated by spaces, followed by a newline.
- `-n` suppresses the trailing newline.

### Errors & Exit Codes
- Success: Exit code `0`

---

## `export`

### Synopsis
`export [-p] [name[=value]...]`

### Behavior
- Marks shell variables for export to child processes.
- If `name=value` is given, assigns `value` and exports `name`.
- `-p` prints all exported variables in POSIX format (`export name="value"`).

### Errors & Exit Codes
- Success: Exit code `0`
- Invalid variable identifier: Exit code `1`

---

## `set`

### Synopsis
`set [-e|-x|-u|-o option]`

### Behavior
- `-e`: Exit immediately if a command exits with a non-zero status.
- `-x`: Print commands and their arguments as they are executed.
- `-u`: Treat unset variables as an error during expansion.
- Without arguments, prints all shell variables and functions.

### Errors & Exit Codes
- Success: Exit code `0`
- Invalid option: Exit code `2`

---

## `alias`

### Synopsis
`alias [name[=value]...]`

### Behavior
- Defines or displays command aliases.
- `alias name='command'` sets an alias.
- `alias` without arguments lists all active aliases.

### Errors & Exit Codes
- Success: Exit code `0`
- Alias name not found: Exit code `1`

---

## `exit`

### Synopsis
`exit [n]`

### Behavior
- Terminates the shell execution.
- If `n` is specified, uses `n` as the shell exit status.
- If `n` is omitted, uses the exit status of the preceding command.

### Errors & Exit Codes
- Exit status `n` (clamped to `0..255`).
