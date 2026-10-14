# SigmaOS Shell POSIX Conformance Specification

## Purpose

Define the strict POSIX compatibility subset supported by `sigma-sh`. Full POSIX compliance is not claimed until the suite defined here passes without exceptions in CI.

## Documented POSIX Subset

### Supported Grammar Constructs

1. **Simple Commands**
   - Word execution: `command [arg1 arg2 ...]`
   - Assignment words prefix: `VAR=val command`

2. **Pipelines**
   - Standard execution: `cmd1 | cmd2`
   - Exit status propagation: Status of the rightmost command (or `pipefail` if set).

3. **Redirections**
   - Input redirection: `< file`
   - Output redirection (truncate): `> file`
   - Output redirection (append): `>> file`
   - Descriptor duplication: `2>&1`, `1>&2`

4. **Environment & Variables**
   - Scalar parameter expansion: `$VAR`, `${VAR}`
   - Special parameters: `$?`, `$$`, `$#`, `$0`

5. **Quoting**
   - Single quotes: `'literal'` - suppress all expansions.
   - Double quotes: `"string"` - allow parameter, command, and arithmetic expansion; preserve whitespace.
   - Backslash escape: `\c` - escape next character.

6. **Control Flow**
   - `if list; then list; [elif list; then list;] [else list;] fi`
   - `while list; do list; done`
   - `for var in word...; do list; done`
   - `case word in pattern) list ;; esac`

7. **Functions**
   - `name() { compound-list; }`

## Test Suite Structure

```text
tests/shell/
├── conformance/
│   ├── simple_command_test.rs
│   └── pipeline_test.rs
├── posix/
│   ├── quoting_test.rs
│   ├── control_flow_test.rs
│   └── variables_test.rs
├── regression/
│   └── parser_bugs_test.rs
└── interactive/
    └── line_editor_test.rs
```

## Non-Goals & Exclusions (Phase 1)

- Process substitution (`<(cmd)`)
- Arrays (Bash-specific)
- [[ Extended Test ]] syntax (POSIX uses `[ ... ]` or `test`)
