# SigmaOS Built-in Command Contracts

Every built-in in `sigma-sh` must maintain an explicit contract documenting synopsis, behavior, side effects, error conditions, and required unit tests.

---

## `cd`

### Synopsis
`cd [-L|-P] [directory]`

### Behavior
- Changes current working directory to `directory`.
- If no argument is provided, defaults to `$HOME`.
- If argument is `-`, changes to `$OLDPWD` and prints new directory.
- Updates environment variables `PWD` and `OLDPWD`.
- Default resolution mode is logical (`-L`).

### Errors
- Directory does not exist: exit status `1`, prints error to `stderr`.
- Permission denied: exit status `1`.
- Too many arguments: exit status `1`.

### Tests
- Valid relative and absolute paths.
- Nonexistent path error handling.
- `cd -` directory toggle.
- `PWD` and `OLDPWD` correctness.

---

## `pwd`

### Synopsis
`pwd [-L|-P]`

### Behavior
- Prints current working directory to `stdout`.
- `-L`: Output logical directory (including symlinks, default).
- `-P`: Output physical directory (resolving symlinks).

### Errors
- Invalid option: exit status `2`.

---

## `echo`

### Synopsis
`echo [-n] [string...]`

### Behavior
- Prints arguments separated by space followed by newline.
- `-n`: Omit trailing newline.

### Errors
- Write failure: exit status `1`.

---

## `export`

### Synopsis
`export [name[=value]...]`

### Behavior
- Marks environment variables for export to child processes.
- If no argument given, prints exported variables.

### Errors
- Invalid variable name: exit status `1`.

---

## `exit`

### Synopsis
`exit [n]`

### Behavior
- Terminates current shell execution.
- If `n` specified, returns exit status `n % 256`.
- If `n` omitted, returns exit status of last executed command.

### Errors
- Non-numeric argument: exit status `2`.

---

## `alias`

### Synopsis
`alias [name[=value]...]`

### Behavior
- Defines or displays command shortcuts.

---

## `set`

### Synopsis
`set [-e] [-u] [-x] [-o option]`

### Behavior
- Sets shell execution flags (`-e` exit on error, `-u` unset variable error, `-x` trace execution).
