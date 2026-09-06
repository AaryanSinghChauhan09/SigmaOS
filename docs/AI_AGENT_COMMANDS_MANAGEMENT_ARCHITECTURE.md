# AI Agent Commands & Command Line Utility Management Architecture

## Executive Overview

SigmaOS includes a zero-dependency sovereign command line utility suite implemented in `src/tools/sovereign_commands.rs`. This suite provides native Rust implementations of standard Linux, FreeBSD, and OpenBSD system commands, including privilege delegation (`sudo`, `doas`), process monitoring (`top`, `htop`), filesystem analysis (`df`, `du`), kernel logging (`dmesg`), device node management (`/dev`), compiler drivers (`gcc`, `clang`), service managers (`systemctl`), journal loggers (`journalctl`), and package manager CLI wrappers (`pacman`, `dnf`, `apt-get`, `apk`).

This document serves as the architectural reference for AI coding agents inspecting, extending, or executing sovereign system commands in SigmaOS.

---

## Subsystem Architecture & Sovereign Tool Drivers

```
                                +-----------------------------------+
                                |     AI Agent / Shell Command      |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |    src/tools/sovereign_commands   |
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | Privilege & Security  |   | Task & Disk Mon   |   | System & Distro CLI   |
            | SovereignSudo         |   | SovereignTopHtop  |   | systemctl, journalctl |
            | SovereignOpenBsdDoas  |   | SovereignDfDu     |   | pacman, dnf, apt, apk |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                |  Kernel Ring Buffer & Sysctl MIB  |
                                | SovereignDevDmesg / BsdSysctl     |
                                +-----------------------------------+
```

### Sovereign Command Modules (`src/tools/sovereign_commands.rs`)

1. **Privilege Elevation Engines**:
   - `SovereignSudo`: Implements credential caching (5-minute timestamp timeout) and root privilege execution.
   - `SovereignOpenBsdDoas`: Implements OpenBSD `doas.conf` rule evaluation (`permit keepenv :wheel`).

2. **Real-Time Task & Process Monitoring**:
   - `SovereignTopHtop`: Collects process CPU usage %, RSS memory (KB), I/O throughput (read/write bytes/sec), and CachyOS BORE interactivity scores (`bore_interactivity_score`).

3. **Filesystem Space Inspection**:
   - `SovereignDfDu`: Analyzes mount point storage space, CoW subvolumes, and directory tree sizes without spawning external binaries.

4. **Kernel Ring Buffer & Device Nodes**:
   - `SovereignDevDmesg`: Manages major/minor device node mappings (`/dev/null`, `/dev/zero`, `/dev/nvme0n1`) and formats kernel ring buffer entries.

5. **Compiler Toolchain Wrapper**:
   - `SovereignGccToolchain`: Formats optimization flags (`-O3`, `-march=native`, `-mavx512f`, `-fPIC`) for sovereign compilation pipelines.

6. **Distro Compatibility CLI Drivers**:
   - `SovereignLinuxCommandSuite`: Standardized command handlers for `systemctl`, `journalctl`, `systemd-analyze`, `pacman`, `dnf`, `apt-get`, and `apk`.
   - `SovereignBsdSysctl`: FreeBSD-style MIB hierarchy parser (`kern.ostype`, `hw.ncpu`, `security.bsd.unprivileged_proc_debug`).

---

## Zero-Allocation Guardrails

AI agents adding new commands or flags must observe these constraints:
- CLI option parsing must rely on zero-copy string slice matching (`&str`).
- Log string formatting must avoid intermediate allocations in loop iterations.
- Process metric collections must sort in-place using slice primitive sort helpers.

---

## Related Architectural References
- `src/tools/sovereign_commands.rs` - Primary command line suite implementation.
- `src/tools/sigmatools.rs` - Master tools registry.
- `docs/AI_AGENT_TOOLS_MANAGEMENT_ARCHITECTURE.md` - AI agent tool invocation architecture.
