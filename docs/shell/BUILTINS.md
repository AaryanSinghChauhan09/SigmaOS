# SigmaOS Shell Builtin Commands

## Builtin Reference

| Command | Description | Return Values |
|---------|-------------|---------------|
| `cd [dir]` | Change working directory | `0` on success, `1` on invalid directory |
| `pwd` | Print working directory | `0` |
| `echo [args...]` | Output arguments to standard output | `0` |
| `export [NAME=VAL]` | Set environment variable | `0` on success, `2` on invalid syntax |
| `exit [N]` | Exit shell session with status N | Exits process |
| `help` | Display list of supported commands | `0` |
| `history` | List command history entries | `0` |
| `type [cmd]` | Describe command classification | `0` if found, `1` if not found |
