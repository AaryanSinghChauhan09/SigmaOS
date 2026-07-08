/// SigmaOS: Î£ SigmaOS Zenith â€” PCI Bus Enumerator
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

// â”€â”€â”€ Module: Sigma::sigma_pci â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// sigma_pci_device â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigma_pci_device {
    pub bus: SigmaU64,
    pub slot: SigmaU64,
    pub func: SigmaU64,
    pub vendor_id: SigmaU64,
    pub device_id: SigmaU64,
    pub class_code: SigmaU64,
    pub subclass: SigmaU64,
    pub prog_if: SigmaU64,
    pub bar: [SigmaU64; 6],
}

// ── Constants ─────────────────────────────────────────────────────────────────
const PCI_CONFIG_ADDRESS: SigmaU16 = 0xCF8;
const PCI_CONFIG_DATA: SigmaU16 = 0xCFC;
const MAX_PCI_DEVICES: SigmaUsize = 64;

// ── PCI Configuration Space Offsets ────────────────────────────────────────────
const PCI_VENDOR_ID_OFFSET: SigmaU8 = 0x00;
const PCI_DEVICE_ID_OFFSET: SigmaU8 = 0x02;
const PCI_CLASS_CODE_OFFSET: SigmaU8 = 0x0B;
const PCI_SUBCLASS_OFFSET: SigmaU8 = 0x0A;
const PCI_PROG_IF_OFFSET: SigmaU8 = 0x09;
const PCI_BAR0_OFFSET: SigmaU8 = 0x10;
const PCI_HEADER_TYPE_OFFSET: SigmaU8 = 0x0E;

// ── PCI Device Registry ───────────────────────────────────────────────────────
static mut PCI_DEVICES: [sigma_pci_device; MAX_PCI_DEVICES] = [sigma_pci_device {
    bus: 0, slot: 0, func: 0, vendor_id: 0, device_id: 0,
    class_code: 0, subclass: 0, prog_if: 0, bar: [0; 6]
}; MAX_PCI_DEVICES];
static mut PCI_DEVICE_COUNT: SigmaUsize = 0;

// ── PCI Configuration Access Functions ─────────────────────────────────────────

/// Read from PCI configuration data port
#[no_mangle]
pub unsafe extern "C" fn sigma_inl(port: SigmaU16) -> SigmaU32 {
    let value: SigmaU32;
    core::arch::asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack));
    value
}

/// Read 32-bit from PCI configuration space
unsafe fn pci_config_read32(bus: SigmaU8, slot: SigmaU8, func: SigmaU8, offset: SigmaU8) -> SigmaU32 {
    let address = ((bus as SigmaU32) << 16) | ((slot as SigmaU32) << 11) | 
                  ((func as SigmaU32) << 8) | ((offset as SigmaU32) & 0xFC) | 0x80000000;
    sigma_outl(PCI_CONFIG_ADDRESS, address);
    sigma_inl(PCI_CONFIG_DATA)
}

/// Read 16-bit from PCI configuration space
unsafe fn pci_config_read16(bus: SigmaU8, slot: SigmaU8, func: SigmaU8, offset: SigmaU8) -> SigmaU16 {
    let address = ((bus as SigmaU32) << 16) | ((slot as SigmaU32) << 11) | 
                  ((func as SigmaU32) << 8) | ((offset as SigmaU32) & 0xFC) | 0x80000000;
    sigma_outl(PCI_CONFIG_ADDRESS, address);
    let shift = (offset & 2) * 8;
    (sigma_inl(PCI_CONFIG_DATA) >> shift) as SigmaU16
}

/// Read 8-bit from PCI configuration space
unsafe fn pci_config_read8(bus: SigmaU8, slot: SigmaU8, func: SigmaU8, offset: SigmaU8) -> SigmaU8 {
    let address = ((bus as SigmaU32) << 16) | ((slot as SigmaU32) << 11) | 
                  ((func as SigmaU32) << 8) | ((offset as SigmaU32) & 0xFC) | 0x80000000;
    sigma_outl(PCI_CONFIG_ADDRESS, address);
    let shift = (offset & 3) * 8;
    (sigma_inl(PCI_CONFIG_DATA) >> shift) as SigmaU8
}

/// Read BAR (Base Address Register)
unsafe fn pci_read_bar(bus: SigmaU8, slot: SigmaU8, func: SigmaU8, bar_num: SigmaU8) -> SigmaU64 {
    let offset = PCI_BAR0_OFFSET + (bar_num * 4);
    let bar_low = pci_config_read32(bus, slot, func, offset) as SigmaU64;
    
    if bar_low & 0x4 != 0 && bar_num < 5 {
        let bar_high = pci_config_read32(bus, slot, func, offset + 4) as SigmaU64;
        (bar_high << 32) | (bar_low & 0xFFFFFFF0)
    } else {
        bar_low & 0xFFFFFFF0
    }
}

/// Check if PCI device exists
unsafe fn pci_device_exists(bus: SigmaU8, slot: SigmaU8, func: SigmaU8) -> bool {
    let vendor_id = pci_config_read16(bus, slot, func, PCI_VENDOR_ID_OFFSET);
    vendor_id != 0xFFFF && vendor_id != 0x0000
}

/// Add PCI device to registry
unsafe fn pci_add_device(bus: SigmaU8, slot: SigmaU8, func: SigmaU8) {
    if PCI_DEVICE_COUNT >= MAX_PCI_DEVICES {
        return;
    }

    let vendor_id = pci_config_read16(bus, slot, func, PCI_VENDOR_ID_OFFSET) as SigmaU64;
    let device_id = pci_config_read16(bus, slot, func, PCI_DEVICE_ID_OFFSET) as SigmaU64;
    let class_code = pci_config_read8(bus, slot, func, PCI_CLASS_CODE_OFFSET) as SigmaU64;
    let subclass = pci_config_read8(bus, slot, func, PCI_SUBCLASS_OFFSET) as SigmaU64;
    let prog_if = pci_config_read8(bus, slot, func, PCI_PROG_IF_OFFSET) as SigmaU64;

    let mut bars = [0u64; 6];
    for i in 0..6 {
        bars[i as usize] = pci_read_bar(bus, slot, func, i);
    }

    PCI_DEVICES[PCI_DEVICE_COUNT] = sigma_pci_device {
        bus: bus as SigmaU64,
        slot: slot as SigmaU64,
        func: func as SigmaU64,
        vendor_id,
        device_id,
        class_code,
        subclass,
        prog_if,
        bar: bars,
    };
    PCI_DEVICE_COUNT += 1;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_outl(port: SigmaU16, value: SigmaU32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack));
}

/// Enumerate all PCI devices on the bus (inspired by Linux pci_scan_slot)
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_enumerate() -> SigmaUsize {
    PCI_DEVICE_COUNT = 0;

    for bus in 0u8..=255 {
        for slot in 0u8..32 {
            if pci_device_exists(bus, slot, 0) {
                pci_add_device(bus, slot, 0);

                let header_type = pci_config_read8(bus, slot, 0, PCI_HEADER_TYPE_OFFSET);
                if header_type & 0x80 != 0 {
                    for func in 1u8..8 {
                        if pci_device_exists(bus, slot, func) {
                            pci_add_device(bus, slot, func);
                        }
                    }
                }
            }
        }
    }

    PCI_DEVICE_COUNT
}

/// Get PCI device by index
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_get_device(index: SigmaUsize) -> sigma_pci_device {
    if index < PCI_DEVICE_COUNT {
        PCI_DEVICES[index]
    } else {
        sigma_pci_device {
            bus: 0, slot: 0, func: 0, vendor_id: 0, device_id: 0,
            class_code: 0, subclass: 0, prog_if: 0, bar: [0; 6]
        }
    }
}

/// Get total PCI device count
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_device_count() -> SigmaUsize {
    PCI_DEVICE_COUNT
}



