# SigmaOS Shell Builtins

Builtins have explicit argument contracts, deterministic diagnostics, and
documented exit statuses. Contract changes require focused tests and rollback
notes.

## `cd`

### Synopsis
`cd [-L|-P] [directory]`

### Behavior
- Changes the shell working directory to `directory` (or to `$HOME` if no argument is supplied).
- Updates `PWD` and `OLDPWD` environment variables.
- When `-` is passed as directory, changes to `$OLDPWD` and prints the path.
- Supports `-L` (logical path resolution including symlinks) and `-P` (physical path resolution).

### Errors & Exit Codes
- `0`: Directory change succeeded.
- `1`: Nonexistent directory or permission denied.
- `2`: Invalid command options supplied.

### Tests
- Valid relative path
- Valid absolute path
- Nonexistent path
- Permission-denied path
- `cd -`
- `PWD` and `OLDPWD` updates

---

## `pwd`

### Synopsis
`pwd [-L|-P]`

### Behavior
- Prints the absolute pathname of the current working directory.
- Defaults to `-L` (logical path honoring `$PWD`).
- `-P` resolves all symlinks to physical disk path.

### Errors & Exit Codes
- `0`: Success.
- `1`: Error retrieving working directory or writing output.

### Tests
- Default logical path display
- Physical path resolution with symlinks
- Exit status verification

---

## `echo`

### Synopsis
`echo [-n|-e|-E] [args...]`

### Behavior
- Outputs arguments separated by a single space, followed by a newline unless `-n` is supplied.
- `-e` enables backslash escape interpretations (`\n`, `\t`, `\\`).
- `-E` disables escape interpretation (default).

### Errors & Exit Codes
- `0`: Always succeeds unless write error occurs on stdout.

### Tests
- Basic space-separated arguments
- Trailing newline suppression (`-n`)
- Escape code expansion (`-e`)

---

## `export`

### Synopsis
`export [-p] [name[=word]...]`

### Behavior
- Marks shell variables for automatic export to the environment of subsequently executed commands.
- If no arguments or `-p` is provided, prints all currently exported variables.

### Errors & Exit Codes
- `0`: Success.
- `1`: Invalid variable name format.

### Tests
- Single and multiple export assignments
- Export reflection in child process environments
- Listing exports with `-p`

---

## `alias`

### Synopsis
`alias [name[=value]...]`

### Behavior
- Defines or displays command aliases.
- Without arguments, prints all defined aliases formatted for re-entry.

### Errors & Exit Codes
- `0`: Success.
- `1`: Named alias not found.

### Tests
- Alias creation and invocation
- Overwrite existing alias
- Unalias and alias inspection

---

## `set`

### Synopsis
`set [--] [-e|-u|-x|-o option] [arg...]`

### Behavior
- Sets or unsets shell options and positional parameters.
- `-e`: Exit immediately if a pipeline returns non-zero status.
- `-u`: Treat unset variables as an error during expansion.
- `-x`: Print commands and their arguments as they are executed.

### Errors & Exit Codes
- `0`: Success.
- `1` or `2`: Invalid option or syntax error.

### Tests
- Set/unset `-e`, `-u`, `-x`
- Positional parameter modification

---

## `exit`

### Synopsis
`exit [n]`

### Behavior
- Causes the shell to terminate with exit status `n`.
- If `n` is omitted, exit status is that of the last executed command.
- In interactive shell, triggers terminal restore and session teardown.

### Errors & Exit Codes
- `n`: User-specified exit code (0–255 modulo 256).
- `2`: Non-numeric argument error.

### Tests
- Explicit exit code (e.g., 0, 1, 42)
- Default exit status inheritance (`$?`)
- Terminal state restoration on exit
