# AI Agent Maintenance Guidelines: Dropped & Historical Linux/BSD Distro Subsystems

This document establishes engineering guidelines, diagnostic protocols, and clean-room safe Rust implementation standards for AI agents maintaining, restoring, and developing dropped, deprecated, or legacy Linux and BSD distribution features in SigmaOS.

---

## 1. Executive Summary & Historical Context

Over decades of kernel evolution, mainstream Linux and BSD distributions have deprecated or removed numerous subsystems to reduce maintenance overhead. However, legacy software, specialized industrial equipment, retro computing applications, and hypervisor compatibility stacks still require support for these dropped components.

SigmaOS maintains **clean-room, zero-dependency safe Rust implementations** of these historical subsystems without inheriting their legacy C vulnerability vectors (such as unsafe buffer overruns, race conditions, or unverified raw memory pointers).

---

## 2. Catalog of Dropped Subsystems & Safe Rust Architecture

| Historical / Dropped Subsystem | Origin & Deprecation Reason | Legacy Vector | SigmaOS Safe Rust Replacement |
| :--- | :--- | :--- | :--- |
| **Linux devfs** | Removed in Linux 2.6 in favor of udev | Dynamic `/dev` race conditions, unmanaged inode locks | Zero-allocation `DevfsEmulationEngine` in `src/filesystem/` |
| **OSS (Open Sound System)** | Replaced by ALSA / PipeWire | Blocking `/dev/dsp` ioctls, lack of software mixing | Safe `/dev/dsp` ioctl translation wrapper to PipeWire SPA nodes |
| **a.out / COFF Binary Formats** | Removed in Linux 5.19 | Fixed 32-bit segment layouts, executable memory stack risks | `LegacyExecutableLoader` with memory isolation and NX stack flags |
| **cgroups v1** | Deprecated in favor of cgroups v2 unified hierarchy | Lock contention across dual-hierarchy resource trees | Unified resource controller with cgroups v1 translation mapping |
| **Token Ring & FDDI Networks** | Removed in Linux 5.15 | Obsolete hardware drivers, unmaintained packet structs | Zero-allocation link-layer packet translator in `src/net/` |
| **FreeBSD UnionFS** | Replaced by OverlayFS / NullFS | File handle deadlock under concurrent CoW mutations | Deadlock-free `BtrfsCoWOverlay` with atomic locks |
| **SunOS / Solaris Doors IPC** | Deprecated after OpenSolaris | Direct kernel-thread handoff vulnerabilities | `DoorIpcEngine` using zero-copy lock-free ring channels |
| **Solaris / Illumos STREAMS IPC** | Removed in modern BSDs | Unbounded MBLK queue memory consumption | Ring-buffer backed `StreamsIpcTranslator` |
| **SysV IPC (shm, sem, msg)** | Replaced by POSIX shared memory / futexes | Global key space collision, uncleaned IPCS state | Safe `SysvIpcManager` with automatic process lifecycle tracking |
| **NetBSD Rump Drivers** | Anykernel driver isolation | FFI boundary memory overhead | Sandboxed safe Rust driver wrappers in `src/drivers/` |

---

## 3. Maintenance & Development Guidelines for AI Agents

### 3.1 Zero-Allocation Mandate
* AI agents MUST NOT use heap allocations (`alloc::vec::Vec`, `alloc::string::String`) inside high-frequency legacy ioctl or packet translation loops.
* Use fixed-size stack buffers (`[u8; 256]`, `heapless::Vec`) or zero-copy page slicing (`ZeroCopyBuffer`).

### 3.2 Thread & Memory Safety
* Prevent race conditions in legacy devfs and SysV IPC structures by utilizing `TicketSpinlock` or atomic atomic pointer swaps (`AtomicUsize`, `AtomicBool`).
* Always enforce Non-Executable (NX) stack and PAGE_NO_EXECUTE flags when loading legacy `a.out` or COFF binary headers.

### 3.3 Diagnostic & Verification Protocol
1. **Compilation Check:** Run `cargo check --lib` or standalone `rustc --test` on modified compatibility modules.
2. **Diagnostic Test Execution:** Run `./run_sigma_tests.sh` to confirm zero regressions across security input validation and launch readiness suites.
3. **Documentation Parity:** Ensure any additions to historical subsystem bridges are documented in `WHAT_IS_WORKING_AND_NOT_WORKING.md` and `docs/LINUX_BSD_DISTRO_COMPONENTS_AND_GUIDELINES.md`.

---

## 4. Architectural Blueprints

### 4.1 OSS `/dev/dsp` Compatibility Wrapper
```rust
pub struct OssAudioIoctlBridge {
    pub sample_rate: u32,
    pub channels: u8,
    pub format_bits: u8,
    pub pipewire_node_id: u32,
}

impl OssAudioIoctlBridge {
    pub fn handle_dsp_ioctl(&mut self, cmd: u32, arg: usize) -> Result<i32, &'static str> {
        match cmd {
            0x40045002 => { // SNDCTL_DSP_SPEED
                self.sample_rate = arg as u32;
                Ok(0)
            }
            0x40045006 => { // SNDCTL_DSP_CHANNELS
                self.channels = arg as u8;
                Ok(0)
            }
            _ => Err("OSS: Unsupported DSP ioctl command"),
        }
    }
}
```

---

*Document Version:* 1.0.0
*Maintained By:* SigmaOS AI Agent Architecture Board
