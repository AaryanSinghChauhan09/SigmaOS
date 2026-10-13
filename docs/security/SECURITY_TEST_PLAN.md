# SigmaOS Security Test Plan and Fuzzing Strategy

## Overview

Security verification in SigmaOS relies on continuous static analysis, memory sanitation checking, and fuzzing of untrusted inputs.

## Fuzzing Strategy

Continuous fuzzing targets in `fuzz/fuzz_targets/` exercise untrusted input parsers:

1. **IPC Parser (`fuzz/fuzz_targets/ipc_parser.rs`):** Fuzzes magic bytes, command flags, and token decoding invariants.
2. **VFS Path Parser (`fuzz/fuzz_targets/vfs_path_parser.rs`):** Fuzzes path normalization, directory traversal attempts, and null byte injection.
3. **Shell Lexer Parser (`fuzz/fuzz_targets/shell_lexer_parser.rs`):** Fuzzes word splitting, quote escaping, and command redirection parsing.

## Execution Procedure

Run fuzz targets locally or via CI (`.github/workflows/security.yml`):

```bash
cargo fuzz run ipc_parser -- -runs=10000
```
