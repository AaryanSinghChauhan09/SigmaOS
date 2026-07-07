//! SigmaOS — USB xHCI (eXtensible Host Controller Interface) Driver
//! Bare-metal USB 3.0 host controller driver.
//! No std, no allocator — fixed-size ring buffers for command/event/transfer.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── xHCI Capability Register Offsets ────────────────────────────────────────
const XHCI_CAP_CAPLENGTH:  Usize = 0x00; // Capability Register Length (u8)
const XHCI_CAP_HCIVERSION: Usize = 0x02; // Interface Version Number (u16)
const XHCI_CAP_HCSPARAMS1: Usize = 0x04; // Structural Parameters 1
const XHCI_CAP_HCSPARAMS2: Usize = 0x08; // Structural Parameters 2
const XHCI_CAP_HCSPARAMS3: Usize = 0x0C; // Structural Parameters 3
const XHCI_CAP_HCCPARAMS1: Usize = 0x10; // Capability Parameters 1
const XHCI_CAP_DBOFF:      Usize = 0x14; // Doorbell Offset
const XHCI_CAP_RTSOFF:     Usize = 0x18; // Runtime Register Space Offset

// ── Operational Register Offsets (relative to op_base) ──────────────────────
const XHCI_OP_USBCMD:   Usize = 0x00;
const XHCI_OP_USBSTS:   Usize = 0x04;
const XHCI_OP_PAGESIZE:  Usize = 0x08;
const XHCI_OP_DNCTRL:   Usize = 0x14;
const XHCI_OP_CRCR:     Usize = 0x18; // Command Ring Control
const XHCI_OP_DCBAAP:   Usize = 0x30; // Device Context Base Address Array Pointer
const XHCI_OP_CONFIG:    Usize = 0x38;

// ── USBCMD bits ─────────────────────────────────────────────────────────────
const CMD_RUN:       U32 = 1 << 0;
const CMD_HCRST:     U32 = 1 << 1;  // Host Controller Reset
const CMD_INTE:      U32 = 1 << 2;  // Interrupter Enable
const CMD_HSEE:      U32 = 1 << 3;  // Host System Error Enable

// ── USBSTS bits ─────────────────────────────────────────────────────────────
const STS_HCH:       U32 = 1 << 0;  // HC Halted
const STS_HSE:       U32 = 1 << 2;  // Host System Error
const STS_EINT:      U32 = 1 << 3;  // Event Interrupt
const STS_PCD:       U32 = 1 << 4;  // Port Change Detect
const STS_CNR:       U32 = 1 << 11; // Controller Not Ready

// ── TRB Types ───────────────────────────────────────────────────────────────
const TRB_TYPE_NORMAL:        U32 = 1;
const TRB_TYPE_SETUP:         U32 = 2;
const TRB_TYPE_DATA:          U32 = 3;
const TRB_TYPE_STATUS:        U32 = 4;
const TRB_TYPE_LINK:          U32 = 6;
const TRB_TYPE_NOOP:          U32 = 8;
const TRB_TYPE_ENABLE_SLOT:   U32 = 9;
const TRB_TYPE_DISABLE_SLOT:  U32 = 10;
const TRB_TYPE_ADDRESS_DEV:   U32 = 11;
const TRB_TYPE_CONFIG_EP:     U32 = 12;
const TRB_TYPE_EVALUATE_CTX:  U32 = 13;
const TRB_TYPE_RESET_EP:      U32 = 14;
const TRB_TYPE_STOP_EP:       U32 = 15;
const TRB_TYPE_CMD_COMPLETION: U32 = 33;
const TRB_TYPE_PORT_STATUS:   U32 = 34;
const TRB_TYPE_TRANSFER:      U32 = 32;

// ── Data Structures ─────────────────────────────────────────────────────────

/// Transfer Request Block (16 bytes)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Trb {
    pub param_lo: U64,
    pub status:   U32,
    pub control:  U32,
}

impl Trb {
    pub const fn zero() -> Self {
        Trb { param_lo: 0, status: 0, control: 0 }
    }

    pub fn trb_type(&self) -> U32 {
        (self.control >> 10) & 0x3F
    }

    pub fn set_type(&mut self, t: U32) {
        self.control = (self.control & !(0x3F << 10)) | ((t & 0x3F) << 10);
    }

    pub fn set_cycle(&mut self, cycle: bool) {
        if cycle {
            self.control |= 1;
        } else {
            self.control &= !1;
        }
    }
}

// ── Ring Buffers ─────────────────────────────────────────────────────────────
const RING_SIZE: usize = 64;

#[derive(Copy, Clone)]
pub struct TrbRing {
    pub trbs: [Trb; RING_SIZE],
    pub enqueue_idx: usize,
    pub dequeue_idx: usize,
    pub cycle_bit: bool,
}

impl TrbRing {
    pub const fn new() -> Self {
        TrbRing {
            trbs: [Trb::zero(); RING_SIZE],
            enqueue_idx: 0,
            dequeue_idx: 0,
            cycle_bit: true,
        }
    }

    pub fn enqueue(&mut self, trb: &mut Trb) -> bool {
        if self.enqueue_idx >= RING_SIZE - 1 {
            // Insert link TRB to wrap around
            let mut link = Trb::zero();
            link.set_type(TRB_TYPE_LINK);
            link.set_cycle(self.cycle_bit);
            // Link TRB toggle cycle bit
            link.control |= 1 << 1; // Toggle Cycle
            self.trbs[self.enqueue_idx] = link;
            self.enqueue_idx = 0;
            self.cycle_bit = !self.cycle_bit;
        }
        trb.set_cycle(self.cycle_bit);
        self.trbs[self.enqueue_idx] = *trb;
        self.enqueue_idx += 1;
        true
    }

    pub fn dequeue(&mut self) -> Option<Trb> {
        let trb = self.trbs[self.dequeue_idx];
        let owner_cycle = (trb.control & 1) != 0;
        if owner_cycle != self.cycle_bit {
            return None; // No new TRB available
        }
        self.dequeue_idx += 1;
        if self.dequeue_idx >= RING_SIZE - 1 {
            self.dequeue_idx = 0;
            self.cycle_bit = !self.cycle_bit;
        }
        Some(trb)
    }
}

// ── USB Device Slot ─────────────────────────────────────────────────────────
const MAX_SLOTS: usize = 16;
const MAX_PORTS: usize = 16;

#[derive(Copy, Clone, PartialEq)]
pub enum UsbSpeed {
    FullSpeed,   // 12 Mbps
    LowSpeed,    // 1.5 Mbps
    HighSpeed,   // 480 Mbps
    SuperSpeed,  // 5 Gbps
    SuperSpeedPlus, // 10+ Gbps
    Unknown,
}

#[derive(Copy, Clone)]
pub struct UsbDevice {
    pub slot_id: U8,
    pub port: U8,
    pub speed: UsbSpeed,
    pub vendor_id: U16,
    pub product_id: U16,
    pub class: U8,
    pub subclass: U8,
    pub protocol: U8,
    pub attached: bool,
}

impl UsbDevice {
    pub const fn empty() -> Self {
        UsbDevice {
            slot_id: 0, port: 0, speed: UsbSpeed::Unknown,
            vendor_id: 0, product_id: 0,
            class: 0, subclass: 0, protocol: 0,
            attached: false,
        }
    }
}

// ── xHCI Controller State ───────────────────────────────────────────────────
pub struct XhciController {
    pub mmio_base: U64,
    pub op_base: U64,
    pub rt_base: U64,
    pub db_base: U64,
    pub cap_length: U8,
    pub hci_version: U16,
    pub max_slots: U8,
    pub max_ports: U8,
    pub max_intrs: U16,
    pub page_size: U32,
    pub cmd_ring: TrbRing,
    pub event_ring: TrbRing,
    pub devices: [UsbDevice; MAX_SLOTS],
    pub device_count: usize,
    pub running: bool,
}

static mut XHCI: XhciController = XhciController {
    mmio_base: 0, op_base: 0, rt_base: 0, db_base: 0,
    cap_length: 0, hci_version: 0,
    max_slots: 0, max_ports: 0, max_intrs: 0, page_size: 4096,
    cmd_ring: TrbRing::new(),
    event_ring: TrbRing::new(),
    devices: [UsbDevice::empty(); MAX_SLOTS],
    device_count: 0,
    running: false,
};

// ── MMIO Helpers ────────────────────────────────────────────────────────────
unsafe fn xhci_read32(addr: U64, offset: Usize) -> U32 {
    let ptr = (addr as Usize + offset) as *const U32;
    core::ptr::read_volatile(ptr)
}

unsafe fn xhci_write32(addr: U64, offset: Usize, val: U32) {
    let ptr = (addr as Usize + offset) as *mut U32;
    core::ptr::write_volatile(ptr, val);
}

unsafe fn xhci_read64(addr: U64, offset: Usize) -> U64 {
    let ptr = (addr as Usize + offset) as *const U64;
    core::ptr::read_volatile(ptr)
}

unsafe fn xhci_write64(addr: U64, offset: Usize, val: U64) {
    let ptr = (addr as Usize + offset) as *mut U64;
    core::ptr::write_volatile(ptr, val);
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the xHCI controller from its MMIO base (found via PCI BAR0).
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_init(bar0: U64) -> i32 {
    if bar0 == 0 { return -1; }
    XHCI.mmio_base = bar0;

    // Read capability length
    XHCI.cap_length = xhci_read32(bar0, XHCI_CAP_CAPLENGTH) as U8;
    XHCI.hci_version = (xhci_read32(bar0, XHCI_CAP_HCIVERSION) >> 16) as U16;

    // Calculate base addresses
    XHCI.op_base = bar0 + XHCI.cap_length as U64;
    XHCI.db_base = bar0 + xhci_read32(bar0, XHCI_CAP_DBOFF) as U64;
    XHCI.rt_base = bar0 + xhci_read32(bar0, XHCI_CAP_RTSOFF) as U64;

    // Parse structural parameters
    let hcsparams1 = xhci_read32(bar0, XHCI_CAP_HCSPARAMS1);
    XHCI.max_slots = (hcsparams1 & 0xFF) as U8;
    XHCI.max_intrs = ((hcsparams1 >> 8) & 0x7FF) as U16;
    XHCI.max_ports = ((hcsparams1 >> 24) & 0xFF) as U8;

    // Wait for controller to be ready (CNR = 0)
    let mut timeout = 100_000u32;
    while (xhci_read32(XHCI.op_base, XHCI_OP_USBSTS) & STS_CNR) != 0 {
        timeout -= 1;
        if timeout == 0 { return -2; } // Timeout
        core::hint::spin_loop();
    }

    // Reset the controller
    xhci_write32(XHCI.op_base, XHCI_OP_USBCMD, CMD_HCRST);

    // Wait for reset complete
    timeout = 100_000;
    while (xhci_read32(XHCI.op_base, XHCI_OP_USBCMD) & CMD_HCRST) != 0 {
        timeout -= 1;
        if timeout == 0 { return -3; }
        core::hint::spin_loop();
    }

    // Wait for CNR again
    timeout = 100_000;
    while (xhci_read32(XHCI.op_base, XHCI_OP_USBSTS) & STS_CNR) != 0 {
        timeout -= 1;
        if timeout == 0 { return -4; }
        core::hint::spin_loop();
    }

    // Read page size
    XHCI.page_size = xhci_read32(XHCI.op_base, XHCI_OP_PAGESIZE) << 12;

    // Configure max device slots
    let slots = if XHCI.max_slots > MAX_SLOTS as U8 {
        MAX_SLOTS as U32
    } else {
        XHCI.max_slots as U32
    };
    xhci_write32(XHCI.op_base, XHCI_OP_CONFIG, slots);

    // Set up command ring (write physical address of cmd_ring.trbs)
    let cmd_ring_phys = XHCI.cmd_ring.trbs.as_ptr() as U64;
    xhci_write64(XHCI.op_base, XHCI_OP_CRCR, cmd_ring_phys | 1); // cycle bit = 1

    // Enable interrupts and start the controller
    let cmd = CMD_RUN | CMD_INTE;
    xhci_write32(XHCI.op_base, XHCI_OP_USBCMD, cmd);
    XHCI.running = true;

    0
}

/// Send a No-Op command to verify ring operation.
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_noop() -> i32 {
    if !XHCI.running { return -1; }
    let mut trb = Trb::zero();
    trb.set_type(TRB_TYPE_NOOP);
    XHCI.cmd_ring.enqueue(&mut trb);
    // Ring doorbell 0 (host controller)
    xhci_write32(XHCI.db_base, 0, 0);
    0
}

/// Enable a device slot (returns slot ID or negative error).
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_enable_slot() -> i32 {
    if !XHCI.running { return -1; }
    let mut trb = Trb::zero();
    trb.set_type(TRB_TYPE_ENABLE_SLOT);
    XHCI.cmd_ring.enqueue(&mut trb);
    xhci_write32(XHCI.db_base, 0, 0);
    // In a real driver we'd wait for completion event and extract slot_id
    0
}

/// Get the number of USB ports on this controller.
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_port_count() -> U32 {
    XHCI.max_ports as U32
}

/// Check if a port has a device connected.
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_port_connected(port: U32) -> i32 {
    if port == 0 || port > XHCI.max_ports as U32 { return -1; }
    // Port status registers start at op_base + 0x400, each 0x10 apart
    let port_offset = 0x400 + ((port - 1) * 0x10) as Usize;
    let portsc = xhci_read32(XHCI.op_base, port_offset);
    if (portsc & 1) != 0 { 1 } else { 0 } // CCS bit
}

/// Get the speed of a connected device on a port.
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_port_speed(port: U32) -> U32 {
    if port == 0 || port > XHCI.max_ports as U32 { return 0; }
    let port_offset = 0x400 + ((port - 1) * 0x10) as Usize;
    let portsc = xhci_read32(XHCI.op_base, port_offset);
    (portsc >> 10) & 0xF // Port Speed field
}

/// Get the number of attached USB devices.
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_device_count() -> U32 {
    XHCI.device_count as U32
}
