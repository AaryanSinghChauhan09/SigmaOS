# Virtualization & Containers Architecture

The **SigmaOS Virtualization and Container Subsystem** provides multi-tiered workload isolation, ranging from microsecond-spawn WebAssembly sandboxes and OS-level FreeBSD Jails/Linux containers up to hardware-accelerated KVM virtual machines.

---

## 1. Subsystem Architecture

### 1.1 Hardware-Assisted KVM Virtualization (VMCS)
- **Location**: `src/virtualization/kvm_vmcs.rs`
- **Specification**: Intel VT-x and AMD-V hardware virtualization engine. Manages Virtual Machine Control Structures (VMCS), VMXON/VMXOFF state transitions, Extended Page Tables (EPT) two-dimensional paging, and VM-exit dispatching (CPUID, MMIO, I/O ports, interrupts).

### 1.2 Firecracker MicroVM Hypervisor
- **Location**: `src/virtualization/firecracker.rs`
- **Specification**: Minimalist microVM hypervisor designed for serverless execution and rapid instantiation (<5ms boot). Strips away legacy PC devices, offering lean VirtIO MMIO peripherals (virtio-net, virtio-block) and minimal ACPI power tables.

### 1.3 FreeBSD Jails Compatibility Bridge
- **Location**: `src/compatibility/freebsd_jails_impl.rs`
- **Specification**: FreeBSD Jails isolation framework. Enforces hierarchical jail IDs (JID), isolated root directories (`chroot` jail boundaries), IP address bindings, and capability permission masks (`allow.raw_sockets`, `allow.sysvipc`).

### 1.4 Linux Containers (Namespaces & Cgroups v2)
- **Location**: `src/container/`, `src/isolation/`
- **Namespaces**: Seven isolation dimensions (PID, Mount, Network, IPC, UTS, User, Cgroup).
- **Control Groups v2**: Unified hierarchy managing CPU bandwidth limits, memory hard/soft caps, swap throttling, and block I/O weight scheduling.

### 1.5 WebAssembly (WASI) Component Sandbox
- **Location**: `src/wasm/`, `src/sandbox/`
- **Specification**: Zero-overhead WASI 0.2 Component Model engine. Runs sandboxed WebAssembly modules with capability-gated access to filesystem descriptors and network sockets, delivering memory safety without VM overhead.

---

## 2. Virtualization & Container Parity Matrix

| Technology | Linux | FreeBSD | OpenBSD | SigmaOS Native Engine |
|---|---|---|---|---|
| **Hardware Hypervisor** | KVM | bhyve | vmm | In-kernel `kvm_vmcs.rs` |
| **MicroVM Engine** | Firecracker / Cloud-Hypervisor | None | None | Native `firecracker.rs` |
| **OS-Level Jails** | None (uses namespaces) | Jails | None | Built-in `freebsd_jails_impl.rs` |
| **Namespaces & Cgroups** | Namespaces + Cgroups v2 | None | None | Unified Container Engine |
| **WASM Runtime** | External (Wasmtime) | External | None | In-tree `src/wasm/` |

---

## ## AI Agent Maintenance Instructions for Future Releases

1. **Language Invariant**: The hypervisor, microVM engine, container runtime, and jail subsystems must be implemented in **100% Rust** or **Zig**.
2. **VM Escape Prevention**: The hypervisor VM-exit handler MUST strictly validate all guest register access, MMIO addresses, and I/O port numbers to eliminate escape vectors.
3. **Resource Leak Prevention**: When tearing down a microVM, container, or jail, all allocated memory frames, file descriptors, and network bridges MUST be reclaimed deterministically.
4. **Zero Predefined Dependencies**: Do not depend on external libvirt, QEMU, or C container runtimes.
5. **Compilation Verification**: All virtualization code must compile with:
   ```bash
   cargo check --lib
   ```
