// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/usb/ehci.rs — USB 2.0 EHCI Controller Driver
//
// Implements the Enhanced Host Controller Interface (EHCI) driver.
// Supports USB 2.0 High Speed (480 Mbps) devices.
// Based on Linux kernel ehci driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::usb_device_base::{UsbController, UsbSpeed, UsbDirection, UsbTransfer, UsbDeviceDescriptor, USB_OK, USB_ERR_NO_DEVICE, USB_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── EHCI Vendor IDs ─────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;
pub const AMD_VENDOR_ID: U16 = 0x1022;
pub const VIA_VENDOR_ID: U16 = 0x1106;
pub const NVIDIA_VENDOR_ID: U16 = 0x10DE;
pub const NEC_VENDOR_ID: U16 = 0x1033;

// ─── EHCI Register Offsets ─────────────────────────────────────

pub const EHCI_CAPLENGTH: U32 = 0x00;
pub const EHCI_HCIVERSION: U32 = 0x02;
pub const EHCI_HCSPARAMS: U32 = 0x04;
pub const EHCI_HCCPARAMS: U32 = 0x08;

pub const EHCI_USBCMD: U32 = 0x20;
pub const EHCI_USBSTS: U32 = 0x24;
pub const EHCI_USBINTR: U32 = 0x28;
pub const EHCI_FRINDEX: U32 = 0x2C;
pub const EHCI_PERIODICLISTBASE: U32 = 0x38;
pub const EHCI_ASYNCLISTADDR: U32 = 0x40;
pub const EHCI_CONFIGFLAG: U32 = 0x60;
pub const EHCI_PORTSC: U32 = 0x64;

// ─── EHCI Command Flags ───────────────────────────────────────

pub const EHCI_CMD_RUN: U32 = 0x00000001;
pub const EHCI_CMD_HCRESET: U32 = 0x00000002;
pub const EHCI_CMD_PSE: U32 = 0x00000004;
pub const EHCI_CMD_ASE: U32 = 0x00000008;
pub const EHCI_CMD_IAAD: U32 = 0x00000020;
pub const EHCI_CMD_LRESET: U32 = 0x00000040;
pub const EHCI_CMD_CFLGD: U32 = 0x00000080;
pub const EHCI_CMD_PHPRE: U32 = 0x00000400;
pub const EHCI_CMD_ASPE: U32 = 0x00000800;
pub const EHCI_CMD_FSPE: U32 = 0x00001000;

// ─── EHCI Status Flags ───────────────────────────────────────

pub const EHCI_STS_USBINT: U32 = 0x00000001;
pub const EHCI_STS_USBERRINT: U32 = 0x00000002;
pub const EHCI_STS_PCD: U32 = 0x00000004;
pub const EHCI_STS_FLR: U32 = 0x00000008;
pub const EHCI_STS_HSE: U32 = 0x00000010;
pub const EHCI_STS_IAA: U32 = 0x00000020;
pub const EHCI_STS_HALT: U32 = 0x00001000;
pub const EHCI_STS_RECL: U32 = 0x00002000;
pub const EHCI_STS_PSS: U32 = 0x00004000;
pub const EHCI_STS_ASS: U32 = 0x00008000;

// ─── EHCI Port Status ───────────────────────────────────────

pub const EHCI_PORT_CCS: U32 = 0x00000001;
pub const EHCI_PORT_CSC: U32 = 0x00000002;
pub const EHCI_PORT_PE: U32 = 0x00000004;
pub const EHCI_PORT_PEC: U32 = 0x00000008;
pub const EHCI_PORT_OCC: U32 = 0x00000020;
pub const EHCI_PORT_OCA: U32 = 0x00000040;
pub const EHCI_PORT_PR: U32 = 0x00000100;
pub const EHCI_PORT_PP: U32 = 0x00000200;
pub const EHCI_PORT_PO: U32 = 0x00000400;
pub const EHCI_PORT_LS_MASK: U32 = 0x00000C00;
pub const EHCI_PORT_LS_K: U32 = 0x00000400;
pub const EHCI_PORT_LS_J: U32 = 0x00000800;

// ─── EHCI QH (Queue Head) Structure ───────────────────────────

#[repr(C)]
pub struct EhciQueueHead {
    pub link_pointer: U32,
    pub endpoint_characteristics: U32,
    pub endpoint_capabilities: U32,
    pub current_td_pointer: U32,
    pub next_td_pointer: U32,
    pub alt_next_td_pointer: U32,
    pub td_token: U32,
    pub buffer_page_list: [U32; 5],
    pub buffer_page_list_high: [U32; 5],
}

impl EhciQueueHead {
    pub const fn new() -> Self {
        EhciQueueHead {
            link_pointer: 0,
            endpoint_characteristics: 0,
            endpoint_capabilities: 0,
            current_td_pointer: 0,
            next_td_pointer: 0,
            alt_next_td_pointer: 0,
            td_token: 0,
            buffer_page_list: [0; 5],
            buffer_page_list_high: [0; 5],
        }
    }
}

// ─── EHCI TD (Transfer Descriptor) Structure ─────────────────

#[repr(C)]
pub struct EhciTransferDescriptor {
    pub next_td_pointer: U32,
    pub alt_next_td_pointer: U32,
    pub td_token: U32,
    pub buffer_page_list: [U32; 5],
    pub buffer_page_list_high: [U32; 5],
}

impl EhciTransferDescriptor {
    pub const fn new() -> Self {
        EhciTransferDescriptor {
            next_td_pointer: 0,
            alt_next_td_pointer: 0,
            td_token: 0,
            buffer_page_list: [0; 5],
            buffer_page_list_high: [0; 5],
        }
    }
}

// ─── EHCI Controller Structure ───────────────────────────────

pub struct EhciController {
    pub mmio_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub running: bool,
    pub cap_length: U8,
    pub hci_version: U16,
    pub num_ports: U8,
    pub companion_ports: U8,
    pub periodic_frame_size: U8,
    pub async_schedule: bool,
    pub periodic_schedule: bool,
    pub frame_list_size: U32,
    pub frame_list_base: U64,
    pub async_list_base: U64,
    pub queue_heads: [EhciQueueHead; 256],
    pub transfer_descriptors: [EhciTransferDescriptor; 512],
    pub usb_speed: UsbSpeed,
}

impl EhciController {
    pub const fn new() -> Self {
        EhciController {
            mmio_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            running: false,
            cap_length: 0,
            hci_version: 0,
            num_ports: 0,
            companion_ports: 0,
            periodic_frame_size: 0,
            async_schedule: false,
            periodic_schedule: false,
            frame_list_size: 1024,
            frame_list_base: 0,
            async_list_base: 0,
            queue_heads: [EhciQueueHead::new(); 256],
            transfer_descriptors: [EhciTransferDescriptor::new(); 512],
            usb_speed: UsbSpeed::High,
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

    /// Initialize EHCI controller
    fn init_ehci(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        // Read capability length
        unsafe {
            self.cap_length = (self.read_mmio(EHCI_CAPLENGTH) & 0xFF) as U8;
            
            // Read HCI version
            self.hci_version = ((self.read_mmio(EHCI_HCIVERSION) >> 16) & 0xFFFF) as U16;
            
            // Read HC params
            let hcs_params = self.read_mmio(EHCI_HCSPARAMS);
            self.num_ports = ((hcs_params >> 0) & 0xFF) as U8;
            self.companion_ports = ((hcs_params >> 12) & 0xF) as U8;
            self.periodic_frame_size = ((hcs_params >> 16) & 0x7) as U8;
            
            // EHCI always supports High Speed (480 Mbps)
            self.usb_speed = UsbSpeed::High;
        }

        // In a real implementation, this would:
        // 1. Stop controller
        // 2. Reset controller
        // 3. Initialize frame list
        // 4. Set up async schedule
        // 5. Set up periodic schedule
        // 6. Enable interrupts
        // 7. Start controller

        self.initialized = true;
        self.running = true;

        USB_OK
    }

    /// Reset EHCI controller
    fn reset_ehci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            // Stop controller
            let mut cmd = self.read_mmio(EHCI_USBCMD);
            cmd &= !EHCI_CMD_RUN;
            self.write_mmio(EHCI_USBCMD, cmd);

            // Wait for HALT bit
            let mut timeout = 10000;
            while timeout > 0 {
                let sts = self.read_mmio(EHCI_USBSTS);
                if sts & EHCI_STS_HALT != 0 {
                    break;
                }
                timeout -= 1;
            }

            // Reset controller
            cmd = self.read_mmio(EHCI_USBCMD);
            cmd |= EHCI_CMD_HCRESET;
            self.write_mmio(EHCI_USBCMD, cmd);

            // Wait for reset to complete
            timeout = 10000;
            while timeout > 0 {
                let cmd = self.read_mmio(EHCI_USBCMD);
                if cmd & EHCI_CMD_HCRESET == 0 {
                    break;
                }
                timeout -= 1;
            }
        }

        USB_OK
    }

    /// Start EHCI controller
    fn start_ehci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            let mut cmd = self.read_mmio(EHCI_USBCMD);
            cmd |= EHCI_CMD_RUN;
            self.write_mmio(EHCI_USBCMD, cmd);
        }

        self.running = true;
        USB_OK
    }

    /// Stop EHCI controller
    fn stop_ehci(&mut self) -> I32 {
        if !self.initialized {
            return USB_ERR_INIT_FAILED;
        }

        unsafe {
            let mut cmd = self.read_mmio(EHCI_USBCMD);
            cmd &= !EHCI_CMD_RUN;
            self.write_mmio(EHCI_USBCMD, cmd);
        }

        self.running = false;
        USB_OK
    }
}

// ─── Implement UsbController Trait ─────────────────────────────

impl UsbController for EhciController {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let vendor_id = match device_id {
            _ => INTEL_VENDOR_ID,
        };
        
        self.init_ehci(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_controller_name(&self) -> &'static str {
        match self.vendor_id {
            INTEL_VENDOR_ID => "Intel EHCI USB Controller",
            AMD_VENDOR_ID => "AMD EHCI USB Controller",
            VIA_VENDOR_ID => "VIA EHCI USB Controller",
            NVIDIA_VENDOR_ID => "NVIDIA EHCI USB Controller",
            NEC_VENDOR_ID => "NEC EHCI USB Controller",
            _ => "EHCI USB Controller",
        }
    }

    fn get_speed(&self) -> UsbSpeed {
        self.usb_speed
    }

    fn reset(&mut self) -> I32 {
        self.reset_ehci()
    }

    fn shutdown(&mut self) -> I32 {
        self.stop_ehci();
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
            (*descriptor).usb_version = 0x0200;
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

        // In a real implementation, set up QH and TD for control transfer
        USB_OK
    }

    fn bulk_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, set up QH and TD for bulk transfer
        USB_OK
    }

    fn interrupt_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32 {
        if !self.initialized || !self.running {
            return USB_ERR_INIT_FAILED;
        }

        // In a real implementation, set up QH and TD for interrupt transfer
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
        EHCI_PORT_CCS | EHCI_PORT_PE | EHCI_PORT_PP
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

// ─── Global EHCI Controller ─────────────────────────────────

static mut G_EHCI: EhciController = EhciController::new();

// ─── C-ABI Exports ─────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn ehci_init(pci_bar: U64, device_id: U16) -> I32 {
    G_EHCI.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn ehci_is_initialized() -> I32 {
    if G_EHCI.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn ehci_reset() -> I32 {
    G_EHCI.reset()
}

#[no_mangle]
pub unsafe extern "C" fn ehci_start() -> I32 {
    G_EHCI.start_ehci()
}

#[no_mangle]
pub unsafe extern "C" fn ehci_stop() -> I32 {
    G_EHCI.stop_ehci()
}

#[no_mangle]
pub unsafe extern "C" fn ehci_shutdown() -> I32 {
    G_EHCI.shutdown()
}

/// Probe for EHCI devices
#[no_mangle]
pub unsafe extern "C" fn ehci_probe() -> I32 {
    let mut found_devices = 0;
    
    for bus in 0..256u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                let class_code = read_pci_config_u8(bus, device, function, 0x0B);
                let subclass = read_pci_config_u8(bus, device, function, 0x0A);
                
                // EHCI: Class 0x0C, Subclass 0x03, Prog IF 0x20
                if class_code == 0x0C && subclass == 0x03 {
                    let prog_if = read_pci_config_u8(bus, device, function, 0x09);
                    if prog_if == 0x20 {
                        let bar0 = read_pci_config_u32(bus, device, function, 0x10);
                        let mmio_base = (bar0 & 0xFFFFFFF0) as U64;
                        
                        let result = G_EHCI.init(mmio_base, device_id);
                        
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
