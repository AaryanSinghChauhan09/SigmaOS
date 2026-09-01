# 🧠 Kernel Development & Subsystem Modules

SigmaOS's kernel combines high-performance microkernel modules with Linux Loadable Kernel Module (LKM) and FreeBSD Kernel Linker Set (`kld`) parity.

---

## 1. Kernel Module Manager (`src/kernel/modules.rs` & `src/kernel/linux_bsd_innovations.rs`)

* **Module Lifecycle (`ModuleState`):** Manages lifecycle transitions (`Unloaded` -> `Loading` -> `Live` -> `Unloading` -> `Failed`).
* **FreeBSD Event Handlers (`ModuleEvent`):** Supports BSD `kld` event notifications (`Load`, `Unload`, `Shutdown`, `Quiesce`).
* **GPL-Only Symbol Table (`EXPORT_SYMBOL_GPL`):** Tracks exported kernel symbol addresses and enforces GPL license restrictions.
* **Kernel Livepatching (`KlpPatch`):** Applies function-level hot-patches to running kernel functions without rebooting.
* **Taint Flag Tracking (`TaintFlag`):** Records system taint conditions (proprietary drivers, forced module loads, kernel page faults).

---

## 2. Process Scheduling & Round-Robin Governor (`src/kernel/roundrobin.rs`)

* **CpuContext Register Frame:** Preserves register states (`rax`, `rbx`, `rcx`, `rdx`, `rsi`, `rdi`, `rsp`, `rip`, `rflags`).
* **Priority Time-Slices:** Assigns priority-weighted quantum time slices with CPU core affinity and interactive score boosting.

---

## 3. Asynchronous I/O Subsystem (`src/kernel/io_uring.rs`)

* **Ring Buffers:** Lockless submission queue and completion queue for microsecond file and network I/O.
* **Supported Opcodes:** `Read`, `Write`, `Fsync`, `PollAdd`, `PollRemove`, `Nop`.
