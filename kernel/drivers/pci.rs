//! SigmaOS — PCI Bus Driver
//! Scans the PCI/PCIe bus, enumerates devices, handles config space access.
//! Pure no_std implementation.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type Usize = usize;

// ── x86 I/O Ports ─────────────────────────────────────────────────────────────
const CONFIG_ADDRESS: u16 = 0x0CF8;
const CONFIG_DATA:    u16 = 0x0CFC;

// Inline assembly to read/write I/O ports. In a real kernel, these would be
// imported from an `arch::x86::io` module.
unsafe fn outl(port: u16, val: u32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") val, options(nomem, nostack));
}

unsafe fn inl(port: u16) -> u32 {
    let val: u32;
    core::arch::asm!("in eax, dx", out("eax") val, in("dx") port, options(nomem, nostack));
    val
}

// ── PCI Configuration Space ───────────────────────────────────────────────────

/// Read a 32-bit word from the PCI configuration space.
#[no_mangle]
pub unsafe extern "C" fn pci_config_read32(bus: U8, slot: U8, func: U8, offset: U8) -> U32 {
    let addr = 0x80000000 |
        ((bus as U32) << 16) |
        ((slot as U32) << 11) |
        ((func as U32) << 8) |
        (offset as U32 & 0xFC);
    outl(CONFIG_ADDRESS, addr);
    inl(CONFIG_DATA)
}

/// Write a 32-bit word to the PCI configuration space.
#[no_mangle]
pub unsafe extern "C" fn pci_config_write32(bus: U8, slot: U8, func: U8, offset: U8, val: U32) {
    let addr = 0x80000000 |
        ((bus as U32) << 16) |
        ((slot as U32) << 11) |
        ((func as U32) << 8) |
        (offset as U32 & 0xFC);
    outl(CONFIG_ADDRESS, addr);
    outl(CONFIG_DATA, val);
}

// ── PCI Device Enumeration ────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PciDevice {
    pub bus:        U8,
    pub slot:       U8,
    pub func:       U8,
    pub vendor_id:  U16,
    pub device_id:  U16,
    pub class_code: U8,
    pub subclass:   U8,
    pub prog_if:    U8,
    pub header_type:U8,
    pub bar0:       U32,
    pub bar1:       U32,
    pub irq_pin:    U8,
    pub irq_line:   U8,
}

impl PciDevice {
    pub const fn zero() -> Self {
        PciDevice {
            bus: 0, slot: 0, func: 0, vendor_id: 0, device_id: 0,
            class_code: 0, subclass: 0, prog_if: 0, header_type: 0,
            bar0: 0, bar1: 0, irq_pin: 0, irq_line: 0,
        }
    }
}

const MAX_DEVICES: Usize = 256;
static mut DEVICES: [PciDevice; MAX_DEVICES] = [PciDevice::zero(); MAX_DEVICES];
static mut DEVICE_COUNT: Usize = 0;

/// Check if a function has a valid device
unsafe fn check_function(bus: U8, slot: U8, func: U8) {
    let id_reg = pci_config_read32(bus, slot, func, 0x00);
    let vendor = (id_reg & 0xFFFF) as U16;
    if vendor == 0xFFFF { return; } // device doesn't exist

    let dev_id = (id_reg >> 16) as U16;
    let class_reg = pci_config_read32(bus, slot, func, 0x08);
    let prog_if = ((class_reg >> 8) & 0xFF) as U8;
    let subclass = ((class_reg >> 16) & 0xFF) as U8;
    let class_code = ((class_reg >> 24) & 0xFF) as U8;

    let hdr_reg = pci_config_read32(bus, slot, func, 0x0C);
    let header_type = ((hdr_reg >> 16) & 0xFF) as U8;

    let bar0 = pci_config_read32(bus, slot, func, 0x10);
    let bar1 = pci_config_read32(bus, slot, func, 0x14);

    let int_reg = pci_config_read32(bus, slot, func, 0x3C);
    let irq_line = (int_reg & 0xFF) as U8;
    let irq_pin = ((int_reg >> 8) & 0xFF) as U8;

    if DEVICE_COUNT < MAX_DEVICES {
        DEVICES[DEVICE_COUNT] = PciDevice {
            bus, slot, func, vendor_id: vendor, device_id: dev_id,
            class_code, subclass, prog_if, header_type,
            bar0, bar1, irq_pin, irq_line,
        };
        DEVICE_COUNT += 1;
    }

    // Handle PCI-to-PCI bridge
    if class_code == 0x06 && subclass == 0x04 {
        let bus_reg = pci_config_read32(bus, slot, func, 0x18);
        let sec_bus = ((bus_reg >> 8) & 0xFF) as U8;
        check_bus(sec_bus);
    }
}

/// Check all slots on a bus
unsafe fn check_bus(bus: U8) {
    for slot in 0..32 {
        let id_reg = pci_config_read32(bus, slot, 0, 0x00);
        let vendor = (id_reg & 0xFFFF) as U16;
        if vendor == 0xFFFF { continue; }

        check_function(bus, slot, 0);

        let hdr_reg = pci_config_read32(bus, slot, 0, 0x0C);
        let header_type = ((hdr_reg >> 16) & 0xFF) as U8;
        if header_type & 0x80 != 0 {
            // Multi-function device
            for func in 1..8 {
                let func_id = pci_config_read32(bus, slot, func, 0x00);
                if (func_id & 0xFFFF) as U16 != 0xFFFF {
                    check_function(bus, slot, func);
                }
            }
        }
    }
}

/// Scan all PCI buses to populate device list
#[no_mangle]
pub unsafe extern "C" fn pci_scan_buses() {
    DEVICE_COUNT = 0;
    let id_reg = pci_config_read32(0, 0, 0, 0x00);
    if (id_reg & 0xFFFF) as U16 == 0xFFFF {
        return; // No host controller?
    }

    let hdr_reg = pci_config_read32(0, 0, 0, 0x0C);
    if ((hdr_reg >> 16) & 0xFF) & 0x80 == 0 {
        // Single PCI host controller
        check_bus(0);
    } else {
        // Multiple PCI host controllers
        for func in 0..8 {
            let func_id = pci_config_read32(0, 0, func, 0x00);
            if (func_id & 0xFFFF) as U16 != 0xFFFF {
                break;
            }
            check_bus(func);
        }
    }
}

/// Get device count
#[no_mangle]
pub unsafe extern "C" fn pci_get_device_count() -> Usize {
    DEVICE_COUNT
}

/// Get device by index
#[no_mangle]
pub unsafe extern "C" fn pci_get_device(idx: Usize, out: *mut PciDevice) -> i32 {
    if idx >= DEVICE_COUNT || out.is_null() { return -1; }
    *out = DEVICES[idx];
    0
}
