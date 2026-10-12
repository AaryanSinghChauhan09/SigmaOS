# SigmaOS Shell Diagnostics Specification

## Diagnostics Requirements

### 1. Parse Error Location Tracking
Parse error diagnostics must include exact line and column coordinates:
```text
sigma-sh: parse error at line 1, col 5: unexpected token '|'
```

### 2. Command Suggestions
When an executed command is not found in `$PATH` or built-ins, the shell performs Levenshtein distance matching against valid command names and generates helpful suggestions:
```text
sigma-sh: htep: command not found (did you mean 'help'?)
```

### 3. Redirection Error Formatting
Redirection failures state the target file path and errno reason:
```text
sigma-sh: /etc/readonly_config: Permission denied
```

### 4. Syntax Diagnostics
Unclosed quote or block diagnostics clearly state the missing matching delimiter:
```text
sigma-sh: syntax error: unexpected EOF while looking for matching '"'
```
