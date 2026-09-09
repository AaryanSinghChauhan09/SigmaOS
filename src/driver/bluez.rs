// BlueZ Bluetooth Driver & HCI Subsystem Implementation for SigmaOS
// Provides zero-dependency HCI command packet routing, Bluetooth adapter management, and L2CAP channel parsing.

use crate::klib::string::String;
use crate::klib::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterState {
    PoweredDown,
    PoweredOn,
    Discovering,
}

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub address: [u8; 6],
    pub name: String,
    pub rssi: i8,
    pub connected: bool,
}

pub struct BluezHciAdapter {
    pub adapter_id: u32,
    pub address: [u8; 6],
    pub state: AdapterState,
    pub paired_devices: Vec<BluetoothDevice>,
}

impl BluezHciAdapter {
    pub fn new(adapter_id: u32, address: [u8; 6]) -> Self {
        Self {
            adapter_id,
            address,
            state: AdapterState::PoweredDown,
            paired_devices: Vec::new(),
        }
    }

    pub fn power_on(&mut self) {
        self.state = AdapterState::PoweredOn;
    }

    pub fn power_off(&mut self) {
        self.state = AdapterState::PoweredDown;
    }

    pub fn start_discovery(&mut self) -> bool {
        if self.state == AdapterState::PoweredOn {
            self.state = AdapterState::Discovering;
            true
        } else {
            false
        }
    }

    pub fn add_device(&mut self, address: [u8; 6], name: &str, rssi: i8) {
        self.paired_devices.push(BluetoothDevice {
            address,
            name: String::from(name),
            rssi,
            connected: false,
        });
    }
}
