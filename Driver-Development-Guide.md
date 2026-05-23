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

## 4. Interrupt Handling
Registering an interrupt handler is done by exposing an `extern "C"` function that the main IDT (Interrupt Descriptor Table) dispatcher will call:

```cpp
extern "C" void sigma_my_driver_irq_handler() {
    // 1. Acknowledge interrupt on device
    // 2. Read data
    // 3. Push to lock-free SPSC ring buffer
}
```

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
