# AI Agent Development Instructions for I/O Port & MMIO Register Management (`src/driver/` & `src/hal/`)

This document outlines guidelines for low-level Memory-Mapped I/O (MMIO), x86 Port I/O (`inb`/`outb`), volatile register access, memory barriers, and hardware buffer register management in SigmaOS.

## Subsystem Architecture & Directives

1. **Volatile Memory-Mapped I/O (MMIO) Registers**
   - All MMIO hardware register reads and writes MUST use `core::ptr::read_volatile` and `core::ptr::write_volatile`. Never use plain pointer dereferences (`*ptr`) as compiler optimizations will elide or reorder hardware accesses.
   - Maintain explicit register offsets relative to the Base Address Register (`bar_base` / `mmio_base`).

2. **Read-Modify-Write Register Patterns & Bitmasking**
   - Modifying individual register control bits MUST follow the explicit Read-Modify-Write pattern:
     ```rust
     let mut val = unsafe { core::ptr::read_volatile(reg_addr as *const u32) };
     val &= !CLEAR_MASK;
     val |= SET_MASK;
     unsafe { core::ptr::write_volatile(reg_addr as *mut u32, val) };
     ```

3. **Memory Barriers & Hardware Doorbell Sequencing**
   - When updating hardware submission/completion ring pointers or doorbells (e.g. NVMe, AHCI, e1000, xHCI), execute explicit CPU memory barriers (`core::sync::atomic::fence(Ordering::SeqCst)`) before writing doorbell registers.

4. **Port I/O Access & HAL Wrappers**
   - Direct Port I/O (`inb`, `outb`, `inw`, `outw`, `inl`, `outl`) must be encapsulated within `X86HAL` / `X86_64HAL` abstraction blocks and require `CapabilityToken` verification before accessing privileged legacy hardware ports (`0x20`/`0x21` PIC, `0x40`-`0x43` PIT, `0xCF8`/`0xCFC` PCI).

5. **Verification**
   - Run `cargo check --lib` to ensure no volatile pointer warnings or invalid imports are introduced.
