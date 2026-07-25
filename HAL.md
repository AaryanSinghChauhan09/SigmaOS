# Hardware Abstraction Layer (HAL)

The SigmaOS HAL provides architecture-independent access to hardware.

---

## Components

| Component | Language | File | Status |
|-----------|----------|------|--------|
| Port I/O | Zig | `drivers/hal/port_io.zig` | ✅ |
| MMIO | Rust | `drivers/hal/mmio.rs` | ✅ |
| PCI enumeration | Rust | `kernel/core/sigma_pci.rs` | ✅ |
| ACPI parsing | Rust | `kernel/core/sigma_acpi.rs` | ✅ |
| 4-level paging | Zig | `arch/x86_64/paging.zig` | ✅ |
| GDT/TSS | NASM | `arch/x86_64/gdt.asm` | ✅ |
| IDT (256 entries) | NASM | `arch/x86_64/idt.asm` | ✅ |
| Context switch | NASM | `arch/x86_64/context_switch.asm` | ✅ |
| PIC (8259) | Rust | `kernel/core/sigma_irq.rs` | ✅ |
| PIT timer | Rust | `kernel/core/sigma_irq.rs` | ✅ |
| VGA text | Rust | `drivers/display/vga_console.rs` | ✅ |
| VESA framebuffer | Rust | `drivers/display/vga_console.rs` | ✅ |
| PS/2 keyboard | Rust | `drivers/input/keyboard.rs` | ✅ |
| Serial (COM1) | Rust | `drivers/char/console.rs` | ✅ |

---

## Port I/O

```zig
// drivers/hal/port_io.zig
pub fn inb(port: u16) u8 { ... }   // read byte from I/O port
pub fn outb(port: u16, val: u8) { ... }  // write byte to I/O port
```

---

## MMIO

```rust
// drivers/hal/mmio.rs
pub fn mmio_read32(base: *const u32, offset: usize) -> u32 {
    unsafe { core::ptr::read_volatile(base.byte_add(offset)) }
}
pub fn mmio_write32(base: *mut u32, offset: usize, val: u32) {
    unsafe { core::ptr::write_volatile(base.byte_add(offset), val); }
}
```

---

## PCI Enumeration

```c
// Enumerate all PCI devices at boot
sigma_pci_enumerate();

// Read PCI config space
uint32_t ids = sigma_pci_read_config32(bus, dev, func, 0x00);
uint16_t vendor = ids & 0xFFFF;
uint16_t device = (ids >> 16) & 0xFFFF;

// Enable bus mastering + memory space access
sigma_pci_enable(bus, dev, func);
```

---

## ACPI

```c
// Parse ACPI tables from RSDP (passed by bootloader)
sigma_acpi_parse(boot_info->rsdp_addr);

// Query results
size_t ncpus = sigma_acpi_cpu_count();
uint64_t lapic = sigma_acpi_lapic_base();
uint64_t ioapic = sigma_acpi_ioapic_base();
```

---

## Paging (4-level, x86_64)

```zig
// arch/x86_64/paging.zig
pub const PageTable = struct {
    entries: [512]PageEntry,
    pub fn map(self: *PageTable, index: usize, phys: u64, flags: u64) void;
    pub fn unmap(self: *PageTable, index: usize) void;
};

pub const FrameAllocator = struct {
    pub fn allocate(self: *FrameAllocator) ?u64;
};
```

Head64.asm sets up identity-mapped 4GB (2MB huge pages) at boot.

---

## Timer

```c
// PIT 1000 Hz initialized at boot
// sigma_clock_ns() = jiffies × 1,000,000 nanoseconds

uint64_t now_ns = sigma_clock_ns();   // nanoseconds since boot
uint64_t jiffies = sigma_jiffies();   // milliseconds since boot
```

---

## Serial Debug

```c
// All kernel log output goes to COM1 (115200 8N1)
// Viewable with: -serial stdio in QEMU

serial_puts("[BOOT] SigmaOS starting\n");
sigma_log(msg_ptr, msg_len);   // also goes to VGA
```

---

*See also: [Bootloader](Bootloader) · [Driver Framework](Driver-Framework) · [Architecture](Architecture-Overview)*
