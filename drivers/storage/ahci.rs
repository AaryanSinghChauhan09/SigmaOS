// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/storage/ahci.rs — AHCI SATA Storage Driver
//
// Implements the Advanced Host Controller Interface (AHCI) driver for SATA devices.
// Supports SATA I, II, III devices.
// Based on Linux kernel ahci driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::storage_device_base::{StorageDevice, StorageType, StorageProtocol, StorageDirection, StorageGeometry, StorageIdentify, StorageRequest, StorageStats, STORAGE_OK, STORAGE_ERR_NO_DEVICE, STORAGE_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── AHCI Vendor IDs ─────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;
pub const AMD_VENDOR_ID: U16 = 0x1022;
pub const VIA_VENDOR_ID: U16 = 0x1106;
pub const NVIDIA_VENDOR_ID: U16 = 0x10DE;
pub const MARVELL_VENDOR_ID: U16 = 0x11AB;

// ─── AHCI Register Offsets ─────────────────────────────────────

pub const AHCI_CAP: U32 = 0x00;
pub const AHCI_GHC: U32 = 0x04;
pub const AHCI_IS: U32 = 0x08;
pub const AHCI_PI: U32 = 0x0C;
pub const AHCI_VS: U32 = 0x10;
pub const AHCI_CAP2: U32 = 0x24;
pub const AHCI_BOHC: U32 = 0x28;

// ─── AHCI Port Register Offsets ─────────────────────────────

pub const AHCI_PxCLB: U32 = 0x00;
pub const AHCI_PxCLBU: U32 = 0x04;
pub const AHCI_PxFB: U32 = 0x08;
pub const AHCI_PxFBU: U32 = 0x0C;
pub const AHCI_PxIS: U32 = 0x10;
pub const AHCI_PxIE: U32 = 0x14;
pub const AHCI_PxCMD: U32 = 0x18;
pub const AHCI_PxTFD: U32 = 0x20;
pub const AHCI_PxSIG: U32 = 0x24;
pub const AHCI_PxSSTS: U32 = 0x28;
pub const AHCI_PxSCTL: U32 = 0x2C;
pub const AHCI_PxSERR: U32 = 0x30;
pub const AHCI_PxSACT: U32 = 0x34;
pub const AHCI_PxCI: U32 = 0x38;
pub const AHCI_PxSDB: U32 = 0x3C;
pub const AHCI_PxBS: U32 = 0x40;
pub const AHCI_PxBS_DW: U32 = 0x44;

// ─── AHCI GHC Flags ─────────────────────────────────────

pub const AHCI_GHC_AE: U32 = 0x80000000;
pub const AHCI_GHC_MRSM: U32 = 0x00000001;
pub const AHCI_GHC_IE: U32 = 0x00000002;

// ─── AHCI Port Command Flags ─────────────────────────────

pub const AHCI_PxCMD_ST: U32 = 0x00000001;
pub const AHCI_PxCMD_SUD: U32 = 0x00000002;
pub const AHCI_PxCMD_POD: U32 = 0x00000004;
pub const AHCI_PxCMD_CLO: U32 = 0x00000008;
pub const AHCI_PxCMD_FRE: U32 = 0x00000010;
pub const AHCI_PxCMD_CCS: U32 = 0x00001E00;
pub const AHCI_PxCMD_MPSS: U32 = 0x0000E000;
pub const AHCI_PxCMD_FR: U32 = 0x40000000;
pub const AHCI_PxCMD_CR: U32 = 0x80000000;

// ─── AHCI Port Status Flags ─────────────────────────────

pub const AHCI_PxTFD_STS_BSY: U32 = 0x00000080;
pub const AHCI_PxTFD_STS_DRQ: U32 = 0x00000008;
pub const AHCI_PxTFD_STS_ERR: U32 = 0x00000001;

// ─── AHCI Command Header ─────────────────────────────────

#[repr(C)]
pub struct AhciCommandHeader {
    pub flags: U16,
    pub prdt_length: U16,
    pub prt: U32,
    prdtb: U64,
}

impl AhciCommandHeader {
    pub const fn new() -> Self {
        AhciCommandHeader {
            flags: 0,
            prdt_length: 0,
            prt: 0,
            prdtb: 0,
        }
    }
}

// ─── AHCI Command FIS ─────────────────────────────────────

#[repr(C)]
pub struct AhciCommandFis {
    pub fis_type: U8,
    pub c: U8,
    pub command: U8,
    pub features: U8,
    pub lba_low: U8,
    pub lba_mid: U8,
    pub lba_high: U8,
    pub device: U8,
    pub lba_low_exp: U8,
    pub lba_mid_exp: U8,
    pub lba_high_exp: U8,
    pub features_exp: U8,
    pub sector_count: U8,
    pub sector_count_exp: U8,
    pub control: U8,
    pub reserved: [U8; 4],
}

impl AhciCommandFis {
    pub const fn new() -> Self {
        AhciCommandFis {
            fis_type: 0x27, // FIS type register H2D
            c: 0x80,
            command: 0,
            features: 0,
            lba_low: 0,
            lba_mid: 0,
            lba_high: 0,
            device: 0,
            lba_low_exp: 0,
            lba_mid_exp: 0,
            lba_high_exp: 0,
            features_exp: 0,
            sector_count: 0,
            sector_count_exp: 0,
            control: 0,
            reserved: [0; 4],
        }
    }
}

// ─── AHCI PRDT (Physical Region Descriptor Table) ─────────

#[repr(C)]
pub struct AhciPrdt {
    pub dba: U64,
    pub dba_upper: U32,
    pub reserved: U32,
    pub dbc: U32,
    pub i: U32,
}

impl AhciPrdt {
    pub const fn new() -> Self {
        AhciPrdt {
            dba: 0,
            dba_upper: 0,
            reserved: 0,
            dbc: 0,
            i: 0,
        }
    }
}

// ─── AHCI Port Structure ───────────────────────────────────

pub struct AhciPort {
    pub port_number: U8,
    pub command_list_base: U64,
    pub fis_base: U64,
    pub command_headers: [AhciCommandHeader; 32],
    pub command_fis: AhciCommandFis,
    pub prdt: [AhciPrdt; 8],
    pub active: bool,
    pub device_present: bool,
}

impl AhciPort {
    pub const fn new() -> Self {
        AhciPort {
            port_number: 0,
            command_list_base: 0,
            fis_base: 0,
            command_headers: [AhciCommandHeader::new(); 32],
            command_fis: AhciCommandFis::new(),
            prdt: [AhciPrdt::new(); 8],
            active: false,
            device_present: false,
        }
    }
}

// ─── AHCI Controller Structure ───────────────────────────────

pub struct AhciController {
    pub mmio_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub enabled: bool,
    pub num_ports: U8,
    pub ports: [AhciPort; 32],
    pub command_slots: U8,
    pub identify: StorageIdentify,
    pub geometry: StorageGeometry,
    pub stats: StorageStats,
}

impl AhciController {
    pub const fn new() -> Self {
        AhciController {
            mmio_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            enabled: false,
            num_ports: 0,
            ports: [AhciPort::new(); 32],
            command_slots: 32,
            identify: StorageIdentify::new(),
            geometry: StorageGeometry::new(),
            stats: StorageStats::new(),
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
        *ptr = value
    }

    /// Read MMIO register 64-bit
    unsafe fn read_mmio64(&self, offset: U32) -> U64 {
        let ptr = (self.mmio_base + offset as U64) as *const U64;
        *ptr
    }

    /// Write MMIO register 64-bit
    unsafe fn write_mmio64(&self, offset: U32, value: U64) {
        let ptr = (self.mmio_base + offset as U64) as *mut U64;
        *ptr = value
    }

    /// Read port register
    unsafe fn read_port_mmio(&self, port: U8, offset: U32) -> U32 {
        let port_offset = 0x100 + (port as U32) * 0x80;
        self.read_mmio(port_offset + offset)
    }

    /// Write port register
    unsafe fn write_port_mmio(&self, port: U8, offset: U32, value: U32) {
        let port_offset = 0x100 + (port as U32) * 0x80;
        self.write_mmio(port_offset + offset, value)
    }

    /// Initialize AHCI controller
    fn init_ahci(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        unsafe {
            // Read capabilities
            let cap = self.read_mmio(AHCI_CAP);
            self.num_ports = ((cap >> 0) & 0x1F) as U8;
            self.command_slots = ((cap >> 8) & 0x1F) as U8;

            // Enable AHCI mode
            let mut ghc = self.read_mmio(AHCI_GHC);
            ghc |= AHCI_GHC_AE;
            self.write_mmio(AHCI_GHC, ghc);

            // Reset controller
            ghc = self.read_mmio(AHCI_GHC);
            ghc |= AHCI_GHC_MRSM;
            self.write_mmio(AHCI_GHC, ghc);

            // Wait for reset
            let mut timeout = 10000;
            while timeout > 0 {
                ghc = self.read_mmio(AHCI_GHC);
                if ghc & AHCI_GHC_MRSM == 0 {
                    break;
                }
                timeout -= 1;
            }

            // Re-enable AHCI mode
            ghc = self.read_mmio(AHCI_GHC);
            ghc |= AHCI_GHC_AE;
            self.write_mmio(AHCI_GHC, ghc);

            // Initialize ports
            for i in 0..self.num_ports {
                self.init_port(i);
            }

            // Identify devices on ports
            for i in 0..self.num_ports {
                self.identify_port(i);
            }
        }

        self.initialized = true;
        self.enabled = true;

        STORAGE_OK
    }

    /// Initialize a port
    unsafe fn init_port(&mut self, port: U8) {
        let port_offset = 0x100 + (port as U32) * 0x80;

        // Stop command engine
        let mut cmd = self.read_mmio(port_offset + AHCI_PxCMD);
        cmd &= !AHCI_PxCMD_ST;
        cmd &= !AHCI_PxCMD_FRE;
        self.write_mmio(port_offset + AHCI_PxCMD, cmd);

        // Wait for engine to stop
        let mut timeout = 10000;
        while timeout > 0 {
            cmd = self.read_mmio(port_offset + AHCI_PxCMD);
            if cmd & AHCI_PxCMD_CR == 0 {
                break;
            }
            timeout -= 1;
        }

        // Set up command list and FIS base addresses
        self.ports[port as usize].command_list_base = 0x1000000 + (port as U64) * 0x1000;
        self.ports[port as usize].fis_base = 0x2000000 + (port as U64) * 0x1000;

        self.write_mmio64(port_offset + AHCI_PxCLB, self.ports[port as usize].command_list_base);
        self.write_mmio64(port_offset + AHCI_PxFB, self.ports[port as usize].fis_base);

        // Clear error status
        let serr = self.read_mmio(port_offset + AHCI_PxSERR);
        self.write_mmio(port_offset + AHCI_PxSERR, serr);

        // Start FIS receive
        cmd = self.read_mmio(port_offset + AHCI_PxCMD);
        cmd |= AHCI_PxCMD_FRE;
        self.write_mmio(port_offset + AHCI_PxCMD, cmd);

        self.ports[port as usize].active = true;
    }

    /// Identify device on port
    unsafe fn identify_port(&mut self, port: U8) {
        let port_offset = 0x100 + (port as U32) * 0x80;

        // Check if device present
        let ssts = self.read_mmio(port_offset + AHCI_PxSSTS);
        if ssts & 0x0F == 0 {
            self.ports[port as usize].device_present = false;
            return;
        }

        self.ports[port as usize].device_present = true;

        // In a real implementation, this would:
        // 1. Send IDENTIFY DEVICE command
        // 2. Read response
        // 3. Parse identify data

        // Stub: set default values
        self.identify.capacity = 1024 * 1024 * 1024; // 1TB
        self.identify.sector_size = 512;
        self.identify.max_lba = self.identify.capacity / self.identify.sector_size as U64;

        self.geometry.total_sectors = self.identify.max_lba;
        self.geometry.sector_size = self.identify.sector_size;
    }

    /// Execute ATA command
    unsafe fn execute_ata_command(&mut self, port: U8, command: U8, lba: U64, buffer: *mut U8, sector_count: U32) -> I32 {
        let port_offset = 0x100 + (port as U32) * 0x80;

        // Set up command FIS
        self.ports[port as usize].command_fis.command = command;
        self.ports[port as usize].command_fis.lba_low = (lba & 0xFF) as U8;
        self.ports[port as usize].command_fis.lba_mid = ((lba >> 8) & 0xFF) as U8;
        self.ports[port as usize].command_fis.lba_high = ((lba >> 16) & 0xFF) as U8;
        self.ports[port as usize].command_fis.lba_low_exp = ((lba >> 24) & 0xFF) as U8;
        self.ports[port as usize].command_fis.lba_mid_exp = ((lba >> 32) & 0xFF) as U8;
        self.ports[port as usize].command_fis.lba_high_exp = ((lba >> 40) & 0xFF) as U8;
        self.ports[port as usize].command_fis.sector_count = sector_count as U8;

        // Set up PRDT
        self.ports[port as usize].prdt[0].dba = buffer as U64;
        self.ports[port as usize].prdt[0].dbc = (sector_count * 512) - 1;

        // Start command
        let mut cmd = self.read_mmio(port_offset + AHCI_PxCMD);
        cmd |= AHCI_PxCMD_ST;
        self.write_mmio(port_offset + AHCI_PxCMD, cmd);

        // Wait for completion
        let mut timeout = 10000;
        while timeout > 0 {
            let tfd = self.read_mmio(port_offset + AHCI_PxTFD);
            if tfd & (AHCI_PxTFD_STS_BSY | AHCI_PxTFD_STS_DRQ) == 0 {
                break;
            }
            timeout -= 1;
        }

        // Stop command
        cmd = self.read_mmio(port_offset + AHCI_PxCMD);
        cmd &= !AHCI_PxCMD_ST;
        self.write_mmio(port_offset + AHCI_PxCMD, cmd);

        // Check for errors
        let tfd = self.read_mmio(port_offset + AHCI_PxTFD);
        if tfd & AHCI_PxTFD_STS_ERR != 0 {
            return STORAGE_ERR_IO;
        }

        STORAGE_OK
    }
}

// ─── Implement StorageDevice Trait ─────────────────────────────

impl StorageDevice for AhciController {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let vendor_id = match device_id {
            _ => INTEL_VENDOR_ID,
        };
        
        self.init_ahci(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        match self.vendor_id {
            INTEL_VENDOR_ID => "Intel AHCI SATA Controller",
            AMD_VENDOR_ID => "AMD AHCI SATA Controller",
            VIA_VENDOR_ID => "VIA AHCI SATA Controller",
            NVIDIA_VENDOR_ID => "NVIDIA AHCI SATA Controller",
            MARVELL_VENDOR_ID => "Marvell AHCI SATA Controller",
            _ => "AHCI SATA Controller",
        }
    }

    fn get_storage_type(&self) -> StorageType {
        StorageType::AHCI
    }

    fn get_storage_protocol(&self) -> StorageProtocol {
        StorageProtocol::ATA
    }

    fn get_geometry(&self) -> StorageGeometry {
        self.geometry
    }

    fn get_identify(&self, identify: *mut StorageIdentify) -> I32 {
        if identify.is_null() {
            return STORAGE_ERR_INIT_FAILED;
        }

        unsafe {
            *identify = self.identify;
        }

        STORAGE_OK
    }

    fn read(&mut self, lba: U64, buffer: *mut U8, sector_count: U32) -> I32 {
        if !self.initialized || !self.enabled {
            return STORAGE_ERR_INIT_FAILED;
        }

        // Find first active port with device
        for i in 0..self.num_ports {
            if self.ports[i as usize].device_present {
                unsafe {
                    let result = self.execute_ata_command(i, 0x25, lba, buffer, sector_count); // READ DMA EXT
                    if result == STORAGE_OK {
                        self.stats.reads += 1;
                        self.stats.read_bytes += sector_count as U64 * self.identify.sector_size as U64;
                    } else {
                        self.stats.read_errors += 1;
                    }
                    return result;
                }
            }
        }

        STORAGE_ERR_NO_DEVICE
    }

    fn write(&mut self, lba: U64, buffer: *const U8, sector_count: U32) -> I32 {
        if !self.initialized || !self.enabled {
            return STORAGE_ERR_INIT_FAILED;
        }

        // Find first active port with device
        for i in 0..self.num_ports {
            if self.ports[i as usize].device_present {
                unsafe {
                    let result = self.execute_ata_command(i, 0x35, lba, buffer as *mut U8, sector_count); // WRITE DMA EXT
                    if result == STORAGE_OK {
                        self.stats.writes += 1;
                        self.stats.write_bytes += sector_count as U64 * self.identify.sector_size as U64;
                    } else {
                        self.stats.write_errors += 1;
                    }
                    return result;
                }
            }
        }

        STORAGE_ERR_NO_DEVICE
    }

    fn submit_request(&mut self, request: *mut StorageRequest) -> I32 {
        if request.is_null() {
            return STORAGE_ERR_INIT_FAILED;
        }

        unsafe {
            let req = &mut *request;
            match req.direction {
                StorageDirection::Read => {
                    self.read(req.lba, req.buffer, req.sector_count)
                }
                StorageDirection::Write => {
                    self.write(req.lba, req.buffer as *const U8, req.sector_count)
                }
                StorageDirection::Flush => {
                    self.flush()
                }
            }
        }
    }

    fn cancel_request(&mut self, request: *mut StorageRequest) -> I32 {
        if request.is_null() {
            return STORAGE_ERR_INIT_FAILED;
        }

        STORAGE_OK
    }

    fn flush(&mut self) -> I32 {
        if !self.initialized || !self.enabled {
            return STORAGE_ERR_INIT_FAILED;
        }

        // Find first active port with device
        for i in 0..self.num_ports {
            if self.ports[i as usize].device_present {
                unsafe {
                    let result = self.execute_ata_command(i, 0xE7, 0, 0 as *mut U8, 0); // FLUSH CACHE EXT
                    if result == STORAGE_OK {
                        self.stats.flushes += 1;
                    }
                    return result;
                }
            }
        }

        STORAGE_NO_DEVICE
    }

    fn get_capacity(&self) -> U64 {
        self.identify.capacity
    }

    fn get_sector_size(&self) -> U32 {
        self.identify.sector_size
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return STORAGE_ERR_INIT_FAILED;
        }

        STORAGE_OK
    }

    fn shutdown(&mut self) -> I32 {
        if !self.initialized {
            return STORAGE_ERR_INIT_FAILED;
        }

        self.enabled = false;
        self.initialized = false;
        STORAGE_OK
    }
}

// ─── Global AHCI Controller ─────────────────────────────────

static mut G_AHCI: AhciController = AhciController::new();

// ─── C-ABI Exports ─────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn ahci_init(pci_bar: U64, device_id: U16) -> I32 {
    G_AHCI.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn ahci_is_initialized() -> I32 {
    if G_AHCI.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn ahci_shutdown() -> I32 {
    G_AHCI.shutdown()
}

/// Probe for AHCI devices
#[no_mangle]
pub unsafe extern "C" fn ahci_probe() -> I32 {
    let mut found_devices = 0;
    
    for bus in 0..256u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                let class_code = read_pci_config_u8(bus, device, function, 0x0B);
                let subclass = read_pci_config_u8(bus, device, function, 0x0A);
                let prog_if = read_pci_config_u8(bus, device, function, 0x09);
                
                // AHCI: Class 0x01, Subclass 0x06, Prog IF 0x01
                if class_code == 0x01 && subclass == 0x06 && prog_if == 0x01 {
                    let bar5 = read_pci_config_u32(bus, device, function, 0x24);
                    let mmio_base = (bar5 & 0xFFFFFFF0) as U64;
                    
                    let result = G_AHCI.init(mmio_base, device_id);
                    
                    if result == STORAGE_OK {
                        found_devices += 1;
                        return STORAGE_OK;
                    }
                }
            }
        }
    }
    
    if found_devices > 0 {
        STORAGE_OK
    } else {
        STORAGE_ERR_NO_DEVICE
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
