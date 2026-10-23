# SigmaOS Shell Diagnostics Engine

## Error Formatting Standards

### Command Not Found
```
sigma-sh: <command>: command not found (did you mean '<suggestion>'?)
```

### Parse Error
```
sigma-sh: parse error at line <line>, col <col>: <description>
```

### Redirection Error
```
sigma-sh: redirection error: <reason>
```
