use alloc::string::String;

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluetoothState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluetoothType {
    Classic,
    Ble,
}

pub struct SigmaBluetoothManager2;
