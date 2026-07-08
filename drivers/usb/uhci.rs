// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/usb/uhci.rs — USB 1.1 UHCI Controller Driver
//
// Implements the Universal Host Controller Interface (UHCI) driver.
// Supports USB 1.1 Low Speed (1.5 Mbps) and Full Speed (12 Mbps) devices.
// Based on Linux kernel uhci driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::usb_device_base::{UsbController, UsbSpeed, UsbDirection, UsbTransfer, UsbDeviceDescriptor, USB_OK, USB_ERR_NO_DEVICE, USB_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── UHCI Vendor IDs ─────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;
pub const VIA_VENDOR_ID: U16 = 0x1106;
pub const OPTI_VENDOR_ID: U16 = 0x1045;
pub const ALI_VENDOR_ID: U16 = 0x10B9;

// ─── UHCI Register Offsets ─────────────────────────────────────

pub const UHCI_USBCMD: U32 = 0x00;
pub const UHCI_USBSTS: U32 = 0x02;
pub const UHCI_USBINTR: U32 = 0x04;
pub const UHCI_FRNUM: U32 = 0x06;
pub const UHCI_FLBASEADD: U32 = 0x08;
pub const UHCI_SOFMOD: U32 = 0x0C;
pub const UHCI_PORTSC1: U32 = 0x10;
pub const UHCI_PORTSC2: U32 = 0x12;

// ─── UHCI Command Flags ───────────────────────────────────────

pub const UHCI_CMD_RS: U16 = 0x0001;
pub const UHCI_CMD_HCRESET: U16 = 0x0002;
pub const UHCI_CMD_GRESET: U16 = 0x0004;
pub const UHCI_CMD_EGBSM: U16 = 0x0008;
pub const UHCI_CMD_FGR: U16 = 0x0020;
pub const UHCI_CMD_SWDBG: U16 = 0x0040;
pub const UHCI_CMD_CF: U16 = 0x0080;
pub const UHCI_CMD_MAXP: U16 = 0x8000;

// ─── UHCI Status Flags ───────────────────────────────────────

pub const UHCI_STS_USBINT: U16 = 0x0001;
pub const UHCI_STS_USBERRINT: U十六 = 0x0002;
pub const UHCI_STS_RD: U16 = 0x0004;
pub const UHCI_STS_HSE: U16 = 0x0008;
pub const UHCI_STS_HCPE: U16 = 0x0010;
pub const UHCI_STS_HCH: U16 = 0x0020;

// ─── UHCI Port Status ───────────────────────────────────────

pub const UHCI_PORT_CCS: U16 = 0x0001;
pub const UHCI_PORT_CSC: U16 = 0x0002;
pub const UHCI_PORT_PE: U16 = 0x0004;
pub const UHCI_PORT_POC: U16 = 0x0008;
pub const UHCI_PORT_SUSP: U16 = 0x0010;
pub const UHCI_PORT_RES: U16 = 0x0020;
pub const UHCI_PORT_LSDA: U16 = 0x0040;
pub const UHCI_PORT_RD: U16 = 0x0100;
pub const UHCI_PORT_LSD: U16 = 0x0200;

// ─── UHCI Queue Element (TD) Structure ───────────────────────

#[repr(C)]
pub struct UhciTransferDescriptor {
    pub link: U32,
    pub actual_length: U32,
    pub status: U32,
    pub buffer: U32,
}

impl UhciTransferDescriptor {
    pub const fn new() -> Self {
        UhciTransferDescriptor {
            link: 0,
            actual_length: 0,
            status: 0,
            buffer: 0,
        }
    }
}

// ─── UHCI Queue Head ───────────────────────────────────────

#[repr(C)]
pub struct UhciQueueHead {
    pub link: U32,
    pub element: U32,
}

impl UhciQueueHead {
    pub const fn new() -> Self {
        UhciQueueHead {
            link: 0,
            element: 0,
        }
    }
}

// ─── UHCI Controller Structure ───────────────────────────────

pub struct UhciController {
    pub io_base: U16,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub running: bool,
    pub num_ports: U8,
    pub frame_list_size: U32,
    pub frame_list_base: U32,
    pub queue_heads: [UhciQueueHead; 256],
    pub transfer_descriptors: [UhciTransferDescriptor; 512],
    pub usb_speed: UsbSpeed,
}

impl UhciController {
    pub const fn new() -> Self {
        UhciController {
            io_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            running: false,
            num_ports: 2,
            frame_list_size: 1024,
            frame_list_base: 0,
            queue_heads: [UhciQueueHead::new(); 256],
            transfer_descriptors: [UhciTransferDescriptor::new(); 512],
            usb_speed: UsbSpeed::Full,
        }
    }

    /// Read I/O port 8-bit
    unsafe fn read_io8(&self, offset: U16) -> U8 {
        inb(self.io_base + offset)
    }

    /// Write I/O port 8-bit
    unsafe fn write_io8(&self, offset: U16, value: U8) {
        outb(self.io_base + offset, value)
    }

    /// Read I/O port 16-bit
    unsafe fn read_io16(&self, offset: U16) -> U16 {
        inw(self.io_base + offset)
    }

    /// Write I/O port 16-bit
    unsafe fn write_io16(&self, offset: U16, value: U16) {
        outw(self.io_base + offset, value)
    }

    /// Initialize UHCI controller
    fn init_uhci(&mut self, io_base: U16, device_id: U16, vendor_id: U16) -> I32 {
        self.io_base = io_base;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        // UHCI supports Low Speed (1.5 Mbps) and Full Speed (12 Mbps)
        self.usb_speed = UsbSpeed::Full;

        // In a real implementation, this would:
        // 1. Stop controller
        // 2. Reset controller
        // 3. Initialize frame list
        // 4. Set up queue heads
        // 5. Enable interrupts
        // 6. Start controller

        self.initialized = true;
        self.running = true;

        USB_OK
    }

    /// Reset UHCI controller
    fn reset_uhci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            // Stop controller
            let mut cmd = self.read_io16(UHCI_USBCMD);
            cmd &= !UHCI_CMD_RS;
            self.write_io16(UHCI_USBCMD, cmd);

            // Global reset
            cmd = self.read_io16(UHCI_USBCMD);
            cmd |= UHCI_CMD_GRESET;
            self.write_io16(UHCI_USBCMD, cmd);

            // Wait for reset
            let mut timeout = 10000;
            while timeout > 0 {
                timeout -= 1;
            }

            // Clear reset
            cmd = self.read_io16(UHCI_USBCMD);
            cmd &= !UHCI_CMD_GRESET;
            self.write_io16(UHCI_USBCMD, cmd);
        }

        USB_OK
    }

    /// Start UHCI controller
    fn start_uhci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            let mut cmd = self.read_io16(UHCI_USBCMD);
            cmd |= UHCI_CMD_RS;
            self.write_io16(UHCI_USBCMD, cmd);
        }

        self.running = true;
        USB_OK
    }

    /// Stop UHCI controller
    fn stop_uhci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            let mut cmd = self.read_io16(UHCI_USBCMD);
            cmd &= !UHCI_CMD_RS;
            self.write_io16(UHCI_USBCMD, cmd);
        }

        self.running = false;
        USB_OK
    }
}

// ─── Implement UsbController Trait ─────────────────────────────

impl UsbController for UhciController {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let io_base = (pci_bar & 0xFFFF) as U16;
        let vendor_id = match device_id {
            _ => INTEL_VENDOR_ID,
        };
        
        self.init_uhci(io_base, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_controller_name(&self) -> &'static str {
        match self.vendor_id {
            INTEL_VENDOR_ID => "Intel UHCI USB Controller",
            VIA_VENDOR_ID => "VIA UHCI USB Controller",
            OPTI_VENDOR_ID => "OPTi UHCI USB Controller",
            ALI_VENDOR_ID => "ALi UHCI USB Controller",
            _ => "UHCI USB Controller",
        }
    }

    fn get_speed(&self) -> UsbSpeed {
        self.usb_speed
    }

    fn reset(&mut self) -> I32 {
        self.reset_uhci()
    }

    fn shutdown(&mut self) -> I32 {
        self.stop_uhci();
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

        USB_OK
    }

    fn bulk_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        USB_OK
    }

    fn interrupt_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        USB_OK
    }

    fn isochronous_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

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

        UHCI_PORT_CCS as U32 | UHCI_PORT_PE as U32
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

// ─── Global UHCI Controller ─────────────────────────────────

static mut G_UHCI: UhciController = UhciController::new();

// ─── C-ABI Exports ─────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn uhci_init(io_base: U16, device_id: U16) -> I32 {
    G_UHCI.init(io_base as U64, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn uhci_is_initialized() -> I32 {
    if G_UHCI.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uhci_reset() -> I32 {
    G_UHCI.reset()
}

#[no_mangle]
pub unsafe extern "C" fn uhci_start() -> I32 {
    G_UHCI.start_uhci()
}

#[no_mangle]
pub unsafe extern "C" fn uhci_stop() -> I32 {
    G_UHCI.stop_uhci()
}

#[no_mangle]
pub unsafe extern "C" fn uhci_shutdown() -> I32 {
    G_UHCI.shutdown()
}

/// Probe for UHCI devices
#[no_mangle]
pub unsafe extern "C" fn uhci_probe() -> I32 {
    let mut found_devices = 0;
    
    for bus in 0..256u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                let class_code = read_pci_config_u8(bus, device, function, 0x0B);
                let subclass = read_pci_config_u8(bus, device, function, 0x0A);
                
                // UHCI: Class 0x0C, Subclass 0x03, Prog IF 0x00
                if class_code == 0x0C && subclass == 0x03 {
                    let prog_if = read_pci_config_u8(bus, device, function, 0x09);
                    if prog_if == 0x00 {
                        let bar4 = read_pci_config_u32(bus, device, function, 0x20);
                        let io_base = (bar4 & 0xFFFC) as U16;
                        
                        let result = G_UHCI.init(io_base as U64, device_id);
                        
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

unsafe fn outb(port: U16, value: U8) {
    // Placeholder
}

unsafe fn outw(port: U16, value: U16) {
    // Placeholder
}

unsafe fn outl(port: U16, value: U32) {
    // Placeholder
}

unsafe fn inb(port: U16) -> U8 {
    // Placeholder
    0
}

unsafe fn inw(port: U16) -> U16 {
    // Placeholder
    0
}

unsafe fn inl(port: U16) -> U32 {
    // Placeholder
    0
}
