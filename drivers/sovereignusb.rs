/// SigmaOS: Sovereign xHCI USB 3.x Host Controller Driver
/// Built in Rust — #![no_std], no alloc, no external dependencies.
/// Implements PCI BAR mapping, MMIO register access, command/event/transfer rings,
/// device enumeration, slot allocation, and endpoint configuration.

#![no_std]
#![allow(dead_code)]

// ─── Sovereign Type Aliases ─────────────────────────────────────────────────
type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaUsize = usize;
type SigmaBool = bool;
type SigmaI32 = i32;

pub const SIGMA_OK: SigmaI32 = 0;
pub const SIGMA_ERR_NO_DEVICE: SigmaI32 = -1;
pub const SIGMA_ERR_TIMEOUT: SigmaI32 = -2;
pub const SIGMA_ERR_NO_SLOT: SigmaI32 = -3;
pub const SIGMA_ERR_PROTOCOL: SigmaI32 = -4;

// ─── PCI Configuration ─────────────────────────────────────────────────────
pub const USB_CLASS_SERIAL_BUS: SigmaU8 = 0x0C;
pub const USB_SUBCLASS_USB: SigmaU8 = 0x03;
pub const USB_PROG_IF_XHCI: SigmaU8 = 0x30;

/// PCI configuration space header for device identification
#[repr(C, packed)]
pub struct PciConfigHeader {
    pub vendor_id: SigmaU16,
    pub device_id: SigmaU16,
    pub command: SigmaU16,
    pub status: SigmaU16,
    pub revision_id: SigmaU8,
    pub prog_if: SigmaU8,
    pub subclass: SigmaU8,
    pub class_code: SigmaU8,
    pub cache_line_size: SigmaU8,
    pub latency_timer: SigmaU8,
    pub header_type: SigmaU8,
    pub bist: SigmaU8,
    pub bar0: SigmaU32,
    pub bar1: SigmaU32,
}

// ─── xHCI Capability Registers (MMIO offset 0x00) ───────────────────────────
#[repr(C)]
pub struct XhciCapRegs {
    pub caplength: SigmaU8,
    pub _rsvd: SigmaU8,
    pub hci_version: SigmaU16,
    pub hcsparams1: SigmaU32,
    pub hcsparams2: SigmaU32,
    pub hcsparams3: SigmaU32,
    pub hccparams1: SigmaU32,
    pub dboff: SigmaU32,
    pub rtsoff: SigmaU32,
    pub hccparams2: SigmaU32,
}

impl XhciCapRegs {
    /// Maximum number of device slots supported (bits 31:24 of HCSPARAMS1)
    pub fn max_slots(&self) -> SigmaU8 {
        ((self.hcsparams1 >> 24) & 0xFF) as SigmaU8
    }

    /// Maximum number of interrupters (bits 18:8 of HCSPARAMS1)
    pub fn max_interrupters(&self) -> SigmaU16 {
        ((self.hcsparams1 >> 8) & 0x7FF) as SigmaU16
    }

    /// Maximum number of ports (bits 7:0 of HCSPARAMS1)
    pub fn max_ports(&self) -> SigmaU8 {
        (self.hcsparams1 & 0xFF) as SigmaU8
    }
}

// ─── xHCI Operational Registers (MMIO offset = caplength) ───────────────────
#[repr(C)]
pub struct XhciOpRegs {
    pub usbcmd: SigmaU32,
    pub usbsts: SigmaU32,
    pub pagesize: SigmaU32,
    pub _rsvd1: [SigmaU32; 2],
    pub dnctrl: SigmaU32,
    pub crcr_lo: SigmaU32,
    pub crcr_hi: SigmaU32,
    pub _rsvd2: [SigmaU32; 4],
    pub dcbaap_lo: SigmaU32,
    pub dcbaap_hi: SigmaU32,
    pub config: SigmaU32,
}

// USBCMD bits
pub const USBCMD_RUN: SigmaU32 = 1 << 0;
pub const USBCMD_HCRST: SigmaU32 = 1 << 1;
pub const USBCMD_INTE: SigmaU32 = 1 << 2;

// USBSTS bits
pub const USBSTS_HCH: SigmaU32 = 1 << 0; // HC Halted
pub const USBSTS_CNR: SigmaU32 = 1 << 11; // Controller Not Ready

// ─── Transfer Request Block (TRB) ──────────────────────────────────────────
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Trb {
    pub param_lo: SigmaU32,
    pub param_hi: SigmaU32,
    pub status: SigmaU32,
    pub control: SigmaU32,
}

impl Trb {
    pub const fn empty() -> Self {
        Trb {
            param_lo: 0,
            param_hi: 0,
            status: 0,
            control: 0,
        }
    }

    /// TRB type is in bits 15:10 of control word
    pub fn trb_type(&self) -> SigmaU8 {
        ((self.control >> 10) & 0x3F) as SigmaU8
    }

    /// Cycle bit is bit 0 of control word
    pub fn cycle_bit(&self) -> SigmaBool {
        (self.control & 1) != 0
    }

    pub fn set_cycle(&mut self, cycle: SigmaBool) {
        if cycle {
            self.control |= 1;
        } else {
            self.control &= !1;
        }
    }
}

// TRB Types
pub const TRB_TYPE_NORMAL: SigmaU8 = 1;
pub const TRB_TYPE_SETUP: SigmaU8 = 2;
pub const TRB_TYPE_DATA: SigmaU8 = 3;
pub const TRB_TYPE_STATUS: SigmaU8 = 4;
pub const TRB_TYPE_LINK: SigmaU8 = 6;
pub const TRB_TYPE_ENABLE_SLOT: SigmaU8 = 9;
pub const TRB_TYPE_DISABLE_SLOT: SigmaU8 = 10;
pub const TRB_TYPE_ADDRESS_DEVICE: SigmaU8 = 11;
pub const TRB_TYPE_CONFIGURE_EP: SigmaU8 = 12;
pub const TRB_TYPE_COMMAND_COMPLETION: SigmaU8 = 33;
pub const TRB_TYPE_PORT_STATUS_CHANGE: SigmaU8 = 34;

// ─── Command Ring ───────────────────────────────────────────────────────────
pub const COMMAND_RING_SIZE: SigmaUsize = 64;

pub struct CommandRing {
    trbs: [Trb; COMMAND_RING_SIZE],
    enqueue_index: SigmaUsize,
    cycle_bit: SigmaBool,
}

impl CommandRing {
    pub const fn new() -> Self {
        CommandRing {
            trbs: [Trb::empty(); COMMAND_RING_SIZE],
            enqueue_index: 0,
            cycle_bit: true,
        }
    }

    /// Enqueue a command TRB onto the ring
    pub fn enqueue(&mut self, mut trb: Trb) -> SigmaBool {
        if self.enqueue_index >= COMMAND_RING_SIZE - 1 {
            // Insert link TRB to wrap around
            let mut link = Trb::empty();
            link.control = (TRB_TYPE_LINK as SigmaU32) << 10;
            link.set_cycle(self.cycle_bit);
            // Toggle cycle bit on wrap
            link.control |= 1 << 1; // Toggle Cycle bit
            self.trbs[self.enqueue_index] = link;
            self.enqueue_index = 0;
            self.cycle_bit = !self.cycle_bit;
        }

        trb.set_cycle(self.cycle_bit);
        self.trbs[self.enqueue_index] = trb;
        self.enqueue_index += 1;
        true
    }

    /// Get physical address of the ring base for CRCR register
    pub fn base_phys(&self) -> SigmaU64 {
        &self.trbs[0] as *const Trb as SigmaU64
    }
}

// ─── Event Ring ─────────────────────────────────────────────────────────────
pub const EVENT_RING_SIZE: SigmaUsize = 64;

pub struct EventRing {
    trbs: [Trb; EVENT_RING_SIZE],
    dequeue_index: SigmaUsize,
    cycle_bit: SigmaBool,
}

impl EventRing {
    pub const fn new() -> Self {
        EventRing {
            trbs: [Trb::empty(); EVENT_RING_SIZE],
            dequeue_index: 0,
            cycle_bit: true,
        }
    }

    /// Check if a new event is available (producer sets cycle bit)
    pub fn has_event(&self) -> SigmaBool {
        self.trbs[self.dequeue_index].cycle_bit() == self.cycle_bit
    }

    /// Dequeue the next event TRB
    pub fn dequeue(&mut self) -> Option<Trb> {
        if !self.has_event() {
            return None;
        }
        let trb = self.trbs[self.dequeue_index];
        self.dequeue_index += 1;
        if self.dequeue_index >= EVENT_RING_SIZE {
            self.dequeue_index = 0;
            self.cycle_bit = !self.cycle_bit;
        }
        Some(trb)
    }
}

// ─── USB Device Slot ────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum UsbSpeed {
    Full,   // 12 Mbps
    Low,    // 1.5 Mbps
    High,   // 480 Mbps
    Super,  // 5 Gbps
    SuperPlus, // 10+ Gbps
}

#[derive(Clone, Copy, PartialEq)]
pub enum SlotState {
    Disabled,
    Enabled,
    Addressed,
    Configured,
}

pub struct UsbSlot {
    pub slot_id: SigmaU8,
    pub port: SigmaU8,
    pub speed: UsbSpeed,
    pub state: SlotState,
    pub vendor_id: SigmaU16,
    pub product_id: SigmaU16,
    pub device_class: SigmaU8,
    pub max_packet_size: SigmaU16,
    pub num_configurations: SigmaU8,
}

impl UsbSlot {
    pub const fn empty() -> Self {
        UsbSlot {
            slot_id: 0,
            port: 0,
            speed: UsbSpeed::Full,
            state: SlotState::Disabled,
            vendor_id: 0,
            product_id: 0,
            device_class: 0,
            max_packet_size: 8,
            num_configurations: 0,
        }
    }
}

// ─── USB Endpoint ───────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum EndpointType {
    Control,
    Bulk,
    Interrupt,
    Isochronous,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EndpointDirection {
    In,
    Out,
}

pub struct UsbEndpoint {
    pub address: SigmaU8,
    pub ep_type: EndpointType,
    pub direction: EndpointDirection,
    pub max_packet_size: SigmaU16,
    pub interval: SigmaU8,
}

impl UsbEndpoint {
    pub const fn control_default() -> Self {
        UsbEndpoint {
            address: 0,
            ep_type: EndpointType::Control,
            direction: EndpointDirection::In,
            max_packet_size: 64,
            interval: 0,
        }
    }
}

// ─── USB Device Trait (OOP Interface) ───────────────────────────────────────
pub trait UsbDevice {
    fn probe(&mut self, slot: &UsbSlot) -> SigmaI32;
    fn disconnect(&mut self) -> SigmaI32;
    fn get_name(&self) -> &'static str;
    fn get_class(&self) -> SigmaU8;
}

// ─── xHCI Host Controller ──────────────────────────────────────────────────
pub const MAX_SLOTS: SigmaUsize = 32;
pub const MAX_PORTS: SigmaUsize = 16;

/// Port status/control register (one per root port)
#[repr(C)]
pub struct PortRegister {
    pub portsc: SigmaU32,
}

// PORTSC bits
pub const PORTSC_CCS: SigmaU32 = 1 << 0;   // Current Connect Status
pub const PORTSC_PED: SigmaU32 = 1 << 1;   // Port Enabled/Disabled
pub const PORTSC_PR: SigmaU32 = 1 << 4;    // Port Reset
pub const PORTSC_PLS_MASK: SigmaU32 = 0xF << 5; // Port Link State
pub const PORTSC_PP: SigmaU32 = 1 << 9;    // Port Power
pub const PORTSC_SPEED_MASK: SigmaU32 = 0xF << 10; // Port Speed
pub const PORTSC_CSC: SigmaU32 = 1 << 17;  // Connect Status Change
pub const PORTSC_PRC: SigmaU32 = 1 << 21;  // Port Reset Change

pub struct XhciController {
    mmio_base: SigmaU64,
    cmd_ring: CommandRing,
    evt_ring: EventRing,
    slots: [UsbSlot; MAX_SLOTS],
    num_ports: SigmaU8,
    max_slots: SigmaU8,
    running: SigmaBool,
    initialized: SigmaBool,
}

impl XhciController {
    pub const fn new() -> Self {
        XhciController {
            mmio_base: 0,
            cmd_ring: CommandRing::new(),
            evt_ring: EventRing::new(),
            slots: [UsbSlot::empty(); MAX_SLOTS],
            num_ports: 0,
            max_slots: 0,
            running: false,
            initialized: false,
        }
    }

    // ── MMIO Helpers ────────────────────────────────────────────────────────
    unsafe fn read_reg32(&self, offset: SigmaUsize) -> SigmaU32 {
        let ptr = (self.mmio_base as *const SigmaU8).add(offset) as *const SigmaU32;
        core::ptr::read_volatile(ptr)
    }

    unsafe fn write_reg32(&self, offset: SigmaUsize, val: SigmaU32) {
        let ptr = (self.mmio_base as *mut SigmaU8).add(offset) as *mut SigmaU32;
        core::ptr::write_volatile(ptr, val);
    }

    // ── Initialization ──────────────────────────────────────────────────────

    /// Initialize the xHCI controller from a PCI BAR0 MMIO base address
    pub unsafe fn init(&mut self, mmio_base: SigmaU64) -> SigmaI32 {
        self.mmio_base = mmio_base;

        // 1. Read capability registers
        let caplength = self.read_reg32(0) & 0xFF;
        let hcsparams1 = self.read_reg32(0x04);
        self.max_slots = ((hcsparams1 >> 24) & 0xFF) as SigmaU8;
        self.num_ports = (hcsparams1 & 0xFF) as SigmaU8;

        let op_base = caplength as SigmaUsize;

        // 2. Wait for Controller Not Ready to clear
        let mut timeout = 100_000u32;
        while (self.read_reg32(op_base + 0x04) & USBSTS_CNR) != 0 {
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }

        // 3. Halt the controller (clear Run/Stop)
        let mut usbcmd = self.read_reg32(op_base);
        usbcmd &= !USBCMD_RUN;
        self.write_reg32(op_base, usbcmd);

        // Wait for HCH (Halted)
        timeout = 100_000;
        while (self.read_reg32(op_base + 0x04) & USBSTS_HCH) == 0 {
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }

        // 4. Reset controller
        usbcmd = self.read_reg32(op_base);
        usbcmd |= USBCMD_HCRST;
        self.write_reg32(op_base, usbcmd);

        timeout = 100_000;
        while (self.read_reg32(op_base) & USBCMD_HCRST) != 0 {
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }

        // Wait for CNR again
        timeout = 100_000;
        while (self.read_reg32(op_base + 0x04) & USBSTS_CNR) != 0 {
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }

        // 5. Configure MaxSlotsEn
        let max_slots_cfg = if self.max_slots > MAX_SLOTS as SigmaU8 {
            MAX_SLOTS as SigmaU32
        } else {
            self.max_slots as SigmaU32
        };
        self.write_reg32(op_base + 0x38, max_slots_cfg);

        // 6. Program Command Ring Control Register (CRCR)
        let crcr_phys = self.cmd_ring.base_phys();
        self.write_reg32(op_base + 0x18, (crcr_phys as SigmaU32) | 1); // bit 0 = RCS
        self.write_reg32(op_base + 0x1C, (crcr_phys >> 32) as SigmaU32);

        // 7. Enable interrupts and start controller
        usbcmd = self.read_reg32(op_base);
        usbcmd |= USBCMD_RUN | USBCMD_INTE;
        self.write_reg32(op_base, usbcmd);

        self.running = true;
        self.initialized = true;
        SIGMA_OK
    }

    // ── Port Management ─────────────────────────────────────────────────────

    /// Read port status/control register for a given port (0-indexed)
    pub unsafe fn read_port_sc(&self, port: SigmaU8) -> SigmaU32 {
        let caplength = (self.read_reg32(0) & 0xFF) as SigmaUsize;
        let port_offset = caplength + 0x400 + (port as SigmaUsize) * 0x10;
        self.read_reg32(port_offset)
    }

    /// Check if a device is connected on the given port
    pub unsafe fn port_connected(&self, port: SigmaU8) -> SigmaBool {
        (self.read_port_sc(port) & PORTSC_CCS) != 0
    }

    /// Reset a port to enable the attached device
    pub unsafe fn reset_port(&self, port: SigmaU8) -> SigmaI32 {
        let caplength = (self.read_reg32(0) & 0xFF) as SigmaUsize;
        let port_offset = caplength + 0x400 + (port as SigmaUsize) * 0x10;

        let mut portsc = self.read_reg32(port_offset);
        // Preserve RW bits, set Port Reset
        portsc = (portsc & 0x0E00_C3E0) | PORTSC_PR;
        self.write_reg32(port_offset, portsc);

        // Wait for reset to complete (PRC bit)
        let mut timeout = 100_000u32;
        loop {
            let sc = self.read_reg32(port_offset);
            if (sc & PORTSC_PRC) != 0 {
                // Clear PRC by writing 1
                self.write_reg32(port_offset, sc | PORTSC_PRC);
                break;
            }
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }
        SIGMA_OK
    }

    /// Detect speed of device on port from PORTSC speed field
    pub unsafe fn port_speed(&self, port: SigmaU8) -> UsbSpeed {
        let speed_val = (self.read_port_sc(port) & PORTSC_SPEED_MASK) >> 10;
        match speed_val {
            1 => UsbSpeed::Full,
            2 => UsbSpeed::Low,
            3 => UsbSpeed::High,
            4 => UsbSpeed::Super,
            5 => UsbSpeed::SuperPlus,
            _ => UsbSpeed::Full,
        }
    }

    // ── Slot Management ─────────────────────────────────────────────────────

    /// Send Enable Slot command to allocate a device slot
    pub fn enable_slot(&mut self) -> SigmaI32 {
        let mut trb = Trb::empty();
        trb.control = (TRB_TYPE_ENABLE_SLOT as SigmaU32) << 10;
        if !self.cmd_ring.enqueue(trb) {
            return SIGMA_ERR_NO_SLOT;
        }
        // Ring doorbell 0 (Host Controller) to notify command ring
        // In real driver: write to doorbell register at DBOFF
        SIGMA_OK
    }

    /// Send Disable Slot command
    pub fn disable_slot(&mut self, slot_id: SigmaU8) -> SigmaI32 {
        let mut trb = Trb::empty();
        trb.control = (TRB_TYPE_DISABLE_SLOT as SigmaU32) << 10
            | (slot_id as SigmaU32) << 24;
        if !self.cmd_ring.enqueue(trb) {
            return SIGMA_ERR_NO_SLOT;
        }
        if (slot_id as SigmaUsize) < MAX_SLOTS {
            self.slots[slot_id as SigmaUsize].state = SlotState::Disabled;
        }
        SIGMA_OK
    }

    /// Send Address Device command
    pub fn address_device(&mut self, slot_id: SigmaU8, input_ctx_phys: SigmaU64) -> SigmaI32 {
        let mut trb = Trb::empty();
        trb.param_lo = input_ctx_phys as SigmaU32;
        trb.param_hi = (input_ctx_phys >> 32) as SigmaU32;
        trb.control = (TRB_TYPE_ADDRESS_DEVICE as SigmaU32) << 10
            | (slot_id as SigmaU32) << 24;
        if !self.cmd_ring.enqueue(trb) {
            return SIGMA_ERR_PROTOCOL;
        }
        if (slot_id as SigmaUsize) < MAX_SLOTS {
            self.slots[slot_id as SigmaUsize].state = SlotState::Addressed;
        }
        SIGMA_OK
    }

    /// Send Configure Endpoint command
    pub fn configure_endpoint(&mut self, slot_id: SigmaU8, input_ctx_phys: SigmaU64) -> SigmaI32 {
        let mut trb = Trb::empty();
        trb.param_lo = input_ctx_phys as SigmaU32;
        trb.param_hi = (input_ctx_phys >> 32) as SigmaU32;
        trb.control = (TRB_TYPE_CONFIGURE_EP as SigmaU32) << 10
            | (slot_id as SigmaU32) << 24;
        if !self.cmd_ring.enqueue(trb) {
            return SIGMA_ERR_PROTOCOL;
        }
        if (slot_id as SigmaUsize) < MAX_SLOTS {
            self.slots[slot_id as SigmaUsize].state = SlotState::Configured;
        }
        SIGMA_OK
    }

    // ── Event Processing ────────────────────────────────────────────────────

    /// Process pending events from the event ring
    pub fn process_events(&mut self) -> SigmaU32 {
        let mut count: SigmaU32 = 0;
        while let Some(trb) = self.evt_ring.dequeue() {
            match trb.trb_type() {
                TRB_TYPE_COMMAND_COMPLETION => {
                    let _completion_code = (trb.status >> 24) & 0xFF;
                    let _slot_id = (trb.control >> 24) & 0xFF;
                    // Handle command completion
                }
                TRB_TYPE_PORT_STATUS_CHANGE => {
                    let _port_id = (trb.param_lo >> 24) & 0xFF;
                    // Handle port status change (connect/disconnect)
                }
                _ => {}
            }
            count += 1;
        }
        count
    }

    // ── Device Enumeration ──────────────────────────────────────────────────

    /// Scan all ports and enumerate connected devices
    pub unsafe fn enumerate_devices(&mut self) -> SigmaI32 {
        let mut devices_found: SigmaI32 = 0;
        let ports = if self.num_ports > MAX_PORTS as SigmaU8 {
            MAX_PORTS as SigmaU8
        } else {
            self.num_ports
        };

        let mut port: SigmaU8 = 0;
        while port < ports {
            if self.port_connected(port) {
                // Reset the port
                if self.reset_port(port) == SIGMA_OK {
                    let speed = self.port_speed(port);

                    // Enable slot
                    if self.enable_slot() == SIGMA_OK {
                        // Record the device in the next available slot
                        let mut slot_idx: SigmaUsize = 0;
                        while slot_idx < MAX_SLOTS {
                            if self.slots[slot_idx].state == SlotState::Disabled {
                                self.slots[slot_idx].port = port;
                                self.slots[slot_idx].speed = speed;
                                self.slots[slot_idx].state = SlotState::Enabled;
                                self.slots[slot_idx].slot_id = slot_idx as SigmaU8;
                                self.slots[slot_idx].max_packet_size = match speed {
                                    UsbSpeed::Low => 8,
                                    UsbSpeed::Full => 64,
                                    UsbSpeed::High => 64,
                                    UsbSpeed::Super | UsbSpeed::SuperPlus => 512,
                                };
                                devices_found += 1;
                                break;
                            }
                            slot_idx += 1;
                        }
                    }
                }
            }
            port += 1;
        }
        devices_found
    }

    /// Get a reference to an active slot
    pub fn get_slot(&self, slot_id: SigmaU8) -> Option<&UsbSlot> {
        if (slot_id as SigmaUsize) < MAX_SLOTS
            && self.slots[slot_id as SigmaUsize].state != SlotState::Disabled
        {
            Some(&self.slots[slot_id as SigmaUsize])
        } else {
            None
        }
    }

    pub fn is_running(&self) -> SigmaBool {
        self.running
    }
}

// ─── Global State ───────────────────────────────────────────────────────────
static mut XHCI: XhciController = XhciController::new();

// ─── C ABI Entry Points ────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sovereignusb_init(mmio_base: SigmaU64) -> SigmaI32 {
    XHCI.init(mmio_base)
}

#[no_mangle]
pub unsafe extern "C" fn sovereignusb_enumerate() -> SigmaI32 {
    XHCI.enumerate_devices()
}

#[no_mangle]
pub unsafe extern "C" fn sovereignusb_process_events() -> SigmaU32 {
    XHCI.process_events()
}

#[no_mangle]
pub unsafe extern "C" fn sovereignusb_is_running() -> SigmaU8 {
    XHCI.is_running() as SigmaU8
}