# SigmaOS Redirection Subsystem: Development Blueprint & Action Plan

## Executive Summary

Redirection mechanisms in **SigmaOS** span 4 distinct operating system layers:
1. **Userland Shell I/O Redirection**: Standard streams (`stdin`, `stdout`, `stderr`) redirection (`>`, `>>`, `<`, `2>&1`, `|`) via file descriptor duplication (`dup2`/`dup3`).
2. **Network Packet & Socket Redirection**: Fast-path eBPF `XDP_REDIRECT` / `bpf_redirect` and OpenBSD PF transparent NAT/proxy port redirection (`rdr-to`).
3. **Kernel Livepatch & Function Redirection**: Rebootless livepatching via Ftrace-style instruction patching (`redirect_call`).
4. **Hardware Interrupt Redirection**: Dynamic IO-APIC IRQ vector redirection across CPU cores (`irqbalance`).

This document provides a gap analysis comparing SigmaOS redirection features against Linux v6.8+ and OpenBSD/FreeBSD standards, followed by an actionable 3-phase development blueprint.

---

## 1. Existing Redirection Capabilities in SigmaOS

| Layer | Implementation Module | Capabilities Provided |
| :--- | :--- | :--- |
| **Shell Stream I/O** | `src/userland/coreutils.rs`, `src/filesystem/vfs.rs` | Standard stream duplication (`dup2`), append mode (`O_APPEND`), and pipeline piping between process descriptors. |
| **Network & Socket Redirection** | `src/kernel/sigma_ebpf_runtime.rs`, `src/distro/sovereign_2028_distro_supremacy_engine.rs` | eBPF XDP zero-copy packet redirection (`XDP_REDIRECT`), VNET network namespace routing, and OpenBSD PF `rdr-to` port forwarding. |
| **Kernel Function Redirection** | `src/distro/nextgen.rs` (`LivepatchManager`) | Ftrace-style instruction patching redirecting function execution (`redirect_call("sys_read")`) to dynamic patch routines without rebooting. |
| **Hardware IRQ Redirection** | `src/hardware/compatibility.rs`, `src/drivers/msix_engine.rs` | IO-APIC redirection table entry remapping (`balance_irq_routing`) and MSI-X vector steering. |

---

## 2. Exhaustive Gap Analysis vs. Linux & BSD Standards

```
                  ┌──────────────────────────────────────────────────────────┐
                  │              SigmaOS Redirection Subsystem               │
                  └────────────────────────────┬─────────────────────────────┘
                                               │
      ┌────────────────────────────────────────┼────────────────────────────────────────┐
      ▼                                        ▼                                        ▼
┌───────────────────────────┐    ┌───────────────────────────┐    ┌───────────────────────────┐
│ Atomic Dup3 & O_CLOEXEC   │    │ eBPF Sockmap Zero-Copy    │    │ Livepatch Ftrace Safety   │
│ GAP: POSIX dup3 missing   │    │ GAP: sk_msg socket-to-    │    │ GAP: Lacks RCU / ftrace   │
│ atomic O_CLOEXEC flag     │    │ socket direct bypass      │    │ trampoline synchronization│
└───────────────────────────┘    └───────────────────────────┘    └───────────────────────────┘
```

### 2.1. Shell Stream I/O Redirection & POSIX `dup3(2)` Atomicity
* **Linux / POSIX Baseline**: Linux `dup3(2)` allows atomic file descriptor duplication with `O_CLOEXEC` flag setting, preventing race conditions where descriptors leak into child processes during concurrent `fork`/`exec`.
* **SigmaOS Gap**: `vfs.rs` performs non-atomic descriptor duplication post-allocation.

### 2.2. eBPF `sockmap` / `sk_msg` Socket-to-Socket Zero-Copy Redirection
* **Linux Baseline**: Linux eBPF `BPF_MAP_TYPE_SOCKMAP` and `bpf_msg_redirect_map` allow TCP sockets to redirect incoming payload streams directly to target sockets in kernel space, bypassing TCP/IP stack overhead entirely.
* **SigmaOS Gap**: Packet redirection in `sigma_ebpf_runtime.rs` operates on raw L2/L3 XDP frames, lacking socket-level `sockmap` stream redirection.

### 2.3. Livepatch Ftrace Trampoline & RCU Synchronization
* **Linux Baseline**: Linux `klp_patch_object` uses ftrace trampolines (`ftrace_caller`) with Read-Copy-Update (RCU) grace period synchronization, ensuring no thread is executing inside a patched function during instruction overwriting.
* **SigmaOS Gap**: `redirect_call` in `nextgen.rs` modifies function entry points without RCU thread barrier synchronization.

---

## 3. Actionable Strategic Development Roadmap

### Phase 1: Shell Descriptor Redirection & POSIX `dup3` Atomicity (Months 1–3)
1. **Implement `sys_dup3` Syscall**:
   - Add atomic `dup3(oldfd, newfd, flags)` supporting `O_CLOEXEC` in `src/filesystem/vfs.rs` and `src/kernel/boot_foundations.rs`.
2. **Enhanced Shell Redirection Syntax (`2>&1`, `&>`)**:
   - Update `sigma_sh` shell parser to handle combined stdout/stderr redirection streams.

### Phase 2: eBPF/XDP Zero-Copy Network & Socket Redirection (Months 3–6)
1. **`BPF_MAP_TYPE_SOCKMAP` Implementation**:
   - Add socket redirection maps in `src/kernel/sigma_ebpf_runtime.rs`.
2. **Transparent Proxy Redirection (`TPROXY` / OpenBSD `rdr-to`)**:
   - Integrate PF NAT firewall rules with socket buffers for transparent proxy forwarding.

### Phase 3: Livepatch Ftrace Function Redirection & Hardware Vector Remapping (Months 6–12)
1. **RCU Barrier Livepatch Synchronization**:
   - Ensure `redirect_call` waits for active thread RCU grace periods before committing NOP patches.
2. **Dynamic IO-APIC Hardware Vector Remapping**:
   - Automatically rewrite IO-APIC redirection table entries during high-interrupt workload spikes.

---

## 4. Verification and Benchmark Plan

| Verification Task | Test Target | Success Criterion |
| :--- | :--- | :--- |
| **Atomic `dup3` Redirection** | `test_dup3_cloexec_atomic` | Atomically sets `O_CLOEXEC` on duplicated descriptor |
| **eBPF Sockmap Redirection** | `test_ebpf_sockmap_redirect` | Bypasses TCP stack for 10Gbps socket-to-socket transfer |
| **Livepatch Redirection Safety** | `test_livepatch_redirect_rcu` | Safely redirects function execution without multi-thread crashes |
