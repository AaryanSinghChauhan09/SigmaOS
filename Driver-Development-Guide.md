# 🚗 Driver Development Guide

> Writing hardware drivers for SigmaOS requires a strict adherence to the **Zero-Dependency** rule. You cannot use Linux headers, POSIX APIs, or any predefined C standard library functions.

## 1. The Zero-Dependency Rule
Every driver shard must be fully self-contained. 

**Forbidden:**
- `#include <stdio.h>`
- `#include <stdlib.h>`
- `#include <string.h>`
- `malloc()`, `free()`, `printf()`

**Allowed:**
- Custom type definitions (`u8`, `u16`, `u32`, `u64`).
- Custom utility functions within the shard (e.g., `sovereign_memset`).
- Inline GCC assembly (`__asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));`).

## 2. Port I/O (x86_64)
Instead of relying on external HALs initially, drivers must define their own static inline port I/O wrappers if they need them:

```cpp
static inline u8 sigma_inb(u16 port) {
    u8 ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void sigma_outb(u16 port, u8 val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}
```

## 3. Memory Mapped I/O (MMIO)
For modern drivers like NVMe or e1000, read directly from the physical memory locations provided by the PCI enumeration phase:

```cpp
static inline u32 mmio_read32(u64 base, u32 offset) {
    return *((volatile u32*)(base + offset));
}
```

## 4. Concrete Driver Implementations
SigmaOS features several zero-dependency drivers designed around Silicon-Direct hardware access:
1. **PCI Bus Enumerator (`kernel/drivers/pci/sigma_pci.cpp`)**: Iterates through x86 configuration ports `0xCF8`/`0xCFC` to query and catalog peripheral devices, BAR sizes, and classes.
2. **AHCI/SATA Controller (`kernel/drivers/storage/sigma_ahci.cpp`)**: Probes host adapters, maps HBA operational registers, detects connected SATA drives, and sets up raw sector reads.
3. **USB xHCI Driver (`kernel/drivers/usb/sigma_xhci.cpp`)**: Initializes USB 3.0 controllers, resets slots, allocates Device Context structures, and maps port status registers.
4. **AC97 Audio Driver (`kernel/drivers/sound/sigma_ac97.cpp`)**: Controls master volume registers,PCM channels, sets sample rates, and initiates DMA playback buffers.

## 5. Ring Buffers (SPSC)
Drivers should never block. Instead, use a Single-Producer Single-Consumer (SPSC) ring buffer to pass events (like keystrokes or network packets) to the userland or scheduler.

```cpp
#define RING_SIZE 128
static struct Event ring[RING_SIZE];
static volatile u32 head = 0, tail = 0;

static void push(struct Event* ev) {
    u32 next = (head + 1) % RING_SIZE;
    if (next != tail) {
        ring[head] = *ev;
        head = next;
    }
}
```

