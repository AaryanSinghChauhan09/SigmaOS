# SigmaOS Shell Exit Codes Standard

- `0`: Success
- `1`: General command error
- `2`: Misuse / Shell syntax error
- `126`: Command invoked cannot execute
- `127`: Command not found
- `128+N`: Terminated by signal N (e.g. 130 for SIGINT)
