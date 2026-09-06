# AI Agent Guidelines: Bit Tables Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Bit Tables Management**, physical memory bitmaps, security capability bitmasks, CPU register flags, hardware bitfield descriptors, and filesystem block allocation bit tables in SigmaOS.

SigmaOS maintains high-performance, `#![no_std]` zero-dependency bit table representations to achieve sub-nanosecond hardware status queries, constant-time memory allocation checks, and lock-free capability enforcement.

---

## 1. Bit Table Subsystems in SigmaOS

AI agents interacting with bit tables in SigmaOS must interface with the following architectural subsystems:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Physical Memory Bitmap (`PmmAllocator`)** | `src/memory/pmm_vmm.rs` | Page-frame allocation bitmap tracking physical memory frame states (`0` = free, `1` = allocated). |
| **Capability Bitmasks (`CapabilitySet`)** | `src/access/control.rs` | 64-bit capability mask (`capability_mask`) for ultra-fast privilege checking (`is_capability_permitted`). |
| **PCI BAR & Command Bitfields** | `src/driver/pci_enumeration.rs`, `src/driver/pci_bus.rs` | PCI configuration space 32-bit/64-bit BAR type bit representations and command status bitmasks. |
| **Audio Bit Depths & Formats** | `src/driver/audio_codec_hda.rs` | Audio sample bit depth representations (`Bits8`, `Bits16`, `Bits24`, `Bits32`). |
| **HID Mouse/Keyboard Bitmaps** | `src/driver/hid_input_device.rs` | HID button bitmaps (bit 0=left, bit 1=right, bit 2=middle) and LED bit indicators. |
| **Cgroup v2 Resource Masks** | `src/compatibility/linux_standards.rs` | Controller resource bitmasks for CPU/Memory/IO resource governor tracking. |

---

## 2. Bitwise Operations & Purity Conventions

When manipulating bit tables, AI agents must adhere to strict bitwise arithmetic standards:

### 1. Bitmask Manipulation Rules
- **Setting a Bit:** Always use bitwise OR (`table |= (1 << bit_index)`).
- **Clearing a Bit:** Always use bitwise AND with bitwise NOT (`table &= !(1 << bit_index)`).
- **Toggling a Bit:** Always use bitwise XOR (`table ^= (1 << bit_index)`).
- **Checking a Bit:** Always use bitwise AND (`(table & (1 << bit_index)) != 0`).

```rust
// Standard capability bit manipulation in SigmaOS
pub fn drop_capability(&mut self, cap_bit: u64) {
    self.capability_mask &= !(1 << cap_bit);
}

pub fn is_capability_permitted(&self, cap_bit: u64) -> bool {
    (self.capability_mask & (1 << cap_bit)) != 0
}
```

### 2. Multi-Word Bit Array Allocation
- When managing bit tables exceeding 64 bits (such as frame bitmaps for physical memory), group bits into `u64` words.
- Calculate word index via shift right (`word_idx = bit_idx >> 6`) and remainder via bitwise AND (`bit_offset = bit_idx & 63`).

```rust
// Frame bit table lookup formula
let word_idx = frame_idx / 64;
let bit_offset = frame_idx % 64;
let is_allocated = (bitmap[word_idx] & (1u64 << bit_offset)) != 0;
```

---

## 3. Hardware Register & BAR Bit Table Protocols

When interacting with PCI, AHCI, NVMe, USB xHCI, or GPU memory-mapped registers:
1. **Masking Control Bits:** Never overwrite reserved bits in hardware control registers. Read the register, apply bitmasks to target bits, and write back (`read-modify-write`).
2. **Phase & Cycle Bits:** For ring buffers (e.g., xHCI or NVMe completion queues), maintain cycle bit toggling (`cycle_bit`) to ensure atomic hardware/software synchronization without lock contention.
3. **64-bit Address Consumption:** Remember that 64-bit PCI BARs consume two adjacent 32-bit configuration space registers. Always skip the subsequent index (`bar_idx += 2`).

---

## 4. Safety & Memory Constraints

- **Zero-Dependency `#![no_std]`:** Never introduce external bit-field or bitmap crates. Use native Rust primitive integer bitwise operations (`u8`, `u16`, `u32`, `u64`, `u128`).
- **Overflow & Shift Safety:** Ensure bit shift operands are bounded to prevent panics or undefined shifts (e.g., shift amount must be `< 64` for `u64`).

---

## 5. AI Agent Checklist for Bit Tables

Before finalizing any changes to bit table management code:

- [ ] Are bit operations bounded and shift-safe across 32-bit and 64-bit architectures?
- [ ] Is `read-modify-write` applied when updating hardware bitfield registers?
- [ ] Do unit tests verify setting, clearing, toggling, and querying individual bits?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
