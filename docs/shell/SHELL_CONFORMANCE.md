# POSIX Shell Compatibility Subset Specification

## Baseline Objective
Define the precise subset of POSIX IEEE Std 1003.1-2017 (Shell & Utilities) supported by the SigmaOS shell (`sigma_sh`). No claim of full POSIX compliance is made unless tested and validated against this specification.

## Documented POSIX Subset

### 1. Simple Commands
- Command word lookup: Built-in commands first, followed by `$PATH` resolution.
- Argument vector construction: Null-terminated parameter array preserving inner whitespace when quoted.
- Environment assignment prefixes: `VAR=val command` scope overrides for child execution.

### 2. Quoting Rules
- Single Quotes (`'...'`): Preserves literal value of all characters inside quotes. No variable or glob expansion.
- Double Quotes (`"..."`): Preserves literal value of characters except `$`, `` ` ``, and `\`.
- Escape Character (`\`): Preserves literal value of the next character.

### 3. Pipelines & Redirections
- Pipe (`|`): Connects standard output of left command to standard input of right command. Exit status is the exit status of the rightmost command (unless `pipefail` is enabled).
- Input Redirection (`< file`): Opens file for reading on fd 0.
- Output Redirection (`> file`, `>> file`): Opens file for writing/appending on fd 1.
- Error Redirection (`2> file`, `2>&1`): Handles standard error descriptor remapping.

### 4. Control Structures
- `if condition; then list; [elif list; then list;] [else list;] fi`
- `while condition; do list; done`
- `for var in words; do list; done`
- `case word in pattern) list ;; esac`

### 5. Parameter and Variable Expansion
- Basic variable substitution: `$VAR`, `${VAR}`.
- Default value substitution: `${VAR:-default}`.
- String length: `${#VAR}`.
- Special variables: `$?` (exit code), `$$` (PID), `$#` (arg count), `$0`..`$9` (positional parameters).

### 6. Functions
- Syntax: `name() { list; }` or `function name { list; }`.
- Local execution context with positional argument isolation.

## Test Suite Structure
- `tests/shell/conformance/`: Full compliance tests.
- `tests/shell/posix/`: POSIX standard behavior validation.
- `tests/shell/regression/`: Fixed bug edge cases.
