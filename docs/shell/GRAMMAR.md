# SigmaOS Shell Grammar Specification

## Lexer Token Classes

The lexer converts raw input streams into discrete tokens:

- **WORD**: Literals, unquoted/quoted strings, variable references.
- **ASSIGNMENT_WORD**: Words containing `=` used before command names (`KEY=val`).
- **NAME**: Valid shell identifier names `[a-zA-Z_][a-zA-Z0- his_0-9]*`.
- **OPERATORS**:
  - `|` (PIPE)
  - `&&` (AND_IF)
  - `||` (OR_IF)
  - `;` (SEMI)
  - `&` (AMP)
  - `>` (GREAT), `>>` (DGREAT), `<` (LESS)
  - `\n` (NEWLINE)
- **RESERVED_WORDS**: `if`, `then`, `else`, `elif`, `fi`, `while`, `do`, `done`, `for`, `case`, `esac`, `in`.

## Operator Precedence

From highest to lowest binding precedence:

1. Redirection (`>`, `>>`, `<`)
2. Pipeline (`|`)
3. AND / OR lists (`&&`, `||`)
4. Sequential / Asynchronous lists (`;`, `&`, `\n`)

## Pipeline and List Grammar

```ebnf
complete_command ::= list (';' | '&' | '\n' | EOF)

list             ::= and_or ((';' | '&' | '&&' | '||') and_or)*

and_or           ::= pipeline (('&&' | '||') pipeline)*

pipeline         ::= ['!'] command ('|' command)*

command          ::= simple_command | compound_command

simple_command   ::= (assignment)* cmd_name (cmd_arg)* (redirection)*

compound_command ::= if_clause | while_clause | for_clause | case_clause | subshell
```

## Error Recovery Behavior

- **Non-interactive mode**: Parse errors cause immediate script abort with exit code `2`. No partial command execution occurs.
- **Interactive mode**: Parse errors render a diagnostic error message with line and column information, reset parser state, and return prompt control to the user without terminating the process.
