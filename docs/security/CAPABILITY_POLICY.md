# SigmaOS Capability Policy

## Overview
SigmaOS implements capability-based security. Rather than granting monolithic root privileges, processes possess explicit, granular capability bits modeled after POSIX capabilities and Capsicum rights.

---

## Granular Capability Hierarchy

| Capability | Description | Permitted Subsystems |
|---|---|---|
| `CAP_SYS_ADMIN` | Reserved for core init and mount management | Init (PID 1), Mount daemon |
| `CAP_NET_BIND_SERVICE` | Bind to privileged network ports (< 1024) | NetworkManager, Web servers |
| `CAP_NET_RAW` | Transmit raw Ethernet and IP frames | Packet diagnostics, Ping |
| `CAP_SYS_RAWIO` | Direct I/O port and MMIO register access | Serial driver, PCI bus controller |
| `CAP_IPC_LOCK` | Lock pages into physical RAM (mlock) | Cryptographic vault, Real-time audio |
| `CAP_SYS_PTRACE` | Trace system calls and debug arbitrary tasks | Process debugger (`kdb`, `gdb`) |
| `CAP_DAC_OVERRIDE` | Bypass standard file read/write permissions | Backup service, Package installer |

---

## Capability Enforcement Rules
1. **Inheritance Restriction**: Capabilities are dropped across `execve` unless explicitly flagged as inheritable in the binary metadata.
2. **Ambient Disallowance**: Ambient capability sets default to empty.
3. **Bounding Sets**: Subprocesses cannot acquire capabilities outside their configured bounding set.
