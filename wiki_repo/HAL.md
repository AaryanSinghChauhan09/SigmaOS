# Hardware Abstraction Layer (HAL)

The SigmaOS HAL provides a thin, zero-overhead interface between the architecture-specific assembly stubs and the portable microkernel core. It allows the same kernel code to run on x86_64, ARM64, and RISC-V by swapping the HAL implementation, without changing any core logic.

---

## Design Principle

The HAL is intentionally minimal. It only abstracts operations that are genuinely different across architectures:

- CPU halt / wait-for-interrupt
- Hardware timer initialization and calibration
- Interrupt controller setup (APIC on x86, GIC on ARM)
- MMU page table manipulation
- I/O port read/write (x86 only — other architectures use MMIO)

Everything else — scheduling logic, VFS, TCP/IP, syscall dispatch — is pure portable C++ with no HAL involvement.

---

## API (`hal.h` / `sigma_hal.h`)

```c
// Halt the CPU until the next interrupt
void cpu_halt(void);

// Initialize the hardware timer at the given frequency (Hz)
void timer_init(sigma_u32 hz);

// Initialize the interrupt controller (APIC/PIC/GIC)
void interrupt_init(void);

// Map a virtual address to a physical address with given flags
void mmu_map(sigma_u64 virt, sigma_u64 phys, sigma_u32 flags);

// Unmap a virtual address
void mmu_unmap(sigma_u64 virt);

// Read a byte from an I/O port (x86 only)
sigma_u8 read_io(sigma_u16 port);

// Write a byte to an I/O port (x86 only)
void write_io(sigma_u16 port, sigma_u8 value);

// Read the current CPU timestamp counter (for timing)
sigma_u64 cpu_rdtsc(void);

// Enable/disable interrupts
void interrupts_enable(void);
void interrupts_disable(void);
```

---

## Architecture Implementations

### x86_64 (`arch/x86_64/`)

| File | Contents |
|---|---|
| `paging.asm` | PML4/PDPT/PD/PT setup, `cr3` load |
| `paging.c` | VMM helper functions using the asm primitives |
| `switch.asm` | Context switch — saves/restores registers, updates `cr3` |
| `vmm_fast.asm` | Fast path for TLB invalidation and page table walks |

The x86_64 HAL uses:
- **PIT (Programmable Interval Timer)** or **LAPIC timer** for `timer_init`.
- **APIC** for `interrupt_init` (replaces the legacy 8259 PIC).
- `inb`/`outb` instructions for `read_io`/`write_io`.
- `hlt` instruction for `cpu_halt`.

### ARM64 (planned)

The ARM64 HAL stub will use:
- **GIC (Generic Interrupt Controller)** for `interrupt_init`.
- **Generic Timer** (`CNTPCT_EL0`) for `timer_init` and `cpu_rdtsc`.
- MMIO for all I/O (no `inb`/`outb`).
- `wfi` (Wait For Interrupt) for `cpu_halt`.

### RISC-V (planned)

- **PLIC** for interrupt control.
- `rdtime` CSR for timestamps.
- `wfi` for halt.
- Sv48 paging for MMU.

---

## Boot Sequence and HAL Initialization

The HAL is initialized very early in the boot sequence, before any other kernel subsystem:

```
boot/sovereign_boot.asm
    │
    ├─ Set up initial GDT (flat segments)
    ├─ Enter 64-bit long mode
    ├─ Set up initial page tables (identity map first 4 GB)
    └─ Call kmain()
            │
            ├─ sigma_hal_init()          ← HAL first
            │    ├─ interrupt_init()     (APIC)
            │    └─ timer_init(1000)     (1 kHz tick)
            ├─ sigma_idt_init()          ← IDT second
            ├─ sigma_mm_init()           ← Memory manager
            ├─ sigma_vfs_init()          ← VFS
            ├─ sigma_net_init()          ← Network stack
            └─ sigma_init_start()        ← PID 1
```

---

## Multiboot Header

SigmaOS uses the **Multiboot2** specification for GRUB compatibility:

```asm
; arch/boot/multiboot_header.asm
MULTIBOOT2_MAGIC    equ 0xE85250D6
MULTIBOOT2_ARCH     equ 0           ; i386 / x86_64
MULTIBOOT2_LENGTH   equ header_end - header_start
MULTIBOOT2_CHECKSUM equ -(MULTIBOOT2_MAGIC + MULTIBOOT2_ARCH + MULTIBOOT2_LENGTH)

section .multiboot
header_start:
    dd MULTIBOOT2_MAGIC
    dd MULTIBOOT2_ARCH
    dd MULTIBOOT2_LENGTH
    dd MULTIBOOT2_CHECKSUM
    ; end tag
    dw 0, 0
    dd 8
header_end:
```

GRUB reads this header and passes a Multiboot2 info struct to `kmain` in `rbx`, containing the physical memory map, framebuffer info, and command line.

---

*See also: [Kernel](Kernel) · [Building from Source](Building-from-Source) · [Architecture Overview](Architecture-Overview)*
