// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/net/sigma_e1000.rs — Intel e1000 Gigabit NIC Driver
// Replaces: SovereignE1000.cpp (C++ stub, removed)
//
// Language: Rust #![no_std] — no libc, no alloc, no third-party crates
// Pattern: OOP via E1000Driver struct implementing NicDevice trait

#![no_std]

use crate::kernel::net::sigma_net::{MacAddr, MTU, NicDevice, RX_RING_SZ, TX_RING_SZ};

// ── MMIO Register Offsets ────────────────────────────────────────────────────

const REG_CTRL:    usize = 0x0000;  // Device Control
const REG_STATUS:  usize = 0x0008;  // Device Status
const REG_EECD:    usize = 0x0010;  // EEPROM Control/Data
const REG_EERD:    usize = 0x0014;  // EEPROM Read
const REG_ICR:     usize = 0x00C0;  // Interrupt Cause Read
const REG_IMS:     usize = 0x00D0;  // Interrupt Mask Set
const REG_IMC:     usize = 0x00D8;  // Interrupt Mask Clear
const REG_RCTL:    usize = 0x0100;  // Receive Control
const REG_TCTL:    usize = 0x0400;  // Transmit Control
const REG_RDBAL:   usize = 0x2800;  // RX Descriptor Base Low
const REG_RDBAH:   usize = 0x2804;  // RX Descriptor Base High
const REG_RDLEN:   usize = 0x2808;  // RX Descriptor Length
const REG_RDH:     usize = 0x2810;  // RX Descriptor Head
const REG_RDT:     usize = 0x2818;  // RX Descriptor Tail
const REG_TDBAL:   usize = 0x3800;  // TX Descriptor Base Low
const REG_TDBAH:   usize = 0x3804;  // TX Descriptor Base High
const REG_TDLEN:   usize = 0x3808;  // TX Descriptor Length
const REG_TDH:     usize = 0x3810;  // TX Descriptor Head
const REG_TDT:     usize = 0x3818;  // TX Descriptor Tail
const REG_MTA:     usize = 0x5200;  // Multicast Table Array (128 entries)
const REG_RAL0:    usize = 0x5400;  // Receive Address Low
const REG_RAH0:    usize = 0x5404;  // Receive Address High

// Control register bits
const CTRL_FD:    u32 = 1 << 0;  // Full duplex
const CTRL_ASDE:  u32 = 1 << 5;  // Auto-Speed Detection Enable
const CTRL_SLU:   u32 = 1 << 6;  // Set Link Up
const CTRL_RST:   u32 = 1 << 26; // Device Reset

// Receive Control bits
const RCTL_EN:     u32 = 1 << 1;  // Receiver Enable
const RCTL_SBP:    u32 = 1 << 2;  // Store Bad Packets
const RCTL_UPE:    u32 = 1 << 3;  // Unicast Promiscuous
const RCTL_MPE:    u32 = 1 << 4;  // Multicast Promiscuous
const RCTL_BAM:    u32 = 1 << 15; // Broadcast Accept Mode
const RCTL_SZ_2K:  u32 = 0 << 16; // Buffer size 2048 bytes
const RCTL_SECRC:  u32 = 1 << 26; // Strip Ethernet CRC

// Transmit Control bits
const TCTL_EN:    u32 = 1 << 1;   // Transmit Enable
const TCTL_PSP:   u32 = 1 << 3;   // Pad Short Packets
const TCTL_CT:    u32 = 0x10 << 4; // Collision Threshold = 16
const TCTL_COLD:  u32 = 0x40 << 12; // Collision Distance = 64

// ── Descriptor Types ─────────────────────────────────────────────────────────

#[repr(C, align(16))]
struct RxDesc {
    addr:   u64,
    length: u16,
    chksum: u16,
    status: u8,
    errors: u8,
    special:u16,
}

#[repr(C, align(16))]
struct TxDesc {
    addr:    u64,
    length:  u16,
    cso:     u8,
    cmd:     u8,
    status:  u8,
    css:     u8,
    special: u16,
}

const TX_CMD_EOP:  u8 = 1 << 0; // End of Packet
const TX_CMD_IFCS: u8 = 1 << 1; // Insert FCS
const TX_CMD_RS:   u8 = 1 << 3; // Report Status

// ── Driver ───────────────────────────────────────────────────────────────────

pub struct E1000Driver {
    mmio_base:  usize,
    mac:        MacAddr,
    rx_descs:   [RxDesc; RX_RING_SZ],
    tx_descs:   [TxDesc; TX_RING_SZ],
    rx_bufs:    [[u8; 2048]; RX_RING_SZ],
    tx_bufs:    [[u8; MTU];  TX_RING_SZ],
    rx_cur:     usize,
    tx_cur:     usize,
}

impl E1000Driver {
    pub fn new(mmio_base: usize) -> Self {
        let mut d = Self {
            mmio_base,
            mac:      MacAddr([0; 6]),
            rx_descs: core::array::from_fn(|_| RxDesc {
                addr: 0, length: 0, chksum: 0, status: 0, errors: 0, special: 0,
            }),
            tx_descs: core::array::from_fn(|_| TxDesc {
                addr: 0, length: 0, cso: 0, cmd: 0, status: 0, css: 0, special: 0,
            }),
            rx_bufs: [[0u8; 2048]; RX_RING_SZ],
            tx_bufs: [[0u8; MTU];  TX_RING_SZ],
            rx_cur:  0,
            tx_cur:  0,
        };
        d
    }

    /// Probe: check vendor/device ID via PCI config space
    pub fn probe(vendor: u16, device: u16) -> bool {
        vendor == 0x8086 && matches!(device,
            0x100E | 0x100F | 0x10D3 | 0x1533 | 0x1563 | 0x15B7 | 0x15B8)
    }

    pub fn init(&mut self) {
        // 1. Reset device
        self.write32(REG_CTRL, CTRL_RST);
        // Spin until reset clears
        for _ in 0..10_000 {
            if self.read32(REG_CTRL) & CTRL_RST == 0 { break; }
        }
        // 2. Set link up, full-duplex, auto-speed
        self.write32(REG_CTRL, CTRL_SLU | CTRL_ASDE | CTRL_FD);

        // 3. Read MAC from RAL0/RAH0
        let ral = self.read32(REG_RAL0);
        let rah = self.read32(REG_RAH0);
        self.mac.0[0] = (ral & 0xFF) as u8;
        self.mac.0[1] = ((ral >> 8) & 0xFF) as u8;
        self.mac.0[2] = ((ral >> 16) & 0xFF) as u8;
        self.mac.0[3] = ((ral >> 24) & 0xFF) as u8;
        self.mac.0[4] = (rah & 0xFF) as u8;
        self.mac.0[5] = ((rah >> 8) & 0xFF) as u8;

        // 4. Clear multicast table
        for i in 0..128usize {
            self.write32(REG_MTA + i * 4, 0);
        }

        // 5. Set up RX descriptors
        for i in 0..RX_RING_SZ {
            self.rx_descs[i].addr   = self.rx_bufs[i].as_ptr() as u64;
            self.rx_descs[i].status = 0;
        }
        let rdba = self.rx_descs.as_ptr() as u64;
        self.write32(REG_RDBAL, (rdba & 0xFFFF_FFFF) as u32);
        self.write32(REG_RDBAH, (rdba >> 32) as u32);
        self.write32(REG_RDLEN, (RX_RING_SZ * core::mem::size_of::<RxDesc>()) as u32);
        self.write32(REG_RDH, 0);
        self.write32(REG_RDT, (RX_RING_SZ - 1) as u32);

        // 6. Set up TX descriptors
        for i in 0..TX_RING_SZ {
            self.tx_descs[i].status = 0xFF; // mark all as done initially
        }
        let tdba = self.tx_descs.as_ptr() as u64;
        self.write32(REG_TDBAL, (tdba & 0xFFFF_FFFF) as u32);
        self.write32(REG_TDBAH, (tdba >> 32) as u32);
        self.write32(REG_TDLEN, (TX_RING_SZ * core::mem::size_of::<TxDesc>()) as u32);
        self.write32(REG_TDH, 0);
        self.write32(REG_TDT, 0);

        // 7. Enable RX and TX
        self.write32(REG_RCTL, RCTL_EN | RCTL_BAM | RCTL_SZ_2K | RCTL_SECRC);
        self.write32(REG_TCTL, TCTL_EN | TCTL_PSP | TCTL_CT | TCTL_COLD);
    }

    fn read32(&self, offset: usize) -> u32 {
        let ptr = (self.mmio_base + offset) as *const volatile u32;
        unsafe { ptr.read_volatile() }
    }

    fn write32(&self, offset: usize, val: u32) {
        let ptr = (self.mmio_base + offset) as *mut volatile u32;
        unsafe { ptr.write_volatile(val) }
    }
}

impl NicDevice for E1000Driver {
    fn mac(&self) -> MacAddr { self.mac }

    fn recv(&mut self, buf: &mut [u8; MTU]) -> usize {
        let desc = &self.rx_descs[self.rx_cur];
        if desc.status & 0x01 == 0 { return 0; } // DD bit not set
        let len = (desc.length as usize).min(MTU);
        buf[..len].copy_from_slice(&self.rx_bufs[self.rx_cur][..len]);
        // Reset descriptor and advance tail
        self.rx_descs[self.rx_cur].status = 0;
        self.write32(REG_RDT, self.rx_cur as u32);
        self.rx_cur = (self.rx_cur + 1) % RX_RING_SZ;
        len
    }

    fn send(&mut self, buf: &[u8], len: usize) {
        let len = len.min(MTU);
        self.tx_bufs[self.tx_cur][..len].copy_from_slice(&buf[..len]);
        let desc = &mut self.tx_descs[self.tx_cur];
        desc.addr   = self.tx_bufs[self.tx_cur].as_ptr() as u64;
        desc.length = len as u16;
        desc.cmd    = TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS;
        desc.status = 0;
        self.tx_cur = (self.tx_cur + 1) % TX_RING_SZ;
        self.write32(REG_TDT, self.tx_cur as u32);
        // Spin-wait for TX done (DD bit)
        let mut tries = 0usize;
        while self.tx_descs[(self.tx_cur + TX_RING_SZ - 1) % TX_RING_SZ].status & 0xFF == 0
              && tries < 10_000
        { tries += 1; }
    }

    fn link_up(&self) -> bool {
        self.read32(REG_STATUS) & (1 << 1) != 0 // LU bit
    }
}
