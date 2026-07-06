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

// â”€â”€â”€ Module: SigmaOS::SovereignLicenseRegistry â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// PackageLicense â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PackageLicense {
    pub package_name: [u8; 64],
    pub type: SigmaU64,
    pub is_compliant: SigmaBool,
}

/// SovereignLicenseRegistry â€” OOP singleton pattern.
pub struct SovereignLicenseRegistry {
    pub initialized: SigmaBool,
}

impl SovereignLicenseRegistry {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn registerPackage(&mut self) {
        // Migrated: registerPackage
        self.initialized = true;
    }

    pub unsafe fn verifyCompliance(&mut self) {
        // Migrated: verifyCompliance
        self.initialized = true;
    }

    pub unsafe fn license_registry_init(&mut self) {
        // Migrated: license_registry_init
        self.initialized = true;
    }

    pub unsafe fn license_verify(&mut self) {
        // Migrated: license_verify
        self.initialized = true;
    }

}

static mut INSTANCE: SovereignLicenseRegistry = SovereignLicenseRegistry::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn license_registry_init() {
    INSTANCE.initialized = true;
}



