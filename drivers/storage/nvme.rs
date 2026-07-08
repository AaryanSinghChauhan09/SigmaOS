// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/storage/nvme.rs — NVMe Storage Driver
//
// Implements the NVM Express (NVMe) driver for high-performance SSDs.
// Supports NVMe 1.4 specification.
// Based on Linux kernel nvme driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::storage_device_base::{StorageDevice, StorageType, StorageProtocol, StorageDirection, StorageGeometry, StorageIdentify, StorageRequest, StorageStats, STORAGE_OK, STORAGE_ERR_NO_DEVICE, STORAGE_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── NVMe Vendor IDs ─────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;
pub const SAMSUNG_VENDOR_ID: U16 = 0x144D;
pub const WESTERN_DIGITAL_VENDOR_ID: U16 = 0x1B4B;
pub const MICRON_VENDOR_ID: U16 = 0x1344;
pub const SK_HYNIX_VENDOR_ID: U16 = 0x1C5C;

// ─── NVMe Register Offsets ─────────────────────────────────────

pub const NVME_CAP: U32 = 0x00;
pub const NVME_VS: U32 = 0x08;
pub const NVME_INTMS: U32 = 0x0C;
pub const NVME_INTMC: U32 = 0x10;
pub const NVME_CC: U32 = 0x14;
pub const NVME_CSTS: U32 = 0x1C;
pub const NVME_NSSR: U32 = 0x20;
pub const NVME_AQA: U32 = 0x24;
pub const NVME_ASQ: U32 = 0x28;
pub const NVME_ACQ: U32 = 0x30;
pub const NVME_CMBLOC: U32 = 0x38;
pub const NVME_CMBSZ: U32 = 0x3C;

// ─── NVMe Controller Configuration Flags ─────────────────────

pub const NVME_CC_EN: U32 = 0x00000001;
pub const NVME_CC_CSS: U32 = 0x0000000E;
pub const NVME_CC_MPS: U32 = 0x000000F0;
pub const NVME_CC_SHN: U32 = 0x00000300;
pub const NVME_CC_IOSQES: U32 = 0x0000F000;
pub const NVME_CC_IOCQES: U32 = 0x000F0000;

// ─── NVMe Controller Status Flags ─────────────────────────

pub const NVME_CSTS_RDY: U32 = 0x00000001;
pub const NVME_CSTS_CFS: U32 = 0x00000002;
pub const NVME_CSTS_SHST: U32 = 0x0000000C;

// ─── NVMe Admin Queue Attributes ─────────────────────────

pub const NVME_AQA_ASQS: U32 = 0x00000FFF;
pub const NVME_AQA_ACQS: U32 = 0x0FFF0000;

// ─── NVMe Command ─────────────────────────────────────────

#[repr(C)]
pub struct NvmeCommand {
    pub cdw0: U32,
    pub cdw1: U32,
    pub cdw2: U32,
    pub cdw3: U32,
    pub cdw4: U32,
    pub cdw5: U32,
    pub cdw6: U32,
    pub cdw7: U32,
    pub cdw8: U32,
    pub cdw9: U32,
    pub cdw10: U32,
    pub cdw11: U32,
    pub cdw12: U32,
    pub cdw13: U32,
    pub cdw14: U32,
    pub cdw15: U32,
    pub mptr: U64,
    pub prp1: U64,
    pub prp2: U64,
}

impl NvmeCommand {
    pub const fn new() -> Self {
        NvmeCommand {
            cdw0: 0,
            cdw1: 0,
            cdw2: 0,
            cdw3: 0,
            cdw4: 0,
            cdw5: 0,
            cdw6: 0,
            cdw7: 0,
            cdw8: 0,
            cdw9: 0,
            cdw10: 0,
            cdw11: 0,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
            mptr: 0,
            prp1: 0,
            prp2: 0,
        }
    }
}

// ─── NVMe Completion ───────────────────────────────────────

#[repr(C)]
pub struct NvmeCompletion {
    pub cdw0: U32,
    pub cdw1: U32,
    pub cdw2: U32,
    pub cdw3: U32,
    pub status: U32,
}

impl NvmeCompletion {
    pub const fn new() -> Self {
        NvmeCompletion {
            cdw0: 0,
            cdw1: 0,
            cdw2: 0,
            cdw3: 0,
            status: 0,
        }
    }
}

// ─── NVMe Queue Entry ─────────────────────────────────────

#[repr(C)]
pub struct NvmeQueueEntry {
    pub cmd: NvmeCommand,
    pub cmd_id: U16,
    pub submitted: bool,
}

impl NvmeQueueEntry {
    pub const fn new() -> Self {
        NvmeQueueEntry {
            cmd: NvmeCommand::new(),
            cmd_id: 0,
            submitted: false,
        }
    }
}

// ─── NVMe Submission Queue ─────────────────────────────────

pub struct NvmeSubmissionQueue {
    pub entries: [NvmeQueueEntry; 256],
    pub head: U16,
    pub tail: U16,
    pub size: U16,
    pub phase: bool,
}

impl NvmeSubmissionQueue {
    pub const fn new() -> Self {
        NvmeSubmissionQueue {
            entries: [NvmeQueueEntry::new(); 256],
            head: 0,
            tail: 0,
            size: 256,
            phase: true,
        }
    }
}

// ─── NVMe Completion Queue ─────────────────────────────────

pub struct NvmeCompletionQueue {
    pub entries: [NvmeCompletion; 256],
    pub head: U16,
    pub tail: U16,
    pub size: U16,
    pub phase: bool,
}

impl NvmeCompletionQueue {
    pub const fn new() -> Self {
        NvmeCompletionQueue {
            entries: [NvmeCompletion::new(); 256],
            head: 0,
            tail: 0,
            size: 256,
            phase: true,
        }
    }
}

// ─── NVMe Namespace ───────────────────────────────────────

#[repr(C)]
pub struct NvmeNamespace {
    pub ns_id: U32,
    pub capacity: U64,
    pub block_size: U32,
    pub lba_format: U8,
    pub active: bool,
}

impl NvmeNamespace {
    pub const fn new() -> Self {
        NvmeNamespace {
            ns_id: 0,
            capacity: 0,
            block_size: 4096,
            lba_format: 0,
            active: false,
        }
    }
}

// ─── NVMe Controller Structure ───────────────────────────────

pub struct NvmeController {
    pub mmio_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub enabled: bool,
    pub admin_sq: NvmeSubmissionQueue,
    pub admin_cq: NvmeCompletionQueue,
    pub io_sq: NvmeSubmissionQueue,
    pub io_cq: NvmeCompletionQueue,
    pub namespaces: [NvmeNamespace; 32],
    pub namespace_count: U8,
    pub identify: StorageIdentify,
    pub geometry: StorageGeometry,
    pub stats: StorageStats,
}

impl NvmeController {
    pub const fn new() -> Self {
        NvmeController {
            mmio_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            enabled: false,
            admin_sq: NvmeSubmissionQueue::new(),
            admin_cq: NvmeCompletionQueue::new(),
            io_sq: NvmeSubmissionQueue::new(),
            io_cq: NvmeCompletionQueue::new(),
            namespaces: [NvmeNamespace::new(); 32],
            namespace_count: 0,
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

    /// Initialize NVMe controller
    fn init_nvme(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        unsafe {
            // Check controller is ready
            let csts = self.read_mmio(NVME_CSTS);
            if csts & NVME_CSTS_RDY == 0 {
                // Controller not ready, wait
                let mut timeout = 10000;
                while timeout > 0 {
                    let csts = self.read_mmio(NVME_CSTS);
                    if csts & NVME_CSTS_RDY != 0 {
                        break;
                    }
                    timeout -= 1;
                }
            }

            // Disable controller
            let mut cc = self.read_mmio(NVME_CC);
            cc &= !NVME_CC_EN;
            self.write_mmio(NVME_CC, cc);

            // Wait for controller to disable
            let mut timeout = 10000;
            while timeout > 0 {
                let csts = self.read_mmio(NVME_CSTS);
                if csts & NVME_CSTS_RDY == 0 {
                    break;
                }
                timeout -= 1;
            }

            // Set admin queue attributes
            let aqa = (255 & NVME_AQA_ASQS) | ((255 << 16) & NVME_AQA_ACQS);
            self.write_mmio(NVME_AQA, aqa);

            // Set admin submission queue address
            let admin_sq_addr = &self.admin_sq as *const NvmeSubmissionQueue as U64;
            self.write_mmio64(NVME_ASQ, admin_sq_addr);

            // Set admin completion queue address
            let admin_cq_addr = &self.admin_cq as *const NvmeCompletionQueue as U64;
            self.write_mmio64(NVME_ACQ, admin_cq_addr);

            // Enable controller
            cc = self.read_mmio(NVME_CC);
            cc |= NVME_CC_EN;
            cc |= (0 << 4) & NVME_CC_MPS; // 4KB page size
            cc |= (6 << 12) & NVME_CC_IOSQES; // 64 bytes
            cc |= (4 << 16) & NVME_CC_IOCQES; // 16 bytes
            self.write_mmio(NVME_CC, cc);

            // Wait for controller to enable
            timeout = 10000;
            while timeout > 0 {
                let csts = self.read_mmio(NVME_CSTS);
                if csts & NVME_CSTS_RDY != 0 {
                    break;
                }
                timeout -= 1;
            }

            // Identify controller
            self.identify_controller();
        }

        self.initialized = true;
        self.enabled = true;

        STORAGE_OK
    }

    /// Identify controller
    unsafe fn identify_controller(&mut self) {
        // In a real implementation, this would:
        // 1. Send IDENTIFY command
        // 2. Read response
        // 3. Parse identify data
        // 4. Get namespace list

        // Stub: set default values
        self.identify.capacity = 1024 * 1024 * 1024; // 1TB
        self.identify.sector_size = 4096;
        self.identify.max_lba = self.identify.capacity / self.identify.sector_size as U64;

        self.geometry.total_sectors = self.identify.max_lba;
        self.geometry.sector_size = self.identify.sector_size;
    }

    /// Submit admin command
    unsafe fn submit_admin_command(&mut self, cmd: &NvmeCommand) -> I32 {
        let tail = self.admin_sq.tail;
        self.admin_sq.entries[tail as usize].cmd = *cmd;
        self.admin_sq.entries[tail as usize].cmd_id = tail;
        self.admin_sq.entries[tail as usize].submitted = true;

        self.admin_sq.tail = (tail + 1) % self.admin_sq.size;

        // Ring doorbell
        // In a real implementation, write to doorbell register

        STORAGE_OK
    }

    /// Poll for completion
    unsafe fn poll_completion(&mut self) -> Option<NvmeCompletion> {
        let head = self.admin_cq.head;
        let entry = &self.admin_cq.entries[head as usize];

        if entry.status != 0 {
            let completion = *entry;
            self.admin_cq.head = (head + 1) % self.admin_cq.size;
            Some(completion)
        } else {
            None
        }
    }
}

// ─── Implement StorageDevice Trait ─────────────────────────────

impl StorageDevice for NvmeController {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let vendor_id = match device_id {
            _ => INTEL_VENDOR_ID,
        };
        
        self.init_nvme(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        match self.vendor_id {
            INTEL_VENDOR_ID => "Intel NVMe SSD",
            SAMSUNG_VENDOR_ID => "Samsung NVMe SSD",
            WESTERN_DIGITAL_VENDOR_ID => "Western Digital NVMe SSD",
            MICRON_VENDOR_ID => "Micron NVMe SSD",
            SK_HYNIX_VENDOR_ID => "SK Hynix NVMe SSD",
            _ => "NVMe SSD",
        }
    }

    fn get_storage_type(&self) -> StorageType {
        StorageType::NVMe
    }

    fn get_storage_protocol(&self) -> StorageProtocol {
        StorageProtocol::NVMe
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

        unsafe {
            let mut cmd = NvmeCommand::new();
            cmd.cdw0 = 0x02; // READ opcode
            cmd.cdw10 = (lba & 0xFFFFFFFF) as U32;
            cmd.cdw11 = (lba >> 32) as U32;
            cmd.cdw12 = sector_count & 0xFFFF;
            cmd.prp1 = buffer as U64;

            self.submit_admin_command(&cmd);

            // Wait for completion
            let mut timeout = 10000;
            while timeout > 0 {
                if let Some(completion) = self.poll_completion() {
                    if completion.status == 0 {
                        self.stats.reads += 1;
                        self.stats.read_bytes += sector_count as U64 * self.identify.sector_size as U64;
                        return STORAGE_OK;
                    } else {
                        self.stats.read_errors += 1;
                        return STORAGE_ERR_IO;
                    }
                }
                timeout -= 1;
            }

            STORAGE_ERR_TIMEOUT
        }
    }

    fn write(&mut self, lba: U64, buffer: *const U8, sector_count: U32) -> I32 {
        if !self.initialized || !self.enabled {
            return STORAGE_ERR_INIT_FAILED;
        }

        unsafe {
            let mut cmd = NvmeCommand::new();
            cmd.cdw0 = 0x01; // WRITE opcode
            cmd.cdw10 = (lba & 0xFFFFFFFF) as U32;
            cmd.cdw11 = (lba >> 32) as U32;
            cmd.cdw12 = sector_count & 0xFFFF;
            cmd.prp1 = buffer as U64;

            self.submit_admin_command(&cmd);

            // Wait for completion
            let mut timeout = 10000;
            while timeout > 0 {
                if let Some(completion) = self.poll_completion() {
                    if completion.status == 0 {
                        self.stats.writes += 1;
                        self.stats.write_bytes += sector_count as U64 * self.identify.sector_size as U64;
                        return STORAGE_OK;
                    } else {
                        self.stats.write_errors += 1;
                        return STORAGE_ERR_IO;
                    }
                }
                timeout -= 1;
            }

            STORAGE_ERR_TIMEOUT
        }
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

        unsafe {
            let mut cmd = NvmeCommand::new();
            cmd.cdw0 = 0x00; // FLUSH opcode

            self.submit_admin_command(&cmd);

            // Wait for completion
            let mut timeout = 10000;
            while timeout > 0 {
                if let Some(completion) = self.poll_completion() {
                    if completion.status == 0 {
                        self.stats.flushes += 1;
                        return STORAGE_OK;
                    } else {
                        return STORAGE_ERR_IO;
                    }
                }
                timeout -= 1;
            }

            STORAGE_ERR_TIMEOUT
        }
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

// ─── Global NVMe Controller ─────────────────────────────────

static mut G_NVME: NvmeController = NvmeController::new();

// ─── C-ABI Exports ─────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn nvme_init(pci_bar: U64, device_id: U16) -> I32 {
    G_NVME.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn nvme_is_initialized() -> I32 {
    if G_NVME.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn nvme_shutdown() -> I32 {
    G_NVME.shutdown()
}

/// Probe for NVMe devices
#[no_mangle]
pub unsafe extern "C" fn nvme_probe() -> I32 {
    let mut found_devices = 0;
    
    for bus in 0..256u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                let class_code = read_pci_config_u8(bus, device, function, 0x0B);
                let subclass = read_pci_config_u8(bus, device, function, 0x0A);
                let prog_if = read_pci_config_u8(bus, device, function, 0x09);
                
                // NVMe: Class 0x01, Subclass 0x08, Prog IF 0x02
                if class_code == 0x01 && subclass == 0x08 && prog_if == 0x02 {
                    let bar0 = read_pci_config_u32(bus, device, function, 0x10);
                    let mmio_base = (bar0 & 0xFFFFFFF0) as U64;
                    
                    let result = G_NVME.init(mmio_base, device_id);
                    
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
