# SigmaOS Shell Grammar Specification

## Overview

This document specifies the formal grammar, token classes, expansion precedence, and error recovery rules for `sigma-sh`.

## Token Classes

- **WORD**: A sequence of characters representing a command, argument, variable, or string literal.
- **ASSIGNMENT_WORD**: Word containing `=` used for environment assignments (e.g., `FOO=bar`).
- **PIPE**: `|`
- **AND_IF**: `&&`
- **OR_IF**: `||`
- **SEMI**: `;`
- **AMP**: `&`
- **GREAT**: `>`
- **DGREAT**: `>>`
- **LESS**: `<`
- **GREATAND**: `>&`
- **LESSAND**: `<&`
- **LPAREN**: `(`
- **RPAREN**: `)`
- **LBRACE**: `{`
- **RBRACE**: `}`
- **NEWLINE**: `\n` or `\r\n`

## Grammar Rules

```ebnf
complete_command ::= list (SEMI | AMP | NEWLINE)*
list             ::= pipeline ( (AND_IF | OR_IF | SEMI | AMP) pipeline )*
pipeline         ::= [ "!" ] command ( PIPE command )*
command          ::= simple_command | compound_command [ redirection_list ]
simple_command   ::= ( ASSIGNMENT_WORD )* WORD ( WORD | redirection )*
compound_command ::= if_clause | while_clause | for_clause | case_clause | subshell | group_command
if_clause        ::= "if" list "then" list ( "elif" list "then" list )* [ "else" list ] "fi"
while_clause     ::= "while" list "do" list "done"
for_clause       ::= "for" WORD [ "in" ( WORD )* ] ( SEMI | NEWLINE ) "do" list "done"
redirection      ::= ( GREAT | DGREAT | LESS ) WORD | ( GREATAND | LESSAND ) WORD
```

## Operator Precedence

1. Redirections (`>`, `<`, `>>`, `2>&1`)
2. Pipeline (`|`)
3. Logical AND/OR (`&&`, `||`)
4. Sequential / Background (`Wait`, `;`, `&`, `\n`)

## Expansion Sequence

Execution evaluates expansions strictly in the following sequence:
1. Brace Expansion (`{a,b}`)
2. Tilde Expansion (`~`)
3. Parameter Expansion (`$VAR`)
4. Command Substitution (`$(cmd)`)
5. Arithmetic Expansion (`$((a + b))`)
6. Word Splitting (`IFS`)
7. Pathname Expansion (`*`, `?`)
8. Quote Removal

## Reserved Words

`if`, `then`, `else`, `elif`, `fi`, `case`, `esac`, `for`, `while`, `until`, `do`, `done`, `in`, `{`, `}`.

## Error Recovery

- **Non-Interactive Mode**: Lexical or syntactic errors abort script parsing immediately, returning exit status `2`.
- **Interactive Mode**: Syntax errors render a diagnostic error message with line/column pointers and restore the line editor for new input.
