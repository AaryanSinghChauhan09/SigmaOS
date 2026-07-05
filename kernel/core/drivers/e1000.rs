#![no_std]
#![allow(dead_code)]

/// SigmaOS Intel Gigabit Ethernet (e1000) Driver Stub
/// Bare-metal driver connecting to the SigmaNet stack.

const E1000_NUM_TX_DESC: usize = 8;
const E1000_NUM_RX_DESC: usize = 8;

#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
struct E1000TxDesc {
    addr: u64,
    length: u16,
    cso: u8,
    cmd: u8,
    status: u8,
    css: u8,
    special: u16,
}

#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
struct E1000RxDesc {
    addr: u64,
    length: u16,
    csum: u16,
    status: u8,
    errors: u8,
    special: u16,
}

pub struct E1000Driver {
    mmio_base: u64,
    tx_descs: [E1000TxDesc; E1000_NUM_TX_DESC],
    rx_descs: [E1000RxDesc; E1000_NUM_RX_DESC],
    tx_tail: usize,
    rx_tail: usize,
}

impl E1000Driver {
    pub const fn new() -> Self {
        Self {
            mmio_base: 0,
            tx_descs: [E1000TxDesc { addr: 0, length: 0, cso: 0, cmd: 0, status: 0, css: 0, special: 0 }; E1000_NUM_TX_DESC],
            rx_descs: [E1000RxDesc { addr: 0, length: 0, csum: 0, status: 0, errors: 0, special: 0 }; E1000_NUM_RX_DESC],
            tx_tail: 0,
            rx_tail: 0,
        }
    }

    pub fn init(&mut self, mmio_base: u64) {
        self.mmio_base = mmio_base;
        // Stub: In a real system, setup TX/RX ring base addresses via MMIO writes
    }

    pub fn send_packet(&mut self, buffer_phys_addr: u64, len: u16) {
        if self.mmio_base == 0 { return; }

        let tail = self.tx_tail;
        self.tx_descs[tail].addr = buffer_phys_addr;
        self.tx_descs[tail].length = len;
        self.tx_descs[tail].cmd = (1 << 3) | (1 << 0); // RS | EOP

        self.tx_tail = (self.tx_tail + 1) % E1000_NUM_TX_DESC;
        
        // Notify hardware
        // unsafe { core::ptr::write_volatile((self.mmio_base + 0x3818) as *mut u32, self.tx_tail as u32); }
    }
}

static mut G_E1000: E1000Driver = E1000Driver::new();

#[no_mangle]
pub unsafe extern "C" fn e1000_init(mmio_base: u64) {
    G_E1000.init(mmio_base);
}

#[no_mangle]
pub unsafe extern "C" fn e1000_tx(phys_addr: u64, len: u32) {
    G_E1000.send_packet(phys_addr, len as u16);
}
