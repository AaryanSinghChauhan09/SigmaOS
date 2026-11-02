//! SigmaOS Bluetooth stack integration.
//!
//! Provides BlueZ D-Bus bindings, device pairing, audio routing, and
//! profile management.

#![warn(missing_docs)]

/// State of a Bluetooth device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    /// Device is connected.
    Connected,
    /// Device is paired but disconnected.
    Paired,
    /// Device is discoverable but not paired.
    Discoverable,
    /// Device is off.
    Off,
}

/// A Bluetooth device.
#[derive(Debug, Clone)]
pub struct Device {
    /// MAC address of the device.
    pub address: String,
    /// Display name.
    pub name: String,
    /// Current state.
    pub state: DeviceState,
}

/// Trait for the Bluetooth adapter controller.
pub trait Adapter {
    /// Scan for nearby devices.
    fn scan(&self) -> Result<Vec<Device>, BluetoothError>;
    /// Pair with a device by address.
    fn pair(&mut self, address: &str) -> Result<Device, BluetoothError>;
    /// Connect to a paired device.
    fn connect(&mut self, address: &str) -> Result<(), BluetoothError>;
}

/// Errors returned by the Bluetooth integration.
#[derive(Debug)]
pub enum BluetoothError {
    /// The adapter is not available.
    NoAdapter,
    /// The device was not found.
    DeviceNotFound,
    /// Pairing or connecting failed.
    LinkError(String),
}

impl std::fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BluetoothError::NoAdapter => write!(f, "no bluetooth adapter"),
            BluetoothError::DeviceNotFound => write!(f, "device not found"),
            BluetoothError::LinkError(msg) => write!(f, "link error: {msg}"),
        }
    }
}

impl std::error::Error for BluetoothError {}
