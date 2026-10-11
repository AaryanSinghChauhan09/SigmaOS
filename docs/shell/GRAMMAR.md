# SigmaOS Shell Grammar Specification

## Lexer & Token Classes

| Token Class | Pattern / Examples | Description |
|---|---|---|
| `WORD` | `alpha`, `123`, `foo.txt` | Unquoted string or literal argument |
| `ASSIGNMENT_WORD` | `NAME=value` | Environment/shell variable assignment |
| `STRING_LITERAL` | `'single quoted'` | Literal string without expansion |
| `STRING_EXPAND` | `"double $quoted"` | Interpolated string allowing parameter expansion |
| `PIPE` | `\|` | Pipeline separator |
| `AND_IF` | `&&` | Logical AND list separator |
| `OR_IF` | `\|\|` | Logical OR list separator |
| `SEMI` | `;` | Sequential command separator |
| `AMP` | `&` | Asynchronous background execution operator |
| `GREAT` / `DGREAT` | `>`, `>>` | Truncate / Append output redirection |
| `LESS` | `<` | Input redirection |
| `GREATAND` | `>&`, `2>&1` | File descriptor duplication operator |
| `LPAREN` / `RPAREN` | `(`, `)` | Subshell group delimiter |
| `LBRACE` / `RBRACE` | `{`, `}` | Group command block delimiter |

## Operator Precedence

1. **Redirections**: `<`, `>`, `>>`, `2>&1` (evaluated left-to-right per command)
2. **Pipelines**: `|` (associates commands into pipeline nodes)
3. **Sequential & Async Lists**: `;`, `&`, `\n`
4. **Logical Control Lists**: `&&`, `||`

## Strict Expansion Order

To enforce `[rule.safe_expansion_order]`, expansion MUST strictly occur in this sequence:

1. **Brace Expansion**: `{a,b}` -> `a b`
2. **Tilde Expansion**: `~` -> `$HOME`
3. **Parameter Expansion**: `$VAR`, `${VAR:-default}`
4. **Command Substitution**: `$(command)` or `` `command` ``
5. **Arithmetic Expansion**: `$((1 + 2))`
6. **Word Splitting**: Splitting unquoted expansion results on `$IFS` (default space/tab/newline)
7. **Pathname Expansion (Globbing)**: `*.rs`, `?oo` (skipped if word is quoted)
8. **Quote Removal**: Stripping unescaped outer quote characters

## Pipeline & List Grammar

```ebnf
complete_command ::= list newline_or_semi?

list             ::= pipeline (('&&' | '||' | ';' | '&') pipeline)*

pipeline         ::= '!''? command ('|' command)*

command          ::= simple_command
                   | compound_command redirection*
                   | function_definition

simple_command   ::= (ASSIGNMENT_WORD)* WORD+ redirection*
```

## Error Recovery & Parse Safety

- **Non-Interactive Mode**: A parse error aborts execution immediately. No partially parsed AST node is executed.
- **Interactive Mode**: A parse error prints diagnostic details (line, column, token) and resets the prompt without terminating the shell session.
