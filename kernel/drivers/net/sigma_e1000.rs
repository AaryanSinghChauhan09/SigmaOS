//! SigmaOS — Intel e1000 Network Interface Controller Driver
//! Bare-metal driver for Intel 82540EM Gigabit Ethernet (QEMU default NIC).
//! No std, no allocator — fixed-size ring buffers for TX/RX.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── E1000 Register Offsets ──────────────────────────────────────────────────
const E1000_CTRL:    Usize = 0x0000; // Device Control
const E1000_STATUS:  Usize = 0x0008; // Device Status
const E1000_EECD:    Usize = 0x0010; // EEPROM/Flash Control
const E1000_EERD:    Usize = 0x0014; // EEPROM Read
const E1000_ICR:     Usize = 0x00C0; // Interrupt Cause Read
const E1000_IMS:     Usize = 0x00D0; // Interrupt Mask Set
const E1000_IMC:     Usize = 0x00D8; // Interrupt Mask Clear
const E1000_RCTL:    Usize = 0x0100; // Receive Control
const E1000_TCTL:    Usize = 0x0400; // Transmit Control
const E1000_RDBAL:   Usize = 0x2800; // RX Descriptor Base Address Low
const E1000_RDBAH:   Usize = 0x2804; // RX Descriptor Base Address High
const E1000_RDLEN:   Usize = 0x2808; // RX Descriptor Length
const E1000_RDH:     Usize = 0x2810; // RX Descriptor Head
const E1000_RDT:     Usize = 0x2818; // RX Descriptor Tail
const E1000_TDBAL:   Usize = 0x3800; // TX Descriptor Base Address Low
const E1000_TDBAH:   Usize = 0x3804; // TX Descriptor Base Address High
const E1000_TDLEN:   Usize = 0x3808; // TX Descriptor Length
const E1000_TDH:     Usize = 0x3810; // TX Descriptor Head
const E1000_TDT:     Usize = 0x3818; // TX Descriptor Tail
const E1000_MTA:     Usize = 0x5200; // Multicast Table Array
const E1000_RAL:     Usize = 0x5400; // Receive Address Low
const E1000_RAH:     Usize = 0x5404; // Receive Address High
const E1000_TIPG:    Usize = 0x0410; // TX Inter-Packet Gap

// ── Control Register bits ───────────────────────────────────────────────────
const CTRL_SLU:   U32 = 1 << 6;  // Set Link Up
const CTRL_RST:   U32 = 1 << 26; // Device Reset
const CTRL_ASDE:  U32 = 1 << 5;  // Auto-Speed Detection Enable

// ── Receive Control bits ────────────────────────────────────────────────────
const RCTL_EN:    U32 = 1 << 1;  // Receiver Enable
const RCTL_SBP:   U32 = 1 << 2;  // Store Bad Packets
const RCTL_UPE:   U32 = 1 << 3;  // Unicast Promiscuous Enable
const RCTL_MPE:   U32 = 1 << 4;  // Multicast Promiscuous Enable
const RCTL_BAM:   U32 = 1 << 15; // Broadcast Accept Mode
const RCTL_BSIZE_2048: U32 = 0;  // Buffer size 2048 bytes
const RCTL_SECRC: U32 = 1 << 26; // Strip Ethernet CRC

// ── Transmit Control bits ───────────────────────────────────────────────────
const TCTL_EN:    U32 = 1 << 1;  // Transmit Enable
const TCTL_PSP:   U32 = 1 << 3;  // Pad Short Packets
const TCTL_CT:    U32 = 0x10 << 4;  // Collision Threshold
const TCTL_COLD:  U32 = 0x40 << 12; // Collision Distance

// ── Interrupt bits ──────────────────────────────────────────────────────────
const ICR_TXDW:   U32 = 1 << 0;  // TX Descriptor Written Back
const ICR_TXQE:   U32 = 1 << 1;  // TX Queue Empty
const ICR_LSC:    U32 = 1 << 2;  // Link Status Change
const ICR_RXDMT0: U32 = 1 << 4;  // RX Descriptor Minimum Threshold
const ICR_RXT0:   U32 = 1 << 7;  // Receiver Timer Interrupt

// ── TX/RX Descriptors ──────────────────────────────────────────────────────
const NUM_RX_DESC: usize = 32;
const NUM_TX_DESC: usize = 32;
const RX_BUF_SIZE: usize = 2048;

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct RxDescriptor {
    pub addr:     U64,  // Buffer address
    pub length:   U16,  // Packet length
    pub checksum: U16,  // Packet checksum
    pub status:   U8,   // Status bits
    pub errors:   U8,   // Error bits
    pub special:  U16,  // VLAN tag
}

impl RxDescriptor {
    pub const fn zero() -> Self {
        RxDescriptor { addr: 0, length: 0, checksum: 0, status: 0, errors: 0, special: 0 }
    }
    pub fn is_done(&self) -> bool { self.status & 1 != 0 } // DD bit
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct TxDescriptor {
    pub addr:     U64,  // Buffer address
    pub length:   U16,  // Data length
    pub cso:      U8,   // Checksum offset
    pub cmd:      U8,   // Command bits
    pub status:   U8,   // Status bits
    pub css:      U8,   // Checksum start
    pub special:  U16,  // VLAN tag
}

impl TxDescriptor {
    pub const fn zero() -> Self {
        TxDescriptor { addr: 0, length: 0, cso: 0, cmd: 0, status: 0, css: 0, special: 0 }
    }
    pub fn is_done(&self) -> bool { self.status & 1 != 0 } // DD bit
}

// TX command bits
const TX_CMD_EOP:  U8 = 1 << 0; // End of Packet
const TX_CMD_IFCS: U8 = 1 << 1; // Insert FCS
const TX_CMD_RS:   U8 = 1 << 3; // Report Status

// ── NIC State ───────────────────────────────────────────────────────────────
pub struct E1000State {
    pub mmio_base: U64,
    pub mac: [U8; 6],
    pub link_up: bool,
    pub speed_mbps: U32,

    pub rx_desc: [RxDescriptor; NUM_RX_DESC],
    pub tx_desc: [TxDescriptor; NUM_TX_DESC],
    pub rx_bufs: [[U8; RX_BUF_SIZE]; NUM_RX_DESC],
    pub tx_bufs: [[U8; RX_BUF_SIZE]; NUM_TX_DESC],
    pub rx_cur: usize,
    pub tx_cur: usize,

    pub rx_packets: U64,
    pub tx_packets: U64,
    pub rx_bytes:   U64,
    pub tx_bytes:   U64,
    pub rx_errors:  U64,

    pub initialized: bool,
}

static mut NIC: E1000State = E1000State {
    mmio_base: 0,
    mac: [0u8; 6],
    link_up: false,
    speed_mbps: 0,
    rx_desc: [RxDescriptor::zero(); NUM_RX_DESC],
    tx_desc: [TxDescriptor::zero(); NUM_TX_DESC],
    rx_bufs: [[0u8; RX_BUF_SIZE]; NUM_RX_DESC],
    tx_bufs: [[0u8; RX_BUF_SIZE]; NUM_TX_DESC],
    rx_cur: 0,
    tx_cur: 0,
    rx_packets: 0, tx_packets: 0,
    rx_bytes: 0,   tx_bytes: 0,
    rx_errors: 0,
    initialized: false,
};

// ── MMIO Helpers ────────────────────────────────────────────────────────────
unsafe fn e1000_read(offset: Usize) -> U32 {
    let ptr = (NIC.mmio_base as Usize + offset) as *const U32;
    core::ptr::read_volatile(ptr)
}

unsafe fn e1000_write(offset: Usize, val: U32) {
    let ptr = (NIC.mmio_base as Usize + offset) as *mut U32;
    core::ptr::write_volatile(ptr, val);
}

// ── EEPROM Read ─────────────────────────────────────────────────────────────
unsafe fn e1000_eeprom_read(addr: U8) -> U16 {
    e1000_write(E1000_EERD, 1 | ((addr as U32) << 8));
    // Wait for done bit
    let mut timeout = 10000u32;
    loop {
        let val = e1000_read(E1000_EERD);
        if val & (1 << 4) != 0 {
            return ((val >> 16) & 0xFFFF) as U16;
        }
        timeout -= 1;
        if timeout == 0 { return 0; }
        core::hint::spin_loop();
    }
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the e1000 NIC from its MMIO base address.
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_init(mmio_base: U64) -> i32 {
    if mmio_base == 0 { return -1; }
    NIC.mmio_base = mmio_base;

    // Read MAC address from EEPROM
    let mac01 = e1000_eeprom_read(0);
    let mac23 = e1000_eeprom_read(1);
    let mac45 = e1000_eeprom_read(2);
    NIC.mac[0] = (mac01 & 0xFF) as U8;
    NIC.mac[1] = ((mac01 >> 8) & 0xFF) as U8;
    NIC.mac[2] = (mac23 & 0xFF) as U8;
    NIC.mac[3] = ((mac23 >> 8) & 0xFF) as U8;
    NIC.mac[4] = (mac45 & 0xFF) as U8;
    NIC.mac[5] = ((mac45 >> 8) & 0xFF) as U8;

    // Set link up
    let ctrl = e1000_read(E1000_CTRL);
    e1000_write(E1000_CTRL, ctrl | CTRL_SLU | CTRL_ASDE);

    // Clear multicast table
    for i in 0..128 {
        e1000_write(E1000_MTA + i * 4, 0);
    }

    // Set up receive descriptors
    for i in 0..NUM_RX_DESC {
        NIC.rx_desc[i].addr = NIC.rx_bufs[i].as_ptr() as U64;
        NIC.rx_desc[i].status = 0;
    }
    let rx_desc_addr = NIC.rx_desc.as_ptr() as U64;
    e1000_write(E1000_RDBAL, rx_desc_addr as U32);
    e1000_write(E1000_RDBAH, (rx_desc_addr >> 32) as U32);
    e1000_write(E1000_RDLEN, (NUM_RX_DESC * core::mem::size_of::<RxDescriptor>()) as U32);
    e1000_write(E1000_RDH, 0);
    e1000_write(E1000_RDT, (NUM_RX_DESC - 1) as U32);
    NIC.rx_cur = 0;

    // Enable receiver
    e1000_write(E1000_RCTL, RCTL_EN | RCTL_BAM | RCTL_BSIZE_2048 | RCTL_SECRC);

    // Set up transmit descriptors
    for i in 0..NUM_TX_DESC {
        NIC.tx_desc[i].addr = NIC.tx_bufs[i].as_ptr() as U64;
        NIC.tx_desc[i].status = 1; // DD set = descriptor available
        NIC.tx_desc[i].cmd = 0;
    }
    let tx_desc_addr = NIC.tx_desc.as_ptr() as U64;
    e1000_write(E1000_TDBAL, tx_desc_addr as U32);
    e1000_write(E1000_TDBAH, (tx_desc_addr >> 32) as U32);
    e1000_write(E1000_TDLEN, (NUM_TX_DESC * core::mem::size_of::<TxDescriptor>()) as U32);
    e1000_write(E1000_TDH, 0);
    e1000_write(E1000_TDT, 0);
    NIC.tx_cur = 0;

    // Set transmit IPG
    e1000_write(E1000_TIPG, 10 | (8 << 10) | (6 << 20));

    // Enable transmitter
    e1000_write(E1000_TCTL, TCTL_EN | TCTL_PSP | TCTL_CT | TCTL_COLD);

    // Enable interrupts
    e1000_write(E1000_IMS, ICR_RXT0 | ICR_TXDW | ICR_LSC);

    // Check link status
    let status = e1000_read(E1000_STATUS);
    NIC.link_up = (status & 2) != 0; // LU bit
    NIC.speed_mbps = match (status >> 6) & 3 {
        0 => 10,
        1 => 100,
        2 | 3 => 1000,
        _ => 0,
    };

    NIC.initialized = true;
    0
}

/// Transmit a packet. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_transmit(data: *const U8, len: U16) -> i32 {
    if !NIC.initialized || data.is_null() || len == 0 { return -1; }
    if len as usize > RX_BUF_SIZE { return -2; }

    let idx = NIC.tx_cur;
    if !NIC.tx_desc[idx].is_done() { return -3; } // Descriptor busy

    // Copy data to TX buffer
    let src = core::slice::from_raw_parts(data, len as usize);
    for i in 0..len as usize {
        NIC.tx_bufs[idx][i] = src[i];
    }

    // Set up descriptor
    NIC.tx_desc[idx].addr = NIC.tx_bufs[idx].as_ptr() as U64;
    NIC.tx_desc[idx].length = len;
    NIC.tx_desc[idx].cmd = TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS;
    NIC.tx_desc[idx].status = 0;

    // Advance tail pointer
    NIC.tx_cur = (NIC.tx_cur + 1) % NUM_TX_DESC;
    e1000_write(E1000_TDT, NIC.tx_cur as U32);

    NIC.tx_packets += 1;
    NIC.tx_bytes += len as U64;
    0
}

/// Receive a packet. Copies into provided buffer. Returns packet length or -1.
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_receive(buf: *mut U8, buf_len: U16) -> i32 {
    if !NIC.initialized || buf.is_null() { return -1; }

    let idx = NIC.rx_cur;
    if !NIC.rx_desc[idx].is_done() { return 0; } // No packet

    let pkt_len = NIC.rx_desc[idx].length;
    let copy_len = (pkt_len as usize).min(buf_len as usize);

    let dst = core::slice::from_raw_parts_mut(buf, copy_len);
    for i in 0..copy_len {
        dst[i] = NIC.rx_bufs[idx][i];
    }

    // Reset descriptor
    NIC.rx_desc[idx].status = 0;
    let old_cur = NIC.rx_cur;
    NIC.rx_cur = (NIC.rx_cur + 1) % NUM_RX_DESC;
    e1000_write(E1000_RDT, old_cur as U32);

    NIC.rx_packets += 1;
    NIC.rx_bytes += pkt_len as U64;
    pkt_len as i32
}

/// Get the MAC address.
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_mac(out: *mut U8) {
    if out.is_null() { return; }
    for i in 0..6 {
        *out.add(i) = NIC.mac[i];
    }
}

/// Check if link is up.
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_link_up() -> i32 {
    if NIC.link_up { 1 } else { 0 }
}

/// Get link speed in Mbps.
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_speed() -> U32 { NIC.speed_mbps }

/// Get TX packet count.
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_tx_count() -> U64 { NIC.tx_packets }

/// Get RX packet count.
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_rx_count() -> U64 { NIC.rx_packets }
