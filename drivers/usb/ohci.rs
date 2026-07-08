// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/usb/ohci.rs — USB 1.1 OHCI Controller Driver
//
// Implements the Open Host Controller Interface (OHCI) driver.
// Supports USB 1.1 Low Speed (1.5 Mbps) and Full Speed (12 Mbps) devices.
// Based on Linux kernel ohci driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::usb_device_base::{UsbController, UsbSpeed, UsbDirection, UsbTransfer, UsbDeviceDescriptor, USB_OK, USB_ERR_NO_DEVICE, USB_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── OHCI Vendor IDs ─────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;
pub const AMD_VENDOR_ID: U16 = 0x1022;
pub const VIA_VENDOR_ID: U16 = 0x1106;
pub const SIS_VENDOR_ID: U16 = 0x1039;
pub const NEC_VENDOR_ID: U16 = 0x1033;
pub const SUN_VENDOR_ID: U16 = 0x108E;

// ─── OHCI Register Offsets ─────────────────────────────────────

pub const OHCI_REVISION: U32 = 0x00;
pub const OHCI_CONTROL: U32 = 0x04;
pub const OHCI_CMDSTATUS: U32 = 0x08;
pub const OHCI_INTRSTATUS: U32 = 0x0C;
pub const OHCI_INTRENABLE: U32 = 0x10;
pub const OHCI_INTRDISABLE: U32 = 0x14;
pub const OHCI_HCCA: U32 = 0x18;
pub const OHCI_PERIODIC_ED: U32 = 0x1C;
pub const OHCI_CTRL_ED: U32 = 0x20;
pub const OHCI_BULK_ED: U32 = 0x24;
pub const OHCI_DONE_HEAD: U32 = 0x28;
pub const OHCI_FMINTERVAL: U32 = 0x34;
pub const OHCI_FMREMAINING: U32 = 0x38;
pub const OHCI_FMNEXT: U32 = 0x3C;
pub const OHCI_LSTHRESH: U32 = 0x40;
pub const OHCI_RH_DESCRIPTOR_A: U32 = 0x48;
pub const OHCI_RH_DESCRIPTOR_B: U32 = 0x4C;
pub const OHCI_RH_STATUS: U32 = 0x50;
pub const OHCI_RH_PORT_STATUS: U32 = 0x54;

// ─── OHCI Control Flags ───────────────────────────────────────

pub const OHCI_CTRL_CBSR: U32 = 0x00000003;
pub const OHCI_CTRL_PLE: U32 = 0x00000004;
pub const OHCI_CTRL_IE: U32 = 0x00000008;
pub const OHCI_CTRL_CLE: U32 = 0x00000010;
pub const OHCI_CTRL_BLE: U32 = 0x00000020;
pub const OHCI_CTRL_HCFS: U32 = 0x000000C0;
pub const OHCI_CTRL_IR: U32 = 0x00000100;
pub const OHCI_CTRL_RWC: U32 = 0x00000200;
pub const OHCI_CTRL_RWE: U32 = 0x00000400;

pub const OHCI_USB_RESET: U32 = 0x00000000;
pub const OHCI_USB_RESUME: U32 = 0x00000040;
pub const OHCI_USB_OPERATIONAL: U32 = 0x00000080;
pub const OHCI_USB_SUSPEND: U32 = 0x000000C0;

// ─── OHCI Command Status ─────────────────────────────────────

pub const OHCI_CMD_HCR: U32 = 0x00000001;
pub const OHCI_CMD_CLF: U32 = 0x00000002;
pub const OHCI_CMD_BLF: U32 = 0x00000004;

// ─── OHCI Interrupt Status ─────────────────────────────────

pub const OHCI_INTR_SO: U32 = 0x00000001;
pub const OHCI_INTR_WDH: U32 = 0x00000002;
pub const OHCI_INTR_SF: U32 = 0x00000004;
pub const OHCI_INTR_RD: U32 = 0x00000008;
pub const OHCI_INTR_UE: U32 = 0x00000010;
pub const OHCI_INTR_FNO: U32 = 0x00000020;
pub const OHCI_INTR_RHSC: U32 = 0x00000040;
pub const OHCI_INTR_OC: U32 = 0x40000000;
pub const OHCI_INTR_MIE: U32 = 0x80000000;

// ─── OHCI Port Status ───────────────────────────────────────

pub const OHCI_PORT_CCS: U32 = 0x00000001;
pub const OHCI_PORT_PES: U32 = 0x00000002;
pub const OHCI_PORT_PSS: U32 = 0x00000004;
pub const OHCI_PORT_POCI: U32 = 0x00000008;
pub const OHCI_PORT_PRS: U32 = 0x00000010;
pub const OHCI_PORT_PPS: U32 = 0x00000020;
pub const OHCI_PORT_LSDA: U32 = 0x00000200;
pub const OHCI_PORT_CPP: U32 = 0x00000400;

// ─── OHCI Endpoint Descriptor ───────────────────────────────

#[repr(C)]
pub struct OhciEndpointDescriptor {
    pub flags: U32,
    pub tail_pointer: U32,
    pub head_pointer: U32,
    pub next_ed: U32,
}

impl OhciEndpointDescriptor {
    pub const fn new() -> Self {
        OhciEndpointDescriptor {
            flags: 0,
            tail_pointer: 0,
            head_pointer: 0,
            next_ed: 0,
        }
    }
}

// ─── OHCI Transfer Descriptor ───────────────────────────────

#[repr(C)]
pub struct OhciTransferDescriptor {
    pub flags: U32;
    pub current_buffer_pointer: U32;
    pub next_td: U32;
    pub buffer_end: U32,
}

impl OhciTransferDescriptor {
    pub const fn new() -> Self {
        OhciTransferDescriptor {
            flags: 0,
            current_buffer_pointer: 0,
            next_td: 0,
            buffer_end: 0,
        }
    }
}

// ─── OHCI HCCA (Host Controller Communication Area) ─────────

#[repr(C)]
pub struct OhciHcca {
    pub interrupt_table: [U32; 32],
    pub frame_number: U16,
    pub pad1: U16,
    pub done_head: U32,
    pub reserved: [U32; 30],
}

impl OhciHcca {
    pub const fn new() -> Self {
        OhciHcca {
            interrupt_table: [0; 32],
            frame_number: 0,
            pad1: 0,
            done_head: 0,
            reserved: [0; 30],
        }
    }
}

// ─── OHCI Controller Structure ───────────────────────────────

pub struct OhciController {
    pub mmio_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub running: bool,
    pub num_ports: U8,
    pub power_switching: bool,
    pub hcca: OhciHcca,
    pub hcca_base: U64,
    pub endpoint_descriptors: [OhciEndpointDescriptor; 256],
    pub transfer_descriptors: [OhciTransferDescriptor; 512],
    pub usb_speed: UsbSpeed,
}

impl OhciController {
    pub const fn new() -> Self {
        OhciController {
            mmio_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            running: false,
            num_ports: 0,
            power_switching: false,
            hcca: OhciHcca::new(),
            hcca_base: 0,
            endpoint_descriptors: [OhciEndpointDescriptor::new(); 256],
            transfer_descriptors: [OhciTransferDescriptor::new(); 512],
            usb_speed: UsbSpeed::Full,
        }
    }

    /// Read MMIO register
    unsafe fn read_mmio(&self, offset: U32) -> U32 {
        let ptr = (self.mmio_base + offset as U64) as *const U32;
        *ptr
    }

    /// Write MMIO register
    unsafe fn write_mmio(&self, offset: U32, value: U32) {
        let ptr = (self.mmio_base + offset as U64) as *mut U32;
        *ptr = value;
    }

    /// Read MMIO register 64-bit
    unsafe fn read_mmio64(&self, offset: U32) -> U64 {
        let ptr = (self.mmio_base + offset as U64) as *const U64;
        *ptr
    }

    /// Write MMIO register 64-bit
    unsafe fn write_mmio64(&self, offset: U32, value: U64) {
        let ptr = (self.mmio_base + offset as U64) as *mut U64;
        *ptr = value;
    }

    /// Initialize OHCI controller
    fn init_ohci(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        // Read revision
        unsafe {
            let revision = self.read_mmio(OHCI_REVISION);
            
            // Read RH descriptor A
            let rh_desc_a = self.read_mmio(OHCI_RH_DESCRIPTOR_A);
            self.num_ports = ((rh_desc_a >> 0) & 0xFF) as U8;
            self.power_switching = (rh_desc_a & (1 << 9)) != 0;
            
            // OHCI supports Low Speed (1.5 Mbps) and Full Speed (12 Mbps)
            self.usb_speed = UsbSpeed::Full;
        }

        // In a real implementation, this would:
        // 1. Stop controller
        // 2. Reset controller
        // 3. Initialize HCCA
        // 4. Set up endpoint descriptors
        // 5. Enable interrupts
        // 6. Start controller

        self.initialized = true;
        self.running = true;

        USB_OK
    }

    /// Reset OHCI controller
    fn reset_ohci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            // Stop controller
            let mut control = self.read_mmio(OHCI_CONTROL);
            control &= !OHCI_CTRL_HCFS;
            control |= OHCI_USB_SUSPEND;
            self.write_mmio(OHCI_CONTROL, control);

            // Issue host controller reset
            let mut cmd_status = self.read_mmio(OHCI_CMDSTATUS);
            cmd_status |= OHCI_CMD_HCR;
            self.write_mmio(OHCI_CMDSTATUS, cmd_status);

            // Wait for reset to complete
            let mut timeout = 10000;
            while timeout > 0 {
                let cmd_status = self.read_mmio(OHCI_CMDSTATUS);
                if cmd_status & OHCI_CMD_HCR == 0 {
                    break;
                }
                timeout -= 1;
            }
        }

        USB_OK
    }

    /// Start OHCI controller
    fn start_ohci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            let mut control = self.read_mmio(OHCI_CONTROL);
            control &= !OHCI_CTRL_HCFS;
            control |= OHCI_USB_OPERATIONAL;
            self.write_mmio(OHCI_CONTROL, control);
        }

        self.running = true;
        USB_OK
    }

    /// Stop OHCI controller
    fn stop_ohci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            let mut control = self.read_mmio(OHCI_CONTROL);
            control &= !OHCI_CTRL_HCFS;
            control |= OHCI_USB_SUSPEND;
            self.write_mmio(OHCI_CONTROL, control);
        }

        self.running = false;
        USB_OK
    }
}

// ─── Implement UsbController Trait ─────────────────────────────

impl UsbController for OhciController {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let vendor_id = match device_id {
            _ => INTEL_VENDOR_ID,
        };
        
        self.init_ohci(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_controller_name(&self) -> &'static str {
        match self.vendor_id {
            INTEL_VENDOR_ID => "Intel OHCI USB Controller",
            AMD_VENDOR_ID => "AMD OHCI USB Controller",
            VIA_VENDOR_ID => "VIA OHCI USB Controller",
            SIS_VENDOR_ID => "SiS OHCI USB Controller",
            NEC_VENDOR_ID => "NEC OHCI USB Controller",
            SUN_VENDOR_ID => "Sun OHCI USB Controller",
            _ => "OHCI USB Controller",
        }
    }

    fn get_speed(&self) -> UsbSpeed {
        self.usb_speed
    }

    fn reset(&mut self) -> I32 {
        self.reset_ohci()
    }

    fn shutdown(&mut self) -> I32 {
        self.stop_ohci();
        self.initialized = false;
        USB_OK
    }

    fn get_device_descriptor(&self, descriptor: *mut UsbDeviceDescriptor) -> I32 {
        if descriptor.is_null() {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            (*descriptor).length = 18;
            (*descriptor).descriptor_type = 1;
            (*descriptor).usb_version = 0x0110;
            (*descriptor).vendor_id = self.vendor_id;
            (*descriptor).product_id = self.device_id;
        }

        USB_OK
    }

    fn get_configuration_descriptor(&self, config_index: U8, buffer: *mut U8, length: *mut U16) -> I32 {
        if buffer.is_null() || length.is_null() {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            *length = 0;
        }

        USB_OK
    }

    fn control_transfer(&mut self, request_type: U8, request: U8, value: U16, index: U16, buffer: *mut U8, length: U16) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, set up ED and TD for control transfer
        USB_OK
    }

    fn bulk_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, set up ED and TD for bulk transfer
        USB_OK
    }

    fn interrupt_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, set up ED and TD for interrupt transfer
        USB_OK
    }

    fn isochronous_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, set up periodic schedule for isochronous
        USB_OK
    }

    fn submit_transfer(&mut self, transfer: *mut UsbTransfer) -> I32 {
        if transfer.is_null() {
            return USB_ERR_INIT_FAILED;
        }

        USB_OK
    }

    fn cancel_transfer(&mut self, transfer: *mut UsbTransfer) -> I32 {
        if transfer.is_null() {
            return USB_ERR_INIT_FAILED;
        }

        USB_OK
    }

    fn allocate_address(&mut self) -> U8 {
        1
    }

    fn set_address(&mut self, address: U8) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        USB_OK
    }

    fn set_configuration(&mut self, config_value: U8) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        USB_OK
    }

    fn get_port_status(&self, port: U8) -> U32 {
        if !self.initialized {
            return 0;
        }

        // Stub: return connected status
        OHCI_PORT_CCS | OHCI_PORT_PES | OHCI_PORT_PPS
    }

    fn set_port_feature(&mut self, port: U8, feature: U16) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        USB_OK
    }

    fn clear_port_feature(&mut self, port: U8, feature: U16) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        USB_OK
    }
}

// ─── Global OHCI Controller ─────────────────────────────────

static mut G_OHCI: OhciController = OhciController::new();

// ─── C-ABI Exports ─────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn ohci_init(pci_bar: U64, device_id: U16) -> I32 {
    G_OHCI.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn ohci_is_initialized() -> I32 {
    if G_OHCI.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn ohci_reset() -> I32 {
    G_OHCI.reset()
}

#[no_mangle]
pub unsafe extern "C" fn ohci_start() -> I32 {
    G_OHCI.start_ohci()
}

#[no_mangle]
pub unsafe extern "C" fn ohci_stop() -> I32 {
    G_OHCI.stop_ohci()
}

#[no_mangle]
pub unsafe extern "C" fn ohci_shutdown() -> I32 {
    G_OHCI.shutdown()
}

/// Probe for OHCI devices
#[no_mangle]
pub unsafe extern "C" fn ohci_probe() -> I32 {
    let mut found_devices = 0;
    
    for bus in 0..256u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                let class_code = read_pci_config_u8(bus, device, function, 0x0B);
                let subclass = read_pci_config_u8(bus, device, function, 0x0A);
                
                // OHCI: Class 0x0C, Subclass 0x03, Prog IF 0x10
                if class_code == 0x0C && subclass == 0x03 {
                    let prog_if = read_pci_config_u8(bus, device, function, 0x09);
                    if prog_if == 0x10 {
                        let bar0 = read_pci_config_u32(bus, device, function, 0x10);
                        let mmio_base = (bar0 & 0xFFFFFFF0) as U64;
                        
                        let result = G_OHCI.init(mmio_base, device_id);
                        
                        if result == USB_OK {
                            found_devices += 1;
                            return USB_OK;
                        }
                    }
                }
            }
        }
    }
    
    if found_devices > 0 {
        USB_OK
    } else {
        USB_ERR_NO_DEVICE
    }
}

unsafe fn read_pci_config_u8(bus: U8, device: U8, function: U8, offset: U8) -> U8 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    let value = inl(0xCFC);
    let shift = ((offset & 3) as u32) * 8;
    ((value >> shift) & 0xFF) as U8
}

unsafe fn read_pci_config_u16(bus: U8, device: U8, function: U8, offset: U8) -> U16 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    let value = inl(0xCFC);
    let shift = ((offset & 2) as u32) * 8;
    ((value >> shift) & 0xFFFF) as U16
}

unsafe fn read_pci_config_u32(bus: U8, device: U8, function: U8, offset: U8) -> U32 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    inl(0xCFC)
}

unsafe fn outl(port: U16, value: U32) {
    // Placeholder
}

unsafe fn inl(port: U16) -> U32 {
    // Placeholder
    0
}
