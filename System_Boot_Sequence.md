# 🚀 System Boot Sequence: Zero-Dependency Initialization

In a standard operating system, the boot process is a slow, convoluted chain of hand-offs (UEFI → GRUB → `initramfs` → Linux Kernel → `systemd` → Display Manager).

SigmaOS violently rejects this bloat. The boot process is a sheer, unabstracted drop from hardware execution directly into the C11 Ring-0 kernel. This guide outlines exactly how the machine wakes up.

---

## 🕒 Stage 1: Hardware Handshake (`SigmaCore.asm`)

### Time taken: &lt; 0.1s

<<<<<<< HEAD
The moment power is mapped to the CPU, SigmaOS ignores legacy BIOS calls and hooks directly into the UEFI physical payload.
=======
The moment power is mapped to the CPU, SigmaOS ignores legacy BIOS calls and hooks directly into the UEFI physical payload. 
>>>>>>> 99f2ef5 (chore: precise lint eradication via AST script algorithms)

- `SigmaCore.asm` establishes the fundamental **Interrupt Descriptor Table (IDT)** and the Global Descriptor Table (GDT).
- It switches the CPU into absolute **64-bit Long Mode**.
- It bypasses standard RAM sweeps, relying instead on pre-calculated amnesic page validations.
- Once the hardware stack is aligned, it immediately invokes `_start_kernel_c`.

## ⚙️ Stage 2: C11 Kernel Handover (`main.c`)

### Time taken: &lt; 0.05s

The Assembly stub yields execution directly to `kernel/main.c`. Here, the OS asserts its sovereign dominance.

- **Physical Memory Manager (PMM)** and **Slab Allocator** initialize, creating rigid object bounds in RAM. No `malloc` is used.
- The **`SovereignProcessManager.c`** wakes up, establishing the PID 1 state (known internally as the *Sigma Orchestrator*).
- The **Keyboard Master** (`keyboard_master.c`) binds raw hardware keystrokes (like the `Alt+C` Omni Shell shortcut) directly to the CPU interrupt vector.

## 🕵️ Stage 3: VFS and Persona Detection

### Time taken: &lt; 0.02s

Before any graphical layer is attempted, the OS needs to know *who* or *what* it is running as.

- The native **Virtual File System (VFS)** mounts, reading physical disk snapshots via the `sigma_fnv1a` hashing bounds.
- The `SovereignPersonalizerZenith.c` checks the default config matrix to identify the active context (e.g., Developer, Gamer, Forensic Analyst).
- Kernel CPU governors are instantly adjusted. If the Gamer persona is detected, TSX and clock multipliers are maximized.

## 🌉 Stage 4: Shard Loader and Network DMA Binding

### Time taken: &lt; 0.05s

SigmaOS does not load a massive library of drivers. It uses the **Shard-On-Demand (SOD)** framework.

- `SovereignAetherShardLoader.asm` loads only the explicitly permitted `.c` Shards configured for the active persona.
- **Network Initializer**: Instead of building standard BSD sockets, `SovereignNetMesh.c` locks exclusive DMA ring-buffers directly into the Network Interface Card (NIC) for zero-copy packet interception.

## 🌟 Stage 5: Zenith-Gold JS Orchestrator Matrix

### Time taken: &lt; 0.1s

Once the C11/Assembly sub-system is absolute, SigmaOS initiates its visual representation. It does not spawn heavy window compositors like X11 or Wayland.

- A fractional native Javascript engine reads `index.js` and `index.css`.
- The UI floats into existence, rendering the **Zenith-Gold UX**, mapping DOM elements directly to raw C11 execution hooks beneath.
- The Omni Shell prompts the user, ready for interaction without any intermediate abstraction layers.

> **Total Boot Latency:** Under a fraction of a second from power-on to absolute sovereign control.
