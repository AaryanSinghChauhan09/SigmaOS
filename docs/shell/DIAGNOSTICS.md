# SigmaOS Shell Diagnostics

Diagnostics identify the shell, error class, source location when available,
and a safe recovery suggestion. Messages must not expose secrets or execute
input while formatting an error.

## Required Diagnostics

### 1. Parse Error with Line and Column
- Format: `sigma-sh: parse error at line <LINE>, col <COL>: <REASON> [Near token '<TOKEN>']`
- Example: `sigma-sh: parse error at line 1, col 5: unexpected token Near token ';'`

### 2. Unknown Command with Suggestions
- Format: `sigma-sh: <COMMAND>: command not found (did you mean '<SUGGESTION>'?)`
- Calculates Levenshtein edit distance against registered built-ins and known executables (threshold <= 3).

### 3. Redirection Failure
- Format: `sigma-sh: <TARGET>: <ERRNO/REASON>`
- Example: `sigma-sh: /etc/shadow: Permission denied`

### 4. Unclosed Construct or Quote
- Format: `sigma-sh: unclosed construct '<BLOCK_TYPE>' originating at line <LINE>`
- Example: `sigma-sh: unclosed construct 'single_quote' originating at line 3`

### 5. Nonzero Exit Status Visibility
- When verbose diagnostics or debugging mode is enabled, non-zero return codes are formatted cleanly to stderr without crashing the host session.
