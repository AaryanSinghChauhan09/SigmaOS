// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// drivers/storage/sigma_ahci.rs — SATA AHCI Controller Driver
// Language: Rust #![no_std] — no libc, no third-party, OOP via AhciController

#![no_std]
use crate::drivers::storage::sigma_nvme::BlockDevice;

// ── AHCI HBA Memory Registers ─────────────────────────────────────────────────
const HBA_CAP:     usize = 0x00;
const HBA_GHC:     usize = 0x04;
const HBA_IS:      usize = 0x08;
const HBA_PI:      usize = 0x0C;  // Ports Implemented bitmask
const HBA_VS:      usize = 0x10;
const PORT_BASE:   usize = 0x100; // Port 0 starts here
const PORT_SIZE:   usize = 0x80;  // each port = 128 bytes

// Port register offsets (relative to port base)
const PORT_CLB:    usize = 0x00;  // Command List Base Address (low)
const PORT_CLBU:   usize = 0x04;  // Command List Base Address (high)
const PORT_FB:     usize = 0x08;  // FIS Base Address (low)
const PORT_FBU:    usize = 0x0C;
const PORT_IS:     usize = 0x10;  // Interrupt Status
const PORT_IE:     usize = 0x14;  // Interrupt Enable
const PORT_CMD:    usize = 0x18;  // Command and Status
const PORT_TFD:    usize = 0x20;  // Task File Data
const PORT_SIG:    usize = 0x24;  // Signature
const PORT_SSTS:   usize = 0x28;  // SATA Status
const PORT_SERR:   usize = 0x30;  // SATA Error
const PORT_SACT:   usize = 0x34;  // SATA Active
const PORT_CI:     usize = 0x38;  // Command Issue

// CMD register bits
const CMD_ST:  u32 = 1 << 0;   // Start
const CMD_FRE: u32 = 1 << 4;   // FIS Receive Enable
const CMD_FR:  u32 = 1 << 14;  // FIS Receive Running
const CMD_CR:  u32 = 1 << 15;  // Command List Running

// ATA commands
const ATA_CMD_READ_DMA_EX:  u8 = 0x25;
const ATA_CMD_WRITE_DMA_EX: u8 = 0x35;
const ATA_CMD_IDENTIFY:     u8 = 0xEC;

const MAX_PORTS: usize = 32;
const BLOCK_SIZE: usize = 512;

// ── FIS Types ─────────────────────────────────────────────────────────────────
const FIS_TYPE_REG_H2D: u8 = 0x27;
const FIS_TYPE_REG_D2H: u8 = 0x34;

// ── Command Header (32 bytes) ─────────────────────────────────────────────────
#[repr(C)]
struct CmdHeader {
    dw0:    u32,  // PRDTL[15:0], PMP[19:16], bits 20-23, ATAPI, W, P, R, B, C, bit31
    dw1:    u32,  // PRDBC
    ctba:   u64,  // Command Table Base Address
    _rsvd:  [u32; 4],
}

// ── Physical Region Descriptor Table Entry (16 bytes) ────────────────────────
#[repr(C)]
struct PrdtEntry {
    dba:  u64,    // Data Base Address
    _rsvd: u32,
    dbc:  u32,    // Data Byte Count | IOC bit 31
}

// ── Host-to-Device Register FIS (20 bytes) ───────────────────────────────────
#[repr(C)]
struct FisH2D {
    fis_type:  u8,
    pmport:    u8,   // bit 7 = command (1) / control (0)
    command:   u8,
    featurel:  u8,
    lba0:      u8,
    lba1:      u8,
    lba2:      u8,
    device:    u8,
    lba3:      u8,
    lba4:      u8,
    lba5:      u8,
    featureh:  u8,
    countl:    u8,
    counth:    u8,
    _icc:      u8,
    control:   u8,
    _aux:      [u8; 4],
}

// ── AHCI Port ─────────────────────────────────────────────────────────────────
struct AhciPort {
    index:   usize,
    mmio:    usize,  // port MMIO base
    cmd_list: [CmdHeader; 32],
    fis_buf:  [u8; 256],
    cmd_table: [[u8; 256]; 32],
    prdt:    [[PrdtEntry; 1]; 32],
    sectors: u64,   // device capacity
    present: bool,
}

impl AhciPort {
    fn port_read32(&self, off: usize) -> u32 {
        unsafe { ((self.mmio + off) as *const volatile u32).read_volatile() }
    }
    fn port_write32(&self, off: usize, v: u32) {
        unsafe { ((self.mmio + off) as *mut volatile u32).write_volatile(v); }
    }

    fn start_cmd(&self) {
        // Wait until CR is clear
        let mut i = 0u32;
        while self.port_read32(PORT_CMD) & CMD_CR != 0 && i < 1_000_000 { i += 1; }
        let cmd = self.port_read32(PORT_CMD);
        self.port_write32(PORT_CMD, cmd | CMD_FRE | CMD_ST);
    }

    fn stop_cmd(&self) {
        let cmd = self.port_read32(PORT_CMD);
        self.port_write32(PORT_CMD, cmd & !(CMD_ST));
        let mut i = 0u32;
        while self.port_read32(PORT_CMD) & (CMD_FR | CMD_CR) != 0 && i < 1_000_000 { i += 1; }
        let cmd = self.port_read32(PORT_CMD);
        self.port_write32(PORT_CMD, cmd & !(CMD_FRE));
    }

    fn issue_cmd(&self, slot: usize, data_phys: u64, byte_count: u32,
                 lba: u64, count: u16, write: bool) -> bool {
        // Build command table FIS
        let ct_ptr = self.cmd_table[slot].as_ptr() as usize;
        unsafe {
            let fis = ct_ptr as *mut FisH2D;
            (*fis).fis_type = FIS_TYPE_REG_H2D;
            (*fis).pmport   = 0x80; // command
            (*fis).command  = if write { ATA_CMD_WRITE_DMA_EX } else { ATA_CMD_READ_DMA_EX };
            (*fis).device   = 1 << 6; // LBA mode
            (*fis).lba0     = (lba & 0xFF) as u8;
            (*fis).lba1     = ((lba >>  8) & 0xFF) as u8;
            (*fis).lba2     = ((lba >> 16) & 0xFF) as u8;
            (*fis).lba3     = ((lba >> 24) & 0xFF) as u8;
            (*fis).lba4     = ((lba >> 32) & 0xFF) as u8;
            (*fis).lba5     = ((lba >> 40) & 0xFF) as u8;
            (*fis).countl   = (count & 0xFF) as u8;
            (*fis).counth   = ((count >> 8) & 0xFF) as u8;
        }
        // Set PRDT entry
        unsafe {
            let prdt = &self.cmd_table[slot].as_ptr().add(0x80) as *const _ as *mut PrdtEntry;
            (*prdt).dba = data_phys;
            (*prdt).dbc = (byte_count - 1) | (1 << 31); // IOC
        }
        // Set command header
        let cl_ptr = self.cmd_list.as_ptr() as usize;
        unsafe {
            let hdr = (cl_ptr + slot * core::mem::size_of::<CmdHeader>()) as *mut CmdHeader;
            let flags: u32 = (core::mem::size_of::<FisH2D>() as u32 / 4)
                | if write { 1 << 6 } else { 0 } // W bit
                | (1 << 16); // PRDTL = 1
            (*hdr).dw0  = flags;
            (*hdr).ctba = ct_ptr as u64;
        }
        // Clear error + issue
        self.port_write32(PORT_SERR, 0xFFFFFFFF);
        self.port_write32(PORT_CI, 1 << slot);
        // Wait for completion
        let mut i = 0u32;
        while self.port_read32(PORT_CI) & (1 << slot) != 0 && i < 2_000_000 { i += 1; }
        self.port_read32(PORT_TFD) & 0x01 == 0 // BSY clear = success
    }
}

// ── AHCI Controller ───────────────────────────────────────────────────────────
pub struct AhciController {
    hba_mmio: usize,
    ports:    [Option<AhciPort>; MAX_PORTS],
    n_ports:  usize,
    active:   usize,  // index of first usable port
}

impl AhciController {
    pub fn new(mmio: usize) -> Self {
        Self { hba_mmio: mmio, ports: core::array::from_fn(|_| None), n_ports: 0, active: 0 }
    }

    pub fn probe(class: u8, subclass: u8, prog_if: u8) -> bool {
        class == 0x01 && subclass == 0x06 && prog_if == 0x01
    }

    pub fn init(&mut self) -> bool {
        // Enable AHCI mode
        let ghc = self.read32(HBA_GHC);
        self.write32(HBA_GHC, ghc | (1 << 31));  // AE bit

        let pi = self.read32(HBA_PI);
        for i in 0..MAX_PORTS {
            if pi & (1 << i) == 0 { continue; }
            let port_mmio = self.hba_mmio + PORT_BASE + i * PORT_SIZE;
            let ssts = unsafe { ((port_mmio + PORT_SSTS) as *const volatile u32).read_volatile() };
            let det = ssts & 0xF;
            let ipm = (ssts >> 8) & 0xF;
            if det != 3 || ipm != 1 { continue; } // no device present/active
            let sig = unsafe { ((port_mmio + PORT_SIG) as *const volatile u32).read_volatile() };
            // sig == 0x00000101 = SATA, 0xEB140101 = SATAPI
            if sig != 0x00000101 && sig != 0xEB140101 { continue; }

            let mut port = AhciPort {
                index: i, mmio: port_mmio,
                cmd_list:  core::array::from_fn(|_| CmdHeader { dw0:0,dw1:0,ctba:0,_rsvd:[0;4] }),
                fis_buf:   [0u8; 256],
                cmd_table: [[0u8; 256]; 32],
                prdt:      core::array::from_fn(|_| [PrdtEntry{dba:0,_rsvd:0,dbc:0}; 1]),
                sectors:   0,
                present:   true,
            };
            port.stop_cmd();
            // Set CLB and FB pointers
            let clb = port.cmd_list.as_ptr() as u64;
            let fb  = port.fis_buf.as_ptr() as u64;
            unsafe {
                ((port_mmio + PORT_CLB) as *mut volatile u32).write_volatile((clb & 0xFFFFFFFF) as u32);
                ((port_mmio + PORT_CLBU) as *mut volatile u32).write_volatile((clb >> 32) as u32);
                ((port_mmio + PORT_FB)  as *mut volatile u32).write_volatile((fb  & 0xFFFFFFFF) as u32);
                ((port_mmio + PORT_FBU) as *mut volatile u32).write_volatile((fb  >> 32) as u32);
            }
            port.port_write32(PORT_SERR, 0xFFFFFFFF);
            port.start_cmd();
            if self.active == 0 || self.n_ports == 0 { self.active = i; }
            self.ports[i] = Some(port);
            self.n_ports += 1;
        }
        self.n_ports > 0
    }

    fn read32(&self, off: usize) -> u32 {
        unsafe { ((self.hba_mmio + off) as *const volatile u32).read_volatile() }
    }
    fn write32(&self, off: usize, v: u32) {
        unsafe { ((self.hba_mmio + off) as *mut volatile u32).write_volatile(v); }
    }
}

impl BlockDevice for AhciController {
    fn block_size(&self) -> usize { BLOCK_SIZE }
    fn block_count(&self) -> u64 {
        self.ports[self.active].as_ref().map(|p| p.sectors).unwrap_or(0)
    }
    fn read_blocks(&mut self, lba: u64, count: u16, buf: &mut [u8]) -> bool {
        if let Some(ref port) = self.ports[self.active] {
            port.issue_cmd(0, buf.as_ptr() as u64, count as u32 * 512, lba, count, false)
        } else { false }
    }
    fn write_blocks(&mut self, lba: u64, count: u16, buf: &[u8]) -> bool {
        if let Some(ref port) = self.ports[self.active] {
            port.issue_cmd(0, buf.as_ptr() as u64, count as u32 * 512, lba, count, true)
        } else { false }
    }
    fn flush(&mut self) -> bool { true }
}
