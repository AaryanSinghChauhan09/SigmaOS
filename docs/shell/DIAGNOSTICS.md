# SigmaOS Shell Diagnostics Specification

## Required Diagnostic Outputs

### 1. Parse Errors
Format:
```text
sigma-sh: parse error at line <line>, col <col>: <description> (near '<token>')
```
Example:
```text
sigma-sh: parse error at line 1, col 12: unexpected EOF (near ';')
```

### 2. Unknown Commands
Format:
```text
sigma-sh: <command>: command not found
```
When safe suggestions exist:
```text
sigma-sh: <command>: command not found (did you mean '<suggestion>'?)
```

### 3. Redirection Failures
Format:
```text
sigma-sh: <filename>: <reason>
```
Example:
```text
sigma-sh: /etc/shadow: Permission denied
```

### 4. Unclosed Quotes / Blocks
Format:
```text
sigma-sh: unexpected EOF while looking for matching '<quote>'
```

### 5. Job Termination & Exit-Status Visibility
- Prints job termination status on background job completion or signal termination.
- Exposes exit status of preceding command via `$?`.
