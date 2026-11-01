# SigmaOS Shell Grammar

The grammar is parsed before expansion or execution. Invalid syntax produces a
deterministic parse error and must not partially execute the command list in non-interactive mode.

## Token Classes

- `WORD`: Sequence of non-metacharacters (e.g. identifiers, literal paths).
- `NAME`: Valid identifier token (letters, numbers, underscores, starting with letter/underscore).
- `OPERATOR`: Control operators (`|`, `&`, `;`, `&&`, `||`, `;;`) and redirection operators (`<`, `>`, `>>`, `<<`, `<&`, `>&`).
- `NEWLINE`: Line terminator separating sequential commands.
- `IO_NUMBER`: Optional numeric file descriptor preceding a redirection operator (e.g., `2>`).
- `EOF`: End of file / stream input.

## Operator Precedence

Precedence from highest to lowest:
1. Grouping and substitutions: `( )`, `{ }`, `$( )`, `` ` ` ``
2. Redirections: `<`, `>`, `>>`, `2>`, `>&`
3. Pipeline: `|`
4. Logical AND / OR: `&&`, `||`
5. Sequential lists and background: `;`, `&`, `\n`

## Quoting Rules

1. **Single Quotes (`'`)**: Everything between single quotes is treated strictly as literal data. No escape sequence is evaluated; single quotes cannot be nested.
2. **Double Quotes (`"`)**: Preserves literal characters except `$`, `` ` ``, `\`, and `!`. Parameter expansion, command substitution, and arithmetic expansion are performed, but word splitting and globbing are suppressed.
3. **Backslash (`\`/**Escape**)**: Escapes the subsequent character, turning metacharacters into literals. Inside double quotes, only `\$`, `` \` ``, `\"`, `\\`, and `\newline` retain special escape meaning.

## Authoritative Expansion Order

Expansion must strictly execute in the following 8-stage sequence:
1. `brace_expansion`: `{a,b,c}`, `{1..5}`
2. `tilde_expansion`: `~`, `~user`
3. `parameter_expansion`: `$VAR`, `${VAR:-default}`
4. `command_substitution`: `$(command)`, `` `command` ``
5. `arithmetic_expansion`: `$((expr))`
6. `word_splitting`: Split unquoted expansion results according to `$IFS`
7. `pathname_expansion`: Filename globbing (`*`, `?`, `[...]`)
8. `quote_removal`: Strip enclosing quote delimiters from literal words

## Pipeline Grammar

```ebnf
pipeline = [ '!' ] command { '|' command } ;
```

- Each command executes in its own subshell or isolated execution context.
- Output file descriptor 1 of command `i` is connected to input file descriptor 0 of command `i + 1`.

## List Grammar

```ebnf
and_or   = pipeline { ( '&&' | '||' ) pipeline } ;
command_list = and_or { ( ';' | '&' | NEWLINE ) and_or } [ ';' | '&' | NEWLINE ] ;
```

## Function Syntax

```ebnf
function_def = fname '(' ')' compound_command [ redirection_list ] ;
```

## Reserved Words

The following 12 reserved words are recognized at command start:
`if`, `then`, `else`, `elif`, `fi`, `while`, `do`, `done`, `for`, `case`, `esac`, `in`.

## Error Recovery Behavior

- **Non-interactive mode (Script runner)**: Fail fast. Abort entire parse without executing any prefix command. Output line/col diagnostics and exit with status `2`.
- **Interactive mode (REPL)**: Emit parse diagnostic to stderr, discard corrupted command buffer, reset line editor state, and redisplay prompt.
