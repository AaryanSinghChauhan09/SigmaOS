# Memory Descriptor Lists & Ancient Device DMA Buffers

SigmaOS incorporates a robust Memory Descriptor List (MDL) architecture and ancient ISA hardware device driver DMA buffer abstraction system, heavily inspired by early Linux kernel iterations (0.01 through 1.0 series) from the Princeton University history archives.

## 1. Memory Descriptor Lists (MDL)

The core `MemoryDescriptorList` structure (implemented under `src/kernel/mm/descriptor_list.rs`) allows describing existing buffers in virtual memory, mapping virtual addresses to non-contiguous physical layouts, and linking multiple memory descriptors for chained transfers.

### Struct Anatomy

*   **Virtual Address Mapping**: Associates high-memory virtual segments with distinct physical offset chains.
*   **Locked/Pinned Memory States**: Implements direct physical pinning to prevent pages from being paged out or swapped.
*   **Chained Linked Lists**: Links multiple lists for massive multi-entry DMA queues.
*   **Memory Protection Enforcements**:
    *   `ReadOnly`
    *   `ReadWrite`
    *   `ExecuteRead`
    *   `ExecuteReadWrite`

***

## 2. Ancient Device DMA Buffers

Traditional 8-bit and 16-bit ISA Direct Memory Access (DMA) controllers (such as the classic Intel 8237 controller) enforce a strict **16 Megabyte physical address boundary** limitation. SigmaOS supports ancient devices via a secure OOP emulation layer.

### Supported Vintage Devices

1.  **Floppy Disk Controller**:
    *   Enforces a strict 64KB maximum buffer transfer limit.
    *   Dedicated ISA DMA Channel 2 allocation.
    *   Constrained to physical pages below the 16MB threshold.
2.  **Sound Blaster 16**:
    *   Implements double-buffered ping-pong audio transfers.
    *   Dedicated ISA DMA Channel 5 allocation.
3.  **NE2000 Ethernet Controller**:
    *   Simulates Shared-RAM ring buffer access bounds and packet offsets without needing dedicated ISA channels.
