// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/linux_compat/e1000_main.rs — Intel e1000 NIC compat shim
//
// Provides a SigmaOS-native e1000 driver built by studying the Linux
// e1000 driver structure (cleanroom — no GPL source copied).
// This demonstrates the Linux→SigmaOS porting pattern via the distro
// compat layer.
//
// Hardware: Intel 82540/82541/82542/82543/82544/82545/82546 GbE
// PCI IDs:  0x8086:0x100E (most common QEMU/VirtualBox default NIC)
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

// ── e1000 register offsets (from Intel 8254x datasheet) ───────────────────
const E1000_CTRL:    u32 = 0x00000; // Device Control
const E1000_STATUS:  u32 = 0x00008; // Device Status
const E1000_EECD:    u32 = 0x00010; // EEPROM/Flash Control
const E1000_EERD:    u32 = 0x00014; // EEPROM Read
const E1000_ICR:     u32 = 0x000C0; // Interrupt Cause Read
const E1000_ICS:     u32 = 0x000C8; // Interrupt Cause Set
const E1000_IMS:     u32 = 0x000D0; // Interrupt Mask Set/Read
const E1000_IMC:     u32 = 0x000D8; // Interrupt Mask Clear
const E1000_RCTL:    u32 = 0x00100; // Receive Control
const E1000_TCTL:    u32 = 0x00400; // Transmit Control
const E1000_RDBAL:   u32 = 0x02800; // RX Descriptor Base Low
const E1000_RDBAH:   u32 = 0x02804; // RX Descriptor Base High
const E1000_RDLEN:   u32 = 0x02808; // RX Descriptor Length
const E1000_RDH:     u32 = 0x02810; // RX Descriptor Head
const E1000_RDT:     u32 = 0x02818; // RX Descriptor Tail
const E1000_TDBAL:   u32 = 0x03800; // TX Descriptor Base Low
const E1000_TDBAH:   u32 = 0x03804; // TX Descriptor Base High
const E1000_TDLEN:   u32 = 0x03808; // TX Descriptor Length
const E1000_TDH:     u32 = 0x03810; // TX Descriptor Head
const E1000_TDT:     u32 = 0x03818; // TX Descriptor Tail
const E1000_RAL0:    u32 = 0x05400; // Receive Address Low 0 (MAC[3:0])
const E1000_RAH0:    u32 = 0x05404; // Receive Address High 0 (MAC[5:4] + AV)

// ── Control register bits ─────────────────────────────────────────────────
const E1000_CTRL_RST:    u32 = 1 << 26; // Software reset
const E1000_CTRL_ASDE:   u32 = 1 << 5;  // Auto-speed detection enable
const E1000_CTRL_SLU:    u32 = 1 << 6;  // Set link up
const E1000_CTRL_FRCSPD: u32 = 1 << 11; // Force speed
const E1000_CTRL_FRCDPX: u32 = 1 << 12; // Force duplex

// ── Interrupt bits ────────────────────────────────────────────────────────
const E1000_ICR_TXDW:  u32 = 1 << 0;  // TX descriptor written back
const E1000_ICR_LSC:   u32 = 1 << 2;  // Link status change
const E1000_ICR_RXO:   u32 = 1 << 6;  // RX overrun
const E1000_ICR_RXT0:  u32 = 1 << 7;  // RX timer interrupt

// ── Receive control bits ──────────────────────────────────────────────────
const E1000_RCTL_EN:     u32 = 1 << 1;  // Receiver enable
const E1000_RCTL_SBP:    u32 = 1 << 2;  // Store bad packets
const E1000_RCTL_UPE:    u32 = 1 << 3;  // Unicast promiscuous
const E1000_RCTL_MPE:    u32 = 1 << 4;  // Multicast promiscuous
const E1000_RCTL_BAM:    u32 = 1 << 15; // Broadcast accept mode
const E1000_RCTL_BSIZE_2048: u32 = 0;   // 2048-byte buffer size
const E1000_RCTL_SECRC:  u32 = 1 << 26; // Strip ethernet CRC

// ── TX control bits ───────────────────────────────────────────────────────
const E1000_TCTL_EN:     u32 = 1 << 1;  // Transmit enable
const E1000_TCTL_PSP:    u32 = 1 << 3;  // Pad short packets
const E1000_TCTL_CT_SHIFT: u32 = 4;
const E1000_TCTL_COLD_SHIFT: u32 = 12;

// ── Descriptor structures ─────────────────────────────────────────────────
const RX_DESC_COUNT: usize = 256;
const TX_DESC_COUNT: usize = 256;

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct RxDesc {
    addr:   u64,  // Buffer physical address
    length: u16,
    csum:   u16,
    status: u8,
    errors: u8,
    special:u16,
}

const E1000_RXD_STAT_DD:  u8 = 1 << 0; // Descriptor done
const E1000_RXD_STAT_EOP: u8 = 1 << 1; // End of packet

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct TxDesc {
    addr:    u64,  // Buffer physical address
    length:  u16,
    cso:     u8,   // Checksum offset
    cmd:     u8,   // Command
    status:  u8,
    css:     u8,   // Checksum start
    special: u16,
}

const E1000_TXD_CMD_EOP:  u8 = 1 << 0; // End of packet
const E1000_TXD_CMD_FCS:  u8 = 1 << 1; // Insert FCS
const E1000_TXD_CMD_RS:   u8 = 1 << 3; // Report status
const E1000_TXD_STAT_DD:  u8 = 1 << 0; // Descriptor done

// ── Driver state ──────────────────────────────────────────────────────────
pub struct E1000Adapter {
    bar0:        *mut u8,        // MMIO base
    irq:         u8,
    mac:         [u8; 6],
    rx_descs:    [RxDesc; RX_DESC_COUNT],
    tx_descs:    [TxDesc; TX_DESC_COUNT],
    rx_buf_phys: [u64; RX_DESC_COUNT],
    tx_buf_phys: [u64; TX_DESC_COUNT],
    rx_cur:      usize,
    tx_cur:      usize,
    initialized: bool,
    link_up:     bool,
}

unsafe impl Send for E1000Adapter {}
unsafe impl Sync for E1000Adapter {}

impl E1000Adapter {
    pub const fn zeroed() -> Self {
        Self {
            bar0: core::ptr::null_mut(),
            irq: 0, mac: [0u8; 6],
            rx_descs: [RxDesc { addr:0,length:0,csum:0,status:0,errors:0,special:0 }; RX_DESC_COUNT],
            tx_descs: [TxDesc { addr:0,length:0,cso:0,cmd:0,status:0,css:0,special:0 }; TX_DESC_COUNT],
            rx_buf_phys: [0u64; RX_DESC_COUNT],
            tx_buf_phys: [0u64; TX_DESC_COUNT],
            rx_cur: 0, tx_cur: 0,
            initialized: false, link_up: false,
        }
    }

    #[inline(always)]
    unsafe fn mmio_read(&self, reg: u32) -> u32 {
        core::ptr::read_volatile(self.bar0.add(reg as usize) as *const u32)
    }

    #[inline(always)]
    unsafe fn mmio_write(&mut self, reg: u32, val: u32) {
        core::ptr::write_volatile(self.bar0.add(reg as usize) as *mut u32, val);
    }

    /// Issue a software reset and wait for it to complete.
    unsafe fn reset(&mut self) {
        self.mmio_write(E1000_CTRL, self.mmio_read(E1000_CTRL) | E1000_CTRL_RST);
        // Wait ~1µs (busy-spin — real driver uses udelay)
        for _ in 0..10_000 { core::arch::asm!("nop", options(nomem, nostack)); }
    }

    /// Read MAC address from EEPROM via RAL0/RAH0 registers.
    unsafe fn read_mac(&mut self) {
        let ral = self.mmio_read(E1000_RAL0);
        let rah = self.mmio_read(E1000_RAH0);
        self.mac[0] = (ral & 0xFF) as u8;
        self.mac[1] = ((ral >> 8)  & 0xFF) as u8;
        self.mac[2] = ((ral >> 16) & 0xFF) as u8;
        self.mac[3] = ((ral >> 24) & 0xFF) as u8;
        self.mac[4] = (rah & 0xFF) as u8;
        self.mac[5] = ((rah >> 8)  & 0xFF) as u8;
    }

    /// Initialise RX descriptor ring.
    unsafe fn init_rx(&mut self) {
        extern "C" { fn sigma_dma_alloc(size: usize, phys: *mut u64) -> *mut u8; }
        for i in 0..RX_DESC_COUNT {
            let mut phys: u64 = 0;
            let _buf = sigma_dma_alloc(2048, &mut phys);
            self.rx_buf_phys[i] = phys;
            self.rx_descs[i].addr   = phys;
            self.rx_descs[i].status = 0;
        }
        let ring_phys = self.rx_descs.as_ptr() as u64;
        self.mmio_write(E1000_RDBAL, (ring_phys & 0xFFFF_FFFF) as u32);
        self.mmio_write(E1000_RDBAH, (ring_phys >> 32) as u32);
        self.mmio_write(E1000_RDLEN, (RX_DESC_COUNT * core::mem::size_of::<RxDesc>()) as u32);
        self.mmio_write(E1000_RDH,   0);
        self.mmio_write(E1000_RDT,   (RX_DESC_COUNT - 1) as u32);
        let rctl = E1000_RCTL_EN | E1000_RCTL_BAM | E1000_RCTL_SECRC | E1000_RCTL_BSIZE_2048;
        self.mmio_write(E1000_RCTL, rctl);
    }

    /// Initialise TX descriptor ring.
    unsafe fn init_tx(&mut self) {
        for i in 0..TX_DESC_COUNT {
            self.tx_descs[i].status = E1000_TXD_STAT_DD; // mark all as done initially
            self.tx_buf_phys[i] = 0;
        }
        let ring_phys = self.tx_descs.as_ptr() as u64;
        self.mmio_write(E1000_TDBAL, (ring_phys & 0xFFFF_FFFF) as u32);
        self.mmio_write(E1000_TDBAH, (ring_phys >> 32) as u32);
        self.mmio_write(E1000_TDLEN, (TX_DESC_COUNT * core::mem::size_of::<TxDesc>()) as u32);
        self.mmio_write(E1000_TDH,   0);
        self.mmio_write(E1000_TDT,   0);
        let tctl = E1000_TCTL_EN | E1000_TCTL_PSP
            | (0x10 << E1000_TCTL_CT_SHIFT)
            | (0x40 << E1000_TCTL_COLD_SHIFT);
        self.mmio_write(E1000_TCTL, tctl);
    }

    /// Full hardware init — probe → reset → MAC read → RX/TX rings → link up.
    pub unsafe fn init(&mut self, bar0: *mut u8, irq: u8) -> i32 {
        self.bar0 = bar0;
        self.irq  = irq;
        self.reset();
        self.read_mac();
        self.init_rx();
        self.init_tx();
        // Enable interrupts: RX timer + link status change
        let ims = E1000_ICR_RXT0 | E1000_ICR_LSC;
        self.mmio_write(E1000_IMS, ims);
        // Force link up
        let ctrl = self.mmio_read(E1000_CTRL);
        self.mmio_write(E1000_CTRL, ctrl | E1000_CTRL_SLU);
        self.initialized = true;
        0
    }

    /// Transmit a raw ethernet frame.
    pub unsafe fn send(&mut self, buf_phys: u64, len: u16) -> i32 {
        if !self.initialized { return -1; }
        let cur = self.tx_cur;
        // Wait for descriptor to be free
        let mut spins = 0u32;
        while self.tx_descs[cur].status & E1000_TXD_STAT_DD == 0 {
            spins += 1;
            if spins > 100_000 { return -11; } // EAGAIN
            core::arch::asm!("pause", options(nomem, nostack));
        }
        self.tx_descs[cur].addr   = buf_phys;
        self.tx_descs[cur].length = len;
        self.tx_descs[cur].cmd    = E1000_TXD_CMD_EOP | E1000_TXD_CMD_FCS | E1000_TXD_CMD_RS;
        self.tx_descs[cur].status = 0;
        self.tx_cur = (cur + 1) % TX_DESC_COUNT;
        self.mmio_write(E1000_TDT, self.tx_cur as u32);
        0
    }

    /// IRQ handler — drain RX ring, forward packets to sigma-bus.
    pub unsafe fn handle_irq(&mut self) -> bool {
        let icr = self.mmio_read(E1000_ICR);
        if icr == 0 { return false; } // not our IRQ

        if icr & E1000_ICR_LSC != 0 {
            let status = self.mmio_read(E1000_STATUS);
            self.link_up = status & 0x2 != 0; // STATUS.LU bit
        }

        if icr & (E1000_ICR_RXT0 | E1000_ICR_RXO) != 0 {
            self.drain_rx();
        }
        true
    }

    unsafe fn drain_rx(&mut self) {
        extern "C" { fn sigma_bus_send(ch: u32, data: *const u8, len: usize) -> i32; }
        loop {
            let cur = self.rx_cur;
            let desc = &mut self.rx_descs[cur];
            if desc.status & E1000_RXD_STAT_DD == 0 { break; }
            if desc.status & E1000_RXD_STAT_EOP != 0 && desc.errors == 0 {
                let buf_ptr = self.rx_buf_phys[cur] as *const u8;
                let pkt_len = desc.length as usize;
                // Forward packet to sigma-bus channel 0x20 (NIC RX)
                sigma_bus_send(0x20, buf_ptr, pkt_len);
            }
            // Return descriptor to hardware
            desc.status = 0;
            self.mmio_write(E1000_RDT, cur as u32);
            self.rx_cur = (cur + 1) % RX_DESC_COUNT;
        }
    }

    pub fn mac_addr(&self) -> [u8; 6] { self.mac }
    pub fn is_link_up(&self) -> bool  { self.link_up }
}

// ── Global adapter instance ───────────────────────────────────────────────
static mut G_E1000: E1000Adapter = E1000Adapter::zeroed();

// ── C-ABI exports (SDF lifecycle) ─────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn e1000_probe(pci_bar: u64, irq: u8) -> i32 {
    // Verify PCI vendor/device ID = 0x8086:0x100E
    extern "C" { fn sigma_pci_read_config32(b:u8,d:u8,f:u8,off:u8)->u32; }
    let ids = sigma_pci_read_config32(0, 0, 0, 0);
    let vendor = ids & 0xFFFF;
    let device = (ids >> 16) & 0xFFFF;
    if vendor != 0x8086 { return -1; }
    match device {
        0x100E | 0x100F | 0x1004 | 0x1008 |
        0x1010 | 0x1011 | 0x1016 | 0x1017 |
        0x1018 | 0x101E | 0x1026 | 0x1027 |
        0x1028 | 0x1049 | 0x104A | 0x104C => {}
        _ => return -1,
    }
    let bar0 = pci_bar as *mut u8;
    G_E1000.init(bar0, irq)
}

#[no_mangle]
pub unsafe extern "C" fn e1000_init() -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn e1000_shutdown() {
    if G_E1000.initialized {
        G_E1000.mmio_write(E1000_RCTL, 0); // disable RX
        G_E1000.mmio_write(E1000_TCTL, 0); // disable TX
        G_E1000.mmio_write(E1000_IMC, 0xFFFF_FFFF); // mask all IRQs
        G_E1000.initialized = false;
    }
}

#[no_mangle]
pub unsafe extern "C" fn e1000_send(buf_phys: u64, len: u16) -> i32 {
    G_E1000.send(buf_phys, len)
}

#[no_mangle]
pub unsafe extern "C" fn e1000_irq() -> bool {
    G_E1000.handle_irq()
}

#[no_mangle]
pub unsafe extern "C" fn e1000_get_mac(out: *mut u8) {
    if out.is_null() { return; }
    let mac = G_E1000.mac_addr();
    for i in 0..6 { *out.add(i) = mac[i]; }
}

#[no_mangle]
pub unsafe extern "C" fn e1000_link_up() -> bool {
    G_E1000.is_link_up()
}
