# SigmaOS Shell Built-in Command Contracts

## `cd`

### Synopsis
`cd [-L|-P] [directory]`

### Behavior
- Changes the shell working directory.
- Updates `PWD` and `OLDPWD` environment variables.
- Supports logical (`-L`) and physical (`-P`) path resolution.
- `cd -` changes directory to `OLDPWD` and prints the new directory path.

### Errors
- Nonexistent directory: prints error, exit status `1`.
- Permission denied: prints error, exit status `1`.
- Too many arguments: prints error, exit status `1`.

### Test Vectors
- `cd /tmp` (valid absolute path)
- `cd ..` (valid relative path)
- `cd -` (toggle back to previous directory)
- `cd /nonexistent_dir_1234` (exit status 1)

---

## `pwd`

### Synopsis
`pwd [-L|-P]`

### Behavior
- Prints the absolute path of the current working directory to stdout.
- `-L` prints logical path (including symlinks).
- `-P` resolves physical path (canonicalized).

### Errors
- Invalid option: exit status `2`.

---

## `echo`

### Synopsis
`echo [-n] [-e] [arg ...]`

### Behavior
- Prints arguments separated by single spaces, followed by a newline.
- `-n` suppresses trailing newline.
- `-e` enables backslash escape interpretation (`\n`, `\t`, `\\`).

### Errors
- Write failure: exit status `1`.

---

## `export`

### Synopsis
`export [-p] [name[=value] ...]`

### Behavior
- Sets the export attribute for shell variables, making them visible to child processes.
- `-p` lists all exported variables in output format.

### Errors
- Invalid variable name: exit status `1`.

---

## `exit`

### Synopsis
`exit [n]`

### Behavior
- Terminates script or interactive shell execution.
- Returns exit status `n` (0..255). If `n` is omitted, returns the exit status of the preceding command.

### Errors
- Non-numeric argument: exit status `2`.
