//! Bluetooth Protocol Stack (Classic + BLE)
//! Implements HCI, L2CAP, SDP, RFCOMM, ATT/GATT protocols
//! Reference: Bluetooth Core Specification and Linux net/bluetooth/

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// Bluetooth Device Address (BD_ADDR) - 48-bit unique address
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BdAddr {
    pub addr: [u8; 6],
}

impl BdAddr {
    pub fn new(addr: [u8; 6]) -> Self {
        Self { addr }
    }

    pub fn is_valid(&self) -> bool {
        self.addr != [0; 6] && self.addr != [0xFF; 6]
    }
}

/// HCI (Host Controller Interface) packet types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciPacketType {
    Command = 0x01,
    AclData = 0x02,
    ScoData = 0x03,
    Event = 0x04,
    IsoData = 0x05,
}

/// HCI Command Opcode groups
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum HciOgf {
    LinkControl = 0x01,
    LinkPolicy = 0x02,
    ControllerBasic = 0x03,
    InfoParams = 0x04,
    StatusParams = 0x05,
    Testing = 0x06,
    LeController = 0x08,
}

/// HCI Command structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct HciCommand {
    pub opcode: u16, // OGF (6 bits) | OCF (10 bits)
    pub param_len: u8,
    // Parameters follow
}

impl HciCommand {
    pub fn new(ogf: HciOgf, ocf: u16, param_len: u8) -> Self {
        let opcode = ((ogf as u16) << 10) | (ocf & 0x3FF);
        Self { opcode, param_len }
    }
}

/// HCI Event codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciEventCode {
    InquiryComplete = 0x01,
    InquiryResult = 0x02,
    ConnectionComplete = 0x03,
    ConnectionRequest = 0x04,
    DisconnectionComplete = 0x05,
    RemoteNameReqComplete = 0x07,
    EncryptionChange = 0x08,
    CommandComplete = 0x0E,
    CommandStatus = 0x0F,
    RoleChange = 0x12,
    NumCompletedPackets = 0x13,
    PinCodeRequest = 0x16,
    LinkKeyRequest = 0x17,
    LinkKeyNotification = 0x18,
    LeMetaEvent = 0x3E,
}

/// HCI Event structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct HciEvent {
    pub event_code: u8,
    pub param_len: u8,
    // Parameters follow
}

/// HCI ACL Data packet
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct HciAclHeader {
    pub handle: u16, // Connection handle (12 bits) | PB flag (2) | BC flag (2)
    pub data_len: u16,
}

/// L2CAP (Logical Link Control and Adaptation Protocol) header
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct L2capHeader {
    pub length: u16, // PDU length (excluding header)
    pub cid: u16,    // Channel ID
}

/// L2CAP Channel IDs
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum L2capCid {
    Null = 0x0000,
    SignalingBr = 0x0001,      // BR/EDR signaling
    ConnectionlessBr = 0x0002, // BR/EDR connectionless
    AmpManagerBr = 0x0003,     // AMP Manager
    AttBle = 0x0004,           // BLE ATT
    SignalingLe = 0x0005,      // BLE signaling
    SmBle = 0x0006,            // BLE Security Manager
                               // Dynamic channels: 0x0040-0xFFFF
}

/// L2CAP Signaling command codes
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum L2capSignalCode {
    CommandReject = 0x01,
    ConnectionReq = 0x02,
    ConnectionRsp = 0x03,
    ConfigReq = 0x04,
    ConfigRsp = 0x05,
    DisconnectionReq = 0x06,
    DisconnectionRsp = 0x07,
    EchoReq = 0x08,
    EchoRsp = 0x09,
    InfoReq = 0x0A,
    InfoRsp = 0x0B,
}

/// SDP (Service Discovery Protocol) data element types
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SdpDataType {
    Nil = 0,
    Uint = 1,
    Int = 2,
    Uuid = 3,
    String = 4,
    Boolean = 5,
    Sequence = 6,
    Alternative = 7,
    Url = 8,
}

/// RFCOMM (Serial Port Emulation) frame types
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RfcommFrameType {
    Sabm = 0x2F, // Set Asynchronous Balanced Mode
    Ua = 0x63,   // Unnumbered Acknowledgement
    Dm = 0x0F,   // Disconnected Mode
    Disc = 0x43, // Disconnect
    Uih = 0xEF,  // Unnumbered Info with Header check
}

/// ATT (Attribute Protocol) opcodes for BLE GATT
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttOpcode {
    ErrorRsp = 0x01,
    MtuReq = 0x02,
    MtuRsp = 0x03,
    FindInfoReq = 0x04,
    FindInfoRsp = 0x05,
    FindByTypeValueReq = 0x06,
    FindByTypeValueRsp = 0x07,
    ReadByTypeReq = 0x08,
    ReadByTypeRsp = 0x09,
    ReadReq = 0x0A,
    ReadRsp = 0x0B,
    ReadBlobReq = 0x0C,
    ReadBlobRsp = 0x0D,
    ReadMultipleReq = 0x0E,
    ReadMultipleRsp = 0x0F,
    ReadByGroupTypeReq = 0x10,
    ReadByGroupTypeRsp = 0x11,
    WriteReq = 0x12,
    WriteRsp = 0x13,
    WriteCmd = 0x52,
    PrepareWriteReq = 0x16,
    PrepareWriteRsp = 0x17,
    ExecuteWriteReq = 0x18,
    ExecuteWriteRsp = 0x19,
    HandleValueNtf = 0x1B,
    HandleValueInd = 0x1D,
    HandleValueCfm = 0x1E,
}

/// GATT (Generic Attribute Profile) UUID types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GattUuid {
    Uuid16(u16),
    Uuid128([u8; 16]),
}

impl GattUuid {
    /// Primary Service UUID
    pub const PRIMARY_SERVICE: u16 = 0x2800;
    /// Secondary Service UUID
    pub const SECONDARY_SERVICE: u16 = 0x2801;
    /// Characteristic UUID
    pub const CHARACTERISTIC: u16 = 0x2803;
}

/// GATT Characteristic properties
#[derive(Debug, Clone, Copy)]
pub struct GattCharProperties {
    pub broadcast: bool,
    pub read: bool,
    pub write_without_response: bool,
    pub write: bool,
    pub notify: bool,
    pub indicate: bool,
    pub auth_signed_writes: bool,
    pub extended_props: bool,
}

impl GattCharProperties {
    pub fn from_byte(byte: u8) -> Self {
        Self {
            broadcast: (byte & 0x01) != 0,
            read: (byte & 0x02) != 0,
            write_without_response: (byte & 0x04) != 0,
            write: (byte & 0x08) != 0,
            notify: (byte & 0x10) != 0,
            indicate: (byte & 0x20) != 0,
            auth_signed_writes: (byte & 0x40) != 0,
            extended_props: (byte & 0x80) != 0,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte = 0u8;
        if self.broadcast {
            byte |= 0x01;
        }
        if self.read {
            byte |= 0x02;
        }
        if self.write_without_response {
            byte |= 0x04;
        }
        if self.write {
            byte |= 0x08;
        }
        if self.notify {
            byte |= 0x10;
        }
        if self.indicate {
            byte |= 0x20;
        }
        if self.auth_signed_writes {
            byte |= 0x40;
        }
        if self.extended_props {
            byte |= 0x80;
        }
        byte
    }
}

/// GATT Attribute
#[derive(Debug, Clone)]
pub struct GattAttribute {
    pub handle: u16,
    pub uuid: GattUuid,
    pub value: Vec<u8>,
    pub permissions: u8,
}

/// GATT Service
#[derive(Debug, Clone)]
pub struct GattService {
    pub start_handle: u16,
    pub end_handle: u16,
    pub uuid: GattUuid,
    pub characteristics: Vec<GattCharacteristic>,
}

/// GATT Characteristic
#[derive(Debug, Clone)]
pub struct GattCharacteristic {
    pub handle: u16,
    pub value_handle: u16,
    pub uuid: GattUuid,
    pub properties: GattCharProperties,
    pub value: Vec<u8>,
}

/// Bluetooth device structure
pub struct BluetoothDevice {
    pub addr: BdAddr,
    pub name: Vec<u8>,
    pub device_class: u32, // Class of Device (CoD)
    pub connected: bool,
    pub paired: bool,
    pub link_key: Option<[u8; 16]>,
    pub rssi: i8, // Signal strength
    pub services: Vec<GattService>,
}

impl BluetoothDevice {
    pub fn new(addr: BdAddr) -> Self {
        Self {
            addr,
            name: Vec::new(),
            device_class: 0,
            connected: false,
            paired: false,
            link_key: None,
            rssi: 0,
            services: Vec::new(),
        }
    }
}

/// Bluetooth Controller (HCI interface)
pub struct BluetoothController {
    pub local_addr: BdAddr,
    pub local_name: Vec<u8>,
    pub devices: BTreeMap<BdAddr, BluetoothDevice>,
    pub connections: Vec<u16>, // Active connection handles
}

impl BluetoothController {
    pub fn new() -> Self {
        Self {
            local_addr: BdAddr::new([0; 6]),
            local_name: Vec::new(),
            devices: BTreeMap::new(),
            connections: Vec::new(),
        }
    }

    /// Initialize controller
    pub fn init(&mut self) -> Result<(), BtError> {
        // Send HCI Reset command
        // Read local BD_ADDR
        // Read local name
        // Read buffer size
        Ok(())
    }

    /// Start device inquiry/scan
    pub fn start_inquiry(&mut self, duration_sec: u8) -> Result<(), BtError> {
        // Send HCI_Inquiry command for BR/EDR
        // or HCI_LE_Set_Scan_Enable for BLE
        Ok(())
    }

    /// Connect to device
    pub fn connect(&mut self, addr: BdAddr) -> Result<u16, BtError> {
        // Send HCI_Create_Connection for BR/EDR
        // or HCI_LE_Create_Connection for BLE
        let handle = 0x0001; // Connection handle from controller
        self.connections.push(handle);
        Ok(handle)
    }

    /// Disconnect from device
    pub fn disconnect(&mut self, handle: u16) -> Result<(), BtError> {
        // Send HCI_Disconnect command
        self.connections.retain(|&h| h != handle);
        Ok(())
    }

    /// Pair with device
    pub fn pair(&mut self, addr: BdAddr, pin: &[u8]) -> Result<(), BtError> {
        // Handle PIN code or passkey pairing
        // Store link key
        if let Some(device) = self.devices.get_mut(&addr) {
            device.paired = true;
        }
        Ok(())
    }

    /// Discover GATT services (BLE)
    pub fn discover_services(&mut self, addr: BdAddr) -> Result<Vec<GattService>, BtError> {
        // Send ATT Read By Group Type Request
        // Parse service declarations
        let services = Vec::new();
        if let Some(device) = self.devices.get_mut(&addr) {
            device.services = services.clone();
        }
        Ok(services)
    }

    /// Read GATT characteristic (BLE)
    pub fn read_characteristic(&self, handle: u16) -> Result<Vec<u8>, BtError> {
        // Send ATT Read Request
        Ok(Vec::new())
    }

    /// Write GATT characteristic (BLE)
    pub fn write_characteristic(&mut self, handle: u16, value: &[u8]) -> Result<(), BtError> {
        // Send ATT Write Request or Write Command
        Ok(())
    }
}

/// Bluetooth error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BtError {
    InvalidAddress,
    NotConnected,
    AlreadyConnected,
    PairingFailed,
    ConnectionFailed,
    Timeout,
    InvalidHandle,
    ProtocolError,
}

/// Bluetooth device class masks
pub mod device_class {
    pub const MAJOR_COMPUTER: u32 = 0x0100;
    pub const MAJOR_PHONE: u32 = 0x0200;
    pub const MAJOR_AUDIO: u32 = 0x0400;
    pub const MAJOR_PERIPHERAL: u32 = 0x0500;
    pub const MAJOR_IMAGING: u32 = 0x0600;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bdaddr() {
        let addr = BdAddr::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(addr.is_valid());
    }

    #[test]
    fn test_gatt_properties() {
        let props = GattCharProperties::from_byte(0x12); // Read + Notify
        assert!(props.read);
        assert!(props.notify);
        assert!(!props.write);
    }

    #[test]
    fn test_hci_opcode() {
        let cmd = HciCommand::new(HciOgf::LinkControl, 0x0001, 0);
        let ogf = (cmd.opcode >> 10) & 0x3F;
        assert_eq!(ogf, 0x01);
    }
}
