# Study: Shell Word Splitting Mechanics

## Reference Systems
- POSIX sh / dash:
- Bash:
- Zsh:

## Mechanism
Field splitting on `IFS` whitespace and non-whitespace characters during expansion.

## SigmaOS Current State
Implemented in `src/shell/interactive/mod.rs` and `src/shell/diagnostics/mod.rs`.
