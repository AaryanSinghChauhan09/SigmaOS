# SigmaOS Shell Grammar & Expansion Specification

## Token Classes & Lexer Rules
The lexer converts incoming characters into discrete tokens:
- `Word`: Standard argument, variable name, or literal text.
- `Operator`: `|`, `&`, `;`, `<`, `>`, `>>`, `<<`, `&&`, `||`, `;;`.
- `IoNumber`: File descriptor number immediately preceding `<` or `>`.
- `Newline`: Serves as command terminator when not escaped.
- `Eof`: End of input stream.

## Operator Precedence
1. Redirections (`<`, `>`, `>>`, `2>&1`)
2. Pipelines (`|`)
3. Logical AND / OR (`&&`, `||`)
4. Sequential / Background lists (`;`, `&`, `\n`)

## Expansion Sequence Order
Expansion occurs in strict adherence to `SHELL_RULES.toml` (`rule.safe_expansion_order`):

1. **Brace Expansion**: `{a,b}` -> `a b`
2. **Tilde Expansion**: `~` -> `$HOME`
3. **Parameter & Variable Expansion**: `$VAR` -> value
4. **Command Substitution**: `$(cmd)` or `` `cmd` `` -> output
5. **Arithmetic Expansion**: `$((1 + 2))` -> `3`
6. **Word Splitting**: Splits unquoted expansions according to `$IFS` (default space, tab, newline).
7. **Pathname / Glob Expansion**: Unquoted `*`, `?`, `[...]` expanded against VFS directory contents.
8. **Quote Removal**: Strips enclosing single/double quote delimiters.

## Grammar Specification (EBNF)

```ebnf
program         = complete_command ;
complete_command= list newline_or_eof ;
list            = pipeline { ( ";" | "&&" | "||" | "&" ) pipeline } ;
pipeline        = [ "!" ] command { "|" command } ;
command         = simple_command | compound_command ;
simple_command  = { assignment | redirection } word { word | redirection } ;
compound_command= if_clause | while_clause | for_clause | subshell ;
if_clause       = "if" list "then" list { "elif" list "then" list } [ "else" list ] "fi" ;
while_clause    = "while" list "do" list "done" ;
for_clause      = "for" name [ "in" { word } ] "do" list "done" ;
subshell        = "(" list ")" ;
redirection     = [ IONUMBER ] ( "<" | ">" | ">>" | "<<" ) word ;
```

## Error Recovery
In non-interactive mode, parse errors abort execution immediately prior to running any command.
In interactive mode, parse errors display a formatted diagnostic and reset the prompt state without terminating the process.
