# SigmaOS Shell (sigma-sh) Grammar and Compatibility Specification

**Version:** 1.0  
**Status:** Draft  
**Last Updated:** 2025-01-22  
**Purpose:** Define the shell grammar, parsing, and POSIX compatibility targets for sigma-sh

---

## 1. Overview

SigmaOS Shell (sigma-sh) is a POSIX-compatible command shell designed for SigmaOS. It prioritizes:
- **POSIX.1-2017 compliance** for shell command language
- **Safety** - No unexpected side effects or security issues
- **Simplicity** - Clear, maintainable implementation
- **Performance** - Efficient parsing and execution

---

## 2. Compatibility Target

### 2.1 POSIX Subset

sigma-sh aims to support the **POSIX.1-2017 Shell Command Language** subset, excluding:

- **Bash-specific features** (arrays, [[ ]] test, process substitution)
- **Zsh-specific features** (glob qualifiers, parameter expansion modifiers)
- **Advanced job control** (job control signals, terminal control groups)

### 2.2 Supported Features

#### Supported
- Command substitution `$(command)` and backticks `` `command` ``
- Pipelines `|`
- Redirections `<`, `>`, `>>`, `2>`, `2>&1`
- Simple variables `VAR=value`
- Environment variable expansion `$VAR`, `${VAR}`
- Conditional execution `&&`, `||`
- Basic conditionals `if ... then ... else ... fi`
- Basic loops `for ... do ... done`, `while ... do ... done`
- Background execution `&`
- Quoting `'`, `"`, `\`
- Built-in commands: `cd`, `pwd`, `export`, `unset`, `echo`, `test`, `exit`

#### Not Supported (Initial Version)
- Arrays
- Functions
- Arithmetic expansion `$((...))`
- Brace expansion `{a,b,c}`
- Process substitution `<(command)`, `>(command)`
- Extended globbing
- Here documents
- Coprocesses

---

## 3. Grammar Specification

### 3.1 Token Types

```rust
pub enum Token {
    Word(String),
    Assignment(String, String),  // VAR=value
    Pipe,                         // |
    RedirectIn,                   // <
    RedirectOut,                  // >
    RedirectAppend,              // >>
    RedirectErr,                  // 2>
    RedirectErrOut,              // 2>&1
    And,                          // &&
    Or,                           // ||
    Semicolon,                    // ;
    Ampersand,                    // &
    If,                           // if
    Then,                         // then
    Else,                         // else
    Fi,                           // fi
    For,                          // for
    Do,                           // do
    Done,                         // done
    While,                        // while
    Until,                        // until
    LeftParen,                    // (
    RightParen,                   // )
    Newline,
}
```

### 3.2 AST Structure

```rust
pub enum AstNode {
    Command(Vec<String>, Vec<Redirection>),
    Pipeline(Vec<AstNode>),
    And(Box<AstNode>, Box<AstNode>),
    Or(Box<AstNode>, Box<AstNode>),
    Background(Box<AstNode>),
    Sequence(Vec<AstNode>),
    If(Box<AstNode>, Vec<AstNode>, Option<Vec<AstNode>>),
    For(String, Vec<String>, Vec<AstNode>),
    While(Box<AstNode>, Vec<AstNode>),
    Subshell(Box<AstNode>),
}

pub enum Redirection {
    In(String),           // < file
    Out(String),          // > file
    Append(String),       // >> file
    Err(String),          // 2> file
    ErrOut(String),       // 2>&1
}
```

### 3.3 BNF Grammar

```
program ::= line_list
line_list ::= line | line_list line
line ::= pipeline | line '&' | line ';' | if_stmt | for_stmt | while_stmt
pipeline ::= command | pipeline '|' command
command ::= simple_command | '(' pipeline ')'
simple_command ::= word_list | assignment word_list
word_list ::= word | word_list word
assignment ::= WORD '=' WORD

if_stmt ::= 'if' pipeline 'then' line_list 'fi'
          | 'if' pipeline 'then' line_list 'else' line_list 'fi'

for_stmt ::= 'for' WORD 'in' word_list 'do' line_list 'done'
          | 'for' WORD 'do' line_list 'done'

while_stmt ::= 'while' pipeline 'do' line_list 'done'
```

---

## 4. Implementation Status

| Component | Status | Implementation | Tests |
|-----------|--------|----------------|-------|
| Lexer | Prototype | `src/shell/lexer.rs` stub | Basic tests |
| Parser | Prototype | `src/shell/parser.rs` stub | Basic tests |
| AST | Prototype | Basic structures | None |
| Variable expansion | Not started | None | None |
| Command execution | Not started | None | None |
| Pipeline execution | Not started | None | None |
| Redirection handling | Not started | None | None |
| Built-in commands | Not started | None | None |

---

## 5. Security Considerations

### 5.1 Input Validation

- Reject commands with null bytes
- Validate file paths before opening
- Prevent command injection through variables

### 5.2 Resource Limits

- Limit recursion depth
- Limit pipeline length
- Limit command argument count

### 5.3 Sandboxing

- Shell commands run with Landlock/Capsicum restrictions
- File access scoped via pledge/unveil
- Network access requires explicit permission

---

## 6. Testing Strategy

### 6.1 Unit Tests

- Lexer tokenization tests
- Parser AST generation tests
- Variable expansion tests
- Built-in command tests

### 6.2 Integration Tests

- POSIX shell test suite
- Command execution tests
- Pipeline tests
- Redirection tests

### 6.3 Security Tests

- Command injection tests
- Path traversal tests
- Resource limit tests

---

## 7. Next Steps

1. Complete lexer implementation
2. Complete parser implementation
3. Implement basic command execution
4. Implement variable expansion
5. Implement built-in commands
6. Add comprehensive tests
7. Run POSIX shell test suite

---

## 8. References

- [POSIX.1-2017 Shell Command Language](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html)
- [Bash Reference Manual](https://www.gnu.org/software/bash/manual/)
- [PROJECT_STATUS.md](PROJECT_STATUS.md)
- [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md)
