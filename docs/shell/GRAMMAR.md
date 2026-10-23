# SigmaOS Shell Formal Grammar

## BNF Specification

```bnf
<command_line> ::= <pipeline> { ";" | "&" | "&&" | "||" }
<pipeline>     ::= <command> { "|" <command> }
<command>      ::= { <assignment> | <redirection> } [ <word> { <word> | <redirection> } ]
<redirection>  ::= "<" <word> | ">" <word> | ">>" <word> | "2>" <word>
<assignment>   ::= <IDENTIFIER> "=" <word>
<word>         ::= <LITERAL> | <QUOTED_STRING> | <VARIABLE_EXPANSION>
```

## Parsing Rules
- No partial execution occurs if a syntax parse error is detected during line analysis.
- Pipelines run asynchronously when terminated with `&`.
