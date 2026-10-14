# SigmaOS Shell Grammar & Expansion Specification

## Token Classes

- `WORD`: Normal string arguments, variable literals, and paths.
- `ASSIGNMENT_WORD`: Name=value variable bindings before simple commands.
- `OPERATOR`: Control operators (`|`, `&`, `;`, `\n`, `&&`, `||`) and redirection operators (`<`, `>`, `>>`, `<<`, `>&`, `<&`).
- `RESERVED_WORD`: Shell control keywords (`if`, `then`, `else`, `elif`, `fi`, `while`, `do`, `done`, `for`, `case`, `esac`, `in`).

## Operator Precedence

1. Redirections (`<`, `>`, `>>`)
2. Pipelines (`|`)
3. AND/OR lists (`&&`, `||`)
4. Sequential lists (`;`, `\n`)

## Expansion Order

`sigma-sh` strictly enforces expansion in the following deterministic sequence:

1. **Brace Expansion** (e.g., `a{b,c}` -> `ab ac`)
2. **Tilde Expansion** (e.g., `~` -> `$HOME`)
3. **Parameter Expansion** (e.g., `$VAR`, `${VAR}`)
4. **Command Substitution** (e.g., `$(cmd)`)
5. **Arithmetic Expansion** (e.g., `$((1 + 2))`)
6. **Word Splitting** (occurs on unquoted expansion results using `$IFS`)
7. **Pathname Expansion / Globbing** (e.g., `*.rs`)
8. **Quote Removal** (strips unescaped outer `'` and `"`)

## Quoting Rules

- **Single Quotes (`'...'`)**: All characters inside single quotes are literal. No expansions or parameter evaluations occur.
- **Double Quotes (`"..."`)**: Parameter expansion, command substitution, and arithmetic expansion occur. Word splitting and globbing are **disabled**.
- **Backslash (`\`)**: Escapes the immediately following character.

## Error Recovery & Parse Safety

- Syntax errors immediately stop execution in non-interactive script mode.
- Partially parsed pipelines or compound statements are never executed upon parse error (`no_partial_execution_on_parse_error`).
