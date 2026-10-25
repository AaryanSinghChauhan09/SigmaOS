# SigmaOS Shell Diagnostics Guidelines

## Error Message Standard

All shell error messages use the prefix `sigma-sh:` to ensure greppable and clear diagnostic reporting.

## Standard Diagnostic Formats

### Parse Error
```text
sigma-sh: parse error at line <LINE>, col <COL>: <REASON> Near token '<TOKEN>'
```

### Command Not Found
```text
sigma-sh: <CMD>: command not found (did you mean '<SUGGESTION>'?)
```

### Redirection Error
```text
sigma-sh: <TARGET>: <REASON>
```

### Unclosed Block / Quote
```text
sigma-sh: unclosed construct '<BLOCK_TYPE>' originating at line <LINE>
```

### Job Termination
```text
[<JOB_ID>]+ Terminated: <SIGNAL_NAME>    <COMMAND_LINE>
```
