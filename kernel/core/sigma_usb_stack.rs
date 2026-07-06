// SigmaOS — USB Stack (xHCI + HID + Mass Storage)
// Sovereign implementation — no external dependencies
#![no_std]
#![allow(dead_code)]

// ─── xHCI Register Map ───────────────────────────────────────────────────────
pub const XHCI_CAPLENGTH:  u32 = 0x00;
pub const XHCI_HCSPARAMS1: u32 = 0x04;
pub const XHCI_HCSPARAMS2: u32 = 0x08;
pub const XHCI_HCCPARAMS1: u32 = 0x10;
pub const XHCI_DBOFF:      u32 = 0x14;
pub const XHCI_RTSOFF:     u32 = 0x18;

// Operational registers (base + caplength)
pub const XHCI_USBCMD:  u32 = 0x00;
pub const XHCI_USBSTS:  u32 = 0x04;
pub const XHCI_PAGESIZE: u32 = 0x08;
pub const XHCI_DNCTRL:  u32 = 0x14;
pub const XHCI_CRCR:    u32 = 0x18;
pub const XHCI_DCBAAP:  u32 = 0x30;
pub const XHCI_CONFIG:  u32 = 0x38;

pub const XHCI_CMD_RUN:    u32 = 1 << 0;
pub const XHCI_CMD_RESET:  u32 = 1 << 1;
pub const XHCI_CMD_INTE:   u32 = 1 << 2;
pub const XHCI_STS_HCH:    u32 = 1 << 0;
pub const XHCI_STS_CNR:    u32 = 1 << 11;

// ─── USB Speed ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UsbSpeed { FullSpeed, LowSpeed, HighSpeed, SuperSpeed, SuperSpeedPlus }

impl UsbSpeed {
    pub fn max_packet_size(&self) -> u16 {
        match self {
            UsbSpeed::LowSpeed       =>  8,
            UsbSpeed::FullSpeed      => 64,
            UsbSpeed::HighSpeed      => 512,
            UsbSpeed::SuperSpeed     => 1024,
            UsbSpeed::SuperSpeedPlus => 1024,
        }
    }
    pub fn bandwidth_mbps(&self) -> u32 {
        match self {
            UsbSpeed::LowSpeed       => 1,
            UsbSpeed::FullSpeed      => 12,
            UsbSpeed::HighSpeed      => 480,
            UsbSpeed::SuperSpeed     => 5000,
            UsbSpeed::SuperSpeedPlus => 10000,
        }
    }
}

// ─── USB Descriptor Types ────────────────────────────────────────────────────
pub const USB_DESC_DEVICE:    u8 = 0x01;
pub const USB_DESC_CONFIG:    u8 = 0x02;
pub const USB_DESC_STRING:    u8 = 0x03;
pub const USB_DESC_INTERFACE: u8 = 0x04;
pub const USB_DESC_ENDPOINT:  u8 = 0x05;
pub const USB_DESC_HID:       u8 = 0x21;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UsbDeviceDescriptor {
    pub bLength:            u8,
    pub bDescriptorType:    u8,
    pub bcdUSB:             u16,
    pub bDeviceClass:       u8,
    pub bDeviceSubClass:    u8,
    pub bDeviceProtocol:    u8,
    pub bMaxPacketSize:     u8,
    pub idVendor:           u16,
    pub idProduct:          u16,
    pub bcdDevice:          u16,
    pub iManufacturer:      u8,
    pub iProduct:           u8,
    pub iSerialNumber:      u8,
    pub bNumConfigurations: u8,
}

// ─── USB Device Classes ──────────────────────────────────────────────────────
pub const USB_CLASS_HID:          u8 = 0x03;
pub const USB_CLASS_MASS_STORAGE: u8 = 0x08;
pub const USB_CLASS_HUB:          u8 = 0x09;
pub const USB_CLASS_CDC:          u8 = 0x0A;

// ─── USB Endpoint ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum EndpointType { Control, Isochronous, Bulk, Interrupt }

#[derive(Clone, Copy)]
pub struct UsbEndpoint {
    pub addr:      u8,     // bit 7 = direction (1=in), bits 3:0 = ep number
    pub ep_type:   EndpointType,
    pub max_pkt:   u16,
    pub interval:  u8,
}

impl UsbEndpoint {
    pub fn is_in(&self) -> bool  { self.addr & 0x80 != 0 }
    pub fn ep_num(&self) -> u8   { self.addr & 0x0F }
}

// ─── USB Device ──────────────────────────────────────────────────────────────
pub const MAX_USB_DEVS: usize = 128;
pub const MAX_USB_EPS:  usize = 16;

#[derive(Clone, Copy)]
pub struct UsbDevice {
    pub slot:       u8,
    pub addr:       u8,
    pub speed:      UsbSpeed,
    pub vid:        u16,
    pub pid:        u16,
    pub class:      u8,
    pub subclass:   u8,
    pub protocol:   u8,
    pub endpoints:  [UsbEndpoint; MAX_USB_EPS],
    pub n_eps:      u8,
    pub connected:  bool,
}

impl UsbDevice {
    pub const fn empty() -> Self {
        const EP: UsbEndpoint = UsbEndpoint {
            addr: 0, ep_type: EndpointType::Control, max_pkt: 0, interval: 0
        };
        UsbDevice {
            slot: 0, addr: 0, speed: UsbSpeed::HighSpeed,
            vid: 0, pid: 0, class: 0, subclass: 0, protocol: 0,
            endpoints: [EP; MAX_USB_EPS], n_eps: 0, connected: false,
        }
    }
    pub fn is_hid(&self)          -> bool { self.class == USB_CLASS_HID }
    pub fn is_mass_storage(&self) -> bool { self.class == USB_CLASS_MASS_STORAGE }
    pub fn is_keyboard(&self)     -> bool { self.class == USB_CLASS_HID && self.protocol == 1 }
    pub fn is_mouse(&self)        -> bool { self.class == USB_CLASS_HID && self.protocol == 2 }
}

// ─── xHCI Controller ─────────────────────────────────────────────────────────
pub struct XhciController {
    pub mmio_base:    u64,
    pub cap_len:      u8,
    pub n_slots:      u8,
    pub n_ports:      u8,
    pub devices:      [UsbDevice; MAX_USB_DEVS],
    pub dev_count:    usize,
    pub initialized:  bool,
}

#[derive(Debug, Clone, Copy)]
pub enum UsbError {
    NotInitialized, DeviceFull, InvalidSlot,
    TransferError, Timeout, Stall, BabbleDetected,
}

impl XhciController {
    pub const fn new(mmio_base: u64) -> Self {
        const EMPTY_DEV: UsbDevice = UsbDevice::empty();
        XhciController {
            mmio_base, cap_len: 0x40, n_slots: 0, n_ports: 0,
            devices: [EMPTY_DEV; MAX_USB_DEVS],
            dev_count: 0, initialized: false,
        }
    }

    fn mmio_read32(&self, offset: u32) -> u32 {
        unsafe { ((self.mmio_base + offset as u64) as *const u32).read_volatile() }
    }
    fn mmio_write32(&self, offset: u32, val: u32) {
        unsafe { ((self.mmio_base + offset as u64) as *mut u32).write_volatile(val); }
    }
    fn op_base(&self) -> u64 { self.mmio_base + self.cap_len as u64 }
    fn op_read32(&self, offset: u32) -> u32 {
        unsafe { ((self.op_base() + offset as u64) as *const u32).read_volatile() }
    }
    fn op_write32(&self, offset: u32, val: u32) {
        unsafe { ((self.op_base() + offset as u64) as *mut u32).write_volatile(val); }
    }

    pub fn init(&mut self) -> Result<(), UsbError> {
        // Read capability length
        let cap = self.mmio_read32(XHCI_CAPLENGTH);
        self.cap_len = (cap & 0xFF) as u8;
        // Read HCSPARAMS1 for number of slots and ports
        let params1 = self.mmio_read32(XHCI_HCSPARAMS1);
        self.n_slots = (params1 & 0xFF) as u8;
        self.n_ports = ((params1 >> 24) & 0xFF) as u8;
        // Stop controller
        let cmd = self.op_read32(XHCI_USBCMD);
        self.op_write32(XHCI_USBCMD, cmd & !XHCI_CMD_RUN);
        // Wait until halted
        for _ in 0..100000 {
            if self.op_read32(XHCI_USBSTS) & XHCI_STS_HCH != 0 { break; }
            core::hint::spin_loop();
        }
        // Reset
        self.op_write32(XHCI_USBCMD, XHCI_CMD_RESET);
        for _ in 0..100000 {
            if self.op_read32(XHCI_USBCMD) & XHCI_CMD_RESET == 0 { break; }
            core::hint::spin_loop();
        }
        // Enable interrupts + run
        self.op_write32(XHCI_USBCMD, XHCI_CMD_RUN | XHCI_CMD_INTE);
        self.initialized = true;
        Ok(())
    }

    pub fn probe_port(&mut self, port: u8) -> Option<usize> {
        if !self.initialized || self.dev_count >= MAX_USB_DEVS { return None; }
        // Check port status register (PORTSC at op_base + 0x400 + port*0x10)
        let portsc_off = 0x400u64 + port as u64 * 0x10;
        let portsc = unsafe {
            ((self.op_base() + portsc_off) as *const u32).read_volatile()
        };
        let connected = portsc & 0x1 != 0;
        if !connected { return None; }
        // Decode speed from bits [13:10]
        let speed_bits = (portsc >> 10) & 0xF;
        let speed = match speed_bits {
            1 => UsbSpeed::FullSpeed,
            2 => UsbSpeed::LowSpeed,
            3 => UsbSpeed::HighSpeed,
            4 => UsbSpeed::SuperSpeed,
            5 => UsbSpeed::SuperSpeedPlus,
            _ => UsbSpeed::HighSpeed,
        };
        let idx = self.dev_count;
        self.devices[idx].slot     = port + 1;
        self.devices[idx].addr     = port + 1;
        self.devices[idx].speed    = speed;
        self.devices[idx].connected = true;
        self.dev_count += 1;
        Some(idx)
    }

    pub fn enumerate_all(&mut self) {
        for p in 0..self.n_ports { self.probe_port(p); }
    }

    pub fn find_by_class(&self, class: u8) -> Option<&UsbDevice> {
        self.devices[..self.dev_count].iter().find(|d| d.connected && d.class == class)
    }

    pub fn keyboard_connected(&self) -> bool {
        self.devices[..self.dev_count].iter().any(|d| d.connected && d.is_keyboard())
    }

    pub fn mouse_connected(&self) -> bool {
        self.devices[..self.dev_count].iter().any(|d| d.connected && d.is_mouse())
    }

    pub fn mass_storage_count(&self) -> usize {
        self.devices[..self.dev_count].iter().filter(|d| d.connected && d.is_mass_storage()).count()
    }
}

// ─── USB HID Keyboard Decoder ────────────────────────────────────────────────
pub const HID_MOD_LCTRL:  u8 = 0x01;
pub const HID_MOD_LSHIFT: u8 = 0x02;
pub const HID_MOD_LALT:   u8 = 0x04;
pub const HID_MOD_LGUI:   u8 = 0x08;
pub const HID_MOD_RCTRL:  u8 = 0x10;
pub const HID_MOD_RSHIFT: u8 = 0x20;
pub const HID_MOD_RALT:   u8 = 0x40;
pub const HID_MOD_RGUI:   u8 = 0x80;

#[derive(Clone, Copy)]
pub struct HidKeyboardReport {
    pub modifiers: u8,
    pub reserved:  u8,
    pub keycodes:  [u8; 6],
}

impl HidKeyboardReport {
    pub fn ctrl_pressed(&self)  -> bool { self.modifiers & (HID_MOD_LCTRL  | HID_MOD_RCTRL)  != 0 }
    pub fn shift_pressed(&self) -> bool { self.modifiers & (HID_MOD_LSHIFT | HID_MOD_RSHIFT) != 0 }
    pub fn alt_pressed(&self)   -> bool { self.modifiers & (HID_MOD_LALT   | HID_MOD_RALT)   != 0 }
    pub fn gui_pressed(&self)   -> bool { self.modifiers & (HID_MOD_LGUI   | HID_MOD_RGUI)   != 0 }
    pub fn key_pressed(&self, keycode: u8) -> bool { self.keycodes.contains(&keycode) }
}

/// HID keycode to ASCII (US layout, no shift).
pub fn hid_keycode_to_ascii(code: u8, shift: bool) -> Option<u8> {
    const LOWER: &[u8] = b"??abcdefghijklmnopqrstuvwxyz1234567890\n\x1b\x08\t -=[]\\#;'`,./";
    const UPPER: &[u8] = b"??ABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()\n\x1b\x08\t _+{}|~:\"~<>?";
    let table = if shift { UPPER } else { LOWER };
    let idx = code as usize;
    if idx >= 4 && idx - 4 < table.len() { Some(table[idx - 4]) } else { None }
}

// ─── USB Mass Storage (BBB — Bulk-only transfer) ──────────────────────────────
pub const SCSI_READ10:   u8 = 0x28;
pub const SCSI_WRITE10:  u8 = 0x2A;
pub const SCSI_INQUIRY:  u8 = 0x12;
pub const SCSI_TEST_READY: u8 = 0x00;
pub const SCSI_READ_CAPACITY: u8 = 0x25;

#[repr(C, packed)]
pub struct CbwBlock {
    pub signature:   u32,  // 0x43425355
    pub tag:         u32,
    pub data_length: u32,
    pub flags:       u8,   // 0x80 = data-in, 0x00 = data-out
    pub lun:         u8,
    pub cb_length:   u8,
    pub cb:          [u8; 16],
}

impl CbwBlock {
    pub fn read10(tag: u32, lba: u32, blocks: u16) -> Self {
        let mut cb = [0u8; 16];
        cb[0] = SCSI_READ10;
        cb[2] = (lba >> 24) as u8;
        cb[3] = (lba >> 16) as u8;
        cb[4] = (lba >>  8) as u8;
        cb[5] =  lba        as u8;
        cb[7] = (blocks >> 8) as u8;
        cb[8] =  blocks       as u8;
        CbwBlock {
            signature: 0x43425355, tag,
            data_length: blocks as u32 * 512,
            flags: 0x80, lun: 0, cb_length: 10, cb,
        }
    }
    pub fn write10(tag: u32, lba: u32, blocks: u16) -> Self {
        let mut b = Self::read10(tag, lba, blocks);
        b.flags = 0x00;
        b.cb[0] = SCSI_WRITE10;
        b
    }
}
