# SigmaOS Shell Exit Codes Specification

## Standard Status Codes

- `0`: Success / Normal Completion
- `1`: General command error
- `2`: Shell misusage or parse error
- `126`: Command requested is non-executable
- `127`: Command requested was not found in `$PATH`
- `130`: Command terminated by `SIGINT` (Ctrl+C)
- `137`: Command killed by `SIGKILL`
- `143`: Command terminated by `SIGTERM`
