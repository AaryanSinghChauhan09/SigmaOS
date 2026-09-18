# AI Agent Directive: Open-Source Tools Parity Suite Management

## Overview

The open-source tool parity engines (`src/tools/open_source_tools_engine.rs`, re-exported in `src/tools/mod.rs`) implement zero-dependency, `#![no_std]` native Rust parity engines for popular open-source CLI utilities in SigmaOS.

## Key Architectural Engines

1. **`RsyncDeltaSyncEngine`**:
   - Adler-32 rolling weak checksum and FNV-1a strong checksum matching for `rsync` block delta synchronization.

2. **`HtopProcessMonitorEngine`**:
   - `htop`/`btop` process tree rendering and multi-field sorting (`ByPid`, `ByCpu`, `ByMemory`, `ByName`).

3. **`BatSyntaxHighlighterEngine`**:
   - `bat` code syntax highlighting with custom line numbering and header formatting.

4. **`FzfFuzzyFinderEngine`**:
   - `fzf` Smith-Waterman style fuzzy string matching, boundary bonus scoring, and candidate ranking.

## Directives for AI Agents

- **Zero-Dependency Rule**: Maintain native Rust implementations without linking external C libraries or sub-processes.
- **Verification**: Run standalone unit tests using:
  ```bash
  rustc --test src/tools/open_source_tools_engine.rs --edition=2021 -o build/open_source_tools_test && ./build/open_source_tools_test
  ```
