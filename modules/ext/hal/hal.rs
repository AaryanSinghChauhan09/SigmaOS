/// SigmaOS — modules/ext/hal/hal.rs
/// Hardware Abstraction Layer (HAL) for x86_64.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU64  = u64;
type SigmaU32  = u32;
type SigmaU16  = u16;
type SigmaI32  = i32;

// ─── HAL Interface ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn hal_init() -> SigmaI32 {
    // Phase 1: Disable interrupts during init
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("cli");
    
    // Phase 2: Init LAPIC (Local Advanced Programmable Interrupt Controller)
    // Map LAPIC base (usually 0xFEE00000)
    // Spurious interrupt vector (SIVR)
    #[cfg(target_arch = "x86_64")]
    {
        let sivr = 0xFEE0_00F0usize as *mut SigmaU32;
        // Enable APIC (bit 8) and map spurious IRQ to 0xFF
        core::ptr::write_volatile(sivr, 0x1FF);
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn hal_cpu_halt() -> ! {
    #[cfg(target_arch = "x86_64")]
    loop {
        core::arch::asm!("cli", "hlt");
    }
    
    #[allow(unreachable_code)]
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn hal_get_tsc() -> SigmaU64 {
    #[cfg(target_arch = "x86_64")]
    {
        let mut low: SigmaU32;
        let mut high: SigmaU32;
        core::arch::asm!("rdtsc", out("eax") low, out("edx") high);
        return ((high as u64) << 32) | (low as u64);
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        // Fallback or panic for non-x86 builds
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn hal_tlb_flush_all() {
    #[cfg(target_arch = "x86_64")]
    {
        let cr3: SigmaU64;
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
        core::arch::asm!("mov cr3, {}", in(reg) cr3);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hal_tlb_flush_single(addr: SigmaU64) {
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("invlpg [{}]", in(reg) addr);
}

#[no_mangle]
pub unsafe extern "C" fn hal_map_page(
    phys_addr: SigmaU64,
    virt_addr: SigmaU64,
    flags: SigmaU32
) -> SigmaI32 {
    // Stub for page table manipulation.
    // In production, traverses PML4 -> PDPTE -> PDE -> PTE.
    0
}

#[no_mangle]
pub unsafe extern "C" fn hal_register_interrupt() {
    // Stub linked by the interrupts.rs module.
}
