# Shell Diagnostics Specification

## Diagnostics Principles
Diagnostics must be clear, actionable, and include precise source locations where appropriate.

## Formatting Standards

### 1. Parse Errors
Format: `sigma-sh: parse error at line <LINE>, col <COL>: <REASON> Near token '<TOKEN>'`
Example:
```text
sigma-sh: parse error at line 1, col 14: unexpected end of file, unclosed double quote Near token '"'
```

### 2. Unknown Commands
Format: `sigma-sh: <COMMAND>: command not found`
Example:
```text
sigma-sh: htep: command not found (did you mean 'http' or 'help'?)
```

### 3. Redirection Failures
Format: `sigma-sh: <FILENAME>: <SYS_ERROR_REASON>`
Example:
```text
sigma-sh: /root/protected.log: Permission denied
```

### 4. Job Control Status
Format: `[<JOB_ID>] <STATUS>    <COMMAND_LINE>`
Example:
```text
[1] Running    sleep 100 &
[1] Terminated sleep 100
```
