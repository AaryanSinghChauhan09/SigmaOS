/// SigmaOS Sovereign Symmetric Multi-Processing (SMP) Controller
/// Migrated from C++ to Rust — no_std, no alloc, no external crates.
/// Boots Application Processors and manages Inter-Processor Interrupts.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU32 = u32;
type SigmaU64 = u64;

// ─── Local APIC Constants ───────────────────────────────────────────────────

/// Hardware constants for the Local APIC (Advanced Programmable Interrupt Controller).
struct ApicConstants;
impl ApicConstants {
    /// Default physical base address of the Local APIC MMIO region.
    const LOCAL_APIC_BASE: SigmaU64 = 0xFEE0_0000;
    /// Offset of the Interrupt Command Register (ICR) low 32 bits.
    const ICR_LOW: SigmaU64 = 0x300;
    /// Offset of the ICR high 32 bits (destination field).
    const ICR_HIGH: SigmaU64 = 0x310;
    /// INIT IPI delivery mode.
    const INIT_IPI: SigmaU32 = 0x0000_4500;
    /// Startup IPI (SIPI) delivery mode.
    const SIPI: SigmaU32 = 0x0000_4600;
    /// Maximum number of cores supported.
    const MAX_CORES: SigmaU32 = 256;
}

// ─── Per-CPU Descriptor ─────────────────────────────────────────────────────

/// Represents a single logical CPU core.
#[repr(C)]
struct CpuCore {
    /// APIC ID of this core.
    apic_id: SigmaU32,
    /// Whether this core has been booted and is online.
    online:  bool,
    /// Per-core stack base address.
    stack_base: SigmaU64,
}

impl CpuCore {
    /// Create an uninitialized core descriptor.
    const fn new() -> Self {
        Self {
            apic_id:    0,
            online:     false,
            stack_base: 0,
        }
    }
}

// ─── SovereignSMP ───────────────────────────────────────────────────────────

/// The Sovereign SMP controller.
/// Manages multi-core boot, core enumeration, and IPI dispatch.
struct SovereignSMP {
    /// Array of all discovered CPU cores.
    cores: [CpuCore; ApicConstants::MAX_CORES as usize],
    /// Number of cores detected from ACPI/MADT.
    core_count: SigmaU32,
    /// Whether the SMP subsystem has been initialized.
    initialized: bool,
}

impl SovereignSMP {
    /// Construct a new SMP controller with all cores offline.
    const fn new() -> Self {
        Self {
            cores: [CpuCore::new(); ApicConstants::MAX_CORES as usize],
            core_count: 0,
            initialized: false,
        }
    }

    /// Write a 32-bit value to a Local APIC register via MMIO.
    #[inline(always)]
    unsafe fn apic_write(&self, offset: SigmaU64, value: SigmaU32) {
        let addr = (ApicConstants::LOCAL_APIC_BASE + offset) as *mut SigmaU32;
        core::ptr::write_volatile(addr, value);
    }

    /// Read a 32-bit value from a Local APIC register via MMIO.
    #[inline(always)]
    unsafe fn apic_read(&self, offset: SigmaU64) -> SigmaU32 {
        let addr = (ApicConstants::LOCAL_APIC_BASE + offset) as *const SigmaU32;
        core::ptr::read_volatile(addr)
    }

    /// Send an INIT IPI followed by a Startup IPI (SIPI) to boot an AP core.
    unsafe fn init_core(&mut self, core_id: SigmaU32) {
        // Set destination in ICR high (APIC ID in bits [31:24])
        self.apic_write(ApicConstants::ICR_HIGH, core_id << 24);

        // Send INIT IPI
        self.apic_write(ApicConstants::ICR_LOW, ApicConstants::INIT_IPI);

        // Delay (simplified — a real implementation would use APIC timer or TSC)
        let mut spin: SigmaU32 = 0;
        while spin < 100_000 {
            spin += 1;
        }

        // Send SIPI with trampoline page at 0x8000 (vector = 0x08)
        self.apic_write(ApicConstants::ICR_LOW, ApicConstants::SIPI | 0x08);

        self.cores[core_id as usize].apic_id = core_id;
        self.cores[core_id as usize].online = true;
    }

    /// Boot all Application Processors.
    /// Detects core count from ACPI/MADT (simulated at 16 cores).
    /// Sends INIT + SIPI sequence to each AP.
    unsafe fn boot_aps(&mut self) {
        // In a real implementation, parse MADT to discover core count
        self.core_count = 16;

        let mut i: SigmaU32 = 1; // Skip BSP (core 0)
        while i < self.core_count {
            self.init_core(i);
            i += 1;
        }

        self.initialized = true;
    }

    /// Send an Inter-Processor Interrupt to a specific core.
    unsafe fn send_ipi(&self, core_id: SigmaU32, vector: SigmaU32) {
        self.apic_write(ApicConstants::ICR_HIGH, core_id << 24);
        self.apic_write(ApicConstants::ICR_LOW, vector);
    }
}

// ─── Global Singleton ───────────────────────────────────────────────────────

/// The single SMP controller instance.
static mut SMP: SovereignSMP = SovereignSMP::new();

// ─── C-ABI Bridge ───────────────────────────────────────────────────────────

/// Initialize SMP and boot all Application Processors.
/// Replaces the original `extern "C" void sigma_smp_init()`.
#[no_mangle]
pub unsafe extern "C" fn sigma_smp_init() {
    SMP.boot_aps();
}

/// Send an IPI to the specified core with the given interrupt vector.
/// Replaces the original `extern "C" void sigma_smp_send_ipi(...)`.
#[no_mangle]
pub unsafe extern "C" fn sigma_smp_send_ipi(core_id: u32, vector: u32) {
    SMP.send_ipi(core_id, vector);
}
