//! Virtio block device driver
//!
//! Implements virtio-blk support for QEMU/KVM guests following
//! WDM-style driver architecture and capability model.

use core::fmt;
use crate::capability::CapabilityToken;
use crate::memory::PhysicalAddress;

/// Virtio block device status
#[derive(Debug, Clone, PartialEq)]
pub enum VirtioBlkStatus {
    /// Device is offline
    Offline,
    /// Device is watching for driver
    Driver,
    /// Driver is watching for device
    Standby,
    /// Device is configured
    Configured,
    /// Device is running
    Running,
}

/// Virtio block device configuration
#[derive(Debug, Clone)]
pub struct VirtioBlkConfig {
    /// Maximum segment size
    pub max_segments: u32,
    /// Segment size
    pub segment_size: u32,
    /// Alignment requirement
    pub alignment: u32,
    /// Physical segment size
    pub physical_segment_size: u64,
}

/// Virtio block device capabilities
#[derive(Debug, Clone)]
pub struct VirtioBlkCapabilities {
    /// Config generation
    pub config_generation: u32,
    /// Write zeroes flag
    pub write_zeroes: bool,
    /// Write zeroes max bytes
    pub write_zeroes_max: u64,
    /// Topology information
    pub topology: VirtioBlkTopology,
}

/// Virtio block device topology
#[derive(Debug, Clone)]
pub struct VirtioBlkTopology {
    /// Physical and logical block alignment
    pub alignment_min: u64,
    pub alignment_opt: u64,
    /// Minimum I/O size
    pub minimum_io_size: u64,
    /// Optimal I/O size
    pub optimal_io_size: u64,
}

/// Virtio block request descriptor
#[derive(Debug, Clone)]
pub struct VirtioBlkRequest {
    /// Descriptor table address
    pub desc_table: PhysicalAddress,
    /// Available ring
    pub avail_ring: PhysicalAddress,
    /// Used ring
    pub used_ring: PhysicalAddress,
    /// Configuration address
    pub config: PhysicalAddress,
    /// Status register
    pub status: VirtioBlkStatus,
    /// Configuration
    pub config_data: VirtioBlkConfig,
    /// Capabilities
    pub capabilities: VirtioBlkCapabilities,
}

/// Virtio block command types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirtioBlkCommand {
    /// Read sectors
    Read,
    /// Write sectors
    Write,
    /// Flush cache
    Flush,
    /// Discard sectors
    Discard,
    /// Write same sectors
    WriteSame,
}

/// Virtio block index
#[derive(Debug, Clone, Copy)]
pub struct VirtioBlkIndex {
    /// First sector index
    pub sector: u64,
    /// Number of sectors
    pub count: u32,
}

/// Virtio block iovec entry
#[derive(Debug, Clone)]
pub struct VirtioBlkIovec {
    /// Buffer address
    pub base: PhysicalAddress,
    /// Buffer length
    pub length: u32,
    /// Relative offset
    pub offset: u64,
}

impl fmt::Display for VirtioBlkStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtioBlkStatus::Offline => write!(f, "Offline"),
            VirtioBlkStatus::Driver => write!(f, "Driver"),
            VirtioBlkStatus::Standby => write!(f, "Standby"),
            VirtioBlkStatus::Configured => write!(f, "Configured"),
            VirtioBlkStatus::Running => write!(f, "Running"),
        }
    }
}

impl fmt::Display for VirtioBlkCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtioBlkCommand::Read => write!(f, "Read"),
            VirtioBlkCommand::Write => write!(f, "Write"),
            VirtioBlkCommand::Flush => write!(f, "Flush"),
            VirtioBlkCommand::Discard => write!(f, "Discard"),
            VirtioBlkCommand::WriteSame => write!(f, "WriteSame"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", VirtioBlkStatus::Running), "Running");
        assert_eq!(format!("{}", VirtioBlkStatus::Offline), "Offline");
    }

    #[test]
    fn test_command_display() {
        assert_eq!(format!("{}", VirtioBlkCommand::Read), "Read");
        assert_eq!(format!("{}", VirtioBlkCommand::Write), "Write");
    }

    #[test]
    fn test_config_defaults() {
        let config = VirtioBlkConfig {
            max_segments: 65536,
            segment_size: 4096,
            alignment: 512,
            physical_segment_size: 4096,
        };
        assert_eq!(config.max_segments, 65536);
        assert_eq!(config.segment_size, 4096);
    }
}
