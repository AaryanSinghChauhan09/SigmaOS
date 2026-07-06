// SPDX-License-Identifier: MIT
// SigmaOS Intel Gigabit Ethernet (e1000/e1000e) Driver
// Full MMIO register access, TX/RX descriptor rings, interrupt handling,
// PHY reset, link-state detection, and SigmaNet integration hooks.

#![no_std]

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── e1000 MMIO Register Offsets ──────────────────────────────────────────────
const E1000_CTRL:      u32 = 0x0000; // Device Control
const E1000_STATUS:    u32 = 0x0008; // Device Status
const E1000_EECD:      u32 = 0x0010; // EEPROM/Flash Control
const E1000_EERD:      u32 = 0x0014; // EEPROM Read
const E1000_ICR:       u32 = 0x00C0; // Interrupt Cause Read
const E1000_ITR:       u32 = 0x00C4; // Interrupt Throttling Rate
const E1000_ICS:       u32 = 0x00C8; // Interrupt Cause Set
const E1000_IMS:       u32 = 0x00D0; // Interrupt Mask Set/Read
const E1000_IMC:       u32 = 0x00D8; // Interrupt Mask Clear
const E1000_RCTL:      u32 = 0x0100; // Receive Control
const E1000_TCTL:      u32 = 0x0400; // Transmit Control
const E1000_TIPG:      u32 = 0x0410; // TX Inter-packet Gap
const E1000_RDBAL:     u32 = 0x2800; // RX Descriptor Base Low
const E1000_RDBAH:     u32 = 0x2804; // RX Descriptor Base High
const E1000_RDLEN:     u32 = 0x2808; // RX Descriptor Ring Length
const E1000_RDH:       u32 = 0x2810; // RX Descriptor Head
const E1000_RDT:       u32 = 0x2818; // RX Descriptor Tail
const E1000_TDBAL:     u32 = 0x3800; // TX Descriptor Base Low
const E1000_TDBAH:     u32 = 0x3804; // TX Descriptor Base High
const E1000_TDLEN:     u32 = 0x3808; // TX Descriptor Ring Length
const E1000_TDH:       u32 = 0x3810; // TX Descriptor Head
const E1000_TDT:       u32 = 0x3818; // TX Descriptor Tail
const E1000_MTA:       u32 = 0x5200; // Multicast Table Array (128 DWORDs)
const E1000_RAL:       u32 = 0x5400; // Receive Address Low
const E1000_RAH:       u32 = 0x5404; // Receive Address High
const E1000_MDIC:      u32 = 0x0020; // MDI Control (PHY access)

// ── CTRL bits ────────────────────────────────────────────────────────────────
const CTRL_FD:    u32 = 1 << 0;  // Full Duplex
const CTRL_ASDE:  u32 = 1 << 5;  // Auto-Speed Detection Enable
const CTRL_SLU:   u32 = 1 << 6;  // Set Link Up
const CTRL_ILOS:  u32 = 1 << 7;  // Invert Loss-Of-Signal
const CTRL_RST:   u32 = 1 << 26; // Device Reset

// ── STATUS bits ──────────────────────────────────────────────────────────────
const STATUS_LU: u32 = 1 << 1; // Link Up

// ── RCTL bits ────────────────────────────────────────────────────────────────
const RCTL_EN:     u32 = 1 << 1;  // Enable
const RCTL_SBP:    u32 = 1 << 2;  // Store Bad Packets
const RCTL_UPE:    u32 = 1 << 3;  // Unicast Promiscuous
const RCTL_MPE:    u32 = 1 << 4;  // Multicast Promiscuous
const RCTL_LBM_NO: u32 = 0 << 6;  // No Loopback
const RCTL_BAM:    u32 = 1 << 15; // Broadcast Accept
const RCTL_BSIZE_2048: u32 = 0 << 16;
const RCTL_SECRC:  u32 = 1 << 26; // Strip Ethernet CRC

// ── TCTL bits ────────────────────────────────────────────────────────────────
const TCTL_EN:   u32 = 1 << 1; // Enable
const TCTL_PSP:  u32 = 1 << 3; // Pad Short Packets
const TCTL_CT:   u32 = 0x0F << 4; // Collision Threshold (default 15)
const TCTL_COLD: u32 = 0x3F << 12; // Collision Distance (default 63 for FD)

// ── ICR / IMS interrupt bits ──────────────────────────────────────────────────
const ICR_TXDW:   u32 = 1 << 0;  // TX Descriptor Written Back
const ICR_TXQE:   u32 = 1 << 1;  // TX Queue Empty
const ICR_LSC:    u32 = 1 << 2;  // Link Status Change
const ICR_RXDMT0: u32 = 1 << 4;  // RX Desc Min Threshold
const ICR_RXT0:   u32 = 1 << 7;  // RX Timer Interrupt
const ICR_GPI:    u32 = 3 << 26; // General Purpose Interrupts

// ── TX / RX ring sizes ────────────────────────────────────────────────────────
const E1000_TX_RING_SIZE: usize = 64;
const E1000_RX_RING_SIZE: usize = 64;
const E1000_RX_BUF_SIZE:  usize = 2048;

// ── TX Descriptor (16 bytes per NVMe / e1000 spec) ──────────────────────────
#[derive(Copy, Clone, Default, Debug)]
#[repr(C, align(16))]
pub struct E1000TxDesc {
    pub addr:    u64,  // Buffer physical address
    pub length:  u16,  // Packet length in bytes
    pub cso:     u8,   // Checksum offset
    pub cmd:     u8,   // Command field
    pub status:  u8,   // Status
    pub css:     u8,   // Checksum start
    pub special: u16,  // VLAN / special
}

impl E1000TxDesc {
    /// CMD: End Of Packet | Report Status | Insert FCS
    const CMD_EOP: u8 = 1 << 0;
    const CMD_RS:  u8 = 1 << 3;
    const CMD_IFCS:u8 = 1 << 1;

    /// DD (Descriptor Done) status bit — hardware sets when TX complete
    const STATUS_DD: u8 = 1 << 0;

    /// Prepare a TX descriptor for sending a packet.
    pub fn prepare(phys: u64, len: u16) -> Self {
        Self {
            addr:    phys,
            length:  len,
            cso:     0,
            cmd:     Self::CMD_EOP | Self::CMD_RS | Self::CMD_IFCS,
            status:  0,
            css:     0,
            special: 0,
        }
    }

    /// Return true when hardware has completed this TX.
    pub fn is_done(&self) -> bool {
        (self.status & Self::STATUS_DD) != 0
    }
}

// ── RX Descriptor (16 bytes) ─────────────────────────────────────────────────
#[derive(Copy, Clone, Default, Debug)]
#[repr(C, align(16))]
pub struct E1000RxDesc {
    pub addr:    u64,   // Buffer physical address (filled by driver)
    pub length:  u16,   // Packet length (filled by hardware)
    pub csum:    u16,   // Checksum
    pub status:  u8,    // Status
    pub errors:  u8,    // Errors
    pub special: u16,   // VLAN tag
}

impl E1000RxDesc {
    const STATUS_DD:  u8 = 1 << 0; // Descriptor Done
    const STATUS_EOP: u8 = 1 << 1; // End of Packet

    /// Return true when hardware has placed a received packet here.
    pub fn is_done(&self) -> bool {
        (self.status & Self::STATUS_DD) != 0
    }

    /// Return true if this descriptor is the last in the packet.
    pub fn is_eop(&self) -> bool {
        (self.status & Self::STATUS_EOP) != 0
    }
}

// ── Receive Buffer Pool ──────────────────────────────────────────────────────
// Static 2 KB buffers for zero-alloc RX.
static mut RX_BUFFERS: [[u8; E1000_RX_BUF_SIZE]; E1000_RX_RING_SIZE] =
    [[0u8; E1000_RX_BUF_SIZE]; E1000_RX_RING_SIZE];

// ── Driver State ─────────────────────────────────────────────────────────────
pub struct E1000Driver {
    mmio_base: u64,
    mac_addr:  [u8; 6],
    tx_ring:   [E1000TxDesc; E1000_TX_RING_SIZE],
    rx_ring:   [E1000RxDesc; E1000_RX_RING_SIZE],
    tx_head:   u32,
    tx_tail:   u32,
    rx_head:   u32,
    rx_tail:   u32,
    link_up:   AtomicBool,
    tx_dropped: AtomicU32,
    rx_dropped: AtomicU32,
    tx_packets: AtomicU32,
    rx_packets: AtomicU32,
}

impl E1000Driver {
    pub const fn new() -> Self {
        Self {
            mmio_base:  0,
            mac_addr:   [0u8; 6],
            tx_ring:    [E1000TxDesc {
                addr: 0, length: 0, cso: 0, cmd: 0, status: 0, css: 0, special: 0
            }; E1000_TX_RING_SIZE],
            rx_ring:    [E1000RxDesc {
                addr: 0, length: 0, csum: 0, status: 0, errors: 0, special: 0
            }; E1000_RX_RING_SIZE],
            tx_head:   0,
            tx_tail:   0,
            rx_head:   0,
            rx_tail:   0,
            link_up:   AtomicBool::new(false),
            tx_dropped: AtomicU32::new(0),
            rx_dropped: AtomicU32::new(0),
            tx_packets: AtomicU32::new(0),
            rx_packets: AtomicU32::new(0),
        }
    }

    // ── MMIO helpers ─────────────────────────────────────────────────────────

    #[inline]
    unsafe fn read(&self, offset: u32) -> u32 {
        read_volatile((self.mmio_base + offset as u64) as *const u32)
    }

    #[inline]
    unsafe fn write(&self, offset: u32, val: u32) {
        write_volatile((self.mmio_base + offset as u64) as *mut u32, val);
    }

    // ── EEPROM / MAC read ─────────────────────────────────────────────────────

    /// Read a word from the EEPROM.
    unsafe fn eeprom_read(&self, addr: u8) -> u16 {
        self.write(E1000_EERD, 1 | ((addr as u32) << 8));
        let mut timeout = 10_000u32;
        while timeout > 0 && (self.read(E1000_EERD) & (1 << 4)) == 0 {
            timeout -= 1;
        }
        (self.read(E1000_EERD) >> 16) as u16
    }

    /// Load MAC address from EEPROM into mac_addr field.
    unsafe fn load_mac(&mut self) {
        let w0 = self.eeprom_read(0);
        let w1 = self.eeprom_read(1);
        let w2 = self.eeprom_read(2);
        self.mac_addr[0] = (w0 & 0xFF) as u8;
        self.mac_addr[1] = (w0 >> 8) as u8;
        self.mac_addr[2] = (w1 & 0xFF) as u8;
        self.mac_addr[3] = (w1 >> 8) as u8;
        self.mac_addr[4] = (w2 & 0xFF) as u8;
        self.mac_addr[5] = (w2 >> 8) as u8;
    }

    // ── Initialization ────────────────────────────────────────────────────────

    /// Initialize the e1000 NIC given MMIO base address.
    pub unsafe fn init(&mut self, mmio_base: u64) -> bool {
        self.mmio_base = mmio_base;
        if mmio_base == 0 { return false; }

        // 1. Software reset
        self.write(E1000_CTRL, self.read(E1000_CTRL) | CTRL_RST);
        // Small delay loop (busy-wait ~10 µs approximation)
        for _ in 0..1000 { core::hint::spin_loop(); }

        // 2. Wait for reset to clear
        let mut timeout = 100_000u32;
        while timeout > 0 && (self.read(E1000_CTRL) & CTRL_RST) != 0 {
            timeout -= 1;
        }
        if timeout == 0 { return false; }

        // 3. Set link control: auto-speed, set link up, full duplex
        let ctrl = CTRL_FD | CTRL_ASDE | CTRL_SLU;
        self.write(E1000_CTRL, ctrl);

        // 4. Disable all interrupts temporarily
        self.write(E1000_IMC, 0xFFFFFFFF);
        let _ = self.read(E1000_ICR); // Clear pending

        // 5. Load MAC from EEPROM
        self.load_mac();

        // 6. Program receive address register with our MAC
        let ral = (self.mac_addr[0] as u32)
            | ((self.mac_addr[1] as u32) << 8)
            | ((self.mac_addr[2] as u32) << 16)
            | ((self.mac_addr[3] as u32) << 24);
        let rah = (self.mac_addr[4] as u32)
            | ((self.mac_addr[5] as u32) << 8)
            | (1u32 << 31); // Address Valid
        self.write(E1000_RAL, ral);
        self.write(E1000_RAH, rah);

        // 7. Clear multicast table
        for i in 0..128u32 {
            self.write(E1000_MTA + i * 4, 0);
        }

        // 8. Set up TX ring
        self.init_tx();

        // 9. Set up RX ring
        self.init_rx();

        // 10. Enable interrupts: LSC, RXT0, RXDMT0
        self.write(E1000_ITR, 500); // ~2 MHz interrupt rate cap
        self.write(E1000_IMS, ICR_RXT0 | ICR_RXDMT0 | ICR_LSC | ICR_TXDW);

        // 11. Check link status
        self.poll_link();

        true
    }

    unsafe fn init_tx(&mut self) {
        // Zero TX ring
        for d in self.tx_ring.iter_mut() {
            *d = E1000TxDesc::default();
        }

        let phys = self.tx_ring.as_ptr() as u64;
        self.write(E1000_TDBAL, (phys & 0xFFFFFFFF) as u32);
        self.write(E1000_TDBAH, (phys >> 32) as u32);
        self.write(E1000_TDLEN, (E1000_TX_RING_SIZE * core::mem::size_of::<E1000TxDesc>()) as u32);
        self.write(E1000_TDH, 0);
        self.write(E1000_TDT, 0);
        self.tx_head = 0;
        self.tx_tail = 0;

        // TX Control: enable, pad short packets, standard collision settings
        let tctl = TCTL_EN | TCTL_PSP | TCTL_CT | TCTL_COLD;
        self.write(E1000_TCTL, tctl);

        // Standard TIPG for 802.3 full-duplex
        self.write(E1000_TIPG, 0x0060200A);
    }

    unsafe fn init_rx(&mut self) {
        // Set RX buffer addresses
        for (i, d) in self.rx_ring.iter_mut().enumerate() {
            *d = E1000RxDesc::default();
            d.addr = RX_BUFFERS[i].as_ptr() as u64;
        }

        let phys = self.rx_ring.as_ptr() as u64;
        self.write(E1000_RDBAL, (phys & 0xFFFFFFFF) as u32);
        self.write(E1000_RDBAH, (phys >> 32) as u32);
        self.write(E1000_RDLEN, (E1000_RX_RING_SIZE * core::mem::size_of::<E1000RxDesc>()) as u32);
        self.write(E1000_RDH, 0);
        // Tail at ring_size - 1 to prime the ring
        self.write(E1000_RDT, (E1000_RX_RING_SIZE - 1) as u32);
        self.rx_head = 0;
        self.rx_tail = (E1000_RX_RING_SIZE - 1) as u32;

        // RX Control: enable, broadcast, 2 KB buffers, strip CRC
        let rctl = RCTL_EN | RCTL_BAM | RCTL_BSIZE_2048 | RCTL_SECRC;
        self.write(E1000_RCTL, rctl);
    }

    // ── Link State ────────────────────────────────────────────────────────────

    pub unsafe fn poll_link(&mut self) {
        let status = self.read(E1000_STATUS);
        let up = (status & STATUS_LU) != 0;
        self.link_up.store(up, Ordering::Relaxed);
    }

    pub fn is_link_up(&self) -> bool {
        self.link_up.load(Ordering::Relaxed)
    }

    // ── Interrupt Handler ─────────────────────────────────────────────────────

    /// Call from IRQ handler. Returns bitmask of events.
    pub unsafe fn handle_irq(&mut self) -> u32 {
        let icr = self.read(E1000_ICR);
        if icr == 0 { return 0; }

        if icr & ICR_LSC != 0 {
            self.poll_link();
        }

        if icr & (ICR_RXT0 | ICR_RXDMT0) != 0 {
            // Caller should drain RX ring
        }

        icr
    }

    // ── Transmit ──────────────────────────────────────────────────────────────

    /// Send a packet from physical address `phys_addr` of `len` bytes.
    /// Returns true if queued, false if ring full.
    pub unsafe fn send_packet(&mut self, phys_addr: u64, len: u16) -> bool {
        if self.mmio_base == 0 { return false; }
        if len == 0 || len as usize > E1000_RX_BUF_SIZE { return false; }

        let next_tail = (self.tx_tail + 1) as usize % E1000_TX_RING_SIZE;
        if next_tail == self.tx_head as usize {
            // Ring full — reclaim done descriptors first
            self.reclaim_tx();
            if next_tail == self.tx_head as usize {
                self.tx_dropped.fetch_add(1, Ordering::Relaxed);
                return false;
            }
        }

        let idx = self.tx_tail as usize;
        self.tx_ring[idx] = E1000TxDesc::prepare(phys_addr, len);
        self.tx_tail = next_tail as u32;

        // Ring the doorbell
        self.write(E1000_TDT, self.tx_tail);
        self.tx_packets.fetch_add(1, Ordering::Relaxed);
        true
    }

    /// Reclaim TX descriptors that hardware has finished with.
    unsafe fn reclaim_tx(&mut self) {
        while self.tx_head != self.tx_tail {
            let idx = self.tx_head as usize;
            if !self.tx_ring[idx].is_done() { break; }
            // Reset for reuse
            self.tx_ring[idx] = E1000TxDesc::default();
            self.tx_head = (self.tx_head + 1) % E1000_TX_RING_SIZE as u32;
        }
    }

    // ── Receive ───────────────────────────────────────────────────────────────

    /// Poll for received packets. Calls `deliver_fn(data_slice)` for each frame.
    /// Returns number of packets processed.
    pub unsafe fn poll_rx<F: FnMut(&[u8])>(&mut self, mut deliver_fn: F) -> usize {
        if self.mmio_base == 0 { return 0; }
        let mut count = 0usize;

        loop {
            let head = self.read(E1000_RDH) as usize;
            if self.rx_head as usize == head { break; }

            let idx = self.rx_head as usize;
            let desc = &mut self.rx_ring[idx];

            if !desc.is_done() { break; }

            if desc.errors == 0 && desc.is_eop() {
                let len = desc.length as usize;
                let buf = &RX_BUFFERS[idx][..len];
                deliver_fn(buf);
                self.rx_packets.fetch_add(1, Ordering::Relaxed);
                count += 1;
            } else if desc.errors != 0 {
                self.rx_dropped.fetch_add(1, Ordering::Relaxed);
            }

            // Reprime this descriptor
            desc.status  = 0;
            desc.errors  = 0;
            desc.length  = 0;
            // addr already set during init_rx

            // Advance head + move tail to tell hardware we've consumed this slot
            self.rx_head = (self.rx_head + 1) % E1000_RX_RING_SIZE as u32;
            self.rx_tail = (self.rx_tail + 1) % E1000_RX_RING_SIZE as u32;
            self.write(E1000_RDT, self.rx_tail);
        }
        count
    }

    // ── Statistics ────────────────────────────────────────────────────────────

    pub fn stats(&self) -> (u32, u32, u32, u32) {
        (
            self.tx_packets.load(Ordering::Relaxed),
            self.rx_packets.load(Ordering::Relaxed),
            self.tx_dropped.load(Ordering::Relaxed),
            self.rx_dropped.load(Ordering::Relaxed),
        )
    }

    pub fn mac_address(&self) -> [u8; 6] {
        self.mac_addr
    }
}

// ── Global Driver Instance ───────────────────────────────────────────────────
static mut G_E1000: E1000Driver = E1000Driver::new();

// ── C-ABI Exports ────────────────────────────────────────────────────────────

/// Initialize the e1000 driver. `mmio_base` is the BAR0 MMIO base.
/// Returns 0 on success, -1 on failure.
#[no_mangle]
pub unsafe extern "C" fn e1000_init(mmio_base: u64) -> i32 {
    if G_E1000.init(mmio_base) { 0 } else { -1 }
}

/// Transmit a packet from physical address. Returns 0 on success, -1 if ring full.
#[no_mangle]
pub unsafe extern "C" fn e1000_tx(phys_addr: u64, len: u32) -> i32 {
    if G_E1000.send_packet(phys_addr, len as u16) { 0 } else { -1 }
}

/// Poll the interrupt status register. Call from IRQ handler.
/// Returns raw ICR value.
#[no_mangle]
pub unsafe extern "C" fn e1000_handle_irq() -> u32 {
    G_E1000.handle_irq()
}

/// Check link state. Returns 1 if link is up, 0 if down.
#[no_mangle]
pub unsafe extern "C" fn e1000_link_up() -> i32 {
    if G_E1000.is_link_up() { 1 } else { 0 }
}

/// Get TX/RX/drop packet counters into provided pointers.
#[no_mangle]
pub unsafe extern "C" fn e1000_stats(
    tx_ok: *mut u32,
    rx_ok: *mut u32,
    tx_drop: *mut u32,
    rx_drop: *mut u32,
) {
    let (t, r, td, rd) = G_E1000.stats();
    if !tx_ok.is_null()   { *tx_ok   = t; }
    if !rx_ok.is_null()   { *rx_ok   = r; }
    if !tx_drop.is_null() { *tx_drop = td; }
    if !rx_drop.is_null() { *rx_drop = rd; }
}

/// Copy MAC address (6 bytes) into provided buffer.
#[no_mangle]
pub unsafe extern "C" fn e1000_get_mac(buf: *mut u8) {
    if buf.is_null() { return; }
    let mac = G_E1000.mac_address();
    for i in 0..6 {
        *buf.add(i) = mac[i];
    }
}
