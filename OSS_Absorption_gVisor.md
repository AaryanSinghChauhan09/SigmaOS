# OSS Absorption: gVisor & Firejail — Syscall Interception Sandboxing

> **Status**: 🔄 Active | **Source Projects**: gVisor (Google), Firejail | **Target Shard**: `SigmaOS Application Sandbox`

---

## 1. Executive Summary

While `sigma-sandbox` (based on Bubblewrap/Namespaces) and `sigma-vm` (based on KVM/Firecracker) provide excellent isolation, some untrusted applications require a middle-ground: they need to think they are running on a full Linux kernel, but we cannot trust them to make direct kernel syscalls.

SigmaOS absorbs the **Syscall Proxying** model of gVisor (Google's container sandbox) to provide an application-level kernel that intercepts and safely handles all system calls before they ever reach the real SigmaOS kernel.

---

## 2. Key Features Absorbed

### 2.1 The Userspace Kernel (`sigma-sentry`)

When an untrusted binary makes a system call (e.g., `sys_open` or `sys_socket`), the ptrace/KVM trap redirects it to `sigma-sentry`. The sentry is a memory-safe userspace application that emulates the Linux kernel ABI.

```bash
# Run an untrusted binary through the sentry proxy
$ sigma run --sandbox=sentry ./untrusted_malware
Σ [SENTRY] Syscall interception active.
  [untrusted_malware] sys_open("/etc/shadow") → SENTRY INTERCEPT
  [SENTRY] Emulating response: EACCES (Permission denied)
  [untrusted_malware] sys_socket(AF_INET, SOCK_RAW) → SENTRY INTERCEPT
  [SENTRY] Emulating response: EPERM (Operation not permitted)
```

Because the untrusted code never talks to the real kernel, a kernel privilege escalation exploit (like a buffer overflow in the network stack) is completely neutralized. The exploit only compromises the unprivileged userspace sentry.

### 2.2 Firejail-Style Profiles

For desktop applications that don't need the heavy overhead of syscall proxying but still need strict filesystem isolation, SigmaOS utilizes a profile system inspired by Firejail.

```bash
# Apply a pre-built isolation profile to a legacy app
$ sigma run --profile=strict-network-only ./download_manager
```

---

## 3. Architecture

```
┌────────────────────────────────────────────────────────────────┐
│               SIGMA-SENTRY (gVisor-inspired)                   │
│                                                                │
│  ┌──────────────────────────┐                                  │
│  │ Untrusted Linux Binary   │                                  │
│  │ (e.g., malicious.elf)    │                                  │
│  └────────────┬─────────────┘                                  │
│               │ SYSCALL (INT 0x80 / SYSCALL)                   │
│  ┌────────────▼─────────────┐                                  │
│  │ KVM / ptrace trap        │                                  │
│  └────────────┬─────────────┘                                  │
│               │ Redirected                                     │
│  ┌────────────▼─────────────────────────────────────────────┐  │
│  │ sigma-sentry (Userspace "Kernel" written in Rust)        │  │
│  │ - VFS Emulation                                          │  │
│  │ - Network Stack Emulation (netstack)                     │  │
│  │ - Memory Management                                      │  │
│  └────────────┬─────────────────────────────────────────────┘  │
│               │ Safe, validated, limited host syscalls         │
│  ┌────────────▼─────────────────────────────────────────────┐  │
│  │ SIGMA MICROKERNEL                                        │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────┘
```

---

## 4. References & Standards

- gVisor — `gvisor.dev` (Apache-2.0)
- Firejail — `firejail.wordpress.com` (GPL-2.0)
