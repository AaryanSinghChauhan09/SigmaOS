# 🛡️ SigmaOS Shard & Domain Isolation (Qubes OS Parity) Roadmap

This document outlines the architectural strategy and implementation specification for the **SigmaQubes Compartmentalised Domain Isolation Manager** inside **SigmaOS**, defeating the traditional heavy-virtualization boundaries of Qubes Linux.

---

## 🗺️ 1. Paradigm Vision: SigmaOS vs. Qubes OS

Qubes OS provides excellent security through compartmentalization, but it relies on a heavy hypervisor (Xen) and runs full monolithic Linux kernels inside every single AppVM, NetVM, and USBVM. This introduces:
- **Massive Performance Overhead**: Spawning a single VM takes several seconds.
- **Inflated Disk & Memory Footprint**: Running dozens of VMs consumes gigabytes of memory.
- **Broad Attack Surface**: Each VM runs a full Linux kernel containing millions of lines of code.

**SigmaOS** supersedes Qubes OS by utilizing **Native Microkernel Compartmentalization**:

```text
  +-----------------------------------------------------------------------+
  |                             SigmaOS Kernel                            |
  |                                                                       |
  |   +------------------+   +------------------+   +-----------------+   |
  |   |    AdminDomain   |   |     NetDomain    |   |    AppDomain    |   |
  |   |   (sys-admin)    |   |    (sys-net)     |   |     (work)      |   |
  |   +------------------+   +------------------+   +-----------------+   |
  |            ^                      ^                      ^            |
  +------------|----------------------|----------------------|------------+
               |                      |                      |
               +============== IPC ===+======================+ (Capability Gated)
```

* **Micro-shards instead of heavy VMs**: Isolated security domains run natively as isolated microkernel shards/processes, reducing startup latency from seconds to **under 1 microsecond**.
* **Zero-Trust Capability-Gated IPC**: Communication between domains is handled via capability-gated microkernel IPC (replacing Qubes' legacy Qrexec protocol).
* **Minimal Footprint**: Compartments share read-only memory segments dynamically, keeping memory overhead to less than 1 MB per compartment.

---

## 🏗️ 2. Architectural Components

### 2.1 Domain Class Hierarchy
SigmaQubes orchestrates the following security domains:
* **AdminDomain (`sys-admin`)**: Holds sovereign master tokens and monitors system-wide orchestration.
* **NetDomain (`sys-net`)**: Interacts directly with physical network controllers. Holds exclusive capabilities to write packets.
* **StorageDomain (`sys-storage`)**: Exclusive access to the raw block-device driver. Serves read/write sector requests to other domains via encrypted channels.
* **AppDomain (`work`, `personal`)**: Standard work domains with no raw hardware access capabilities.
* **DisposableDomain (`disp-browser`)**: Transient domains spawned to run risky, untrusted tasks (e.g. rendering a PDF or opening a web link) and instantly self-destruct upon completion.

---

## 🛡️ 3. Security, IPC & Policy Routing

### 3.1 Capability-Gated Policies
Security policies are declared in simple declarative files. The `DomainOrchestrator` verifies sender capability tokens before routing any inter-domain IPC:
* An `AppDomain` is blocked from sending direct commands to `NetDomain` unless it carries an active network transit capability token.
* If a domain gets compromised, the attacker is completely sandboxed within that user-space micro-shard with zero access to the base system or other domains.

---

## 📅 4. 12-Month Execution Roadmap

* **Month 1-3**: Implement basic `IsolatedDomain` model and `DomainOrchestrator` core structure.
* **Month 4-6**: Connect capability-gated IPC pathways to block storage, networking, and USB hubs.
* **Month 7-9**: Develop disposable micro-shard auto-cleanup pipelines.
* **Month 10-12**: Integrate Zenith Desktop isolated visual templates (drawing colored borders around windows to clearly mark their active security domains).
