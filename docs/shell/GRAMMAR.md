# Shell Formal Grammar Specification

```ebnf
command_line  ::= pipeline ( ( ';' | '&' | '&&' | '||' ) pipeline )* [ ';' | '&' ] ;
pipeline      ::= [ '!' ] command ( '|' command )* ;
command       ::= ( assignment | redirection | word )+ ;
redirection   ::= ( '<' | '>' | '>>' | '2>' | '2>&1' ) word ;
assignment    ::= IDENTIFIER '=' word ;
word          ::= ( CHAR | QUOTED_STRING | VARIABLE )+ ;
```
