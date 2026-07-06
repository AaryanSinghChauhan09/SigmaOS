// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/drivers/SovereignE1000.rs — Intel e1000 NIC Driver
//
// Implements a minimal but functional Intel 8254x (e1000) Gigabit Ethernet
// driver for the SigmaOS Sovereign HAL.
//
// Design mirrors Linux drivers/net/ethernet/intel/e1000/ (GPLv2) but
// re-implemented from scratch for no_std, no alloc, no external crates.
//
// Supports:
//   - MMIO register read/write
//   - PHY link detection
//   - 32-entry TX/RX descriptor rings (statically allocated)
//   - Packet transmit + receive poll
//   - MAC address programming from EEPROM
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaUsize = usize;
type SigmaBool  = bool;

// ── e1000 MMIO Register Offsets (Intel 8254x datasheet §13) ──────────────────
const E1000_CTRL:   u32 = 0x0000; // Device Control
const E1000_STATUS: u32 = 0x0008; // Device Status
const E1000_EECD:   u32 = 0x0010; // EEPROM/Flash Control
const E1000_EERD:   u32 = 0x0014; // EEPROM Read
const E1000_ICR:    u32 = 0x00C0; // Interrupt Cause Read
const E1000_ICS:    u32 = 0x00C8; // Interrupt Cause Set
const E1000_IMS:    u32 = 0x00D0; // Interrupt Mask Set/Read
const E1000_IMC:    u32 = 0x00D8; // Interrupt Mask Clear
const E1000_RCTL:   u32 = 0x0100; // Receive Control
const E1000_TCTL:   u32 = 0x0400; // Transmit Control
const E1000_RDBAL:  u32 = 0x2800; // RX Descriptor Base Low
const E1000_RDBAH:  u32 = 0x2804; // RX Descriptor Base High
const E1000_RDLEN:  u32 = 0x2808; // RX Descriptor Length
const E1000_RDH:    u32 = 0x2810; // RX Descriptor Head
const E1000_RDT:    u32 = 0x2818; // RX Descriptor Tail
const E1000_TDBAL:  u32 = 0x3800; // TX Descriptor Base Low
const E1000_TDBAH:  u32 = 0x3804; // TX Descriptor Base High
const E1000_TDLEN:  u32 = 0x3808; // TX Descriptor Length
const E1000_TDH:    u32 = 0x3810; // TX Descriptor Head
const E1000_TDT:    u32 = 0x3818; // TX Descriptor Tail
const E1000_MTA:    u32 = 0x5200; // Multicast Table Array (128 entries × 4 B)
const E1000_RAL:    u32 = 0x5400; // Receive Address Low
const E1000_RAH:    u32 = 0x5404; // Receive Address High

// CTRL register bits
const CTRL_RST:  u32 = 1 << 26; // Software reset
const CTRL_SLU:  u32 = 1 << 6;  // Set Link Up
const CTRL_ASDE: u32 = 1 << 5;  // Auto-Speed Detection Enable
const CTRL_FD:   u32 = 1 << 0;  // Full-Duplex

// RCTL register bits
const RCTL_EN:   u32 = 1 << 1;  // Receiver Enable
const RCTL_BAM:  u32 = 1 << 15; // Broadcast Accept Mode
const RCTL_SZ_2048: u32 = 0 << 16; // Buffer size 2048 bytes

// TCTL register bits
const TCTL_EN:   u32 = 1 << 1;  // Transmit Enable
const TCTL_PSP:  u32 = 1 << 3;  // Pad Short Packets
const TCTL_CT_SHIFT: u32 = 4;   // Collision Threshold shift

// TX descriptor command bits
const TX_CMD_EOP:  u8 = 1 << 0; // End of Packet
const TX_CMD_IFCS: u8 = 1 << 1; // Insert FCS
const TX_CMD_RS:   u8 = 1 << 3; // Report Status

// TX/RX descriptor status bits
const DESC_STATUS_DD: u8 = 1 << 0; // Descriptor Done

// ── Descriptor Ring Sizes ─────────────────────────────────────────────────────
const TX_RING_SIZE: SigmaUsize = 32;
const RX_RING_SIZE: SigmaUsize = 32;
/// Maximum Ethernet frame size including FCS.
const MAX_FRAME_SIZE: SigmaUsize = 1522;
/// RX buffer size per descriptor (must match RCTL_SZ).
const RX_BUF_SIZE: SigmaUsize = 2048;

// ── TX Descriptor (Legacy format, §3.3.3 of Intel 8254x datasheet) ───────────
#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct TxDesc {
    /// Physical address of the packet buffer.
    pub buf_addr: SigmaU64,
    /// Packet length in bytes.
    pub length:   SigmaU16,
    /// Checksum offset.
    pub cso:      SigmaU8,
    /// Command field (EOP | IFCS | RS).
    pub cmd:      SigmaU8,
    /// Status field — DD bit set by NIC when done.
    pub status:   SigmaU8,
    /// Checksum start.
    pub css:      SigmaU8,
    /// Special field.
    pub special:  SigmaU16,
}

impl TxDesc {
    pub const fn zeroed() -> Self {
        Self { buf_addr: 0, length: 0, cso: 0, cmd: 0, status: 0, css: 0, special: 0 }
    }
}

// ── RX Descriptor (Legacy format) ────────────────────────────────────────────
#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct RxDesc {
    pub buf_addr: SigmaU64,
    pub length:   SigmaU16,
    pub checksum: SigmaU16,
    pub status:   SigmaU8,
    pub errors:   SigmaU8,
    pub special:  SigmaU16,
}

impl RxDesc {
    pub const fn zeroed() -> Self {
        Self { buf_addr: 0, length: 0, checksum: 0, status: 0, errors: 0, special: 0 }
    }
}

// ── Static Descriptor Rings and Packet Buffers ────────────────────────────────
static mut TX_RING: [TxDesc; TX_RING_SIZE] = [TxDesc::zeroed(); TX_RING_SIZE];
static mut RX_RING: [RxDesc; RX_RING_SIZE] = [RxDesc::zeroed(); RX_RING_SIZE];

/// Flat packet staging buffer for TX (one frame at a time for simplicity).
static mut TX_STAGING: [SigmaU8; MAX_FRAME_SIZE] = [0u8; MAX_FRAME_SIZE];
/// RX buffers — one per RX descriptor.
static mut RX_BUFFERS: [[SigmaU8; RX_BUF_SIZE]; RX_RING_SIZE] =
    [[0u8; RX_BUF_SIZE]; RX_RING_SIZE];

// ── Driver State ──────────────────────────────────────────────────────────────
pub struct SovereignE1000 {
    /// MMIO base address of the NIC BAR0.
    pub mmio_base:   SigmaU64,
    pub initialized: SigmaBool,
    /// Current TX ring tail pointer.
    pub tx_tail:     SigmaUsize,
    /// Current RX ring tail pointer.
    pub rx_tail:     SigmaUsize,
    /// MAC address bytes.
    pub mac:         [SigmaU8; 6],
}

impl SovereignE1000 {
    pub const fn new() -> Self {
        Self {
            mmio_base:   0xFEBC_0000, // QEMU e1000 default BAR0
            initialized: false,
            tx_tail:     0,
            rx_tail:     0,
            mac:         [0u8; 6],
        }
    }

    // ── MMIO helpers ──────────────────────────────────────────────────────────

    #[inline(always)]
    unsafe fn read32(&self, offset: u32) -> u32 {
        let addr = (self.mmio_base + offset as SigmaU64) as *const u32;
        core::ptr::read_volatile(addr)
    }

    #[inline(always)]
    unsafe fn write32(&self, offset: u32, val: u32) {
        let addr = (self.mmio_base + offset as SigmaU64) as *mut u32;
        core::ptr::write_volatile(addr, val);
    }

    // ── Busy-wait helpers ─────────────────────────────────────────────────────

    unsafe fn delay_loops(n: u32) {
        for _ in 0..n {
            core::arch::asm!("pause", options(nomem, nostack));
        }
    }

    // ── EEPROM: read MAC address ──────────────────────────────────────────────

    unsafe fn eeprom_read(&self, word_offset: u8) -> SigmaU16 {
        // Trigger read via EERD register.
        self.write32(E1000_EERD, (word_offset as u32) << 8 | 1);
        // Wait for done bit (bit 4).
        let mut retries = 10_000u32;
        while retries > 0 {
            let v = self.read32(E1000_EERD);
            if v & (1 << 4) != 0 {
                return (v >> 16) as SigmaU16;
            }
            Self::delay_loops(10);
            retries -= 1;
        }
        0xFFFF // timeout
    }

    unsafe fn read_mac_from_eeprom(&self) -> [SigmaU8; 6] {
        let w0 = self.eeprom_read(0);
        let w1 = self.eeprom_read(1);
        let w2 = self.eeprom_read(2);
        [
            (w0 & 0xFF) as SigmaU8, (w0 >> 8) as SigmaU8,
            (w1 & 0xFF) as SigmaU8, (w1 >> 8) as SigmaU8,
            (w2 & 0xFF) as SigmaU8, (w2 >> 8) as SigmaU8,
        ]
    }

    // ── Descriptor ring setup ─────────────────────────────────────────────────

    unsafe fn setup_tx_ring(&mut self) {
        let ring_phys = TX_RING.as_ptr() as SigmaU64;
        self.write32(E1000_TDBAL, (ring_phys & 0xFFFF_FFFF) as u32);
        self.write32(E1000_TDBAH, (ring_phys >> 32) as u32);
        self.write32(E1000_TDLEN, (TX_RING_SIZE * core::mem::size_of::<TxDesc>()) as u32);
        self.write32(E1000_TDH, 0);
        self.write32(E1000_TDT, 0);
        self.tx_tail = 0;
    }

    unsafe fn setup_rx_ring(&mut self) {
        // Point each RX descriptor to its static buffer.
        for i in 0..RX_RING_SIZE {
            RX_RING[i].buf_addr = RX_BUFFERS[i].as_ptr() as SigmaU64;
            RX_RING[i].status   = 0;
        }
        let ring_phys = RX_RING.as_ptr() as SigmaU64;
        self.write32(E1000_RDBAL, (ring_phys & 0xFFFF_FFFF) as u32);
        self.write32(E1000_RDBAH, (ring_phys >> 32) as u32);
        self.write32(E1000_RDLEN, (RX_RING_SIZE * core::mem::size_of::<RxDesc>()) as u32);
        self.write32(E1000_RDH, 0);
        self.write32(E1000_RDT, (RX_RING_SIZE - 1) as u32);
        self.rx_tail = RX_RING_SIZE - 1;
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// e1000_init — full NIC initialisation sequence.
    ///
    /// Sequence mirrors Linux e1000_probe() / e1000_hw_init():
    ///  1. Software reset (CTRL.RST)
    ///  2. Read MAC from EEPROM
    ///  3. Set MAC into RAL/RAH receive address registers
    ///  4. Clear multicast table
    ///  5. Set up TX descriptor ring
    ///  6. Set up RX descriptor ring
    ///  7. Enable TX (TCTL) and RX (RCTL)
    ///  8. Enable link (CTRL.SLU)
    pub unsafe fn e1000_init(&mut self) {
        // 1. Software reset — sets all registers to defaults.
        self.write32(E1000_CTRL, CTRL_RST);
        Self::delay_loops(50_000); // ~10 µs at 5 GHz
        // Wait for reset to clear.
        let mut retries = 1000u32;
        while self.read32(E1000_CTRL) & CTRL_RST != 0 && retries > 0 {
            Self::delay_loops(100);
            retries -= 1;
        }

        // 2. Disable all interrupts.
        self.write32(E1000_IMC, 0xFFFF_FFFF);

        // 3. Read MAC from EEPROM and program RAL/RAH.
        self.mac = self.read_mac_from_eeprom();
        let ral: u32 = (self.mac[0] as u32)
            | ((self.mac[1] as u32) << 8)
            | ((self.mac[2] as u32) << 16)
            | ((self.mac[3] as u32) << 24);
        let rah: u32 = (self.mac[4] as u32)
            | ((self.mac[5] as u32) << 8)
            | (1 << 31); // AV = Address Valid
        self.write32(E1000_RAL, ral);
        self.write32(E1000_RAH, rah);

        // 4. Clear multicast table (128 × 32-bit entries).
        for i in 0..128u32 {
            self.write32(E1000_MTA + i * 4, 0);
        }

        // 5. TX ring.
        self.setup_tx_ring();
        // Enable TX: EN + PSP + CT=0x10 (collision threshold 16).
        self.write32(E1000_TCTL, TCTL_EN | TCTL_PSP | (0x10 << TCTL_CT_SHIFT));

        // 6. RX ring.
        self.setup_rx_ring();
        // Enable RX: EN + BAM (accept broadcasts) + 2048-byte buffers.
        self.write32(E1000_RCTL, RCTL_EN | RCTL_BAM | RCTL_SZ_2048);

        // 7. Set link up (auto-negotiation).
        let ctrl = self.read32(E1000_CTRL);
        self.write32(E1000_CTRL, ctrl | CTRL_SLU | CTRL_ASDE);

        self.initialized = true;
    }

    /// transmit — send a raw Ethernet frame.
    ///
    /// Copies up to `len` bytes from `data` into the next TX descriptor
    /// buffer, sets command flags, advances the tail pointer.
    /// Polls for completion (DD bit) with a spin-wait.
    pub unsafe fn transmit(&mut self, data: *const SigmaU8, len: SigmaU16) -> SigmaI32 {
        if !self.initialized { return -1; }
        let len = (len as SigmaUsize).min(MAX_FRAME_SIZE) as SigmaU16;

        let idx = self.tx_tail;

        // Copy frame into staging buffer then point descriptor at it.
        let dst = TX_STAGING.as_mut_ptr();
        core::ptr::copy_nonoverlapping(data, dst, len as SigmaUsize);

        TX_RING[idx].buf_addr = TX_STAGING.as_ptr() as SigmaU64;
        TX_RING[idx].length   = len;
        TX_RING[idx].cmd      = TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS;
        TX_RING[idx].status   = 0;

        // Advance tail (wraps).
        let next = (idx + 1) % TX_RING_SIZE;
        self.tx_tail = next;
        self.write32(E1000_TDT, next as u32);

        // Spin until NIC sets DD bit (descriptor done).
        let mut timeout = 100_000u32;
        while TX_RING[idx].status & DESC_STATUS_DD == 0 && timeout > 0 {
            Self::delay_loops(10);
            timeout -= 1;
        }

        if timeout == 0 { -2 } else { 0 }
    }

    /// nic_tx_packet — alias for transmit for C-ABI compatibility.
    pub unsafe fn nic_tx_packet(&mut self, data: *const SigmaU8, len: SigmaU16) -> SigmaI32 {
        self.transmit(data, len)
    }

    /// receive — poll for a received frame.
    ///
    /// If a frame is available, copies it into `out_buf` (up to `buf_len` bytes),
    /// returns frame length. Returns 0 if no frame ready, -1 on error.
    pub unsafe fn receive(&mut self, out_buf: *mut SigmaU8, buf_len: SigmaUsize) -> SigmaI32 {
        if !self.initialized { return -1; }

        let next = (self.rx_tail + 1) % RX_RING_SIZE;
        let desc = &mut RX_RING[next];

        if desc.status & DESC_STATUS_DD == 0 {
            return 0; // No frame ready.
        }

        let frame_len = (desc.length as SigmaUsize).min(buf_len).min(RX_BUF_SIZE);
        let src = RX_BUFFERS[next].as_ptr();
        core::ptr::copy_nonoverlapping(src, out_buf, frame_len);

        // Re-arm descriptor.
        desc.status = 0;
        self.rx_tail = next;
        self.write32(E1000_RDT, next as u32);

        frame_len as SigmaI32
    }
}

// ── Global Driver Instance ────────────────────────────────────────────────────
static mut INSTANCE: SovereignE1000 = SovereignE1000::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn e1000_init() {
    INSTANCE.e1000_init();
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    INSTANCE.e1000_init();
}

/// Send a raw Ethernet frame. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn transmit(data: *const SigmaU8, len: SigmaU16) -> SigmaI32 {
    INSTANCE.transmit(data, len)
}

/// Send a raw Ethernet frame (alias).
#[no_mangle]
pub unsafe extern "C" fn nic_tx_packet(data: *const SigmaU8, len: SigmaU16) -> SigmaI32 {
    INSTANCE.nic_tx_packet(data, len)
}

/// Poll for a received Ethernet frame.
/// Returns frame length in bytes, 0 if no frame, -1 on error.
#[no_mangle]
pub unsafe extern "C" fn e1000_receive(out: *mut SigmaU8, buf_len: SigmaUsize) -> SigmaI32 {
    INSTANCE.receive(out, buf_len)
}

/// Returns 1 if the NIC is initialized, 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn e1000_is_ready() -> SigmaU32 {
    if INSTANCE.initialized { 1 } else { 0 }
}

/// Returns pointer to the MAC address bytes (6 bytes).
#[no_mangle]
pub unsafe extern "C" fn e1000_mac_addr() -> *const SigmaU8 {
    INSTANCE.mac.as_ptr()
}
