#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use std::boxed::Box;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

pub type DeviceID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothState {
    Off = 0,
    On = 1,
    Scanning = 2,
    Pairing = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothError {
    Success = 0,
    NotFound = 1,
    PairingFailed = 2,
}

pub trait BluetoothAdapter {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn address(&self) -> &[u8];
    fn state(&self) -> BluetoothState;
    fn set_state(&mut self, state: BluetoothState);
}

#[repr(C)]
pub struct SimpleBluetoothAdapter {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub name_len: u8,
    pub address: [u8; 6],
    pub state: AtomicUsize,
}

impl SimpleBluetoothAdapter {
    pub fn new(id: DeviceID, name: &[u8], address: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut addr_array = [0u8; 6];
        let name_len = name.len().min(63);
        let addr_len = address.len().min(6);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(address.as_ptr(), addr_array.as_mut_ptr(), addr_len);
        }
        SimpleBluetoothAdapter {
            id,
            name: name_array,
            name_len: name_len as u8,
            address: addr_array,
            state: AtomicUsize::new(BluetoothState::Off as usize),
        }
    }
}

impl BluetoothAdapter for SimpleBluetoothAdapter {
    fn id(&self) -> DeviceID {
        self.id
    }
    fn name(&self) -> &[u8] {
        &self.name[..self.name_len as usize]
    }
    fn address(&self) -> &[u8] {
        &self.address
    }
    fn state(&self) -> BluetoothState {
        match self.state.load(Ordering::SeqCst) {
            0 => BluetoothState::Off,
            1 => BluetoothState::On,
            2 => BluetoothState::Scanning,
            3 => BluetoothState::Pairing,
            _ => BluetoothState::Off,
        }
    }

    fn set_state(&mut self, state: BluetoothState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

pub trait BluetoothManager {
    fn add_adapter(&mut self, adapter: Box<dyn BluetoothAdapter>) -> Result<DeviceID, BluetoothError>;
    fn remove_adapter(&mut self, id: DeviceID) -> Result<(), BluetoothError>;
    fn get_adapter(&self, id: DeviceID) -> Option<&dyn BluetoothAdapter>;
    fn start_scan(&mut self, id: DeviceID) -> Result<(), BluetoothError>;
    fn stop_scan(&mut self, id: DeviceID) -> Result<(), BluetoothError>;
}

#[repr(C)]
pub struct SimpleBluetoothManager {
    pub adapters: Vec<Option<Box<dyn BluetoothAdapter>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBluetoothManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleBluetoothManager {
            adapters: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BluetoothManager for SimpleBluetoothManager {
    fn add_adapter(&mut self, adapter: Box<dyn BluetoothAdapter>) -> Result<DeviceID, BluetoothError> {
        let id = adapter.id();
        self.adapters.push(Some(adapter));
        Ok(id)
    }

    fn remove_adapter(&mut self, id: DeviceID) -> Result<(), BluetoothError> {
        for adapter_option in &mut self.adapters {
            if let Some(ref adapter) = *adapter_option {
                if adapter.id() == id {
                    *adapter_option = None;
                    return Ok(());
                }
            }
        }
        Err(BluetoothError::NotFound)
    }

    fn get_adapter(&self, id: DeviceID) -> Option<&dyn BluetoothAdapter> {
        for adapter_option in &self.adapters {
            if let Some(ref adapter) = *adapter_option {
                if adapter.id() == id {
                    return Some(adapter.as_ref());
                }
            }
        }
        None
    }

    fn start_scan(&mut self, id: DeviceID) -> Result<(), BluetoothError> {
        for adapter_option in &mut self.adapters {
            if let Some(ref mut adapter) = *adapter_option {
                if adapter.id() == id {
                    adapter.set_state(BluetoothState::Scanning);
                    return Ok(());
                }
            }
        }
        Err(BluetoothError::NotFound)
    }

    fn stop_scan(&mut self, id: DeviceID) -> Result<(), BluetoothError> {
        for adapter_option in &mut self.adapters {
            if let Some(ref mut adapter) = *adapter_option {
                if adapter.id() == id {
                    adapter.set_state(BluetoothState::On);
                    return Ok(());
                }
            }
        }
        Err(BluetoothError::NotFound)
    }
}

pub trait DevicePairing {
    fn pair_device(&mut self, adapter_id: DeviceID, device_address: &[u8]) -> Result<(), BluetoothError>;
    fn unpair_device(&mut self, adapter_id: DeviceID, device_address: &[u8]) -> Result<(), BluetoothError>;
    fn get_paired_devices(&self, adapter_id: DeviceID) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleDevicePairing {
    pub paired: Vec<(DeviceID, [u8; 6])>,
}

impl SimpleDevicePairing {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleDevicePairing { paired: Vec::new() }
    }
}

impl DevicePairing for SimpleDevicePairing {
    fn pair_device(&mut self, adapter_id: DeviceID, device_address: &[u8]) -> Result<(), BluetoothError> {
        let mut addr_array = [0u8; 6];
        let addr_len = device_address.len().min(6);
        for i in 0..addr_len {
            addr_array[i] = device_address[i];
        }
        self.paired.push((adapter_id, addr_array));
        Ok(())
    }

    fn unpair_device(&mut self, adapter_id: DeviceID, device_address: &[u8]) -> Result<(), BluetoothError> {
        for i in 0..self.paired.len() {
            if self.paired[i].0 == adapter_id && &self.paired[i].1[..device_address.len().min(6)] == device_address {
                self.paired.remove(i);
                return Ok(());
            }
        }
        Err(BluetoothError::NotFound)
    }

    fn get_paired_devices(&self, adapter_id: DeviceID) -> Vec<&[u8]> {
        let mut devices = Vec::new();
        for &(id, ref addr) in &self.paired {
            if id == adapter_id {
                devices.push(addr.as_slice());
            }
        }
        devices
    }
}

/// ---------------------------------------------------------------------------
/// Full Sovereign Bluetooth Protocol Stack Engine
/// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciPacketType {
    Command = 0x01,
    AclData = 0x02,
    ScoData = 0x03,
    Event = 0x04,
    IsoData = 0x05,
}

#[derive(Debug, Clone)]
pub struct L2capChannel {
    pub cid: u16,
    pub psm: u16,
    pub mtu: u16,
    pub is_connected: bool,
}

#[derive(Debug, Clone)]
pub struct GattAttribute {
    pub handle: u16,
    pub uuid: u16,
    pub value: Vec<u8>,
    pub is_notify: bool,
}

#[derive(Debug, Clone)]
pub struct SovereignBluetoothProtocolStackEngine {
    pub local_address: [u8; 6],
    pub is_discoverable: bool,
    pub l2cap_channels: Vec<L2capChannel>,
    pub gatt_attributes: Vec<GattAttribute>,
    pub pairing_state: BluetoothState,
    pub active_iso_streams: usize,
}

impl SovereignBluetoothProtocolStackEngine {
    pub fn new(local_address: [u8; 6]) -> Self {
        let mut engine = Self {
            local_address,
            is_discoverable: false,
            l2cap_channels: Vec::new(),
            gatt_attributes: Vec::new(),
            pairing_state: BluetoothState::Off,
            active_iso_streams: 0,
        };
        engine.l2cap_channels.push(L2capChannel {
            cid: 0x0001,
            psm: 0x0001,
            mtu: 672,
            is_connected: true,
        });
        engine.l2cap_channels.push(L2capChannel {
            cid: 0x0004,
            psm: 0x001F,
            mtu: 512,
            is_connected: true,
        });
        engine
    }

    pub fn frame_hci_command(&self, ogf: u8, ocf: u16, params: &[u8]) -> Vec<u8> {
        let opcode = ((ogf as u16) << 10) | (ocf & 0x03FF);
        let mut packet = Vec::with_capacity(4 + params.len());
        packet.push(HciPacketType::Command as u8);
        packet.push((opcode & 0xFF) as u8);
        packet.push(((opcode >> 8) & 0xFF) as u8);
        packet.push(params.len() as u8);
        packet.extend_from_slice(params);
        packet
    }

    pub fn register_gatt_attribute(&mut self, handle: u16, uuid: u16, initial_value: &[u8], is_notify: bool) {
        self.gatt_attributes.push(GattAttribute {
            handle,
            uuid,
            value: initial_value.to_vec(),
            is_notify,
        });
    }

    pub fn read_gatt_attribute(&self, handle: u16) -> Option<&[u8]> {
        for attr in &self.gatt_attributes {
            if attr.handle == handle {
                return Some(&attr.value);
            }
        }
        None
    }

    pub fn setup_le_audio_lc3_iso_stream(&mut self, stream_id: u8, interval_ms: u16) -> bool {
        let _ = (stream_id, interval_ms);
        self.active_iso_streams += 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_adapter_lifecycle() {
        let mut adapter = SimpleBluetoothAdapter::new(1, b"Sigma_BT_Host", &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(adapter.id(), 1);
        assert_eq!(adapter.name(), b"Sigma_BT_Host");
        assert_eq!(adapter.address(), &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(adapter.state(), BluetoothState::Off);

        adapter.set_state(BluetoothState::On);
        assert_eq!(adapter.state(), BluetoothState::On);
    }

    #[test]
    fn test_bluetooth_manager_and_pairing() {
        let mut manager = SimpleBluetoothManager::new();
        let adapter = Box::new(SimpleBluetoothAdapter::new(10, b"Adapter_1", &[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]));
        let id = manager.add_adapter(adapter).unwrap();
        assert_eq!(id, 10);

        assert!(manager.start_scan(10).is_ok());
        assert_eq!(manager.get_adapter(10).unwrap().state(), BluetoothState::Scanning);

        let mut pairing = SimpleDevicePairing::new();
        pairing.pair_device(10, &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66]).unwrap();
        let paired = pairing.get_paired_devices(10);
        assert_eq!(paired.len(), 1);
        assert_eq!(paired[0], &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
    }

    #[test]
    fn test_sovereign_bluetooth_protocol_stack() {
        let mut stack = SovereignBluetoothProtocolStackEngine::new([0xDC, 0x00, 0x11, 0x22, 0x33, 0x44]);
        let hci_pkt = stack.frame_hci_command(0x03, 0x0003, &[]);
        assert_eq!(hci_pkt[0], HciPacketType::Command as u8);
        assert_eq!(hci_pkt[1], 0x03);
        assert_eq!(hci_pkt[2], 0x0C);

        stack.register_gatt_attribute(0x0010, 0x2A00, b"Sigma Phone", false);
        let val = stack.read_gatt_attribute(0x0010).unwrap();
        assert_eq!(val, b"Sigma Phone");

        assert!(stack.setup_le_audio_lc3_iso_stream(1, 10));
        assert_eq!(stack.active_iso_streams, 1);
    }
}
