//! SigmaOS Bluetooth configuration and device management UI.
//!
//! Exposes pairing, audio, and file transfer features through a GTK interface.

#![warn(missing_docs)]

/// Kind of Bluetooth transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferKind {
    /// Send a file to a device.
    Send,
    /// Receive a file from a device.
    Receive,
}

/// A file transfer request.
#[derive(Debug, Clone)]
pub struct Transfer {
    /// Path of the file being transferred.
    pub path: String,
    /// Address of the peer device.
    pub device: String,
    /// Direction of the transfer.
    pub kind: TransferKind,
}

/// Trait for the UI controller that drives the Bluetooth tool.
pub trait BluetoothUi {
    /// Show the device list panel.
    fn show_devices(&self);
    /// Start a file transfer.
    fn transfer(&mut self, transfer: Transfer) -> Result<(), ToolError>;
    /// Toggle audio routing to a device.
    fn toggle_audio(&mut self, address: &str) -> Result<(), ToolError>;
}

/// Errors returned by the Bluetooth tool.
#[derive(Debug)]
pub enum ToolError {
    /// The selected device is unavailable.
    DeviceUnavailable,
    /// The transfer could not start.
    TransferFailed(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::DeviceUnavailable => write!(f, "device unavailable"),
            ToolError::TransferFailed(msg) => write!(f, "transfer failed: {msg}"),
        }
    }
}

impl std::error::Error for ToolError {}
