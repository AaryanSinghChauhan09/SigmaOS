// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_pci.rs — PCI/PCIe bus enumeration and BAR setup
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

const PCI_CFG_ADDR: u16 = 0xCF8;
const PCI_CFG_DATA: u16 = 0xCFC;

unsafe fn pci_read32(bus: u8, dev: u8, func: u8, off: u8) -> u32 {
    let addr: u32 = (1 << 31)
        | ((bus  as u32) << 16)
        | ((dev  as u32) << 11)
        | ((func as u32) <<  8)
        | (off as u32 & 0xFC);
    let val: u32;
    core::arch::asm!(
        "out dx, eax",
        "mov dx, 0xCFC",
        "in eax, dx",
        in("dx") PCI_CFG_ADDR,
        in("eax") addr,
        lateout("eax") val,
        options(nostack)
    );
    val
}

unsafe fn pci_write32(bus: u8, dev: u8, func: u8, off: u8, val: u32) {
    let addr: u32 = (1 << 31)
        | ((bus  as u32) << 16)
        | ((dev  as u32) << 11)
        | ((func as u32) <<  8)
        | (off as u32 & 0xFC);
    core::arch::asm!(
        "out dx, eax",
        "mov dx, 0xCFC",
        "out dx, eax",
        in("dx") PCI_CFG_ADDR,
        in("eax") addr,
        options(nostack)
    );
    let _: () = {
        core::arch::asm!("out dx, eax", in("dx") PCI_CFG_DATA, in("eax") val, options(nostack));
    };
}

#[derive(Copy, Clone, Default)]
pub struct PciDevice {
    pub bus:       u8,
    pub dev:       u8,
    pub func:      u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class:     u8,
    pub subclass:  u8,
    pub prog_if:   u8,
    pub bar0:      u64,
    pub bar1:      u64,
    pub irq:       u8,
    pub valid:     bool,
}

const MAX_PCI_DEVICES: usize = 64;

pub struct PciBus {
    devices: [PciDevice; MAX_PCI_DEVICES],
    count:   usize,
}

impl PciBus {
    pub const fn new() -> Self {
        Self {
            devices: [PciDevice {
                bus:0,dev:0,func:0,vendor_id:0,device_id:0,
                class:0,subclass:0,prog_if:0,bar0:0,bar1:0,irq:0,valid:false
            }; MAX_PCI_DEVICES],
            count: 0,
        }
    }

    pub unsafe fn enumerate(&mut self) {
        for bus in 0u8..=255 {
            for dev in 0u8..32 {
                for func in 0u8..8 {
                    let id = pci_read32(bus, dev, func, 0x00);
                    let vid = (id & 0xFFFF) as u16;
                    if vid == 0xFFFF { continue; }
                    let did = ((id >> 16) & 0xFFFF) as u16;

                    let class_rev = pci_read32(bus, dev, func, 0x08);
                    let class   = ((class_rev >> 24) & 0xFF) as u8;
                    let subclass= ((class_rev >> 16) & 0xFF) as u8;
                    let prog_if = ((class_rev >>  8) & 0xFF) as u8;

                    // Read BAR0 and BAR1
                    let bar0_raw = pci_read32(bus, dev, func, 0x10);
                    let bar1_raw = pci_read32(bus, dev, func, 0x14);
                    let bar0 = if bar0_raw & 1 == 0 {
                        (bar0_raw & !0xF) as u64
                    } else { 0 };
                    let bar1 = if bar1_raw & 1 == 0 {
                        (bar1_raw & !0xF) as u64
                    } else { 0 };

                    let irq_line = (pci_read32(bus, dev, func, 0x3C) & 0xFF) as u8;

                    // Enable bus mastering + memory space
                    let cmd = pci_read32(bus, dev, func, 0x04);
                    pci_write32(bus, dev, func, 0x04, cmd | 0x06);

                    if self.count < MAX_PCI_DEVICES {
                        self.devices[self.count] = PciDevice {
                            bus, dev, func, vendor_id: vid, device_id: did,
                            class, subclass, prog_if, bar0, bar1, irq: irq_line,
                            valid: true,
                        };
                        self.count += 1;
                    }

                    // Single-function device
                    let hdr = ((pci_read32(bus, dev, func, 0x0C) >> 16) & 0x80) != 0;
                    if func == 0 && !hdr { break; }
                }
            }
        }
    }

    pub fn find(&self, vendor_id: u16, device_id: u16) -> Option<&PciDevice> {
        self.devices[..self.count].iter()
            .find(|d| d.valid && d.vendor_id == vendor_id && d.device_id == device_id)
    }

    pub fn find_class(&self, class: u8, subclass: u8) -> Option<&PciDevice> {
        self.devices[..self.count].iter()
            .find(|d| d.valid && d.class == class && d.subclass == subclass)
    }
}

static mut G_PCI: PciBus = PciBus::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_pci_enumerate() {
    G_PCI.enumerate();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pci_device_count() -> usize { G_PCI.count }

#[no_mangle]
pub unsafe extern "C" fn sigma_pci_read_config32(b: u8, d: u8, f: u8, off: u8) -> u32 {
    pci_read32(b, d, f, off)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pci_write_config32(b: u8, d: u8, f: u8, off: u8, val: u32) {
    pci_write32(b, d, f, off, val);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pci_enable(b: u8, d: u8, f: u8) -> i32 {
    let cmd = pci_read32(b, d, f, 0x04);
    pci_write32(b, d, f, 0x04, cmd | 0x06);
    0
}
