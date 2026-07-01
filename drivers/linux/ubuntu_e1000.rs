// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: INTEL 82540EM (E1000) NIC DRIVER — Ubuntu Target (Rust, no_std)
//! =========================================================================
//!
//! Replaces: drivers/linux/ubuntu_e1000.cpp
//! Language: Rust  #![no_std]  #![no_builtins]
//!
//! Bare-metal Intel E1000 Gigabit Ethernet driver.
//! ZERO standard library. ZERO predefined functions. ZERO external crates.
//! All DMA descriptor rings are stack-resident fixed-size arrays.
//!
//! Reference: Intel 82540EM Software Developer's Manual (SDM)
//!
//! Selected at build time with: TARGET_OS=ubuntu
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); } }
}

// ── Primitive types ───────────────────────────────────────────────────────
type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ═══════════════════════════════════════════════════════════════════════════
// § 1. Intel E1000 Register Map (82540EM SDM)
//      All offsets defined from the Intel data sheet — no header import.
// ═══════════════════════════════════════════════════════════════════════════

const E1000_CTRL  : U32 = 0x0000; // Device Control
const E1000_STATUS: U32 = 0x0008; // Device Status
const E1000_EECD  : U32 = 0x0010; // EEPROM / Flash Control & Data
const E1000_EERD  : U32 = 0x0014; // EEPROM Read
const E1000_ICR   : U32 = 0x00C0; // Interrupt Cause Read
const E1000_IMS   : U32 = 0x00D0; // Interrupt Mask Set/Read
const E1000_IMC   : U32 = 0x00D8; // Interrupt Mask Clear
const E1000_RCTL  : U32 = 0x0100; // Receive Control
const E1000_TCTL  : U32 = 0x0400; // Transmit Control
const E1000_RDBAL : U32 = 0x2800; // RX Descriptor Base Address Low
const E1000_RDBAH : U32 = 0x2804; // RX Descriptor Base Address High
const E1000_RDLEN : U32 = 0x2808; // RX Descriptor Length
const E1000_RDH   : U32 = 0x2810; // RX Descriptor Head
const E1000_RDT   : U32 = 0x2818; // RX Descriptor Tail
const E1000_TDBAL : U32 = 0x3800; // TX Descriptor Base Address Low
const E1000_TDBAH : U32 = 0x3804; // TX Descriptor Base Address High
const E1000_TDLEN : U32 = 0x3808; // TX Descriptor Length
const E1000_TDH   : U32 = 0x3810; // TX Descriptor Head
const E1000_TDT   : U32 = 0x3818; // TX Descriptor Tail
const E1000_RAL0  : U32 = 0x5400; // Receive Address Low (entry 0)
const E1000_RAH0  : U32 = 0x5404; // Receive Address High (entry 0)
const E1000_MTA   : U32 = 0x5200; // Multicast Table Array (128 × 4 bytes)

// CTRL register bits
const E1000_CTRL_RST : U32 = 1 << 26; // Device Reset
const E1000_CTRL_SLU : U32 = 1 << 6;  // Set Link Up
const E1000_CTRL_ASDE: U32 = 1 << 5;  // Auto-Speed Detection Enable

// RCTL register bits
const E1000_RCTL_EN  : U32 = 1 << 1;  // Receiver Enable
const E1000_RCTL_BAM : U32 = 1 << 15; // Broadcast Accept Mode
const E1000_RCTL_BSEX: U32 = 1 << 25; // Buffer Size Extension
const E1000_RCTL_SECRC: U32 = 1 << 26; // Strip Ethernet CRC

// TCTL register bits
const E1000_TCTL_EN  : U32 = 1 << 1;  // Transmitter Enable
const E1000_TCTL_PSP : U32 = 1 << 3;  // Pad Short Packets
const E1000_TCTL_CT  : U32 = 0x10 << 4;  // Collision Threshold
const E1000_TCTL_COLD: U32 = 0x40 << 12; // Collision Distance (Full Duplex)

// STATUS bits
const E1000_STATUS_LU : U32 = 1 << 1; // Link Up

// PCI Vendor/Device
const E1000_VENDOR_ID: U32 = 0x8086;
const E1000_DEVICE_ID: U32 = 0x100E; // 82540EM

const POLL_MAX: U32 = 200_000;

// ═══════════════════════════════════════════════════════════════════════════
// § 2. TX Descriptor (Legacy format — 16 bytes)
// ═══════════════════════════════════════════════════════════════════════════

/// Legacy TX Descriptor (Intel SDM §3.3.3).
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct TxDesc {
    buffer_addr: U64, // Physical address of data buffer
    length     : U16, // Data buffer length
    cso        : U8,  // Checksum Offset
    cmd        : U8,  // Command field
    status     : U8,  // Status field (bit 0 = DD — Descriptor Done)
    css        : U8,  // Checksum Start
    special    : U16, // Special / VLAN
}

const _: () = assert!(core::mem::size_of::<TxDesc>() == 16);

impl TxDesc {
    const fn zeroed() -> Self {
        TxDesc {
            buffer_addr: 0, length: 0, cso: 0, cmd: 0,
            status: 0, css: 0, special: 0,
        }
    }
}

// TX CMD bits
const TX_CMD_EOP : U8 = 1 << 0; // End Of Packet
const TX_CMD_IFCS: U8 = 1 << 1; // Insert FCS (CRC)
const TX_CMD_RS  : U8 = 1 << 3; // Report Status (set DD on completion)

// ═══════════════════════════════════════════════════════════════════════════
// § 3. RX Descriptor (Legacy format — 16 bytes)
// ═══════════════════════════════════════════════════════════════════════════

/// Legacy RX Descriptor (Intel SDM §3.2.3).
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct RxDesc {
    buffer_addr: U64, // Physical address of RX buffer
    length     : U16, // Received length
    checksum   : U16, // Packet checksum
    status     : U8,  // Status (bit 0 = DD)
    errors     : U8,  // Errors
    special    : U16, // Special / VLAN
}

const _: () = assert!(core::mem::size_of::<RxDesc>() == 16);

impl RxDesc {
    const fn zeroed() -> Self {
        RxDesc {
            buffer_addr: 0, length: 0, checksum: 0,
            status: 0, errors: 0, special: 0,
        }
    }
}

const RX_STATUS_DD  : U8 = 1 << 0; // Descriptor Done
const RX_STATUS_EOP : U8 = 1 << 1; // End Of Packet

// ═══════════════════════════════════════════════════════════════════════════
// § 4. Volatile MMIO helpers
// ═══════════════════════════════════════════════════════════════════════════

#[inline(always)]
unsafe fn read32(base: U64, off: U32) -> U32 {
    core::ptr::read_volatile((base + off as U64) as *const U32)
}

#[inline(always)]
unsafe fn write32(base: U64, off: U32, val: U32) {
    core::ptr::write_volatile((base + off as U64) as *mut U32, val);
}

#[inline]
unsafe fn poll32(base: U64, off: U32, mask: U32, expected: U32) -> bool {
    let mut i: U32 = 0;
    while i < POLL_MAX {
        if (read32(base, off) & mask) == expected { return true; }
        core::arch::asm!("pause", options(nomem, nostack, preserves_flags));
        i += 1;
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════
// § 5. Driver state — stack-resident descriptor rings
// ═══════════════════════════════════════════════════════════════════════════

const TX_RING_SIZE: usize = 256;
const RX_RING_SIZE: usize = 256;
const RX_BUFFER_SIZE: usize = 2048; // Per-descriptor RX buffer

/// TX descriptor ring — page-aligned for DMA.
#[repr(C, align(4096))]
struct TxRing {
    descs: [TxDesc; TX_RING_SIZE],
    tail : usize,
}

impl TxRing {
    const fn new() -> Self {
        TxRing {
            descs: [TxDesc::zeroed(); TX_RING_SIZE],
            tail : 0,
        }
    }
}

/// RX descriptor ring + backing data buffers.
#[repr(C, align(4096))]
struct RxRing {
    descs  : [RxDesc; RX_RING_SIZE],
    buffers: [[U8; RX_BUFFER_SIZE]; RX_RING_SIZE],
    head   : usize,
}

impl RxRing {
    const fn new() -> Self {
        RxRing {
            descs  : [RxDesc::zeroed(); RX_RING_SIZE],
            buffers: [[0u8; RX_BUFFER_SIZE]; RX_RING_SIZE],
            head   : 0,
        }
    }
}

/// Complete E1000 driver state.
pub struct E1000Driver {
    mmio_base  : U64,
    tx_ring    : TxRing,
    rx_ring    : RxRing,
    mac_addr   : [U8; 6],
    tx_count   : U64,
    rx_count   : U64,
    initialized: bool,
}

impl E1000Driver {
    pub const fn new() -> Self {
        E1000Driver {
            mmio_base  : 0,
            tx_ring    : TxRing::new(),
            rx_ring    : RxRing::new(),
            mac_addr   : [0; 6],
            tx_count   : 0,
            rx_count   : 0,
            initialized: false,
        }
    }

    // ── init ───────────────────────────────────────────────────────────────

    /// Initialise the E1000 NIC.
    ///
    /// Sequence (Intel SDM §14.3):
    ///   1. Device reset (CTRL.RST = 1)
    ///   2. Set Link Up (CTRL.SLU = 1)
    ///   3. Clear all interrupts (ICR read + IMC = 0xFFFF_FFFF)
    ///   4. Clear Multicast Table Array
    ///   5. Programme RX descriptor ring (RDBAL/RDBAH/RDLEN/RDH/RDT)
    ///   6. Programme TX descriptor ring (TDBAL/TDBAH/TDLEN/TDH/TDT)
    ///   7. Enable RX (RCTL.EN) and TX (TCTL.EN)
    pub unsafe fn init(&mut self, mmio_base: U64) -> I32 {
        self.mmio_base = mmio_base;

        // Step 1: Reset
        let ctrl = read32(mmio_base, E1000_CTRL);
        write32(mmio_base, E1000_CTRL, ctrl | E1000_CTRL_RST);
        // Wait ~10ms worth of spins for reset to complete
        let mut spin: U32 = 0;
        while spin < 100_000 {
            core::arch::asm!("pause", options(nomem, nostack, preserves_flags));
            spin += 1;
        }

        // Step 2: Set Link Up + Auto-Speed
        write32(mmio_base, E1000_CTRL, E1000_CTRL_SLU | E1000_CTRL_ASDE);

        // Step 3: Clear interrupts
        let _ = read32(mmio_base, E1000_ICR); // Read to clear
        write32(mmio_base, E1000_IMC, 0xFFFF_FFFF);

        // Step 4: Clear MTA (128 entries × 4 bytes each)
        let mut m: U32 = 0;
        while m < 128 {
            write32(mmio_base, E1000_MTA + m * 4, 0);
            m += 1;
        }

        // Step 5: RX ring setup
        // Point each RX descriptor to its backing buffer
        let mut i: usize = 0;
        while i < RX_RING_SIZE {
            self.rx_ring.descs[i].buffer_addr =
                self.rx_ring.buffers[i].as_ptr() as U64;
            self.rx_ring.descs[i].status = 0;
            i += 1;
        }
        let rdba = self.rx_ring.descs.as_ptr() as U64;
        write32(mmio_base, E1000_RDBAL, rdba as U32);
        write32(mmio_base, E1000_RDBAH, (rdba >> 32) as U32);
        write32(mmio_base, E1000_RDLEN, (RX_RING_SIZE * 16) as U32);
        write32(mmio_base, E1000_RDH, 0);
        write32(mmio_base, E1000_RDT, (RX_RING_SIZE - 1) as U32);

        // Step 6: TX ring setup
        let tdba = self.tx_ring.descs.as_ptr() as U64;
        write32(mmio_base, E1000_TDBAL, tdba as U32);
        write32(mmio_base, E1000_TDBAH, (tdba >> 32) as U32);
        write32(mmio_base, E1000_TDLEN, (TX_RING_SIZE * 16) as U32);
        write32(mmio_base, E1000_TDH, 0);
        write32(mmio_base, E1000_TDT, 0);

        // Step 7a: Enable RX
        write32(mmio_base, E1000_RCTL,
                E1000_RCTL_EN | E1000_RCTL_BAM | E1000_RCTL_SECRC);

        // Step 7b: Enable TX
        write32(mmio_base, E1000_TCTL,
                E1000_TCTL_EN | E1000_TCTL_PSP | E1000_TCTL_CT | E1000_TCTL_COLD);

        // Verify link
        if !poll32(mmio_base, E1000_STATUS, E1000_STATUS_LU, E1000_STATUS_LU) {
            // Link not up — non-fatal, proceed anyway
        }

        self.initialized = true;
        0
    }

    // ── transmit ───────────────────────────────────────────────────────────

    /// Transmit a single packet.
    ///
    /// `data_phys` = physical address of the packet buffer.
    /// `len`       = packet length in bytes (max 1514 for Ethernet).
    ///
    /// Returns 0 on success.
    pub unsafe fn transmit(&mut self, data_phys: U64, len: U16) -> I32 {
        if !self.initialized || data_phys == 0 || len == 0 { return -3; }

        let idx = self.tx_ring.tail;
        let desc = &mut self.tx_ring.descs[idx];

        desc.buffer_addr = data_phys;
        desc.length      = len;
        desc.cmd         = TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS;
        desc.status      = 0;

        self.tx_ring.tail = (idx + 1) % TX_RING_SIZE;
        self.tx_count += 1;

        // Advance TDT (doorbell)
        write32(self.mmio_base, E1000_TDT, self.tx_ring.tail as U32);

        0
    }

    // ── receive (poll) ─────────────────────────────────────────────────────

    /// Poll for a received packet.
    ///
    /// If a packet is available, returns the received length (> 0).
    /// If no packet, returns 0.
    /// `out_buf_phys` receives the physical address of the RX buffer
    /// containing the packet data.
    pub unsafe fn poll_rx(&mut self, out_buf_phys: *mut U64) -> I32 {
        if !self.initialized { return -1; }

        let idx = self.rx_ring.head;
        let desc = &self.rx_ring.descs[idx];

        if (desc.status & RX_STATUS_DD) == 0 {
            return 0; // No packet available
        }

        let len = desc.length;
        if !out_buf_phys.is_null() {
            *out_buf_phys = desc.buffer_addr;
        }

        // Reset descriptor for reuse
        let new_desc = &mut self.rx_ring.descs[idx];
        new_desc.status = 0;

        self.rx_ring.head = (idx + 1) % RX_RING_SIZE;
        self.rx_count += 1;

        // Advance RDT
        write32(self.mmio_base, E1000_RDT, idx as U32);

        len as I32
    }
}

// ── Global singleton ──────────────────────────────────────────────────────
static mut G_E1000: E1000Driver = E1000Driver::new();

// ── C bridge ──────────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn linux_e1000_init(mmio_base: U64) -> I32 {
    G_E1000.init(mmio_base)
}

#[no_mangle]
pub unsafe extern "C" fn linux_e1000_tx(data_phys: U64, len: U32) -> I32 {
    G_E1000.transmit(data_phys, len as U16)
}

#[no_mangle]
pub unsafe extern "C" fn linux_e1000_poll_rx(out_buf: *mut U64) -> I32 {
    G_E1000.poll_rx(out_buf)
}
