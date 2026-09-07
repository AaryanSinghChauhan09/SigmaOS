# SigmaOS Virtual Memory Hardening Guide

## Overview

SigmaOS implements W^X (Write XOR Execute), wired page protection from FreeBSD, and OpenBSD-style memory randomization as described in the merged branch `jules-880081283500171861`.

## W^X Enforcement (OpenBSD-inspired)

The W^X policy ensures no memory page is simultaneously writable and executable.

### Implementation (src/mm/virtual_memory.rs)

```rust
pub enum PageFlags {
    Read    = 0b001,
    Write   = 0b010,
    Execute = 0b100,
}

impl VirtualMemoryManager {
    /// Enforce W^X: a page cannot be both Write and Execute
    pub fn set_page_flags(&mut self, vaddr: usize, flags: PageFlags) -> Result<(), VmmError> {
        if flags.contains_both_write_and_exec() {
            return Err(VmmError::WxViolation);
        }
        // ... apply flags to page table entry
    }
}
```

## FreeBSD Wired Page Protection

Wired pages are pinned in physical memory and protected from swapping or modification by unprivileged code:

- Kernel code sections: wired + read-only + executable
- Kernel data sections: wired + read-write (non-executable)
- Interrupt handlers: wired + read-only + executable

## ASLR Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `stack_aslr` | enabled | Randomize stack base |
| `mmap_aslr` | enabled | Randomize mmap allocations |
| `exec_aslr` | enabled | PIE binary base randomization |
| `entropy_bits` | 28 | Bits of address randomization |

## Virtual Memory Layout

```
0x0000_0000_0000_0000  - NULL (unmapped, catches null dereferences)
0x0000_7FFF_FFFF_FFFF  - Userspace limit
0xFFFF_8000_0000_0000  - Kernel space start (W^X enforced)
0xFFFF_FFFF_8000_0000  - Kernel image (wired, read-only+exec)
0xFFFF_FFFF_C000_0000  - Kernel data (wired, read-write, non-exec)
```

## Security Audit

- W^X violations log to the security audit subsystem (src/security/audit.rs)
- Any attempt to map W+X pages triggers an immediate process termination
- Kernel modules must be signed before being granted execute permissions

## Linux Distro Inspirations

| Feature | Source Distro/OS |
|---------|-----------------|
| W^X policy | OpenBSD |
| Wired pages | FreeBSD |
| SMEP/SMAP | Linux (hardware-enforced) |
| Page table isolation (PTI) | Linux (Meltdown mitigation) |
| Guard pages | NetBSD |
