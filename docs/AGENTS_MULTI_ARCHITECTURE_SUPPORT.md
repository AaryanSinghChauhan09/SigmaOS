# AI Agent Guidelines: Multi-Architecture Hardware Support in SigmaOS

## 📌 1. Overview & Architectural Vision

Inspired by the hardware portability of **Linux** (`arch/x86`, `arch/arm64`, `arch/riscv`) and **NetBSD/FreeBSD** cross-platform kernel HALs, **SigmaOS** natively supports multi-architecture CPU targets across ancient and modern hardware generations.

As an AI agent developing microkernel HALs, device drivers, or memory managers, you must enforce **architecture-neutral HAL trait interfaces** (`PlatformHAL` / `ArchitectureHal`), keeping architecture-specific assembly and MMIO registers encapsulated behind HAL implementations.

---

## 🏛️ 2. Supported Target Architectures & Features

```
+-----------------------------------------------------------------------------------+
|                        SIGMAOS MULTI-ARCH HAL TAXONOMY                            |
+-----------------------------------------------------------------------------------+
|  💻 x86_64     | APIC / IOAPIC | ACPI 6.5 MADT | PCIe ECAM | 4-Level/5-Level CR3  |
|  🕹️ X86_32     | 8259 PIC      | 2-Level / PAE | Legacy ISA| 32-Bit Flat Memory   |
|  📱 AArch64    | GICv3 / v4    | FDT / DTB Tree | PSCI Calls| TTBR0_EL1 / TTBR1    |
|  ⚡ Armv7      | GICv2         | Cortex-A LPAE  | PSCI v0.2 | 32-Bit VMSA          |
|  🌌 RISC-V 64  | PLIC / CLINT  | SBI ecalls     | FDT Tree  | Sv39 / Sv48 satp     |
|  🔬 RISC-V 32  | PLIC32        | SBI ecalls     | FDT Tree  | Sv32 satp            |
|  🐉 LoongArch  | ExtIOI / PCH  | ACPI / FDT     | IOCSR     | 4-Level PGDL         |
|  🚀 PPC64LE    | XIVE / OPAL   | Device Tree    | PowerNV   | Radix Page Tables    |
+-----------------------------------------------------------------------------------+
```

---

## ⚙️ 3. Core HAL Abstraction (`PlatformHAL` / `ArchitectureHal`)

* **Module Location:** `src/arch/hal.rs`, `src/hal/advanced_hal.rs`, `src/kernel/architecture.rs`

### 3.1 Unified Interface Methods:
```rust
pub trait PlatformHAL: Send + Sync {
    /// Initialise architecture-specific interrupt controllers (APIC/GIC/PLIC/ExtIOI)
    fn init(&mut self) -> Result<(), DriverError>;

    /// Context switch virtual address space page tables (CR3/TTBR/satp/PGDL)
    unsafe fn switch_address_space(&self, phys_pgd: u64) -> Result<(), DriverError>;

    /// Enable or disable CPU local interrupts
    fn set_interrupt_enabled(&self, enabled: bool) -> bool;

    /// Allocate physical DMA buffers with memory alignment
    fn allocate_dma_buffer(&self, size: usize, alignment: usize) -> Result<HalDmaBuffer, DriverError>;

    /// Register architecture-neutral IRQ handler
    fn register_irq_handler(&mut self, vector: u32, handler: Box<dyn Fn() + Send + Sync>) -> Result<(), DriverError>;

    /// Execute power state transitions (Shutdown / Reboot)
    fn power_transition(&mut self, state: HalPowerState) -> Result<(), DriverError>;
}
```

---

## 🏭 4. Multi-Arch Factory Pattern (`HALFactory`)

At system boot, the microkernel detects CPU capabilities and instantiates the matching architecture HAL via `HALFactory`:

```rust
pub struct HALFactory;

impl HALFactory {
    pub fn create_arch_hal(arch: Architecture) -> Box<dyn ArchitectureHal> {
        match arch {
            Architecture::X86_64 => Box::new(X86_64HAL::new()),
            Architecture::ARM64 => Box::new(ARM64HAL::new()),
            Architecture::RISCV64 => Box::new(RISCV64HAL::new()),
        }
    }
}
```

---

## 🛡️ 5. AI Agent Rules & Code Patterns

1. **Encapsulate Inline Assembly:**
   * Never execute direct `asm!` blocks inside common drivers or VFS code. Always route hardware-specific instructions (`invlpg`, `tlbi`, `sfence`, `isb`) through `PlatformHAL` or `src/klib/isa.rs`.
2. **Conditional Compilation Attributes:**
   * Protect target-specific register definitions using explicit Rust conditional compilation attributes:
     `#[cfg(target_arch = "x86_64")]`, `#[cfg(target_arch = "aarch64")]`, `#[cfg(target_arch = "riscv64")]`.
3. **Cross-Compile Target Validation:**
   * Ensure drivers compile cleanly against target triples:
     * `x86_64-unknown-none`
     * `aarch64-unknown-none`
     * `riscv64gc-unknown-none-elf`

---

## 🧪 6. Standalone Testing Commands

AI agents can verify multi-architecture HAL implementations, IRQ domains, and factory dispatches via standalone unit compilation:

```bash
# Test multi-arch HAL implementations (x86_64, AArch64, RISC-V 64)
rustc --test --edition=2021 src/arch/hal.rs -o build/hal_tests && ./build/hal_tests && rm build/hal_tests

# Test kernel architecture abstractions & CPU register contexts
rustc --test --edition=2021 src/kernel/architecture.rs -o build/arch_tests && ./build/arch_tests && rm build/arch_tests
```
