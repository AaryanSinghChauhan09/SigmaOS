# SigmaOS Shell Grammar Specification

## Token Classes
- `WORD`: Sequence of non-operator characters or quoted strings.
- `ASSIGNMENT_WORD`: Word matching `NAME=VALUE`.
- `NAME`: Identifier matching `[a-zA-Z_][a-zA-Z0-9_]*`.
- `PIPE`: `|`
- `SEMI`: `;`
- `AND_IF`: `&&`
- `OR_IF`: `||`
- `GREAT`: `>`
- `DGREAT`: `>>`
- `LESS`: `<`
- `NEWLINE`: `\n`

## Operator Precedence
1. Redirections (`<`, `>`, `>>`)
2. Pipelines (`|`)
3. Logical AND / OR (`&&`, `||`)
4. Sequential Lists (`;`, `\n`)

## Expansion Order
1. Brace Expansion
2. Tilde Expansion
3. Parameter Expansion
4. Command Substitution
5. Arithmetic Expansion
6. Word Splitting
7. Pathname Expansion (Globbing)
8. Quote Removal

## Pipeline Grammar
```text
pipeline        ::= command ('|' command)*
command         ::= simple_command | compound_command
simple_command  ::= (assignment)* cmd_name (argument)* (redirection)*
```

## Reserved Words
`if`, `then`, `else`, `elif`, `fi`, `while`, `do`, `done`, `for`, `in`, `case`, `esac`

## Error Recovery
- On parse error in non-interactive mode: Halt immediately, emit line/column error diagnostic, exit with status 2.
- On parse error in interactive mode: Print error message, reset parser state, display prompt for new command.
