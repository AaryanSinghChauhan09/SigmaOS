# SigmaOS: Novice Guide and Features Status

## 1. Novice Guide: Performing Tasks on SigmaOS

Welcome to SigmaOS! As a zero-dependency, bare-metal C11 and Assembly based operating system, standard operations differ significantly from traditional Linux or Windows environments. Here is how a novice can get started and navigate the system:

### 1.1 Command Execution and Shards

SigmaOS replaces traditional pre-installed "bloat" applications with **Shards**. Shards are pure C11/ASM highly optimized, silicon-direct executables.

- **Executing Tasks:** Instead of using traditional monolithic binaries, you call the exact Shard required for your task. For network operations, you use the `NetShard`; for filesystem interactions, you use the native `FSShard`.
- **Shard-On-Demand (SOD):** You don't need to manually close background services. Invoking a task spins up its Shard via the SOD system. Once execution is complete, the Shard dissolves from kernel memory automatically.

### 1.2 System File Management

- **No Virtual File System (VFS) Overhead:** File actions happen via raw `SovereignDiskZenith` and `fs` module commands.
- **Basic Commands:** To copy or move files, use the raw CLI mapped to `SovereignCoreUtils`.
Example: Use `sigma-fs --read [PATH]` to view a file, and `sigma-fs --write [PATH]` to create files natively.

### 1.3 Navigating Process Management

- **SovereignPulse:** Monitor system operations and active shards by executing `sigma-pulse`. This directly queries the bare-metal kernel (`kernel_pulse`) for thread information.
- **SovereignSentinel:** If a process locks up, the `SovereignSentinel` subsystem automatically catches kernel panics. You can manually terminate rogue threads via `sigma-kill [THREAD_ID]`.

### 1.4 Creating Custom Tools

As a sovereign OS, you have total autonomy to build custom tools without needing third-party libraries.

- All tools must be written in **pure C11 or x86/ARM Assembly**, making direct syscalls to the kernel.
- Use the included `.asm` templates (like `SovereignEntry.asm`) and compile via the native build system (`make` and the custom compiler orchestration scripts).
- No Python, Node.js, or external standard libraries (`libc` wrappers) are required or supported by default to execute logic.

---

## 2. Features and Tools Status Tracker

Below is the current status of all major features, components, and tools natively orchestrated by the SigmaOS kernel:

| Feature / Tool Name | Category | Status in Wiki | Description |
| :--- | :--- | :--- | :--- |
| **Sovereign Kernel Zenith** | Core OS | Stable | The foundational bare-metal C11/ASM kernel devoid of any external library dependencies. Directly interfaces with hardware. |
| **Shard-On-Demand (SOD)** | Architecture | Active Development | The proprietary resource-allocation unit that spins up native execution shards on the fly, eliminating background bloat. |
| **SovereignPulse** | Process Management | Stable | Real-time thread and silicon pulse tracker mapped exactly to kernel thread tables via natively compiled C11 pointers. |
| **SovereignSentinel** | Security / Stability | Stable | Bare-metal panic catching and interference guarding mechanism, neutralizing potential exploits. |
| **Native AI/LLM Shards** | Ecosystem / Shards | Alpha / Refactoring | Execution shards built directly over bare metal to render complex Deep Learning matrices without Python/PyTorch wrappers. |
| **Sovereign XV6 Bridge** | Compatibility | Stable | Compatibility translation layer for XV6-like system calls into modern bare-metal native operations. |
| **Persona-Aware Automation** | UI / Workflow | Stable | Automatically routes resources, UI styling, and firewall logic based on user personas (dev, gamer, forensic, student). |
| **Sigma Automation Matrix** | Automations | Stable | Replaces cron/systemd with a unified native AI workflow scheduler. Evaluated smoothly at ring-0 without context-switching. |
| **Sovereign Chrono-Vault** | File Systems | Stable | Zero-dependency microsecond block-level snapshots bypassing massive ZFS/APFS logic routines. |
| **Omni-Search Engine** | Search Utilities | Stable | Bypasses background indexing daemons using native SIMD matchers to answer math, logic, or file queries natively. |
| **Sovereign DistroForge** | Command Compatibility | Active Development | Polyfilling standard Linux commands (`ls`, `grep`, `cat`, etc.) as raw C11 execution targets to maintain command parity natively. |
| **ZeroLib & SovereignLibC** | Libraries | Stable | 100% rewritten subset of essential memory operations and string math, enforcing the zero-dependency paradigm. |

> Note: All documentation and features in this document are automatically kept in repository synchronization. Developers should continually push updates to the `WIKI/` directory as new shards are merged into `master`.
