# Shell Exit Codes Specification

| Exit Code | Meaning |
|---|---|
| 0 | Success / Normal exit |
| 1 | General runtime or builtin error |
| 2 | Misuse of shell builtin or syntax error |
| 126 | Command invoked cannot execute |
| 127 | Command not found |
| 128+N | Fatal error signal N |
