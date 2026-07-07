# SigmaOS MicroVM Sandbox Specification

## 1. Concept
SigmaOS shifts away from traditional Linux namespaces/cgroups (like Docker) in favor of lightweight, hardware-assisted virtualization for untrusted workloads and developer environments. By embedding KVM/VMM style hypervisor primitives directly into the core OS, we achieve Firecracker/gVisor levels of isolation natively.

## 2. Architecture (`kernel/security/sigma_sandbox.rs`)
- **Memory Isolation:** Each MicroVM runs in its own guest physical address space (`root_cr3`), preventing page table manipulation attacks against the host.
- **CPU Quotas:** Enforced by the native Round-Robin scheduler (`sigma_rr_sched.rs`), guaranteeing precise CPU cycle allocation.
- **Syscall Filtering:** Unlike BPF-based seccomp filters, the MicroVM traps on a strictly allow-listed set of system calls. Unlisted syscalls result in immediate VM termination.

## 3. Network Policies
Each MicroVM is bound to a virtual network namespace. 
- **Default Posture:** Complete network isolation (air-gapped).
- **Explicit Grants:** Applications must request specific outbound port/IP combinations via a declarative manifesto. SigmaOS generates ephemeral WireGuard tunnels or NAT rules to facilitate this.

## 4. Developer Sandboxes (Ephemeral Dev)
Developers can spawn instantaneous, ephemeral VMs containing complete language toolchains (Rust, C++, Python). 
- When the sandbox is destroyed, the diff layer is completely purged.
- Ensures developer workstations do not suffer from "toolchain rot" or dependency conflicts.
