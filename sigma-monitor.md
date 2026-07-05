# sigma-monitor — Process & Resource Monitor Specification

**Status:** Draft · Target: v0.1
**Owner:** userland/monitor team
**Canonical source:** `userland/sigma-monitor/`

---

## Overview

sigma-monitor is a real-time system monitor for SigmaOS, providing CPU, memory, network I/O, and shard health visibility in a terminal UI (TUI) with an optional lightweight GUI mode rendered via Zenith Desktop.

## Goals

- < 0.5 % CPU overhead at default 1-second refresh rate

- No external library dependencies beyond libc-sigma in TUI mode

- Show shard health alongside traditional process metrics

- Sortable, filterable process list with instant signal dispatch

---

## Data Sources

| Metric | Source |
|--------|--------|
| Per-process CPU | `/proc/<pid>/stat` (or procfs-equivalent SigmaOS VFS) |
| Per-process memory | `/proc/<pid>/status` (VmRSS, VmSwap) |
| System-wide CPU | `/proc/stat` cumulative jiffies |
| Memory totals | `/proc/meminfo` |
| Network I/O | `/proc/net/dev` (bytes/packets per NIC) |
| Cgroup CPU quota | `/sys/fs/cgroup/<pid>/cpu.stat` |
| Cgroup memory | `/sys/fs/cgroup/<pid>/memory.current` |
| Shard health | `sigma-bus://monitor/shards` IPC endpoint (heartbeat reply) |
| Disk I/O | `/proc/diskstats` |

---

## Display Layout (TUI)

```
┌─ sigma-monitor ─ uptime: 3d 12h ── SigmaOS v0.2 ────────────────┐
│ CPU  ████████████░░░░░░░  62 %   Cores: 8   Freq: 3.6 GHz        │
│ MEM  ████████░░░░░░░░░░░  41 %   3.3 GB / 8.0 GB  Swap: 0 MB     │
│ NET  ↑ 1.2 MB/s  ↓ 8.4 MB/s    eth0                               │
│ DISK ↑ 40 MB/s   ↓ 12 MB/s     nvme0                              │
│ SHARDS  15/15 healthy  ░ S034-AI: idle  ░ S007-Net: active        │
├───────────────────────────────────────────────────────────────────┤
│  PID    NAME          CPU%  MEM%   MEM(MB)  STATUS   SHARD        │
│  1      sigma-init    0.0   0.1    8        S (sleep) —           │
│  412    zenith-comp   4.2   6.3    504      R (run)  S03-ZenithUI │
│  507    sigma-ai      8.1   12.4   992      S (wait) S034-AI      │
│  ...                                                               │
├───────────────────────────────────────────────────────────────────┤
│ [q]uit [k]ill [r]enice [s]ort [f]ilter [h]elp  Refresh: 1s       │
└───────────────────────────────────────────────────────────────────┘
```

---

## Refresh Rate

- Default: 1000 ms (1 Hz)

- Configurable: `sigma-monitor --interval <ms>` (min 100 ms, max 60 000 ms)

- High-frequency mode (`--fast`): 250 ms, increases CPU overhead warning shown

- Pause / resume: `Space` key

---

## Sorting and Filtering

**Sort keys** (toggle with `s` then column letter):
`c` = CPU%, `m` = MEM%, `p` = PID, `n` = name, `t` = runtime, `s` = status

**Filters** (press `f`):

- By name substring: `/nginx`

- By shard: `@S034`

- By user: `u:root`

- By state: `state:R` (running), `state:S` (sleeping), `state:Z` (zombie)

---

## Kill / Signal Dispatch

- Highlight process → `k` → choose signal from list (SIGTERM default, SIGKILL, SIGHUP, SIGUSR1/2)

- Batch kill: `F` → mark multiple → `k`

- Requires `sigma_pledge("proc signal")` granted to sigma-monitor at launch

---

## Color Coding

| Level | CPU% | Color |
|-------|------|-------|
| Normal | 0–40 | Green |
| Warning | 40–80 | Yellow |
| Critical | 80–100 | Red |

Memory and disk I/O bars follow same scale. Zombie processes shown in magenta. Shard unhealthy state shown in red with `!` prefix.

---

## TUI vs GUI Mode

- **TUI** (default): ANSI escape sequences, runs in any terminal emulator

- **GUI mode** (`--gui`): Renders via Zenith Desktop window using the sigma-monitor widget library; shows sparkline graphs for CPU/memory history (last 60 s); requires Zenith DE running

---

## Implementation Plan

- [ ] 1. procfs reader abstraction (`src/procfs.c`)

- [ ] 2. cgroup stats reader (`src/cgroup.c`)

- [ ] 3. sigma-bus shard health poller (`src/shard_health.c`)

- [ ] 4. TUI renderer (ANSI, no external TUI lib)

- [ ] 5. Sort engine (qsort with compare-fn dispatch)

- [ ] 6. Filter parser

- [ ] 7. Signal dispatch (`kill(pid, sig)` with pledge check)

- [ ] 8. Refresh timer loop (SIGALRM or `timerfd`)

- [ ] 9. GUI mode (Zenith widget integration)

- [ ] 10. Tests: data parsing, sort, filter, color logic

---

## Status

| Feature | State |
|---------|-------|
| procfs reader | ⬜ Not started |
| TUI renderer | ⬜ Not started |
| Shard health | ⬜ Not started |
| Sort / filter | ⬜ Not started |
| GUI mode | ⬜ Not started |
