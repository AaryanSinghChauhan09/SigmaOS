# Built-In Command Contracts

Every built-in command in SigmaOS shell must strictly adhere to its defined contract.

---

## `cd`

### Synopsis
`cd [-L|-P] [directory]`

### Behavior
- Changes current working directory of the shell process.
- Updates `PWD` to new absolute path and sets `OLDPWD` to former working directory.
- `cd` with no arguments changes directory to `$HOME`.
- `cd -` changes directory to `$OLDPWD` and prints the new directory to stdout.

### Exit Codes
- `0`: Directory changed successfully.
- `1`: Non-existent path, permission denied, or not a directory.

---

## `pwd`

### Synopsis
`pwd [-L|-P]`

### Behavior
- Prints absolute pathname of the current working directory.
- Default mode `-L` displays logical directory path including symlinks.
- Mode `-P` resolves physical path symlinks.

### Exit Codes
- `0`: Success.
- `1`: Invalid option or error reading current working directory.

---

## `echo`

### Synopsis
`echo [-n] [-e] [arg ...]`

### Behavior
- Outputs arguments separated by single spaces and terminated by a newline.
- `-n`: Suppresses trailing newline.
- `-e`: Enables interpretation of backslash escapes (`\n`, `\t`, `\\`).

### Exit Codes
- `0`: Always returns 0.

---

## `export`

### Synopsis
`export [NAME[=VALUE] ...]` or `export -p`

### Behavior
- Marks shell variables to be exported to child processes.
- `export NAME=VALUE` sets variable value and exports it in one step.
- `export -p` prints all exported variables in POSIX reusable format.

### Exit Codes
- `0`: Success.
- `1`: Invalid variable identifier.

---

## `exit`

### Synopsis
`exit [n]`

### Behavior
- Terminates the shell process.
- If `n` is specified, exit status is `n % 256`.
- If `n` is omitted, exit status is the exit code of the last executed command.

### Exit Codes
- `n`: Specified status code or `$?`.

---

## `alias` & `unalias`

### Synopsis
`alias [name[=value] ...]` / `unalias name`

### Behavior
- `alias`: Lists defined aliases.
- `alias name=value`: Creates or updates an alias.
- `unalias name`: Removes an alias definition.

### Exit Codes
- `0`: Success.
- `1`: Alias not found or invalid identifier.
