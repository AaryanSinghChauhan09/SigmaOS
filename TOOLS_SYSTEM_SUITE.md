# SigmaOS Tools System Suite

> Merged from branch: jules-514337451030587058-be8a6425
> Implements: register tools system suite and expose core utility types

## Overview

The Tools System Suite provides a unified interface for all system utilities in SigmaOS. Tools are registered at boot time and can be invoked from the shell, the GUI, or the kernel's administrative interface.

## Tool Categories

### System Tools (src/tools/sys_tools.rs)
- `sigma-df`: Disk free space (no coreutils dependency)
- `sigma-du`: Disk usage (no coreutils dependency)
- `sigma-top`: Process monitor (no procps dependency)
- `sigma-ps`: Process list
- `sigma-kill`: Signal delivery

### Archive Tools (src/tools/archive.rs)
- `sigma-tar`: TAR/TGZ extraction (pure Rust)
- `sigma-zip`: ZIP/ZSTD compression
- `sigma-xz`: XZ compression/decompression

### Text Processing (src/tools/textproc.rs)
- `sigma-grep`: Pattern search (no grep binary needed)
- `sigma-sed`: Stream editor
- `sigma-awk`: Field processor
- `sigma-wc`: Word/line counter

### Package Management (src/sigpkg/)
- `sigpkg install <pkg>`: Install from SigmaOS registry
- `sigpkg remove <pkg>`: Uninstall
- `sigpkg search <query>`: Search package database
- `sigpkg audit`: Security audit of installed packages

### Shell (src/tools/shell.rs, src/shell/repl.rs)
- sigma-sh: Built-in POSIX-compatible shell
- Tab completion, command history, job control
- No bash/dash dependency

## Tool Registration API

Tools are registered at compile time using a declarative macro:

```rust
// src/tools/mod.rs
register_sigma_tool! {
    name: "sigma-grep",
    description: "Search files for patterns",
    binary: tools::textproc::SigmaGrep,
    capabilities: [FileRead],
}
```

## Core Utility Types

The tools system exposes core types used across all utilities:

```rust
// src/tools/sigma_core_utils.rs
pub struct ExitCode(pub i32);
pub struct ByteSize(pub u64);
pub struct PathBuf { /* pure Rust, no std::path */ }
pub struct FileDescriptor(pub i32);
```

## OliveTin Integration (src/tools/olivetin.rs)

SigmaOS integrates OliveTin-style web action runners, allowing system tasks to be triggered via a simple web UI without shell access - useful for servers and kiosks.

## PowerToys Equivalent (src/tools/powertoys.rs)

Inspired by Windows PowerToys, SigmaOS PowerTools provides:
- Screen ruler
- Color picker
- Keyboard shortcut guide
- File renamer with regex
- Image resizer
- Spotlight-like app launcher

## Feature Flags (src/tools/feature_flags/)

All tools support feature-flag gating to allow minimal installs:

```toml
# sigma-core.toml
[features]
minimal = ["sigma-sh", "sigma-ls", "sigma-cat"]
desktop = ["minimal", "zenith-desktop", "sigma-powertools"]
server = ["minimal", "sigma-net", "sigma-sshd"]
```
