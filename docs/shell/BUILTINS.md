# SigmaOS Shell Built-in Command Contracts

## Overview

Built-in commands execute directly within the `sigma-sh` process context to manipulate shell parameters, environment variables, working directory, or execution state.

---

## `cd`

### Synopsis
```sh
cd [-L|-P] [directory]
```

### Behavior
- Changes the current working directory of the shell process.
- Updates `PWD` to the new directory and `OLDPWD` to the previous directory.
- `cd -` changes directory to `OLDPWD` and prints the new directory.
- Default directory (no arguments): resolves to `$HOME`.

### Errors & Exit Codes
- Success: `0`
- Directory does not exist: `1`
- Permission denied: `1`
- Not a directory: `1`

---

## `pwd`

### Synopsis
```sh
pwd [-L|-P]
```

### Behavior
- Prints the absolute pathname of the current working directory.
- `-L` prints logical path (preserving symlinks in `PWD`).
- `-P` prints physical path (resolving symlinks).

### Errors & Exit Codes
- Success: `0`
- Invalid option: `2`

---

## `echo`

### Synopsis
```sh
echo [-n] [string ...]
```

### Behavior
- Outputs arguments separated by spaces, followed by a newline.
- `-n` suppresses trailing newline.

### Errors & Exit Codes
- Always succeeds: `0`

---

## `export`

### Synopsis
```sh
export [name[=value] ...]
```

### Behavior
- Sets export attribute for shell parameters, making them available to child processes.
- Without arguments, prints all exported variables formatted for shell re-entry (`export VAR="val"`).

### Errors & Exit Codes
- Success: `0`
- Invalid variable identifier: `1`

---

## `alias`

### Synopsis
```sh
alias [name[=value] ...]
```

### Behavior
- Defines or displays command aliases.
- `alias name="value"` registers alias.
- Without arguments, prints all defined aliases.

### Errors & Exit Codes
- Success: `0`
- Alias name not found: `1`

---

## `set`

### Synopsis
```sh
set [-o option] [+o option] [arg ...]
```

### Behavior
- Sets or unsets shell options (`set -e` for exit on error, `set -u` for unset variable check).
- Without options, lists all active shell variables.

### Errors & Exit Codes
- Success: `0`
- Invalid option: `2`

---

## `exit`

### Synopsis
```sh
exit [n]
```

### Behavior
- Terminates the shell process with exit status `n`.
- If `n` is omitted, returns the status of the last executed command.

### Errors & Exit Codes
- Non-numeric argument: exit status `2`
