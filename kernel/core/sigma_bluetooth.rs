// SigmaOS — Bluetooth 5.3 Stack (Issue #851-BT)
// HCI/L2CAP/RFCOMM sovereign implementation — no external deps.
#![allow(dead_code)]

// ─── HCI Packet Types ────────────────────────────────────────────────────────
pub const HCI_CMD:  u8 = 0x01;
pub const HCI_ACL:  u8 = 0x02;
pub const HCI_SCO:  u8 = 0x03;
pub const HCI_EVT:  u8 = 0x04;
pub const HCI_ISO:  u8 = 0x05; // BT5 isochronous

// ─── HCI OpCodes ─────────────────────────────────────────────────────────────
pub const HCI_OP_RESET:           u16 = 0x0C03;
pub const HCI_OP_SET_EVENT_MASK:  u16 = 0x0C01;
pub const HCI_OP_LE_SET_ADV_PARAM: u16 = 0x2006;
pub const HCI_OP_LE_SET_ADV_DATA:  u16 = 0x2008;
pub const HCI_OP_LE_SET_ADV_EN:    u16 = 0x200A;
pub const HCI_OP_LE_SCAN_EN:       u16 = 0x200C;
pub const HCI_OP_LE_CONNECT:       u16 = 0x200D;
pub const HCI_OP_DISCONNECT:       u16 = 0x0406;

// ─── HCI Command Header ──────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct HciCmdHdr {
    pub opcode:    u16,
    pub param_len: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct HciEvtHdr {
    pub code:     u8,
    pub param_len: u8,
}

// ─── BD_ADDR ─────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BdAddr(pub [u8; 6]);

impl BdAddr {
    pub const ZERO: BdAddr = BdAddr([0u8; 6]);
}

// ─── L2CAP Channel ───────────────────────────────────────────────────────────
pub const L2CAP_CID_ATT:  u16 = 0x0004; // ATT (GATT)
pub const L2CAP_CID_SMSIG: u16 = 0x0005; // LE signaling
pub const L2CAP_CID_SMPB:  u16 = 0x0006; // Security Manager

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct L2capHdr {
    pub length: u16,
    pub cid:    u16,
}

// ─── Connection State ─────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnState {
    Disconnected,
    Connecting,
    Connected,
    Encrypted,
    Bonded,
}

pub const MAX_BT_CONNS: usize = 16;

#[derive(Clone, Copy)]
pub struct BtConnection {
    pub handle:   u16,
    pub addr:     BdAddr,
    pub state:    ConnState,
    pub role:     bool, // true = central, false = peripheral
    pub interval: u16,  // connection interval (1.25 ms units)
    pub latency:  u16,  // peripheral latency
    pub timeout:  u16,  // supervision timeout (10 ms units)
}

impl BtConnection {
    pub const fn new() -> Self {
        BtConnection {
            handle: 0, addr: BdAddr::ZERO,
            state: ConnState::Disconnected,
            role: true, interval: 24, latency: 0, timeout: 400,
        }
    }
}

// ─── GATT Attribute ──────────────────────────────────────────────────────────
pub const ATT_MAX_VALUE: usize = 512;

#[derive(Clone, Copy)]
pub struct GattAttr {
    pub handle:  u16,
    pub uuid:    u128,
    pub perms:   u8,  // READ | WRITE | NOTIFY
    pub value:   [u8; ATT_MAX_VALUE],
    pub vlen:    usize,
}

pub const GATT_PERM_READ:   u8 = 0x01;
pub const GATT_PERM_WRITE:  u8 = 0x02;
pub const GATT_PERM_NOTIFY: u8 = 0x04;

pub const MAX_GATT_ATTRS: usize = 64;

// ─── BT Stack Controller ─────────────────────────────────────────────────────
pub struct BtStack {
    pub conns:      [BtConnection; MAX_BT_CONNS],
    pub conn_count: usize,
    pub local_addr: BdAddr,
    pub attrs:      [GattAttr; MAX_GATT_ATTRS],
    pub attr_count: usize,
    pub advertising: bool,
    pub scanning:    bool,
    pub version:     u8,  // 0x0C = BT 5.3
}

impl BtStack {
    pub const fn new(local_addr: BdAddr) -> Self {
        const EMPTY_CONN: BtConnection = BtConnection::new();
        const EMPTY_ATTR: GattAttr = GattAttr {
            handle: 0, uuid: 0, perms: 0,
            value: [0u8; ATT_MAX_VALUE], vlen: 0,
        };
        BtStack {
            conns: [EMPTY_CONN; MAX_BT_CONNS],
            conn_count: 0,
            local_addr,
            attrs: [EMPTY_ATTR; MAX_GATT_ATTRS],
            attr_count: 0,
            advertising: false,
            scanning: false,
            version: 0x0C,
        }
    }

    /// Start LE advertising.
    pub fn start_advertising(&mut self, adv_data: &[u8; 31]) -> bool {
        // Would write to HCI transport: LE_Set_Advertising_Data + LE_Set_Advertising_Enable
        let _ = adv_data;
        self.advertising = true;
        true
    }

    pub fn stop_advertising(&mut self) {
        self.advertising = false;
    }

    /// Initiate LE connection to peer.
    pub fn connect(&mut self, peer: BdAddr) -> Option<u16> {
        if self.conn_count >= MAX_BT_CONNS { return None; }
        let idx = self.conn_count;
        self.conns[idx] = BtConnection::new();
        self.conns[idx].addr  = peer;
        self.conns[idx].state = ConnState::Connecting;
        self.conns[idx].handle = (idx + 1) as u16;
        self.conn_count += 1;
        Some(self.conns[idx].handle)
    }

    pub fn disconnect(&mut self, handle: u16) {
        for c in self.conns[..self.conn_count].iter_mut() {
            if c.handle == handle {
                c.state = ConnState::Disconnected;
            }
        }
    }

    /// Add a GATT attribute to the local server.
    pub fn add_gatt_attr(&mut self, uuid: u128, perms: u8, value: &[u8]) -> Option<u16> {
        if self.attr_count >= MAX_GATT_ATTRS { return None; }
        let idx = self.attr_count;
        let handle = (idx + 1) as u16;
        self.attrs[idx].handle = handle;
        self.attrs[idx].uuid   = uuid;
        self.attrs[idx].perms  = perms;
        let vlen = value.len().min(ATT_MAX_VALUE);
        self.attrs[idx].value[..vlen].copy_from_slice(&value[..vlen]);
        self.attrs[idx].vlen = vlen;
        self.attr_count += 1;
        Some(handle)
    }

    /// Process received HCI event packet.
    pub fn process_hci_event(&mut self, evt: &[u8]) {
        if evt.len() < 2 { return; }
        let code = evt[0];
        match code {
            0x3E => self.handle_le_meta_event(&evt[2..]), // LE Meta Event
            0x05 => self.handle_disconnection_complete(&evt[2..]),
            _    => {}
        }
    }

    fn handle_le_meta_event(&mut self, params: &[u8]) {
        if params.is_empty() { return; }
        match params[0] {
            0x01 => { // LE Connection Complete
                if params.len() >= 19 {
                    let handle = u16::from_le_bytes([params[2], params[3]]);
                    let mut addr = BdAddr::ZERO;
                    addr.0.copy_from_slice(&params[8..14]);
                    for c in self.conns[..self.conn_count].iter_mut() {
                        if c.addr == addr {
                            c.handle = handle;
                            c.state  = ConnState::Connected;
                        }
                    }
                }
            }
            0x02 => {} // LE Advertising Report — scan result
            _    => {}
        }
    }

    fn handle_disconnection_complete(&mut self, params: &[u8]) {
        if params.len() < 4 { return; }
        let handle = u16::from_le_bytes([params[1], params[2]]);
        self.disconnect(handle);
    }
}
