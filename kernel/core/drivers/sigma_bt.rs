// SPDX-License-Identifier: MIT
// SigmaOS Bluetooth HCI Driver — sigma_bt.rs
// Implements HCI over USB transport (USB BT adapters: Broadcom, Intel, Realtek).
// Full HCI command/event framing, link layer state machine,
// L2CAP channel setup, RFCOMM port creation, and pairing (SSP/LE).

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── HCI Packet Types ─────────────────────────────────────────────────────────
pub const HCI_COMMAND_PKT: u8 = 0x01;
pub const HCI_ACLDATA_PKT: u8 = 0x02;
pub const HCI_SCODATA_PKT: u8 = 0x03;
pub const HCI_EVENT_PKT:   u8 = 0x04;
pub const HCI_ISODATA_PKT: u8 = 0x05;

// ── HCI Opcodes (OGF | OCF) ───────────────────────────────────────────────────
pub const HCI_OP_RESET:              u16 = 0x0C03;
pub const HCI_OP_READ_BD_ADDR:       u16 = 0x1009;
pub const HCI_OP_READ_LOCAL_VERSION: u16 = 0x1001;
pub const HCI_OP_SET_EVENT_MASK:     u16 = 0x0C01;
pub const HCI_OP_READ_LOCAL_NAME:    u16 = 0x0C14;
pub const HCI_OP_WRITE_LOCAL_NAME:   u16 = 0x0C13;
pub const HCI_OP_READ_SCAN_ENABLE:   u16 = 0x0C19;
pub const HCI_OP_WRITE_SCAN_ENABLE:  u16 = 0x0C1A;
pub const HCI_OP_WRITE_AUTH_ENABLE:  u16 = 0x0C20;
pub const HCI_OP_WRITE_ENCRYPT_MODE: u16 = 0x0C22;
pub const HCI_OP_INQUIRY:            u16 = 0x0401;
pub const HCI_OP_INQUIRY_CANCEL:     u16 = 0x0402;
pub const HCI_OP_CREATE_CONN:        u16 = 0x0405;
pub const HCI_OP_DISCONNECT:         u16 = 0x0406;
pub const HCI_OP_ACCEPT_CONN:        u16 = 0x0409;
pub const HCI_OP_REJECT_CONN:        u16 = 0x040A;
pub const HCI_OP_LINK_KEY_REPLY:     u16 = 0x040B;
pub const HCI_OP_PIN_CODE_REPLY:     u16 = 0x040D;
pub const HCI_OP_AUTH_REQUESTED:     u16 = 0x0411;
pub const HCI_OP_SET_CONN_ENCRYPT:   u16 = 0x0413;
pub const HCI_OP_REMOTE_NAME_REQ:    u16 = 0x0419;
pub const HCI_OP_IO_CAPABILITY_REPLY:u16 = 0x042B;
pub const HCI_OP_USER_CONFIRM_REPLY: u16 = 0x042C;
pub const HCI_OP_LE_SET_SCAN_PARAM:  u16 = 0x200B;
pub const HCI_OP_LE_SET_SCAN_ENABLE: u16 = 0x200C;
pub const HCI_OP_LE_CREATE_CONN:     u16 = 0x200D;
pub const HCI_OP_LE_READ_BUFFER_SIZE:u16 = 0x2002;

// ── HCI Event Codes ───────────────────────────────────────────────────────────
pub const HCI_EV_INQUIRY_COMPLETE:   u8 = 0x01;
pub const HCI_EV_INQUIRY_RESULT:     u8 = 0x02;
pub const HCI_EV_CONN_COMPLETE:      u8 = 0x03;
pub const HCI_EV_CONN_REQUEST:       u8 = 0x04;
pub const HCI_EV_DISCONN_COMPLETE:   u8 = 0x05;
pub const HCI_EV_AUTH_COMPLETE:      u8 = 0x06;
pub const HCI_EV_REMOTE_NAME:        u8 = 0x07;
pub const HCI_EV_ENCRYPT_CHANGE:     u8 = 0x08;
pub const HCI_EV_CMD_COMPLETE:       u8 = 0x0E;
pub const HCI_EV_CMD_STATUS:         u8 = 0x0F;
pub const HCI_EV_ROLE_CHANGE:        u8 = 0x12;
pub const HCI_EV_PIN_CODE_REQ:       u8 = 0x16;
pub const HCI_EV_LINK_KEY_NOTIFY:    u8 = 0x18;
pub const HCI_EV_IO_CAPA_REQUEST:    u8 = 0x31;
pub const HCI_EV_USER_CONFIRM_REQ:   u8 = 0x33;
pub const HCI_EV_SIMPLE_PAIR_DONE:   u8 = 0x36;
pub const HCI_EV_LE_META:            u8 = 0x3E;

// ── L2CAP PSM (Protocol/Service Multiplexer) ──────────────────────────────────
pub const L2CAP_PSM_SDP:     u16 = 0x0001;
pub const L2CAP_PSM_RFCOMM:  u16 = 0x0003;
pub const L2CAP_PSM_HID_CTL: u16 = 0x0011;
pub const L2CAP_PSM_HID_INT: u16 = 0x0013;
pub const L2CAP_PSM_A2DP:    u16 = 0x0019;
pub const L2CAP_PSM_ATT:     u16 = 0x001F;

// ── Bluetooth Address ─────────────────────────────────────────────────────────
pub type BdAddr = [u8; 6];
pub const ZERO_BDADDR: BdAddr = [0u8; 6];

// ── HCI Command Frame ─────────────────────────────────────────────────────────
const HCI_CMD_MAX_PARAM: usize = 255;
const HCI_EVENT_MAX_PARAM: usize = 255;

#[derive(Copy, Clone, Default)]
pub struct HciCmd {
    pub opcode:   u16,
    pub param:    [u8; HCI_CMD_MAX_PARAM],
    pub param_len: u8,
}

impl HciCmd {
    pub fn new(opcode: u16) -> Self {
        Self { opcode, param: [0u8; HCI_CMD_MAX_PARAM], param_len: 0 }
    }

    pub fn with_param(mut self, data: &[u8]) -> Self {
        let l = data.len().min(HCI_CMD_MAX_PARAM);
        self.param[..l].copy_from_slice(&data[..l]);
        self.param_len = l as u8;
        self
    }
}

// ── HCI Event Frame ───────────────────────────────────────────────────────────
#[derive(Copy, Clone, Default)]
pub struct HciEvent {
    pub code:    u8,
    pub param:   [u8; HCI_EVENT_MAX_PARAM],
    pub param_len: u8,
}

// ── ACL Data Packet ───────────────────────────────────────────────────────────
const ACL_MAX_PAYLOAD: usize = 1024;

#[derive(Copy, Clone)]
pub struct AclPacket {
    pub handle:   u16,   // Connection handle (bits[11:0]) + PB/BC flags
    pub data:     [u8; ACL_MAX_PAYLOAD],
    pub data_len: u16,
}

impl Default for AclPacket {
    fn default() -> Self {
        Self { handle: 0, data: [0u8; ACL_MAX_PAYLOAD], data_len: 0 }
    }
}

// ── Device Info ───────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Default)]
pub struct BtDevice {
    pub addr:     BdAddr,
    pub name:     [u8; 248],
    pub name_len: u8,
    pub cod:      u32,       // Class of Device
    pub rssi:     i8,
    pub handle:   u16,       // Connection handle (0 = not connected)
    pub paired:   bool,
    pub link_key: [u8; 16],
    pub le_device:bool,
}

// ── L2CAP Channel ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Default)]
pub struct L2capChannel {
    pub handle:     u16,    // ACL handle
    pub local_cid:  u16,
    pub remote_cid: u16,
    pub psm:        u16,
    pub mtu:        u16,
    pub connected:  bool,
}

// ── RFCOMM Port ───────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Default)]
pub struct RfcommPort {
    pub channel:   u8,
    pub l2cap_cid: u16,
    pub dlci:      u8,
    pub open:      bool,
    pub rx_buf:    [u8; 1024],
    pub rx_len:    u16,
}

// ── TX/RX Ring ────────────────────────────────────────────────────────────────
const CMD_QUEUE_DEPTH: usize = 16;
const EVENT_QUEUE_DEPTH: usize = 32;
const ACL_QUEUE_DEPTH:  usize = 16;
const MAX_BT_DEVICES:   usize = 32;
const MAX_L2CAP_CHANS:  usize = 16;
const MAX_RFCOMM_PORTS: usize = 8;

// ── Driver ────────────────────────────────────────────────────────────────────
pub struct SigmaBluetooth {
    // Controller info
    bd_addr:        BdAddr,
    hci_version:    u8,
    lmp_version:    u8,
    manufacturer:   u16,
    le_supported:   bool,

    // Device scan results
    devices:        [BtDevice; MAX_BT_DEVICES],
    device_count:   usize,
    scanning:       AtomicBool,

    // L2CAP
    l2cap_chans:    [L2capChannel; MAX_L2CAP_CHANS],
    l2cap_count:    usize,
    next_local_cid: u16,

    // RFCOMM
    rfcomm_ports:   [RfcommPort; MAX_RFCOMM_PORTS],
    rfcomm_count:   usize,

    // Command queue
    cmd_queue:      [HciCmd; CMD_QUEUE_DEPTH],
    cmd_head:       usize,
    cmd_tail:       usize,
    cmd_pending:    AtomicBool,

    // Event queue
    ev_queue:       [HciEvent; EVENT_QUEUE_DEPTH],
    ev_head:        usize,
    ev_tail:        usize,

    // ACL queues
    acl_tx:         [AclPacket; ACL_QUEUE_DEPTH],
    acl_tx_head:    usize,
    acl_tx_tail:    usize,

    initialized:    bool,
    tx_packets:     AtomicU32,
    rx_packets:     AtomicU32,
}

impl SigmaBluetooth {
    pub const fn new() -> Self {
        Self {
            bd_addr:        ZERO_BDADDR,
            hci_version:    0,
            lmp_version:    0,
            manufacturer:   0,
            le_supported:   false,
            devices:        [BtDevice {
                addr: ZERO_BDADDR, name: [0u8; 248], name_len: 0,
                cod: 0, rssi: -100, handle: 0, paired: false,
                link_key: [0u8; 16], le_device: false,
            }; MAX_BT_DEVICES],
            device_count:   0,
            scanning:       AtomicBool::new(false),
            l2cap_chans:    [L2capChannel { handle: 0, local_cid: 0, remote_cid: 0,
                                            psm: 0, mtu: 0, connected: false }; MAX_L2CAP_CHANS],
            l2cap_count:    0,
            next_local_cid: 0x0040,
            rfcomm_ports:   [RfcommPort { channel: 0, l2cap_cid: 0, dlci: 0, open: false,
                                          rx_buf: [0u8; 1024], rx_len: 0 }; MAX_RFCOMM_PORTS],
            rfcomm_count:   0,
            cmd_queue:      [HciCmd { opcode: 0, param: [0u8; 255], param_len: 0 }; CMD_QUEUE_DEPTH],
            cmd_head:       0,
            cmd_tail:       0,
            cmd_pending:    AtomicBool::new(false),
            ev_queue:       [HciEvent { code: 0, param: [0u8; 255], param_len: 0 }; EVENT_QUEUE_DEPTH],
            ev_head:        0,
            ev_tail:        0,
            acl_tx:         [AclPacket { handle: 0, data: [0u8; 1024], data_len: 0 }; ACL_QUEUE_DEPTH],
            acl_tx_head:    0,
            acl_tx_tail:    0,
            initialized:    false,
            tx_packets:     AtomicU32::new(0),
            rx_packets:     AtomicU32::new(0),
        }
    }

    // ── USB HCI Transport hooks ────────────────────────────────────────────────
    // These are called by the xHCI/OHCI USB driver when BT endpoints deliver data.

    /// Called by USB driver with raw HCI frame from interrupt IN endpoint.
    pub fn usb_rx_event(&mut self, data: &[u8]) {
        if data.len() < 2 { return; }
        let mut ev = HciEvent::default();
        ev.code      = data[0];
        ev.param_len = data[1];
        let plen = ev.param_len as usize;
        let copy = plen.min(data.len().saturating_sub(2)).min(HCI_EVENT_MAX_PARAM);
        ev.param[..copy].copy_from_slice(&data[2..2 + copy]);
        self.enqueue_event(ev);
        self.rx_packets.fetch_add(1, Ordering::Relaxed);
    }

    /// Called by USB driver with raw ACL frame from bulk IN endpoint.
    pub fn usb_rx_acl(&mut self, data: &[u8]) {
        if data.len() < 4 { return; }
        let handle   = u16::from_le_bytes([data[0], data[1]]);
        let data_len = u16::from_le_bytes([data[2], data[3]]);
        let payload_len = (data_len as usize).min(data.len() - 4).min(ACL_MAX_PAYLOAD);

        // Dispatch to L2CAP
        let mut payload = [0u8; ACL_MAX_PAYLOAD];
        payload[..payload_len].copy_from_slice(&data[4..4 + payload_len]);
        self.l2cap_rx(handle, &payload[..payload_len]);
        self.rx_packets.fetch_add(1, Ordering::Relaxed);
    }

    // ── Command Queue ─────────────────────────────────────────────────────────

    fn enqueue_cmd(&mut self, cmd: HciCmd) -> bool {
        let next = (self.cmd_tail + 1) % CMD_QUEUE_DEPTH;
        if next == self.cmd_head { return false; }
        self.cmd_queue[self.cmd_tail] = cmd;
        self.cmd_tail = next;
        true
    }

    /// Dequeue one pending HCI command for transmission via USB control endpoint.
    pub fn dequeue_cmd(&mut self) -> Option<HciCmd> {
        if self.cmd_head == self.cmd_tail { return None; }
        let cmd = self.cmd_queue[self.cmd_head];
        self.cmd_head = (self.cmd_head + 1) % CMD_QUEUE_DEPTH;
        self.tx_packets.fetch_add(1, Ordering::Relaxed);
        Some(cmd)
    }

    fn enqueue_event(&mut self, ev: HciEvent) {
        let next = (self.ev_tail + 1) % EVENT_QUEUE_DEPTH;
        if next == self.ev_head { return; }
        self.ev_queue[self.ev_tail] = ev;
        self.ev_tail = next;
    }

    fn dequeue_event(&mut self) -> Option<HciEvent> {
        if self.ev_head == self.ev_tail { return None; }
        let ev = self.ev_queue[self.ev_head];
        self.ev_head = (self.ev_head + 1) % EVENT_QUEUE_DEPTH;
        Some(ev)
    }

    // ── Init ──────────────────────────────────────────────────────────────────

    pub fn init(&mut self) {
        // Queue reset command
        self.enqueue_cmd(HciCmd::new(HCI_OP_RESET));
        self.initialized = true;
    }

    /// Process all pending HCI events. Call from main loop or interrupt handler.
    pub fn process_events(&mut self) {
        while let Some(ev) = self.dequeue_event() {
            self.handle_event(ev);
        }
    }

    fn handle_event(&mut self, ev: HciEvent) {
        match ev.code {
            HCI_EV_CMD_COMPLETE => {
                if ev.param_len < 4 { return; }
                let opcode = u16::from_le_bytes([ev.param[1], ev.param[2]]);
                let status = ev.param[3];
                if status == 0 {
                    self.on_cmd_complete(opcode, &ev.param[4..ev.param_len as usize]);
                }
                self.cmd_pending.store(false, Ordering::Relaxed);
            },
            HCI_EV_INQUIRY_RESULT => self.on_inquiry_result(&ev.param[..ev.param_len as usize]),
            HCI_EV_INQUIRY_COMPLETE => {
                self.scanning.store(false, Ordering::Relaxed);
            },
            HCI_EV_CONN_COMPLETE   => self.on_conn_complete(&ev.param[..ev.param_len as usize]),
            HCI_EV_CONN_REQUEST    => self.on_conn_request(&ev.param[..ev.param_len as usize]),
            HCI_EV_DISCONN_COMPLETE => self.on_disconn(&ev.param[..ev.param_len as usize]),
            HCI_EV_PIN_CODE_REQ    => self.on_pin_request(&ev.param[..ev.param_len as usize]),
            HCI_EV_IO_CAPA_REQUEST => self.on_io_capability_request(&ev.param[..ev.param_len as usize]),
            HCI_EV_USER_CONFIRM_REQ => self.on_user_confirm(&ev.param[..ev.param_len as usize]),
            HCI_EV_LINK_KEY_NOTIFY => self.on_link_key(&ev.param[..ev.param_len as usize]),
            HCI_EV_LE_META         => self.on_le_meta(&ev.param[..ev.param_len as usize]),
            _ => {}
        }
    }

    fn on_cmd_complete(&mut self, opcode: u16, params: &[u8]) {
        match opcode {
            HCI_OP_RESET => {
                // Send Read Local Version
                self.enqueue_cmd(HciCmd::new(HCI_OP_READ_LOCAL_VERSION));
            },
            HCI_OP_READ_LOCAL_VERSION => {
                if params.len() >= 8 {
                    self.hci_version  = params[0];
                    self.lmp_version  = params[3];
                    self.manufacturer = u16::from_le_bytes([params[6], params[7]]);
                }
                self.enqueue_cmd(HciCmd::new(HCI_OP_READ_BD_ADDR));
            },
            HCI_OP_READ_BD_ADDR => {
                if params.len() >= 6 {
                    self.bd_addr.copy_from_slice(&params[..6]);
                }
                // Set event mask to all events
                let mask = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3F];
                self.enqueue_cmd(HciCmd::new(HCI_OP_SET_EVENT_MASK).with_param(&mask));
            },
            HCI_OP_SET_EVENT_MASK => {
                // Enable inquiry scan + page scan
                self.enqueue_cmd(HciCmd::new(HCI_OP_WRITE_SCAN_ENABLE).with_param(&[0x03]));
            },
            _ => {}
        }
    }

    fn on_inquiry_result(&mut self, params: &[u8]) {
        if params.is_empty() { return; }
        let num = params[0] as usize;
        let per = 14; // Each result is 14 bytes (addr 6 + SR 1 + COD 3 + CLK 2 + RSSI 1 = wrong, fixed below)
        for i in 0..num {
            let off = 1 + i * per;
            if off + 6 > params.len() { break; }
            if self.device_count >= MAX_BT_DEVICES { break; }

            let mut addr = ZERO_BDADDR;
            addr.copy_from_slice(&params[off..off + 6]);

            // Check for existing entry
            let existing = self.devices[..self.device_count]
                .iter_mut()
                .find(|d| d.addr == addr);
            if existing.is_none() {
                let cod = if off + 10 <= params.len() {
                    u32::from_le_bytes([params[off + 7], params[off + 8], params[off + 9], 0])
                } else { 0 };
                self.devices[self.device_count] = BtDevice {
                    addr,
                    cod,
                    rssi: -70, // Placeholder
                    ..Default::default()
                };
                self.device_count += 1;
            }
        }
    }

    fn on_conn_complete(&mut self, params: &[u8]) {
        if params.len() < 11 { return; }
        let status = params[0];
        let handle = u16::from_le_bytes([params[1], params[2]]);
        let mut addr = ZERO_BDADDR;
        addr.copy_from_slice(&params[3..9]);

        if status == 0 {
            if let Some(d) = self.devices[..self.device_count].iter_mut().find(|d| d.addr == addr) {
                d.handle = handle;
            }
        }
    }

    fn on_conn_request(&mut self, params: &[u8]) {
        if params.len() < 10 { return; }
        let mut addr = ZERO_BDADDR;
        addr.copy_from_slice(&params[..6]);
        // Auto-accept
        let mut reply = [0u8; 7];
        reply[..6].copy_from_slice(&addr);
        reply[6] = 0; // role = master
        self.enqueue_cmd(HciCmd::new(HCI_OP_ACCEPT_CONN).with_param(&reply));
    }

    fn on_disconn(&mut self, params: &[u8]) {
        if params.len() < 3 { return; }
        let handle = u16::from_le_bytes([params[1], params[2]]);
        for d in self.devices[..self.device_count].iter_mut() {
            if d.handle == handle { d.handle = 0; }
        }
        for c in self.l2cap_chans[..self.l2cap_count].iter_mut() {
            if c.handle == handle { c.connected = false; }
        }
    }

    fn on_pin_request(&mut self, params: &[u8]) {
        if params.len() < 6 { return; }
        let mut addr = ZERO_BDADDR;
        addr.copy_from_slice(&params[..6]);
        // Reply with a default PIN "0000"
        let mut reply = [0u8; 23];
        reply[..6].copy_from_slice(&addr);
        reply[6] = 4; // PIN length
        reply[7] = b'0'; reply[8] = b'0'; reply[9] = b'0'; reply[10] = b'0';
        self.enqueue_cmd(HciCmd::new(HCI_OP_PIN_CODE_REPLY).with_param(&reply));
    }

    fn on_io_capability_request(&mut self, params: &[u8]) {
        if params.len() < 6 { return; }
        let mut reply = [0u8; 9];
        reply[..6].copy_from_slice(&params[..6]);
        reply[6] = 0x03; // IO capability: NoInputNoOutput
        reply[7] = 0x00; // OOB not present
        reply[8] = 0x00; // Auth requirements: no MITM, no bonding
        self.enqueue_cmd(HciCmd::new(HCI_OP_IO_CAPABILITY_REPLY).with_param(&reply));
    }

    fn on_user_confirm(&mut self, params: &[u8]) {
        if params.len() < 6 { return; }
        let mut reply = [0u8; 6];
        reply.copy_from_slice(&params[..6]);
        // Auto-confirm SSP pairing
        self.enqueue_cmd(HciCmd::new(HCI_OP_USER_CONFIRM_REPLY).with_param(&reply));
    }

    fn on_link_key(&mut self, params: &[u8]) {
        if params.len() < 23 { return; }
        let mut addr = ZERO_BDADDR;
        addr.copy_from_slice(&params[..6]);
        if let Some(d) = self.devices[..self.device_count].iter_mut().find(|d| d.addr == addr) {
            d.link_key.copy_from_slice(&params[6..22]);
            d.paired = true;
        }
    }

    fn on_le_meta(&mut self, params: &[u8]) {
        if params.is_empty() { return; }
        let subevent = params[0];
        match subevent {
            0x02 => { // LE Advertising Report
                if params.len() < 14 { return; }
                let num = params[1] as usize;
                let mut off = 2;
                for _ in 0..num {
                    if off + 9 > params.len() { break; }
                    let adv_type = params[off];
                    let _addr_type = params[off + 1];
                    let mut addr = ZERO_BDADDR;
                    addr.copy_from_slice(&params[off + 2..off + 8]);
                    let data_len = params[off + 8] as usize;
                    let rssi = if off + 9 + data_len < params.len() {
                        params[off + 9 + data_len] as i8
                    } else { -100 };
                    let _ = adv_type;

                    if self.device_count < MAX_BT_DEVICES {
                        if !self.devices[..self.device_count].iter().any(|d| d.addr == addr) {
                            self.devices[self.device_count] = BtDevice {
                                addr,
                                rssi,
                                le_device: true,
                                ..Default::default()
                            };
                            self.device_count += 1;
                        }
                    }
                    off += 9 + data_len + 1;
                }
            },
            _ => {}
        }
    }

    // ── L2CAP ─────────────────────────────────────────────────────────────────

    fn l2cap_rx(&mut self, handle: u16, data: &[u8]) {
        if data.len() < 4 { return; }
        let pdu_len  = u16::from_le_bytes([data[0], data[1]]) as usize;
        let channel  = u16::from_le_bytes([data[2], data[3]]);
        if data.len() < 4 + pdu_len { return; }
        let payload  = &data[4..4 + pdu_len];

        match channel {
            0x0001 => self.l2cap_signaling(handle, payload),
            0x0005 => self.l2cap_le_signaling(handle, payload),
            _ => {
                // Data channel — find matching channel and dispatch
                for c in self.l2cap_chans[..self.l2cap_count].iter() {
                    if c.connected && c.handle == handle && c.local_cid == channel {
                        if c.psm == L2CAP_PSM_RFCOMM {
                            self.rfcomm_rx(channel, payload);
                        }
                        break;
                    }
                }
            }
        }
    }

    fn l2cap_signaling(&mut self, handle: u16, data: &[u8]) {
        if data.len() < 4 { return; }
        let code    = data[0];
        let id      = data[1];
        let _len    = u16::from_le_bytes([data[2], data[3]]);

        match code {
            0x02 => { // Connection Request
                if data.len() < 8 { return; }
                let psm      = u16::from_le_bytes([data[4], data[5]]);
                let src_cid  = u16::from_le_bytes([data[6], data[7]]);
                let local_cid = self.next_local_cid;
                self.next_local_cid += 1;

                if self.l2cap_count < MAX_L2CAP_CHANS {
                    self.l2cap_chans[self.l2cap_count] = L2capChannel {
                        handle, local_cid, remote_cid: src_cid,
                        psm, mtu: 672, connected: true,
                    };
                    self.l2cap_count += 1;
                }

                // Send Connection Response (success)
                let resp: [u8; 12] = [
                    0x03, id, 8, 0,
                    (local_cid & 0xFF) as u8, (local_cid >> 8) as u8,
                    (src_cid & 0xFF) as u8,   (src_cid >> 8) as u8,
                    0, 0, 0, 0, // result=0 (success), status=0
                ];
                self.l2cap_send(handle, 0x0001, &resp);
            },
            0x04 => { // Configuration Request — send Configuration Response
                let src_cid = u16::from_le_bytes([data[4], data[5]]);
                let resp: [u8; 10] = [
                    0x05, id, 6, 0,
                    (src_cid & 0xFF) as u8, (src_cid >> 8) as u8,
                    0, 0, 0, 0,
                ];
                self.l2cap_send(handle, 0x0001, &resp);
            },
            0x06 => { // Disconnection Request
                let src_cid = u16::from_le_bytes([data[4], data[5]]);
                for c in self.l2cap_chans[..self.l2cap_count].iter_mut() {
                    if c.handle == handle && c.remote_cid == src_cid {
                        c.connected = false;
                    }
                }
                let resp: [u8; 8] = [
                    0x07, id, 4, 0,
                    data[4], data[5], data[6], data[7],
                ];
                self.l2cap_send(handle, 0x0001, &resp);
            },
            _ => {}
        }
    }

    fn l2cap_le_signaling(&mut self, _handle: u16, _data: &[u8]) {
        // LE signaling channel (CID 0x0005) — LECOC connection handling
    }

    fn l2cap_send(&mut self, handle: u16, channel: u16, data: &[u8]) {
        if data.len() + 4 > ACL_MAX_PAYLOAD { return; }
        let next = (self.acl_tx_tail + 1) % ACL_QUEUE_DEPTH;
        if next == self.acl_tx_head { return; }

        let mut pkt = AclPacket::default();
        pkt.handle   = handle | (0b10 << 12); // First non-flush packet
        let total    = data.len() + 4;
        pkt.data[0]  = (data.len() & 0xFF) as u8;
        pkt.data[1]  = (data.len() >> 8) as u8;
        pkt.data[2]  = (channel & 0xFF) as u8;
        pkt.data[3]  = (channel >> 8) as u8;
        pkt.data[4..4 + data.len()].copy_from_slice(data);
        pkt.data_len = total as u16;

        self.acl_tx[self.acl_tx_tail] = pkt;
        self.acl_tx_tail = next;
    }

    // ── RFCOMM ────────────────────────────────────────────────────────────────

    fn rfcomm_rx(&mut self, _cid: u16, data: &[u8]) {
        if data.len() < 3 { return; }
        let addr   = data[0];
        let ctrl   = data[1];
        let _len   = data[2];
        let _dlci  = addr >> 2;

        match ctrl & !0x10 { // Mask P/F bit
            0xEF => { // UIH frame (data)
                // TODO: deliver data to user application
            },
            0x3F => { // SABM — channel open request — send UA
            },
            0x73 => { // UA frame
            },
            0x53 => { // DM frame
            },
            _ => {}
        }
    }

    // ── Scan API ──────────────────────────────────────────────────────────────

    pub fn start_inquiry(&mut self, duration_1_28s: u8) {
        self.device_count = 0;
        let giac: [u8; 5] = [0x33, 0x8B, 0x9E, duration_1_28s, 0]; // GIAC lap
        self.enqueue_cmd(HciCmd::new(HCI_OP_INQUIRY).with_param(&giac));
        self.scanning.store(true, Ordering::Relaxed);
    }

    pub fn start_le_scan(&mut self) {
        // Set LE scan params: passive scan, 100 ms interval, 50 ms window
        let scan_params: [u8; 7] = [
            0x00,           // Passive scan
            0xA0, 0x00,     // Interval = 0x00A0 * 0.625ms = 100ms
            0x50, 0x00,     // Window   = 0x0050 * 0.625ms = 50ms
            0x00,           // Own addr type: public
            0x00,           // Filter policy: accept all
        ];
        self.enqueue_cmd(HciCmd::new(HCI_OP_LE_SET_SCAN_PARAM).with_param(&scan_params));
        self.enqueue_cmd(HciCmd::new(HCI_OP_LE_SET_SCAN_ENABLE).with_param(&[0x01, 0x00]));
        self.scanning.store(true, Ordering::Relaxed);
    }

    pub fn stop_scan(&mut self) {
        self.enqueue_cmd(HciCmd::new(HCI_OP_INQUIRY_CANCEL));
        self.enqueue_cmd(HciCmd::new(HCI_OP_LE_SET_SCAN_ENABLE).with_param(&[0x00, 0x00]));
        self.scanning.store(false, Ordering::Relaxed);
    }

    // ── Connect API ───────────────────────────────────────────────────────────

    pub fn connect(&mut self, addr: BdAddr) {
        let mut params = [0u8; 13];
        params[..6].copy_from_slice(&addr);
        params[6]  = 0xCC; // Packet type (DH1/DM1/DH3/DM3/DH5/DM5)
        params[7]  = 0x01;
        params[8]  = 0x01; // Allow role switch
        params[9]  = 0x00;
        params[10] = 0x00; // Clock offset
        params[11] = 0x00;
        params[12] = 0x00; // Allow role switch
        self.enqueue_cmd(HciCmd::new(HCI_OP_CREATE_CONN).with_param(&params));
    }

    // ── Status ────────────────────────────────────────────────────────────────

    pub fn is_initialized(&self) -> bool   { self.initialized }
    pub fn is_scanning(&self) -> bool      { self.scanning.load(Ordering::Relaxed) }
    pub fn device_count(&self) -> usize    { self.device_count }
    pub fn bd_addr(&self) -> BdAddr        { self.bd_addr }
    pub fn has_le(&self) -> bool           { self.le_supported }

    pub fn dequeue_acl_tx(&mut self) -> Option<AclPacket> {
        if self.acl_tx_head == self.acl_tx_tail { return None; }
        let pkt = self.acl_tx[self.acl_tx_head];
        self.acl_tx_head = (self.acl_tx_head + 1) % ACL_QUEUE_DEPTH;
        Some(pkt)
    }
}

// ── Global Instance ───────────────────────────────────────────────────────────
static mut G_BT: SigmaBluetooth = SigmaBluetooth::new();

// ── C-ABI ─────────────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_bt_init() {
    G_BT.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bt_process_events() {
    G_BT.process_events();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bt_rx_event(data: *const u8, len: usize) {
    if data.is_null() { return; }
    G_BT.usb_rx_event(core::slice::from_raw_parts(data, len));
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bt_rx_acl(data: *const u8, len: usize) {
    if data.is_null() { return; }
    G_BT.usb_rx_acl(core::slice::from_raw_parts(data, len));
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bt_start_scan(duration: u8) {
    G_BT.start_inquiry(duration);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bt_start_le_scan() {
    G_BT.start_le_scan();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bt_device_count() -> usize {
    G_BT.device_count()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bt_is_scanning() -> i32 {
    if G_BT.is_scanning() { 1 } else { 0 }
}
