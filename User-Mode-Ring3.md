# SigmaOS User Mode & Ring 3 Transition

## Overview

SigmaOS supports **Ring 3 (User Mode)** execution using the standard x86_64 privilege mechanism. The kernel uses `iretq` to safely drop from Ring 0 to Ring 3, with a Task State Segment (TSS) ensuring the CPU always knows where to find a safe kernel stack on interrupt.

## Task State Segment (TSS)

The TSS is a hardware-defined structure the CPU reads on every privilege change.

```c
typedef struct {
    sigma_u32 reserved0;
    sigma_u64 rsp0;       // Kernel stack (Ring 0) — MOST IMPORTANT
    sigma_u64 rsp1;
    sigma_u64 rsp2;
    sigma_u64 reserved1;
    sigma_u64 ist1..ist7; // Interrupt Stack Tables
    sigma_u16 iopb_offset;
} sigma_tss_t;
```

On every context switch, `tss_set_kernel_stack(stack_ptr)` is called to update `rsp0` so the correct kernel stack is used.

## The Ring 3 Transition

`switch_to_user_mode(rip, rsp)` builds an `iretq` stack frame and executes it:

```
Stack (before iretq):
  [SS        | 0x23 (user data segment, RPL=3)]
  [RSP       | user stack pointer              ]
  [RFLAGS    | with IF set (interrupts enabled)]
  [CS        | 0x1B (user code segment, RPL=3) ]
  [RIP       | user entry point               ]
```

`iretq` pops all 5 values and jumps to `RIP` at Ring 3.

## GDT Layout (Typical)

| Index | Selector | Descriptor |
|---|---|---|
| 0 | `0x00` | Null |
| 1 | `0x08` | Kernel Code (Ring 0) |
| 2 | `0x10` | Kernel Data (Ring 0) |
| 3 | `0x18` | User Code (Ring 3, RPL=3) |
| 4 | `0x20` | User Data (Ring 3, RPL=3) |
| 5 | `0x28` | TSS Descriptor |

## Security Guarantee

A Ring 3 shard that executes a privileged instruction (e.g., `cli`, `hlt`, `outb`) will trigger a **General Protection Fault** (IDT vector `#13`), which the kernel catches and uses to cleanly reap the shard without system destabilization.

## Relevant Source Files

- `include/kernel/sigma_tss.h` — TSS structure and API
- `kernel/core/hal/sigma_tss.cpp` — TSS init and `switch_to_user_mode`
- `include/kernel/sigma_idt.h` — IDT structures
- `kernel/core/hal/sigma_idt.cpp` — IDT dispatch logic
