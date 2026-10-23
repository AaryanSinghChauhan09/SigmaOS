# SigmaOS Shell Diagnostics Engine Specification

## Overview

The shell diagnostics engine produces clear, actionable error messages for parse, runtime, redirection, and environment failures.

## Formatting Rules

### 1. Parse Errors
Formats syntax errors with explicit line and column coordinates:
```text
sigma-sh: parse error at line <line>, col <col>: <description> near token '<token>'
```

### 2. Command Not Found
Provides distance-based command suggestions for typos:
```text
sigma-sh: <command>: command not found (did you mean '<suggestion>'?)
```

### 3. Redirection Failures
Includes path and underlying system error reason:
```text
sigma-sh: <path>: <error_message>
```
Example: `sigma-sh: /etc/shadow: Permission denied`

### 4. Unclosed Quote or Block
Identifies missing closing delimiter:
```text
sigma-sh: syntax error: unexpected end of file while looking for matching '<quote>'
```

### 5. Exit Status Visibility
When execution tracing or debugging is active (`set -x` or diagnostic level `DEBUG`), prints exit code of failed commands:
```text
[sigma-sh: command exited with status <code >]
```
