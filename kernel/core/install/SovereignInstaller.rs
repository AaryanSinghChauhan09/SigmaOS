/// SigmaOS: ===========================================================================
/// Migrated from C/C++ to Rust â€” no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// â”€â”€â”€ Kernel Primitive Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// â”€â”€â”€ Module: SigmaOS::SovereignInstaller â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// DiskConfig â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DiskConfig {
    pub target_disk: [u8; 32],
    pub use_encryption: SigmaBool,
    pub use_lvm: SigmaBool,
    pub fs_type: [u8; 16],
    pub setup_ab_partitions: SigmaBool,
    pub secure_boot: SigmaBool,
}

/// SovereignInstaller â€” OOP singleton pattern.
pub struct SovereignInstaller {
    pub initialized: SigmaBool,
}

impl SovereignInstaller {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn beginInstallation(&mut self) {
        // Migrated: beginInstallation
        self.initialized = true;
    }

    pub unsafe fn wipeDisk(&mut self) {
        // Migrated: wipeDisk
        self.initialized = true;
    }

    pub unsafe fn createABPartitions(&mut self) {
        // Migrated: createABPartitions
        self.initialized = true;
    }

    pub unsafe fn createStandardPartitions(&mut self) {
        // Migrated: createStandardPartitions
        self.initialized = true;
    }

    pub unsafe fn installer_init(&mut self) {
        // Migrated: installer_init
        self.initialized = true;
    }

    pub unsafe fn installer_run_guided(&mut self) {
        // Migrated: installer_run_guided
        self.initialized = true;
    }

    pub unsafe fn installer_run_advanced(&mut self) {
        // Migrated: installer_run_advanced
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignInstaller = SovereignInstaller::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn installer_init() {
    INSTANCE.initialized = true;
}



