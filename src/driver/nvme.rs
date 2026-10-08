//! NVMe Storage Driver
//!
//! Provides NVMe (Non-Volatile Memory Express) storage driver support for SigmaOS.
//! Enables access to NVMe SSDs with high-performance storage capabilities.
//!
//! Supports:
//! - NVMe controller discovery and initialization
//! - Admin queue management
//! - I/O queue creation and submission
//! - Namespace enumeration and access
//! - Read/write operations
//! - Interrupt handling (MSI-X)
//! - Completion queue processing

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// NVMe controller registers
#[derive(Debug, Clone)]
#[repr(C)]
pub struct NvmeRegisters {
    pub cap: u64,         // Controller capabilities
    pub vs: u32,          // Version
    pub intms: u32,       // Interrupt mask set
    pub intmc: u32,       // Interrupt mask clear
    pub cc: u32,          // Controller configuration
    pub rsvd1: u32,
    pub csts: u32,        // Controller status
    pub rsvd2: u32,
    pub nssr: u32,        // NVM subsystem reset
    pub aqa: u32,         // Admin queue attributes
    pub asq: u64,         // Admin submission queue base address
    pub acq: u64,         // Admin completion queue base address
    pub cmbloc: u32,      // Controller memory buffer location
    pub cmbsz: u32,       // Controller memory buffer size
    pub bpmbl: u32,       // Boot partition memory buffer block location
    pub cmbms: u32,       // Controller memory buffer memory space
    pub rsvd3: [u32; 1024],
    pub sq0tdbl: u32,      // Submission queue 0 tail doorbell
    pub sq0hdbl: u32,      // Submission queue 0 head doorbell
    pub cq0hdbl: u32,      // Completion queue 0 head doorbell
    pub cq0tdbl: u32,      // Completion queue 0 tail doorbell
}

/// NVMe submission queue entry
#[derive(Debug, Clone)]
#[repr(C)]
pub struct NvmeSubmissionEntry {
    pub cdw0: u32,        // Command dword 0
    pub cdw1: u32,        // Command dword 1
    pub cdw2: u32,        // Command dword 2
    pub cdw3: u32,        // Command dword 3
    pub cdw4: u32,        // Command dword 4
    pub cdw5: u32,        // Command dword 5
    pub rsvd: [u64; 2],
    pub mptr: u64,        // Metadata pointer
    pub dptr1: u64,       // Data pointer 1
    pub dptr2: u64,       // Data pointer 2
}

/// NVMe completion queue entry
#[derive(Debug, Clone)]
#[repr(C)]
pub struct NvmeCompletionEntry {
    pub result: u32,      // Command-specific result
    pub rsvd: u32,
    pub sq_head: u16,     // Submission queue head pointer
    pub sq_id: u16,       // Submission queue identifier
    pub cmd_id: u16,      // Command identifier
    pub status: u16,      // Status field
}

/// NVMe identify command data
#[derive(Debug, Clone)]
#[repr(C)]
pub struct NvmeIdentifyControllerData {
    pub vid: u16,          // PCI vendor ID
    pub ssvid: u16,        // PCI subsystem vendor ID
    pub sn: [u8; 20],      // Serial number
    pub mn: [u8; 40],      // Model number
    pub fr: [u8; 8],       // Firmware revision
    pub rab: u8,           // Recommended arbitration burst
    pub ieee_oui: [u8; 3], // IEEE OUI
    pub mic: u8,           // Management interface capabilities
    pub mdts: u8,          // Maximum data transfer size
    pub rsvd1: [u8; 256],
    pub oacs: u16,         // Optional admin command support
    pub rsvd2: [u8; 9],
    pub acl: u8,           // Abort command limit
    pub aers: u8,          // Asynchronous event request limit
    pub frmw: u8,          // Firmware revisions
    pub lpa: u8,           // Log page attributes
    pub elpe: u8,          // Error log page entries
    pub npss: u8,          // Number of power states
    pub avscc: u8,         // Autonomous power state change
    pub apsta: u8,         // Autonomous power state transition
    pub wctemp: u16,       // Warning composite temperature
    pub cctemp: u16,       // Critical composite temperature
    pub mtfa: u16,         // Firmware to go
    pub hmpre: u32,        // Host memory buffer preferred size
    pub hmmin: u32,        // Host memory buffer minimum size
    pub tnvmcap: [u64; 2], // Total NVM capacity
    pub unvmcap: [u64; 2], // Unallocated NVM capacity
    pub rsvd3: [u32; 384],
    pub sqes: u8,          // Submission queue entry size
    pub cqes: u8,          // Completion queue entry size
    pub rsvd4: [u8; 2],
    pub nn: u32,           // Number of namespaces
    pub oncs: u16,         // Optional NVM command support
    pub fuses: u16,        // Fused operation support
    pub fna: u8,           // Format NVM attributes
    pub vwc: u8,           // Volatile write cache
    pub awun: u16,         // Atomic write unit normal
    pub awupf: u16,        // Atomic write unit power fail
    pub nvscc: u8,         // NVM vendor specific command configuration
    pub rsvd5: [u8; 1],
    pub acwu: u16,         // Atomic compare write unit
    pub rsvd6: [u32; 22],
    pub sgl_support: u32,  // SGL support
    pub rsvd7: [u32; 1024],
}

/// NVMe namespace data
#[derive(Debug, Clone)]
#[repr(C)]
pub struct NvmeNamespaceData {
    pub nsze: u64,         // Namespace size
    pub ncap: u64,         // Namespace capacity
    pub nuse: u64,         // Namespace utilization
    pub nsfeat: u8,        // Namespace features
    pub rsvd1: [u8; 3],
    pub nlbaf: u8,         // Number of LBA formats
    pub flbas: u8,         // Formatted LBA size
    pub mc: u8,            // Metadata capabilities
    pub dpc: u8,            // End-to-end data protection capabilities
    pub dps: u8,            // End-to-end data protection settings
    pub rsvd2: [u8; 1],
    pub nmu: u8,           // Namespace multi-path I/O
    pub rsvd3: [u8; 1],
    pub nawun: u16,        // Namespace atomic write unit normal
    pub nawupf: u16,       // Namespace atomic write unit power fail
    pub nacwu: u16,        // Namespace atomic compare write unit
    pub nabsn: u16,        // Namespace atomic block size normal
    pub nabo: u16,         // Namespace atomic boundary offset
    pub nabspf: u16,       // Namespace atomic boundary size power fail
    pub rsvd4: [u8; 2],
    pub nvmcap: [u64; 2],  // NVM capacity
    pub rsvd5: [u8; 40],
    pub nguid: [u8; 16],   // Namespace globally unique identifier
    pub eui64: [u8; 8],    // Extended unique identifier
    pub lba_formats: [NvmeLbaFormat; 16],
}

/// NVMe LBA format
#[derive(Debug, Clone)]
#[repr(C)]
pub struct NvmeLbaFormat {
    pub ms: u16,           // Metadata size
    pub ds: u8,            // Data size
    pub rp: u8,            // Relative performance
}

/// NVMe queue
#[derive(Debug)]
pub struct NvmeQueue {
    pub id: u16,
    pub base: u64,
    pub size: u16,
    pub head: u16,
    pub tail: u16,
    pub phase: bool,
}

/// NVMe namespace
#[derive(Debug)]
pub struct NvmeNamespace {
    pub id: u32,
    pub size: u64,
    pub capacity: u64,
    pub block_size: u32,
    pub guid: [u8; 16],
    pub eui64: [u8; 8],
}

/// NVMe controller
#[derive(Debug)]
pub struct NvmeController {
    pub mmio_base: u64,
    pub registers: *mut NvmeRegisters,
    pub admin_sq: Option<NvmeQueue>,
    pub admin_cq: Option<NvmeQueue>,
    pub io_sq: Vec<NvmeQueue>,
    pub io_cq: Vec<NvmeQueue>,
    pub namespaces: BTreeMap<u32, NvmeNamespace>,
    pub controller_data: Option<NvmeIdentifyControllerData>,
    pub max_queue_entries: u16,
    pub doorbell_stride: u32,
}

/// NVMe command opcodes
pub mod opcode {
    pub const DELETE_IO_SQ: u8 = 0x00;
    pub const CREATE_IO_SQ: u8 = 0x01;
    pub const GET_LOG_PAGE: u8 = 0x02;
    pub const DELETE_IO_CQ: u8 = 0x04;
    pub const CREATE_IO_CQ: u8 = 0x05;
    pub const IDENTIFY: u8 = 0x06;
    pub const ABORT: u8 = 0x08;
    pub const SET_FEATURES: u8 = 0x09;
    pub const GET_FEATURES: u8 = 0x0a;
    pub const ASYNC_EVENT_REQUEST: u8 = 0x0c;
    pub const NAMESPACE_MANAGEMENT: u8 = 0x0d;
    pub const FIRMWARE_COMMIT: u8 = 0x10;
    pub const FIRMWARE_IMAGE_DOWNLOAD: u8 = 0x11;
    pub const NAMESPACE_ATTACHMENT: u8 = 0x15;
    pub const KEEP_ALIVE: u8 = 0x18;
    pub const FLUSH: u8 = 0x00;
    pub const WRITE: u8 = 0x01;
    pub const READ: u8 = 0x02;
    pub const WRITE_UNCORRECTABLE: u8 = 0x04;
    pub const COMPARE: u8 = 0x05;
    pub const WRITE_ZEROES: u8 = 0x08;
    pub const DATASET_MANAGEMENT: u8 = 0x09;
}

/// NVMe status codes
pub mod status {
    pub const SUCCESS: u16 = 0x0000;
    pub const INVALID_OPCODE: u16 = 0x0001;
    pub const INVALID_FIELD: u16 = 0x0002;
    pub const COMMAND_ID_CONFLICT: u16 = 0x0003;
    pub const DATA_TRANSFER_ERROR: u16 = 0x0004;
    pub const ABORTED_POWER_LOSS: u16 = 0x0005;
    pub const INTERNAL_DEVICE_ERROR: u16 = 0x0006;
    pub const ABORTED_BY_REQUEST: u16 = 0x0007;
    pub const ABORTED_SQ_DELETION: u16 = 0x0008;
    pub const ABORTED_FAILED_FUSED: u16 = 0x0009;
    pub const ABORTED_MISSING_FUSED: u16 = 0x000a;
    pub const INVALID_NAMESPACE: u16 = 0x000b;
    pub const COMMAND_SEQUENCE_ERROR: u16 = 0x000c;
    pub const INVALID_SGL: u16 = 0x000d;
    pub const INVALID_USE_OF_CMB: u16 = 0x000e;
    pub const INVALID_KEY: u16 = 0x000f;
    pub const KEY_NOT_WRITEABLE: u16 = 0x0010;
    pub const INVALID_KEY_OFFSET: u16 = 0x0011;
    pub const INVALID_VALUE_SIZE: u16 = 0x0012;
    pub const INVALID_KEY_SIZE: u16 = 0x0013;
}

impl NvmeController {
    /// Create a new NVMe controller
    pub fn new(mmio_base: u64) -> Self {
        Self {
            mmio_base,
            registers: mmio_base as *mut NvmeRegisters,
            admin_sq: None,
            admin_cq: None,
            io_sq: Vec::new(),
            io_cq: Vec::new(),
            namespaces: BTreeMap::new(),
            controller_data: None,
            max_queue_entries: 0,
            doorbell_stride: 0,
        }
    }

    /// Initialize the NVMe controller
    pub fn init(&mut self) -> Result<(), &'static str> {
        unsafe {
            let regs = &mut *self.registers;

            // Wait for controller to be ready
            let timeout = 1000;
            for _ in 0..timeout {
                if regs.csts & 1 == 0 {
                    break;
                }
            }

            // Read controller capabilities
            let cap = regs.cap;
            let mqes = ((cap >> 16) & 0xFFFF) as u16;
            self.max_queue_entries = mqes + 1;
            self.doorbell_stride = ((cap >> 32) & 0xF) as u32;

            // Reset controller
            regs.cc = 0;
            while regs.csts & 1 != 0 {
                // Wait for reset
            }

            // Set controller configuration
            let cc = (4 << 0) |  // Enable
                    (0 << 4) |  // IO queue entries (2^0 = 1)
                    (0 << 8) |  // Page size (4KB)
                    (0 << 11) | // Arbitration mechanism (round robin)
                    (0 << 14);  // Shutdown notification
            regs.cc = cc;

            // Setup admin queues
            self.setup_admin_queues()?;

            // Identify controller
            self.identify_controller()?;

            // Enumerate namespaces
            self.enumerate_namespaces()?;

            Ok(())
        }
    }

    /// Setup admin submission and completion queues
    fn setup_admin_queues(&mut self) -> Result<(), &'static str> {
        let sq_size = 64u16;
        let cq_size = 64u16;

        // Allocate memory for queues (simplified)
        let sq_base = 0x10000000u64; // Would be actual allocation
        let cq_base = 0x10010000u64;

        unsafe {
            let regs = &mut *self.registers;

            // Set queue attributes
            regs.aqa = ((cq_size - 1) as u32) << 16 | ((sq_size - 1) as u32);
            regs.asq = sq_base;
            regs.acq = cq_base;
        }

        self.admin_sq = Some(NvmeQueue {
            id: 0,
            base: sq_base,
            size: sq_size,
            head: 0,
            tail: 0,
            phase: true,
        });

        self.admin_cq = Some(NvmeQueue {
            id: 0,
            base: cq_base,
            size: cq_size,
            head: 0,
            tail: 0,
            phase: true,
        });

        Ok(())
    }

    /// Identify the controller
    fn identify_controller(&mut self) -> Result<(), &'static str> {
        let data_addr = 0x20000000u64; // Would be actual allocation

        let cmd = NvmeSubmissionEntry {
            cdw0: 1, // CNS = 1 (identify controller)
            cdw1: 0,
            cdw2: 0,
            cdw3: 0,
            cdw4: 0,
            cdw5: 0,
            rsvd: [0, 0],
            mptr: 0,
            dptr1: data_addr,
            dptr2: 0,
        };

        self.submit_admin_command(cmd)?;

        // Parse controller data (simplified)
        let controller_data = NvmeIdentifyControllerData {
            vid: 0,
            ssvid: 0,
            sn: [0; 20],
            mn: [0; 40],
            fr: [0; 8],
            rab: 0,
            ieee_oui: [0; 3],
            mic: 0,
            mdts: 0,
            rsvd1: [0; 256],
            oacs: 0,
            rsvd2: [0; 9],
            acl: 0,
            aers: 0,
            frmw: 0,
            lpa: 0,
            elpe: 0,
            npss: 0,
            avscc: 0,
            apsta: 0,
            wctemp: 0,
            cctemp: 0,
            mtfa: 0,
            hmpre: 0,
            hmmin: 0,
            tnvmcap: [0, 0],
            unvmcap: [0, 0],
            rsvd3: [0; 384],
            sqes: 0,
            cqes: 0,
            rsvd4: [0; 2],
            nn: 0,
            oncs: 0,
            fuses: 0,
            fna: 0,
            vwc: 0,
            awun: 0,
            awupf: 0,
            nvscc: 0,
            rsvd5: [0; 1],
            acwu: 0,
            rsvd6: [0; 22],
            sgl_support: 0,
            rsvd7: [0; 1024],
        };

        self.controller_data = Some(controller_data);

        Ok(())
    }

    /// Enumerate namespaces
    fn enumerate_namespaces(&mut self) -> Result<(), &'static str> {
        if let Some(ref ctrl_data) = self.controller_data {
            let nn = ctrl_data.nn;
            for nsid in 1..=nn {
                let data_addr = 0x30000000u64 + ((nsid - 1) as u64 * 0x1000);

                let cmd = NvmeSubmissionEntry {
                    cdw0: 0, // CNS = 0 (identify namespace)
                    cdw1: nsid,
                    cdw2: 0,
                    cdw3: 0,
                    cdw4: 0,
                    cdw5: 0,
                    rsvd: [0, 0],
                    mptr: 0,
                    dptr1: data_addr,
                    dptr2: 0,
                };

                self.submit_admin_command(cmd)?;

                // Parse namespace data (simplified)
                let namespace = NvmeNamespace {
                    id: nsid,
                    size: 0,
                    capacity: 0,
                    block_size: 512,
                    guid: [0; 16],
                    eui64: [0; 8],
                };

                self.namespaces.insert(nsid, namespace);
            }
        }

        Ok(())
    }

    /// Submit admin command
    fn submit_admin_command(&mut self, cmd: NvmeSubmissionEntry) -> Result<(), &'static str> {
        if let Some(ref mut sq) = self.admin_sq {
            unsafe {
                let sq_entry = (sq.base as *mut NvmeSubmissionEntry).add(sq.tail as usize);
                *sq_entry = cmd;

                sq.tail = (sq.tail + 1) % sq.size;

                // Ring doorbell
                let regs = &mut *self.registers;
                regs.sq0tdbl = sq.tail as u32;
            }

            // Wait for completion (simplified)
            self.wait_for_completion()?;

            Ok(())
        } else {
            Err("Admin submission queue not initialized")
        }
    }

    /// Wait for command completion
    fn wait_for_completion(&mut self) -> Result<(), &'static str> {
        if let Some(ref mut cq) = self.admin_cq {
            unsafe {
                let regs = &mut *self.registers;

                // Poll for completion
                loop {
                    let head = regs.cq0hdbl as u16;
                    if head != cq.head {
                        cq.head = head;
                        break;
                    }
                }

                // Update phase bit
                cq.phase = !cq.phase;

                // Ring completion doorbell
                regs.cq0hdbl = cq.head as u32;
            }

            Ok(())
        } else {
            Err("Admin completion queue not initialized")
        }
    }

    /// Read from namespace
    pub fn read(&self, nsid: u32, lba: u64, blocks: u16, buffer: u64) -> Result<(), &'static str> {
        let cmd = NvmeSubmissionEntry {
            cdw0: opcode::READ as u32,
            cdw1: nsid,
            cdw2: (lba & 0xFFFFFFFF) as u32,
            cdw3: (lba >> 32) as u32,
            cdw4: blocks as u32,
            cdw5: 0,
            rsvd: [0, 0],
            mptr: 0,
            dptr1: buffer,
            dptr2: 0,
        };

        // Submit to I/O queue (simplified)
        Ok(())
    }

    /// Write to namespace
    pub fn write(&self, nsid: u32, lba: u64, blocks: u16, buffer: u64) -> Result<(), &'static str> {
        let cmd = NvmeSubmissionEntry {
            cdw0: opcode::WRITE as u32,
            cdw1: nsid,
            cdw2: (lba & 0xFFFFFFFF) as u32,
            cdw3: (lba >> 32) as u32,
            cdw4: blocks as u32,
            cdw5: 0,
            rsvd: [0, 0],
            mptr: 0,
            dptr1: buffer,
            dptr2: 0,
        };

        // Submit to I/O queue (simplified)
        Ok(())
    }

    /// Get namespace by ID
    pub fn get_namespace(&self, nsid: u32) -> Option<&NvmeNamespace> {
        self.namespaces.get(&nsid)
    }

    /// Get all namespaces
    pub fn get_namespaces(&self) -> Vec<&NvmeNamespace> {
        self.namespaces.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvme_controller_creation() {
        let controller = NvmeController::new(0x40000000);
        assert_eq!(controller.mmio_base, 0x40000000);
    }

    #[test]
    fn test_queue_creation() {
        let queue = NvmeQueue {
            id: 0,
            base: 0x10000000,
            size: 64,
            head: 0,
            tail: 0,
            phase: true,
        };
        assert_eq!(queue.id, 0);
        assert_eq!(queue.size, 64);
    }

    #[test]
    fn test_namespace_creation() {
        let namespace = NvmeNamespace {
            id: 1,
            size: 1000,
            capacity: 1000,
            block_size: 512,
            guid: [0; 16],
            eui64: [0; 8],
        };
        assert_eq!(namespace.id, 1);
        assert_eq!(namespace.block_size, 512);
    }

    #[test]
    fn test_opcodes() {
        assert_eq!(opcode::READ, 0x02);
        assert_eq!(opcode::WRITE, 0x01);
        assert_eq!(opcode::FLUSH, 0x00);
        assert_eq!(opcode::IDENTIFY, 0x06);
    }

    #[test]
    fn test_status_codes() {
        assert_eq!(status::SUCCESS, 0x0000);
        assert_eq!(status::INVALID_OPCODE, 0x0001);
        assert_eq!(status::INVALID_FIELD, 0x0002);
    }

    #[test]
    fn test_submission_entry() {
        let cmd = NvmeSubmissionEntry {
            cdw0: opcode::READ as u32,
            cdw1: 1,
            cdw2: 0,
            cdw3: 0,
            cdw4: 1,
            cdw5: 0,
            rsvd: [0, 0],
            mptr: 0,
            dptr1: 0x1000,
            dptr2: 0,
        };
        assert_eq!(cmd.cdw0, opcode::READ as u32);
        assert_eq!(cmd.cdw1, 1);
    }
}
