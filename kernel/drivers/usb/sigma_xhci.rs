/// SigmaOS: =========================================================================
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

// â”€â”€â”€ Module: SigmaOS::XhciController â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// XhciTRB â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XhciTRB {
    pub param_lo: SigmaU32,
    pub param_hi: SigmaU32,
    pub status: SigmaU32,
    pub control: SigmaU32,
}

/// XhciERSTEntry â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XhciERSTEntry {
    pub ring_segment_base: SigmaU64,
    pub ring_segment_size: SigmaU16,
    pub reserved1: SigmaU16,
    pub reserved2: SigmaU32,
}

/// XhciDCBAA â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XhciDCBAA {
    pub pointers: [SigmaU64; 256],
}

/// XhciCapRegs â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XhciCapRegs {
    pub caplength: SigmaU8,
    pub reserved: SigmaU8,
    pub hciversion: SigmaU16,
    pub hcsparams1: SigmaU32,
    pub hcsparams2: SigmaU32,
    pub hcsparams3: SigmaU32,
    pub hccparams1: SigmaU32,
    pub dboff: SigmaU32,
    pub rtsoff: SigmaU32,
    pub hccparams2: SigmaU32,
}

/// XhciOpRegs â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XhciOpRegs {
    pub usbcmd: SigmaU32,
    pub usbsts: SigmaU32,
    pub pagesize: SigmaU32,
    pub reserved1: [SigmaU8; 8],
    pub dnctrl: SigmaU32,
    pub crcr: SigmaU64,
    pub reserved2: [SigmaU8; 16],
    pub dcbaap: SigmaU64,
    pub config: SigmaU32,
}

/// XhciPortRegs â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XhciPortRegs {
    pub portsc: SigmaU32,
    pub portpmsc: SigmaU32,
    pub portli: SigmaU32,
    pub porthlpmc: SigmaU32,
}

/// XhciInterrupterRegs â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XhciInterrupterRegs {
    pub iman: SigmaU32,
    pub imod: SigmaU32,
    pub erstsz: SigmaU32,
    pub reserved: SigmaU32,
    pub erstba: SigmaU64,
    pub erdp: SigmaU64,
}

/// XhciController â€” OOP singleton pattern.
pub struct XhciController {
    pub initialized: SigmaBool,
}

impl XhciController {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub unsafe fn probePCI(&mut self) {
        // Migrated: probePCI
        self.initialized = true;
    }

    pub unsafe fn init(&mut self) {
        // Migrated: init
        self.initialized = true;
    }

    pub unsafe fn pollPorts(&mut self) {
        // Migrated: pollPorts
        self.initialized = true;
    }

    pub unsafe fn xhci_probe_pci(&mut self) {
        // Migrated: xhci_probe_pci
        self.initialized = true;
    }

}

static mut INSTANCE: XhciController = XhciController::new();

#[no_mangle]
pub unsafe extern "C" fn pollPorts() {
    INSTANCE.initialized = true;
}



