# Shell and Coreutils

SigmaOS provides a complete set of GNU-compatible core utilities and shell primitives rewritten entirely from scratch in `no_std`, `no_alloc` Rust to ensure absolute security and memory safety without standard C libraries.

## Shell Core
- **Environment & History (`sigma_env.rs`)**: Manages static arrays for shell environment variables and command history, avoiding heap usage.
- **Tab Completion (`sigma_complete.rs`)**: Prefix matching engine for autocomplete of built-in commands.

## Custom Coreutils
SigmaOS has its own custom implementations of common CLI tools:
- **`sort` (`sigma_sort.rs`)**: In-memory line sorting.
- **`uniq` (`sigma_uniq.rs`)**: Adjacent duplicate line elimination.
- **`sed` (`sigma_sed.rs`)**: Basic string substitution stream editor.
- **`awk` (`sigma_awk.rs`)**: Field extraction based on delimiters.
- **`find` (`sigma_find.rs`)**: Virtual filesystem directory traversal.
- **`xargs` (`sigma_xargs.rs`)**: Standard input to command argument builder.
