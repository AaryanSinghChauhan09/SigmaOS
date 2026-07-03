// SPDX-License-Identifier: MIT
// kernel/arch/x86_64/pci_scanner.rs
//
// Phase 0 — PCI Configuration Space Scanner
// Enumerates all PCI devices via port I/O (CONFIG_ADDRESS=0xCF8, CONFIG_DATA=0xCFC).
// Designed for freestanding x86_64; no std, no alloc, no external crates.
//
// Export surface (C-callable):
//   pci_scan_devices(devices: *mut PciDevice, max: usize) -> usize
//   pci_read_config(bus: u8, slot: u8, func: u8, offset: u8) -> u32

#![no_std]
#![allow(clippy::missing_safety_doc)]

// ── PCI I/O port constants ───────────────────────────────────────────────────

const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

// PCI config-space register offsets (byte offsets, must be DWORD-aligned reads)
const PCI_VENDOR_ID_OFF: u8 = 0x00; // [15:0]  vendor, [31:16] device
const PCI_CLASS_CODE_OFF: u8 = 0x08; // [31:24] class, [23:16] subclass
const PCI_BAR0_OFF: u8 = 0x10;
const PCI_BAR1_OFF: u8 = 0x14;
const PCI_BAR2_OFF: u8 = 0x18;
const PCI_BAR3_OFF: u8 = 0x1C;
const PCI_BAR4_OFF: u8 = 0x20;
const PCI_BAR5_OFF: u8 = 0x24;
const PCI_HEADER_TYPE_OFF: u8 = 0x0C; // bit 7 = multi-function flag

const INVALID_VENDOR: u16 = 0xFFFF;

// COM1 serial port (for debug output)
const COM1_DATA: u16 = 0x3F8;
const COM1_LSR: u16 = 0x3FD; // Line Status Register
const COM1_THR_EMPTY: u8 = 0x20; // Transmit-holding-register empty

// ── Packed device descriptor ─────────────────────────────────────────────────

/// Mirrors the C struct `pci_device_t` in pci_scanner.h.
/// Layout is #[repr(C, packed)] so the pointer can be cast directly in C.
#[repr(C, packed)]
pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    /// Padding so the struct is naturally aligned to u32 (not strictly required
    /// by packed, but keeps sizeof == 32 which is convenient for C callers).
    pub _pad: u16,
    pub bar: [u32; 6],
}

// ── Raw port I/O helpers ──────────────────────────────────────────────────────

/// Write a 32-bit value to an I/O port.
#[inline(always)]
unsafe fn outl(port: u16, val: u32) {
    core::arch::asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") val,
        options(nostack, nomem, preserves_flags)
    );
}

/// Read a 32-bit value from an I/O port.
#[inline(always)]
unsafe fn inl(port: u16) -> u32 {
    let val: u32;
    core::arch::asm!(
        "in eax, dx",
        out("eax") val,
        in("dx") port,
        options(nostack, nomem, preserves_flags)
    );
    val
}

/// Write a single byte to an I/O port.
#[inline(always)]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nostack, nomem, preserves_flags)
    );
}

/// Read a single byte from an I/O port.
#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") val,
        in("dx") port,
        options(nostack, nomem, preserves_flags)
    );
    val
}

// ── Serial debug helpers ──────────────────────────────────────────────────────

/// Spin-wait until the COM1 transmit-holding register is empty, then send one byte.
unsafe fn serial_putc(c: u8) {
    // Poll Line Status Register until THR empty
    while (inb(COM1_LSR) & COM1_THR_EMPTY) == 0 {}
    outb(COM1_DATA, c);
}

/// Write a NUL-terminated byte slice to COM1.
unsafe fn serial_puts(s: &[u8]) {
    for &b in s {
        if b == 0 {
            break;
        }
        serial_putc(b);
    }
}

/// Write a u8 as two hex nibbles to COM1.
unsafe fn serial_hex8(val: u8) {
    const HEX: &[u8] = b"0123456789ABCDEF";
    serial_putc(HEX[((val >> 4) & 0xF) as usize]);
    serial_putc(HEX[(val & 0xF) as usize]);
}

/// Write a u16 as four hex nibbles to COM1.
unsafe fn serial_hex16(val: u16) {
    serial_hex8((val >> 8) as u8);
    serial_hex8(val as u8);
}

/// Write a u32 as eight hex nibbles to COM1.
unsafe fn serial_hex32(val: u32) {
    serial_hex16((val >> 16) as u16);
    serial_hex16(val as u16);
}

// ── PCI configuration space access ───────────────────────────────────────────

/// Build the 32-bit CONFIG_ADDRESS value for a given (bus, slot, func, offset).
///
/// Bit layout per PCI spec §3.2.2.3.2:
///   [31]    = Enable bit (must be 1)
///   [30:24] = Reserved (0)
///   [23:16] = Bus number
///   [15:11] = Device (slot) number
///   [10:8]  = Function number
///   [7:2]   = Register number (offset >> 2)
///   [1:0]   = 0 (DWORD aligned)
#[inline]
fn pci_config_address(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    0x8000_0000u32
        | ((bus as u32) << 16)
        | (((slot as u32) & 0x1F) << 11)
        | (((func as u32) & 0x07) << 8)
        | ((offset as u32) & 0xFC) // mask off low 2 bits
}

/// Read a DWORD from PCI configuration space.
/// This is the fundamental primitive — callers extract fields by masking/shifting.
///
/// # Safety
/// Performs raw I/O port operations. Must only be called from kernel context
/// with CPL 0. `offset` must be DWORD-aligned (low 2 bits ignored via mask).
#[no_mangle]
pub unsafe extern "C" fn pci_read_config(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let addr = pci_config_address(bus, slot, func, offset);
    outl(PCI_CONFIG_ADDRESS, addr);
    inl(PCI_CONFIG_DATA)
}

// ── PCI device enumeration ────────────────────────────────────────────────────

/// Check if a PCI function exists by reading its vendor ID.
/// Returns `None` if the slot/function is empty (0xFFFF).
#[inline]
unsafe fn pci_vendor_device(bus: u8, slot: u8, func: u8) -> Option<(u16, u16)> {
    let dword = pci_read_config(bus, slot, func, PCI_VENDOR_ID_OFF);
    let vendor = (dword & 0xFFFF) as u16;
    if vendor == INVALID_VENDOR {
        return None;
    }
    let device = ((dword >> 16) & 0xFFFF) as u16;
    Some((vendor, device))
}

/// Extract class_code (byte 3) and subclass (byte 2) from the config DWORD at 0x08.
#[inline]
fn decode_class(dword: u32) -> (u8, u8) {
    let class_code = ((dword >> 24) & 0xFF) as u8;
    let subclass = ((dword >> 16) & 0xFF) as u8;
    (class_code, subclass)
}

/// Check the multi-function bit in the header type register.
/// If clear, only function 0 is valid for this slot.
#[inline]
unsafe fn is_multifunction(bus: u8, slot: u8) -> bool {
    let dword = pci_read_config(bus, slot, 0, PCI_HEADER_TYPE_OFF);
    let header_type = ((dword >> 16) & 0xFF) as u8;
    (header_type & 0x80) != 0
}

/// Populate a `PciDevice` entry for the given (bus, slot, func).
/// The vendor/device IDs have already been validated by the caller.
#[inline]
unsafe fn populate_device(
    dev: &mut PciDevice,
    bus: u8,
    slot: u8,
    func: u8,
    vendor: u16,
    device: u16,
) {
    let class_dword = pci_read_config(bus, slot, func, PCI_CLASS_CODE_OFF);
    let (class_code, subclass) = decode_class(class_dword);

    dev.vendor_id = vendor;
    dev.device_id = device;
    dev.class_code = class_code;
    dev.subclass = subclass;
    dev._pad = 0;
    dev.bar[0] = pci_read_config(bus, slot, func, PCI_BAR0_OFF);
    dev.bar[1] = pci_read_config(bus, slot, func, PCI_BAR1_OFF);
    dev.bar[2] = pci_read_config(bus, slot, func, PCI_BAR2_OFF);
    dev.bar[3] = pci_read_config(bus, slot, func, PCI_BAR3_OFF);
    dev.bar[4] = pci_read_config(bus, slot, func, PCI_BAR4_OFF);
    dev.bar[5] = pci_read_config(bus, slot, func, PCI_BAR5_OFF);
}

/// Scan the entire PCI topology (bus 0–255, slot 0–31, func 0–7).
///
/// # Parameters
/// - `devices` — caller-allocated array of at least `max` `PciDevice` entries.
/// - `max`     — capacity of the `devices` array.
///
/// # Returns
/// Number of devices discovered (never exceeds `max`).
///
/// # Safety
/// - `devices` must point to a valid, writable buffer of at least `max` elements.
/// - Must be called from Ring 0 (port I/O privilege).
#[no_mangle]
pub unsafe extern "C" fn pci_scan_devices(devices: *mut PciDevice, max: usize) -> usize {
    if devices.is_null() || max == 0 {
        serial_puts(b"[PCI] pci_scan_devices: null buf or zero max\n\0");
        return 0;
    }

    serial_puts(b"[PCI] scanning buses 0-255...\n\0");

    let mut count: usize = 0;

    // Outer loops: bus 0..=255
    let mut bus: u16 = 0;
    loop {
        // Inner: slot 0..=31
        let mut slot: u8 = 0;
        loop {
            // Probe function 0 first; skip slot if absent
            let (vendor0, device0) = match pci_vendor_device(bus as u8, slot, 0) {
                Some(v) => v,
                None => {
                    slot = slot.wrapping_add(1);
                    if slot >= 32 { break; }
                    continue;
                }
            };

            // Function 0 is always valid at this point
            if count < max {
                let entry = &mut *devices.add(count);
                populate_device(entry, bus as u8, slot, 0, vendor0, device0);

                // Serial debug: "[PCI] BUS:xx SLOT:xx F:0 VID:xxxx DID:xxxx\n"
                serial_puts(b"[PCI] BUS:\0");
                serial_hex8(bus as u8);
                serial_puts(b" SLOT:\0");
                serial_hex8(slot);
                serial_puts(b" F:0 VID:\0");
                serial_hex16(vendor0);
                serial_puts(b" DID:\0");
                serial_hex16(device0);
                serial_puts(b"\n\0");

                count += 1;
            }

            // Check multi-function flag; scan functions 1–7 if set
            if is_multifunction(bus as u8, slot) {
                let mut func: u8 = 1;
                loop {
                    if let Some((vendor, device)) = pci_vendor_device(bus as u8, slot, func) {
                        if count < max {
                            let entry = &mut *devices.add(count);
                            populate_device(entry, bus as u8, slot, func, vendor, device);

                            serial_puts(b"[PCI] BUS:\0");
                            serial_hex8(bus as u8);
                            serial_puts(b" SLOT:\0");
                            serial_hex8(slot);
                            serial_puts(b" F:\0");
                            serial_hex8(func);
                            serial_puts(b" VID:\0");
                            serial_hex16(vendor);
                            serial_puts(b" DID:\0");
                            serial_hex16(device);
                            serial_puts(b"\n\0");

                            count += 1;
                        }
                    }
                    func += 1;
                    if func >= 8 { break; }
                }
            }

            // Exit early if output buffer is full
            if count >= max {
                serial_puts(b"[PCI] device buffer full, stopping scan\n\0");
                return count;
            }

            slot = slot.wrapping_add(1);
            if slot >= 32 { break; }
        }

        bus += 1;
        if bus > 255 { break; }
    }

    serial_puts(b"[PCI] scan complete, count=\0");
    serial_hex8(count as u8); // works for up to 255; fine for debug
    serial_puts(b"\n\0");

    count
}

// ── Panic handler (required for no_std binary crates) ────────────────────────

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        serial_puts(b"[PCI] PANIC\n\0");
    }
    loop {
        core::hint::spin_loop();
    }
}
