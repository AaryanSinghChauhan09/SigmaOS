# SigmaOS Shell Diagnostics Specification

## Diagnostics Requirements

All shell error output must follow greppable, consistent formats matching `sigma-sh: <context>: <message>`.

## Standard Formats

### 1. Parse Errors
```text
sigma-sh: parse error at line <line>, col <col>: <reason> [Near token '<token>']
```

### 2. Command Not Found
```text
sigma-sh: <command>: command not found [(did you mean '<suggestion>'?)]
```

### 3. Redirection Errors
```text
sigma-sh: <target>: <reason>
```

### 4. Unclosed Construct Errors
```text
sigma-sh: unclosed construct '<block_type>' originating at line <line>
```

## Levenshtein Distance Suggestion Engine

To improve developer UX, `sigma-sh` implements a zero-allocation edit-distance heuristic (threshold $\le 3$) to suggest similar commands when a command is not found in `$PATH` or built-in registry.
