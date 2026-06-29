# SigmaOS Utilities Development Roadmap

An interactive single-page roadmap (`userland/roadmap/index.html`) visualises the complete userland utilities plan — 42 utilities across 5 phases and 13 categories, Q1 2026 → Q1 2027.

Open the roadmap: `userland/roadmap/index.html` (pure HTML, no build step needed).

---

## Overview

| Stat | Value | 
| --- | --- | 
| Total utilities | 42 | 
| Critical priority | 9 | 
| Phases | 5 | 
| Categories | 13 | 
| Timeline | Q1 2026 – Q1 2027 | 

---

## Phase 1 — Core System Foundations (Q1 2026) `In Progress`

*Build the bedrock utilities that bridge the microkernel to usable user-space.*

| Utility | Priority | Category | Key Dependencies | 
| --- | --- | --- | --- | 
| Process Manager (ps, top, kill) | **Critical** | System | — | 
| File System Navigator (ls, cd, mkdir, rm) | **Critical** | File System | — | 
| Text File Viewer (cat, less, head, tail) | **Critical** | File System | — | 
| Shell / Command Interpreter (sh) | **Critical** | Shell | Process Manager, FS Navigator | 
| Text Editor (nano/vi-style) | High | Editor | — | 
| Environment Variable Manager (env, export) | High | System | — | 
| Manual Pages System (man) | Medium | Documentation | Shell | 
| Date & Time Utilities (date, cal, uptime) | Low | System | — | 

---

## Phase 2 — Developer & Power Tools (Q2 2026) `Planned`

*Equip developers with the tools needed to build and debug on SigmaOS.*

| Utility | Priority | Category | Key Dependencies | 
| --- | --- | --- | --- | 
| Text Processing (grep, sed, awk, cut, sort) | **Critical** | Text Processing | Text Viewer | 
| File Search & Locate (find, locate, which) | High | File System | FS Navigator | 
| Archive Manager (tar, gzip, zip) | High | File System | FS Navigator | 
| Network Diagnostics (ping, netstat, ifconfig) | High | Network | — | 
| Build System (make) | High | Development | Shell | 
| Hex Editor / Binary Viewer (xxd, hexdump) | Medium | Development | — | 
| Diff & Patch Tools (diff, patch) | Medium | Development | Text Viewer | 
| Disk Usage Analyzer (df, du, ncdu) | Medium | System | FS Navigator | 
| System Monitor Dashboard | Medium | System | Process Manager | 

---

## Phase 3 — Userland Applications (Q3 2026) `Planned`

*Build interactive end-user applications that make SigmaOS a daily driver.*

| Utility | Priority | Category | Key Dependencies | 
| --- | --- | --- | --- | 
| Package Manager (pkg) | **Critical** | System | Build System | 
| Terminal Multiplexer (tmux-style) | High | Terminal | Shell | 
| File Manager (mc/ranger-style) | High | File System | FS Navigator, Text Viewer | 
| Calculator (bc, dc) | High | Application | — | 
| Image Viewer (fim/sxiv-style) | Medium | Application | File Manager | 
| Music Player (cmus-style) | Medium | Application | — | 
| IRC/Chat Client (irssi-style) | Low | Network | Network Diagnostics | 
| Task / To-Do Manager | Low | Application | — | 

---

## Phase 4 — Advanced System & Security (Q4 2026) `Planned`

*Harden the OS and make it production-ready for multi-user networked environments.*

| Utility | Priority | Category | Key Dependencies | 
| --- | --- | --- | --- | 
| User & Permission Manager (useradd, chmod) | **Critical** | Security | Shell | 
| Firewall Configuration (iptables-style) | High | Security | Network Diagnostics | 
| SSH Client & Server (openssh) | High | Network | User Manager, Network | 
| Service Manager (systemd-style) | High | System | Process Manager, Shell | 
| Log Manager (journald/syslog-style) | High | System | Text Viewer | 
| Container Runtime (docker-style) | Medium | System | User Manager, Service Manager | 
| Cryptography Tools (openssl-style) | Medium | Security | — | 
| Backup & Restore Utility (rsync-style) | Medium | File System | Archive Manager | 
| Performance Profiler (perf-style) | Low | Development | Process Manager | 

---

## Phase 5 — Ecosystem & Developer Experience (Q1 2027) `Planned`

*Create a thriving ecosystem and lower the barrier to porting existing software.*

| Utility | Priority | Category | Key Dependencies | 
| --- | --- | --- | --- | 
| SDK & API Documentation | **Critical** | Documentation | — | 
| POSIX Compatibility Layer | **Critical** | Compatibility | Package Manager | 
| Graphics Subsystem (Wayland-style) | High | Graphics | User Manager | 
| Web Browser (lynx/w3m-style) | High | Network | Network Diagnostics | 
| Version Control (git-style) | High | Development | Diff & Patch | 
| Integrated IDE | Medium | Development | Text Editor, Git | 
| Emulator / VM Support (QEMU) | Medium | System | Container Runtime | 
| App Store / Repository Portal | Medium | Application | Package Manager | 

---

## Development Principles

**Shard-Native Design** — every utility is a user-space shard communicating via IPC, not a monolithic binary.

**Message-Passing First** — pipes are implemented as IPC channels between shards via `sigma-bus`.

**No Kernel Bloat** — all logic in user-space; the kernel handles scheduling, memory, and message routing only.

**POSIX Where Possible** — prioritise POSIX-compliant behaviour in the compatibility layer to ease porting.

**Documentation First** — every utility ships a man page and an IPC schema definition before release.

---

## Key Gaps to Address First

| Gap | Why Critical | 
| --- | --- | 
| IPC debugging tools (strace-equiv) | SigmaOS needs a way to trace IPC messages between shards | 
| VFS tools (mount, fsck for SigmaFS) | The user-space filesystem needs dedicated inspection tools | 
| Driver shard inspector | Query and configure hardware driver shards (network, storage, display) | 
| Message schema validator | Validate IPC messages conform to expected schemas between shards | 

---

## Interactive Roadmap

The file `userland/roadmap/index.html` is a zero-dependency single-page app:

- **Expandable phases** — click to drill into individual utilities
- **Live filtering** — search by keyword, filter by category and priority
- **Dependency tracking** — each item shows its upstream dependencies
- **Priority badges** — Critical / High / Medium / Low with colour coding
- **Status indicators** — In Progress / Planned / Completed
- **Overview dashboard** — stats cards for totals, criticals, phases, categories

No build step needed — open directly in any browser.

---

*See also: [Feature Roadmap](Feature-Roadmap) · [Improvements Overview](Improvements-Overview) · [Developer Guide](Developer-Guide) · [System Daemons](System-Daemons)*
