# AI Agent Userland Coreutils Management Guidelines

## 1. Overview & Architecture
This document details AI agent procedures for maintaining and extending native userland replacements, multi-distro format execution (`CoreutilFormat`), GNU Coreutils equivalents (`ls`, `cp`, `mv`, `rm`, `cat`, `grep`, `mkdir`, `head`, `tail`, `uniq`, `cut`, `tr`, `stat`, `df`, `du`, `uname`, `env`, `wc`), and process diagnostic utilities (`ps`, `top`, `free`, `uptime`) in SigmaOS (`src/userland/coreutils.rs`).

---

## 2. Multi-Distro Coreutils Parity Framework

SigmaOS provides multi-format execution and option parsing via `UniversalCoreutilsEngine`:
- **GNU Coreutils (`gnu-`)**: Long flags (`--human-readable`, `--lines`, `--bytes`, `--words`, `--reverse`, `--numeric-sort`), GNU `echo -e` escape sequence interpretation (`\n`, `\t`), GNU `stat` output.
- **BSD Coreutils (`bsd-`)**: Traditional BSD flags, BSD `stat` format string output (`16777220 12345678 ...`).
- **BusyBox Applets (`busybox-`)**: Single multicall binary multiplexing (`MultiCallManager`), lightweight applet execution.
- **Toybox Applets (`toybox-`)**: Android / Toybox compatible multi-applet argument handling.
- **uutils Rust Coreutils (`uutils-`)**: Safe-Rust modern CLI compatibility.
- **Sovereign Native**: Native microkernel-aware core utility execution.

---

## 🚀 3-Phase Coreutils Development Roadmap

### Phase 1: Format Parsing & Multicall Dispatching (0–6 Months)
- **Prefix Inference**: Automatically parse `gnu-`, `bsd-`, `busybox-`, `toybox-`, and `uutils-` prefixes in `UniversalCoreutilsEngine::infer_format`.
- **Multicall Sanitization**: Strip format prefixes cleanly in `MultiCallManager::dispatch` to route calls seamlessly.
- **CLI Option Parsing**: Handle combined short flags (`-la`, `-nr`), key-value pairs (`-n 10`, `-d:`), and positional arguments in `CoreutilOptions::parse`.

### Phase 2: Feature & Flag Parity (6–12 Months)
- **Data Stream Utilities**: Expand `head`, `tail`, `uniq`, `cut`, and `tr` with full option matrix and pipe buffer support.
- **System Information Utilities**: Support `stat`, `df -h`, `du -hs`, `uname -a`, and `env` environment variables.
- **POSIX Error Codes**: Ensure strict POSIX exit code compliance (`0` for success, `1`/`2` for invalid arguments).

### Phase 3: High-Performance Safe Rust Coreutils (12–18 Months)
- **Zero-Copy Stream I/O**: Implement zero-copy buffer passing for high-throughput pipeline operations (`cat`, `tr`, `cut`).
- **SIMD & Multi-Threading**: Accelerate string filtering and sorting (`sort -n`, `grep`) using AVX2/AVX-512 SIMD vectorization.
- **Capability Sandboxing**: Restrict file system access for userland utilities via OpenBSD `pledge` and `unveil`.

---

## 3. Operational Directives for AI Agents

### 3.1 Coreutils Parity Standard
- **Pure Memory-Safe Implementations**: All userland replacements must be written in 100% pure, memory-safe Rust with zero unsafe code blocks and zero C library dependencies.
- **ANSI Color & Terminal Support**: Utilities formatting output for stdout (`ls`, `ps`, `top`) must support standard ANSI terminal color palettes and column alignments.

### 3.2 Diagnostic & Monitoring Tools
- **Process Table Diagnostics**: `ps` and `top` implementations must query kernel process control blocks (`PCB`) and compute CPU/memory usage metrics accurately.
- **POSIX Exit Code Standard**: All core utilities must return standard POSIX exit codes (`0` for success, non-zero for error conditions).

---

## 4. Related Files
- `src/userland/coreutils.rs`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
