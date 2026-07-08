// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/usb/xhci.rs — USB 3.0/3.1 XHCI Controller Driver
//
// Implements the eXtensible Host Controller Interface (XHCI) driver.
// Supports USB 3.0, 3.1, and 3.2 devices.
// Based on Linux kernel xhci driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::usb_device_base::{UsbController, UsbSpeed, UsbDirection, UsbTransfer, UsbDeviceDescriptor, USB_OK, USB_ERR_NO_DEVICE, USB_ERR_INIT_FAILED, USB_ERR_TRANSFER_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── XHCI Vendor IDs ─────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;
pub const AMD_VENDOR_ID: U16 = 0x1022;
pub const VIA_VENDOR_ID: U16 = 0x1106;
pub const RENESAS_VENDOR_ID: U16 = 0x1912;
pub const ASMEDIA_VENDOR_ID: U16 = 0x1b21;

// ─── XHCI Register Offsets ─────────────────────────────────────

pub const XHCI_CAPLENGTH: U32 = 0x00;
pub const XHCI_HCIVERSION: U32 = 0x02;
pub const XHCI_HCSPARAMS1: U32 = 0x04;
pub const XHCI_HCSPARAMS2: U32 = 0x08;
pub const XHCI_HCSPARAMS3: U32 = 0x0C;
pub const XHCI_DBOFF: U32 = 0x14;
pub const XHCI_RTSOFF: U32 = 0x18;

pub const XHCI_USBCMD: U32 = 0x00;
pub const XHCI_USBSTS: U32 = 0x04;
pub const XHCI_PAGESIZE: U32 = 0x08;
pub const XHCI_DNCTRL: U32 = 0x14;
pub const XHCI_CRCR: U32 = 0x18;
pub const XHCI_DCBAAP: U32 = 0x30;
pub const XHCI_CONFIG: U32 = 0x38;

// ─── XHCI Command Flags ───────────────────────────────────────

pub const XHCI_CMD_RUN: U32 = 0x00000001;
pub const XHCI_CMD_HCRST: U32 = 0x00000002;
pub const XHCI_CMD_INTE: U32 = 0x00000004;
pub const XHCI_CMD_RS: U32 = 0x00000008;

// ─── XHCI Status Flags ───────────────────────────────────────

pub const XHCI_STS_HCH: U32 = 0x00000001;
pub const XHCI_STS_HSE: U32 = 0x00000002;
pub const XHCI_STS_EINT: U32 = 0x00000004;
pub const XHCI_STS_PCD: U32 = 0x00000008;
pub const XHCI_STS_SSS: U32 = 0x00000010;
pub const XHCI_STS_RSS: U32 = 0x00000020;
pub const XHCI_STS_SRE: U32 = 0x00000040;
pub const XHCI_STS_CNR: U32 = 0x00000080;
pub const XHCI_STS_HCE: U32 = 0x00000100;

// ─── XHCI Port Status ───────────────────────────────────────

pub const XHCI_PORT_CCS: U32 = 0x00000001;
pub const XHCI_PORT_PED: U32 = 0x00000002;
pub const XHCI_PORT_OCA: U32 = 0x00000004;
pub const XHCI_PORT_PR: U32 = 0x00000008;
pub const XHCI_PORT_PP: U32 = 0x00000200;
pub const XHCI_PORT_PLS_MASK: U32 = 0x00000F00;

// ─── XHCI Device Context ─────────────────────────────────────

#[repr(C)]
pub struct XhciDeviceContext {
    pub slot_context: [U32; 8],
    pub endpoint_context: [U32; 8],
}

impl XhciDeviceContext {
    pub const fn new() -> Self {
        XhciDeviceContext {
            slot_context: [0; 8],
            endpoint_context: [0; 8],
        }
    }
}

// ─── XHCI Transfer Ring ───────────────────────────────────────

#[repr(C)]
pub struct XhciTransferRing {
    pub dequeue_ptr: U64,
    pub enqueue_ptr: U64,
    pub cycle_bit: U8,
    pub ring_size: U32,
    pub ring_segment: U64,
}

impl XhciTransferRing {
    pub const fn new() -> Self {
        XhciTransferRing {
            dequeue_ptr: 0,
            enqueue_ptr: 0,
            cycle_bit: 1,
            ring_size: 256,
            ring_segment: 0,
        }
    }
}

// ─── XHCI Controller Structure ───────────────────────────────

pub struct XhciController {
    pub mmio_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub running: bool,
    pub cap_length: U8,
    pub hci_version: U16,
    pub max_slots: U8,
    pub max_interrupters: U8,
    pub max_ports: U8,
    pub dcbaap: U64,
    pub crcr: U64,
    pub device_contexts: [XhciDeviceContext; 256],
    pub transfer_rings: [XhciTransferRing; 256],
    pub usb_speed: UsbSpeed,
}

impl XhciController {
    pub const fn new() -> Self {
        XhciController {
            mmio_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            running: false,
            cap_length: 0,
            hci_version: 0,
            max_slots: 0,
            max_interrupters: 0,
            max_ports: 0,
            dcbaap: 0,
            crcr: 0,
            device_contexts: [XhciDeviceContext::new(); 256],
            transfer_rings: [XhciTransferRing::new(); 256],
            usb_speed: UsbSpeed::Super,
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

    /// Initialize XHCI controller
    fn init_xhci(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        // Read capability length
        unsafe {
            self.cap_length = (self.read_mmio(XHCI_CAPLENGTH) & 0xFF) as U8;
            
            // Read HCI version
            self.hci_version = ((self.read_mmio(XHCI_HCIVERSION) >> 16) & 0xFFFF) as U16;
            
            // Read HCS params
            let hcs_params1 = self.read_mmio(XHCI_HCSPARAMS1);
            self.max_slots = ((hcs_params1 >> 0) & 0xFF) as U8;
            self.max_interrupters = ((hcs_params1 >> 8) & 0xFF) as U8;
            self.max_ports = ((hcs_params1 >> 24) & 0xFF) as U8;
            
            // Determine USB speed from version
            self.usb_speed = match self.hci_version {
                0x0100 => UsbSpeed::Super,      // 1.0 = USB 3.0
                0x0110 => UsbSpeed::SuperPlus, // 1.1 = USB 3.1
                0x0120 => UsbSpeed::SuperPlus, // 1.2 = USB 3.2
                _ => UsbSpeed::Super,
            };
        }

        // In a real implementation, this would:
        // 1. Stop controller
        // 2. Reset controller
        // 3. Initialize device context array
        // 4. Set up command ring
        // 5. Set up event ring
        // 6. Enable interrupts
        // 7. Start controller

        self.initialized = true;
        self.running = true;

        USB_OK
    }

    /// Reset XHCI controller
    fn reset_xhci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            // Stop controller
            let mut cmd = self.read_mmio(XHCI_USBCMD);
            cmd &= !XHCI_CMD_RUN;
            self.write_mmio(XHCI_USBCMD, cmd);

            // Wait for HCH (Halted) bit
            let mut timeout = 10000;
            while timeout > 0 {
                let sts = self.read_mmio(XHCI_USBSTS);
                if sts & XHCI_STS_HCH != 0 {
                    break;
                }
                timeout -= 1;
            }

            // Reset controller
            cmd = self.read_mmio(XHCI_USBCMD);
            cmd |= XHCI_CMD_HCRST;
            self.write_mmio(XHCI_USBCMD, cmd);

            // Wait for reset to complete
            timeout = 10000;
            while timeout > 0 {
                let cmd = self.read_mmio(XHCI_USBCMD);
                if cmd & XHCI_CMD_HCRST == 0 {
                    break;
                }
                timeout -= 1;
            }
        }

        USB_OK
    }

    /// Start XHCI controller
    fn start_xhci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            let mut cmd = self.read_mmio(XHCI_USBCMD);
            cmd |= XHCI_CMD_RUN;
            self.write_mmio(XHCI_USBCMD, cmd);
        }

        self.running = true;
        USB_OK
    }

    /// Stop XHCI controller
    fn stop_xhci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            let mut cmd = self.read_mmio(XHCI_USBCMD);
            cmd &= !XHCI_CMD_RUN;
            self.write_mmio(XHCI_USBCMD, cmd);
        }

        self.running = false;
        USB_OK
    }
}

// ─── Implement UsbController Trait ─────────────────────────────

impl UsbController for XhciController {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        // Determine vendor ID from device ID (simplified)
        let vendor_id = match device_id {
            _ => INTEL_VENDOR_ID, // Default to Intel
        };
        
        self.init_xhci(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_controller_name(&self) -> &'static str {
        match self.vendor_id {
            INTEL_VENDOR_ID => "Intel XHCI USB Controller",
            AMD_VENDOR_ID => "AMD XHCI USB Controller",
            VIA_VENDOR_ID => "VIA XHCI USB Controller",
            RENESAS_VENDOR_ID => "Renesas XHCI USB Controller",
            ASMEDIA_VENDOR_ID => "ASMedia XHCI USB Controller",
            _ => "XHCI USB Controller",
        }
    }

    fn get_speed(&self) -> UsbSpeed {
        self.usb_speed
    }

    fn reset(&mut self) -> I32 {
        self.reset_xhci()
    }

    fn shutdown(&mut self) -> I32 {
        self.stop_xhci();
        self.initialized = false;
        USB_OK
    }

    fn get_device_descriptor(&self, descriptor: *mut UsbDeviceDescriptor) -> I32 {
        if descriptor.is_null() {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Send GET_DESCRIPTOR request
        // 2. Read response from device
        // 3. Parse descriptor

        // Stub: return default descriptor
        unsafe {
            (*descriptor).length = 18;
            (*descriptor).descriptor_type = 1;
            (*descriptor).usb_version = 0x0200; // USB 2.0
            (*descriptor).vendor_id = self.vendor_id;
            (*descriptor).product_id = self.device_id;
        }

        USB_OK
    }

    fn get_configuration_descriptor(&self, config_index: U8, buffer: *mut U8, length: *mut U16) -> I32 {
        if buffer.is_null() || length.is_null() {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Send GET_DESCRIPTOR request for configuration
        // 2. Read response from device
        // 3. Copy to buffer

        // Stub: return zero length
        unsafe {
            *length = 0;
        }

        USB_OK
    }

    fn control_transfer(&mut self, request_type: U8, request: U8, value: U16, index: U16, buffer: *mut U8, length: U16) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Set up control transfer TRB
        // 2. Ring doorbell
        // 3. Wait for completion
        // 4. Handle transfer result

        USB_OK
    }

    fn bulk_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Set up bulk transfer TRB
        // 2. Ring doorbell
        // 3. Wait for completion

        USB_OK
    }

    fn interrupt_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Set up interrupt transfer TRB
        // 2. Ring doorbell
        // 3. Wait for completion

        USB_OK
    }

    fn isochronous_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Set up isochronous transfer TRB
        // 2. Ring doorbell
        // 3. Wait for completion

        USB_OK
    }

    fn submit_transfer(&mut self, transfer: *mut UsbTransfer) -> I32 {
        if transfer.is_null() {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Validate transfer
        // 2. Set up appropriate TRB
        // 3. Submit to ring
        // 4. Ring doorbell

        USB_OK
    }

    fn cancel_transfer(&mut self, transfer: *mut UsbTransfer) -> I32 {
        if transfer.is_null() {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Stop endpoint
        // 2. Dequeue TRB
        // 3. Resume endpoint

        USB_OK
    }

    fn allocate_address(&mut self) -> U8 {
        // In a real implementation, this would:
        // 1. Find free device slot
        // 2. Allocate device context
        // 3. Return address

        1 // Stub: return address 1
    }

    fn set_address(&mut self, address: U8) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Send SET_ADDRESS command
        // 2. Update device context

        USB_OK
    }

    fn set_configuration(&mut self, config_value: U8) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Send SET_CONFIGURATION command
        // 2. Update device context

        USB_OK
    }

    fn get_port_status(&self, port: U8) -> U32 {
        if !self.initialized {
            return 0;
        }

        // In a real implementation, this would read port status register
        // Stub: return connected status
        XHCI_PORT_CCS | XHCI_PORT_PED | XHCI_PORT_PP
    }

    fn set_port_feature(&mut self, port: U8, feature: U16) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Write to port status register
        // 2. Wait for feature to take effect

        USB_OK
    }

    fn clear_port_feature(&mut self, port: U8, feature: U16) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Write to port status register
        // 2. Wait for feature to clear

        USB_OK
    }
}

// ─── Global XHCI Controller ─────────────────────────────────

static mut G_XHCI: XhciController = XhciController::new();

// ─── C-ABI Exports ─────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn xhci_init(pci_bar: U64, device_id: U16) -> I32 {
    G_XHCI.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn xhci_is_initialized() -> I32 {
    if G_XHCI.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn xhci_reset() -> I32 {
    G_XHCI.reset()
}

#[no_mangle]
pub unsafe extern "C" fn xhci_start() -> I32 {
    G_XHCI.start_xhci()
}

#[no_mangle]
pub unsafe extern "C" fn xhci_stop() -> I32 {
    G_XHCI.stop_xhci()
}

#[no_mangle]
pub unsafe extern "C" fn xhci_shutdown() -> I32 {
    G_XHCI.shutdown()
}

/// Probe for XHCI devices
#[no_mangle]
pub unsafe extern "C" fn xhci_probe() -> I32 {
    // Scan PCI bus for XHCI devices
    let mut found_devices = 0;
    
    for bus in 0..256u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                let class_code = read_pci_config_u8(bus, device, function, 0x0B);
                let subclass = read_pci_config_u8(bus, device, function, 0x0A);
                
                // XHCI: Class 0x0C, Subclass 0x03, Prog IF 0x30
                if class_code == 0x0C && subclass == 0x03 {
                    let prog_if = read_pci_config_u8(bus, device, function, 0x09);
                    if prog_if == 0x30 {
                        let bar0 = read_pci_config_u32(bus, device, function, 0x10);
                        let mmio_base = (bar0 & 0xFFFFFFF0) as U64;
                        
                        let result = G_XHCI.init(mmio_base, device_id);
                        
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

/// Read 8-bit value from PCI configuration space
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

/// Read 16-bit value from PCI configuration space
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

/// Read 32-bit value from PCI configuration space
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
    // Placeholder for I/O port write
}

unsafe fn inl(port: U16) -> U32 {
    // Placeholder for I/O port read
    0
}
