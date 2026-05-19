# SigmaOS Kernel Layer

This directory contains the core microkernel services for SigmaOS Zenith: process scheduling, memory management, and device drivers.

## Sub-Modules

### 1. Process Scheduler (`/kernel/scheduler/`)
The process scheduler handles multi-tasking across active execution shards. It supports:
- **Round-Robin Scheduling**: A simple, failure-isolated time-sharing mechanism.
- **Priority-Based Scheduling**: Dynamic thread priorities for real-time operations (RTOS mode).

### 2. Memory Management (`/kernel/core/mem/` and `/kernel/core/memory/`)
Provides zero-dependency physical and virtual memory allocators:
- **Physical Memory Manager (PMM)**: Bitmapped page allocator.
- **Virtual Memory Manager (VMM)**: Manages page tables and directory mapping.
- **Sovereign Allocator**: A fast slab/heap allocator (`malloc`/`free` equivalent for Ring 0).

### 3. Device Drivers (`/kernel/drivers/`)
Hardware enablement for generic PC architectures:
- **VGA Display (`vga.c`, `vbe.c`)**: Text mode and VESA linear framebuffer rendering.
- **UART Serial (`serial.c`)**: Debug printing to COM1 serial console.
- **Storage Controllers (`ata.c`, `ide.c`)**: Hard drive read/write access.
- **Ethernet Controller (`e1000.c`)**: Intel e1000 PCIe network card interface.
- **Input (`keyboard.c`)**: Standard PS/2 keyboard scancode handler.
