# SigmaOS Shell Grammar Specification

## Token Classes

- `WORD`: Sequence of non-operator characters, quoted strings, or expansions.
- `ASSIGNMENT_WORD`: Token of the form `NAME=VALUE` preceding a command name.
- `NAME`: Valid identifier string `[a-zA-Z_][a-zA-Z0-9_]*`.
- `OPERATOR`: Control operators (`|`, `&`, `;`, `\n`, `&&`, `||`, `;;`, `|&`) and redirection operators (`<`, `>`, `>>`, `<<`, `<&`, `>&`, `<>`, `>|`).
- `NEWLINE`: Line terminator `\n`.
- `EOF`: End-of-file token.

## Operator Precedence

1. Redirections (`<`, `>`, `>>`, `2>`) - Highest precedence.
2. Pipelines (`|`, `|&`).
3. AND/OR lists (`&&`, `||`).
4. Asynchronous / Sequential lists (`&`, `;`, `\n`) - Lowest precedence.

## Quoting Rules

- **Single Quote**: `'...'` disables all special character interpretation.
- **Double Quote**: `"..."` disables word splitting and globbing, but retains `$`, `` ` ``, `\`.
- **Escape Character**: `\` turns off special meaning of the next character.

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

```bnf
<pipeline> ::= [ "!" ] <command> ( "|" <command> )*
```

## List Grammar

```bnf
<list> ::= <and_or> ( ( ";" | "&" | "\n" ) <and_or> )* [ ";" | "&" | "\n" ]
<and_or> ::= <pipeline> ( ( "&&" | "||" ) <pipeline> )*
```

## Function Syntax

```bnf
<function_definition> ::= <name> "()" <compound_command> [ <redirect_list> ]
```

## Reserved Words

`if`, `then`, `else`, `elif`, `fi`, `case`, `esac`, `for`, `while`, `until`, `do`, `done`, `in`, `function`, `{`, `}`.

## Error Recovery Behavior

- **Non-Interactive Mode**: Parsing halts immediately on any parse error. Exit code `2` is returned and subsequent commands in the script are NOT executed.
- **Interactive Mode**: Syntax error message is printed to stderr with line and column information, line buffer is cleared, prompt is re-rendered, and terminal state is preserved.
