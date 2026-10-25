# SigmaOS Built-in Command Contracts

## `cd`
### Synopsis
`cd [-L|-P] [directory]`
### Behavior
- Changes the shell working directory.
- Updates `PWD` and `OLDPWD`.
- Supports logical (`-L`) and physical (`-P`) path resolution.
### Errors
- Nonexistent directory: Exit status 1
- Permission denied: Exit status 1
- Too many arguments: Exit status 1
### Exit Status
- `0`: Success
- `1`: Failure

## `pwd`
### Synopsis
`pwd [-L|-P]`
### Behavior
- Prints current working directory.
- `-L` prints logical path (from `PWD`).
- `-P` prints physical canonical path.
### Exit Status
- `0`: Success

## `echo`
### Synopsis
`echo [-n] [string...]`
### Behavior
- Writes arguments separated by single spaces followed by newline.
- `-n`: Suppresses trailing newline.
### Exit Status
- `0`: Success

## `export`
### Synopsis
`export name[=value]...`
### Behavior
- Sets environment variable and marks for export to child processes.
### Exit Status
- `0`: Success
- `1`: Invalid identifier

## `exit`
### Synopsis
`exit [n]`
### Behavior
- Terminates current shell session.
- Exit code equals `n` (modulo 256). If `n` omitted, defaults to status of last command.

## `alias`
### Synopsis
`alias [name[=value]...]`
### Behavior
- Defines or displays command aliases.

## `set`
### Synopsis
`set [-o option] [arg...]`
### Behavior
- Sets shell flags (e.g., `-e` for errexit, `-x` for xtrace).
