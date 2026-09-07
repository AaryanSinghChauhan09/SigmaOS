# AI Agent Commands Management Guide

## Overview
This wiki guide details command line utility management protocols for AI coding agents operating on SigmaOS. It covers privilege delegation (`sudo`, `doas`), process monitoring (`top`, `htop`), disk space inspection (`df`, `du`), kernel logging (`dmesg`), BSD `sysctl` MIB tuning, and Linux package manager CLI wrappers (`pacman`, `dnf`, `apt`, `apk`).

## Subsystem Commands Suite (`src/tools/sovereign_commands.rs`)

### 1. Privilege Delegation (`SovereignSudo` & `SovereignOpenBsdDoas`)
- **Credential Caching**: `SovereignSudo` caches user authentication for 5 minutes (`300_000ms`).
- **OpenBSD doas Rules**: `SovereignOpenBsdDoas` evaluates rules like `permit keepenv :wheel` and `permit nopass`.

### 2. Real-Time Task Monitor (`SovereignTopHtop`)
```rust
let mut top = SovereignTopHtop::new();
top.update_process_metrics(ProcessTaskMetrics {
    pid: 100,
    command: String::from("cc1"),
    cpu_usage_pct: 99.0,
    memory_rss_kb: 102400,
    io_read_bytes_sec: 5000,
    io_write_bytes_sec: 12000,
    bore_interactivity_score: 10,
});
let sorted = top.get_sorted_by_cpu();
```

### 3. BSD Sysctl MIB Inspector (`SovereignBsdSysctl`)
```rust
let mut sysctl = SovereignBsdSysctl::new();
let value = sysctl.get_mib("kern.ostype"); // Option(&"SigmaOS")
```

## Related Documents
- `docs/AI_AGENT_COMMANDS_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_COMMANDS_MANAGEMENT_GUIDELINES.md`
- `docs/AI_AGENT_TOOLS_MANAGEMENT_ARCHITECTURE.md`
