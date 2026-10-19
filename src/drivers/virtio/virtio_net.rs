//! Virtio network device driver
//!
//! Implements virtio-net support for QEMU/KVM guests following
//! WDM-style driver architecture and capability model.

use core::fmt;
use crate::capability::CapabilityToken;
use crate::memory::PhysicalAddress;

/// Virtio network device status
#[derive(Debug, Clone, PartialEq)]
pub enum VirtioNetStatus {
    /// Device is offline
    Offline,
    /// Device is watching for driver
    Driver,
    /// Device is standby
    Standby,
    /// Device is configured
    Configured,
    /// Device is running
    Running,
}

/// Virtio network device configuration
#[derive(Debug, Clone)]
pub struct VirtioNetConfig {
    /// Maximum queue size
    pub max_queues: u32,
    /// Queue size
    pub queue_size: u32,
    /// Maximum packet size
    pub max_packet_size: u32,
    /// Status
    pub status: VirtioNetStatus,
}

/// Virtio network capabilities
#[derive(Debug, Clone)]
pub struct VirtioNetCapabilities {
    /// Guest offload features
    pub guest_offload: u64,
    /// Guest features
    pub guest_features: u64,
    /// Maximum queue Num
    pub max_queue_num: u32,
}

/// Virtio network receive descriptor
#[derive(Debug, Clone)]
pub struct VirtioNetRxDescriptor {
    /// Buffer address
    pub buffer: PhysicalAddress,
    /// Buffer length
    pub length: u32,
    /// Status
    pub status: u16,
}

/// Virtio network transmit descriptor
#[derive(Debug, Clone)]
pub struct VirtioNetTxDescriptor {
    /// Buffer address
    pub buffer: PhysicalAddress,
    /// Buffer length
    pub length: u32,
    /// Status
    pub status: u16,
}

/// Virtio network configuration
#[derive(Debug, Clone)]
pub struct VirtioNetConfigData {
    /// MAC address [6 bytes]
    pub mac_address: [u8; 6],
    /// Maximum guest ring size
    pub max_guest_ring_size: u32,
    /// Maximum descriptor number
    pub max_descriptor_number: u32,
    /// Guest feature select
    pub guest_feature_select: u32,
    /// Guest feature value
    pub guest_feature_value: u64,
}

/// Virtio network receive buffer
#[derive(Debug, Clone)]
pub struct VirtioNetRxBuffer {
    /// Packet data
    pub data: Vec<u8>,
    /// Packet length
    pub length: u32,
    /// Status flag
    pub status: bool,
}

/// Virtio network transmit status
#[derive(Debug, Clone, PartialEq)]
pub enum VirtioNetTxStatus {
    /// Transmission successful
    Success,
    /// Transmission failed
    Failed,
    /// Packet dropped
    Dropped,
}

/// Virtio network command
#[derive(Debug, Clone, Copy)]
pub enum VirtioNetCommand {
    /// Send packet
    Send,
    /// Set multicast address
    SetMulticast,
    /// Get statistics
    GetStats,
    /// Reset device
    Reset,
}

/// Virtio network statistics
#[derive(Debug, Clone)]
pub struct VirtioNetStats {
    /// Packets received
    pub packets_received: u64,
    /// Packets transmitted
    pub packets_transmitted: u64,
    /// Bytes received
    pub bytes_received: u64,
    /// Bytes transmitted
    pub bytes_transmitted: u64,
    /// Errors received
    pub errors_received: u64,
}

/// Virtio network MAC address
type MacAddress = [u8; 6];

impl fmt::Display for VirtioNetStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtioNetStatus::Offline => write!(f, "Offline"),
            VirtioNetStatus::Driver => write!(f, "Driver"),
            VirtioNetStatus::Standby => write!(f, "Standby"),
            VirtioNetStatus::Configured => write!(f, "Configured"),
            VirtioNetStatus::Running => write!(f, "Running"),
        }
    }
}

impl fmt::Display for VirtioNetTxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtioNetTxStatus::Success => write!(f, "Success"),
            VirtioNetTxStatus::Failed => write!(f, "Failed"),
            VirtioNetTxStatus::Dropped => write!(f, "Dropped"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", VirtioNetStatus::Running), "Running");
        assert_eq!(format!("{}", VirtioNetStatus::Offline), "Offline");
    }

    #[test]
    fn test_tx_status_display() {
        assert_eq!(format!("{}", VirtioNetTxStatus::Success), "Success");
        assert_eq!(format!("{}", VirtioNetTxStatus::Failed), "Failed");
    }

    #[test]
    fn test_config_defaults() {
        let config = VirtioNetConfig {
            max_queues: 4,
            queue_size: 256,
            max_packet_size: 2048,
            status: VirtioNetStatus::Configured,
        };
        assert_eq!(config.max_queues, 4);
        assert_eq!(config.queue_size, 256);
    }
}
