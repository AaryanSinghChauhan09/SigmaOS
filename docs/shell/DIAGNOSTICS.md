# SigmaOS Shell Diagnostics Engine

## Overview

`sigma-sh` includes a structured diagnostics engine providing clear error messages, line/column location indicators, command suggestions, and redirection error reporting.

## Diagnostic Error Types

### 1. Parse & Syntax Errors
- **Format**: `sigma-sh: parse error at line <line>, col <col>: <reason> [Near token '<token>']`
- **Example**: `sigma-sh: parse error at line 1, col 12: unexpected token Near token ';'`

### 2. Command Not Found
- **Format**: `sigma-sh: <cmd>: command not found [(did you mean '<suggestion>'?)]`
- **Example**: `sigma-sh: htep: command not found (did you mean 'help'?)`
- Uses Levenshtein distance matching against `$PATH` executables and built-ins.

### 3. Redirection Errors
- **Format**: `sigma-sh: <target>: <errno_description>`
- **Example**: `sigma-sh: /etc/shadow: Permission denied`

### 4. Unclosed Construct Errors
- **Format**: `sigma-sh: unclosed construct '<construct>' originating at line <line>`
- **Example**: `sigma-sh: unclosed construct 'if' originating at line 3`

### 5. Job Termination Diagnostics
- Output notification when background jobs complete or receive terminating signals (e.g., `[1]+ Terminated: 15  sleep 100`).
