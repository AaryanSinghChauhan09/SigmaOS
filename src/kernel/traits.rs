#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Kernel Evolution Architecture - Abstract Base Traits
// Foundation traits for all kernel subsystems

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    InitFailed,
    IoFailed,
    ShutdownFailed,
    CapabilityDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    InitFailed,
    SendFailed,
    ReceiveFailed,
    SocketError,
    CapabilityDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    InitFailed,
    MountFailed,
    OpenFailed,
    ReadFailed,
    WriteFailed,
    CapabilityDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    InitFailed,
    AllocationFailed,
    FreeFailed,
    MapFailed,
    CapabilityDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerError {
    InitFailed,
    AddProcessFailed,
    RemoveProcessFailed,
    UpdateProcessFailed,
    CapabilityDenied,
}

pub struct DriverMetadata {
    pub name: String,
    pub version: String,
    pub capabilities: u64,
}

pub struct NetworkStackMetadata {
    pub name: String,
    pub version: String,
    pub max_sockets: usize,
}

pub struct FilesystemMetadata {
    pub name: String,
    pub version: String,
    pub max_files: usize,
}

pub struct MemoryManagerMetadata {
    pub name: String,
    pub version: String,
    pub total_memory: usize,
}

pub struct SchedulerMetadata {
    pub name: String,
    pub version: String,
    pub max_processes: usize,
}

pub trait DeviceDriver {
    fn init(&mut self) -> Result<(), DriverError>;
    fn handle_io(&mut self, operation: u32) -> Result<u32, DriverError>;
    fn shutdown(&mut self) -> Result<(), DriverError>;
    fn metadata(&self) -> &DriverMetadata;
    fn has_capability(&self, capability: u64) -> bool;
}

pub trait NetworkStack {
    fn init(&mut self) -> Result<(), NetworkError>;
    fn receive_packet(&mut self, packet: Vec<u8>) -> Result<(), NetworkError>;
    fn send_packet(&mut self, packet: Vec<u8>) -> Result<(), NetworkError>;
    fn create_socket(
        &mut self,
        domain: u32,
        socket_type: u32,
        protocol: u32,
    ) -> Result<u32, NetworkError>;
    fn close_socket(&mut self, handle: u32) -> Result<(), NetworkError>;
    fn metadata(&self) -> &NetworkStackMetadata;
}

pub trait FileSystem {
    fn init(&mut self) -> Result<(), FsError>;
    fn mount(&mut self, device: &str, mount_point: &str) -> Result<(), FsError>;
    fn unmount(&mut self) -> Result<(), FsError>;
    fn open_file(&mut self, path: &str, flags: u32) -> Result<u32, FsError>;
    fn close_file(&mut self, handle: u32) -> Result<(), FsError>;
    fn read_file(&mut self, handle: u32, buffer: &mut [u8]) -> Result<usize, FsError>;
    fn write_file(&mut self, handle: u32, data: &[u8]) -> Result<usize, FsError>;
    fn create_directory(&mut self, path: &str) -> Result<(), FsError>;
    fn remove(&mut self, path: &str) -> Result<(), FsError>;
    fn metadata(&self) -> &FilesystemMetadata;
}

pub trait MemoryManager {
    fn init(&mut self) -> Result<(), MemoryError>;
    fn allocate_physical(&mut self, size: usize) -> Result<u64, MemoryError>;
    fn free_physical(&mut self, address: u64, size: usize) -> Result<(), MemoryError>;
    fn allocate_virtual(&mut self, size: usize) -> Result<u64, MemoryError>;
    fn free_virtual(&mut self, address: u64, size: usize) -> Result<(), MemoryError>;
    fn map_memory(
        &mut self,
        virtual_addr: u64,
        physical_addr: u64,
        size: usize,
        flags: u32,
    ) -> Result<(), MemoryError>;
    fn unmap_memory(&mut self, virtual_addr: u64, size: usize) -> Result<(), MemoryError>;
    fn metadata(&self) -> &MemoryManagerMetadata;
}

pub trait Scheduler {
    fn init(&mut self) -> Result<(), SchedulerError>;
    fn add_process(&mut self, pid: u64, priority: u32) -> Result<(), SchedulerError>;
    fn remove_process(&mut self, pid: u64) -> Result<(), SchedulerError>;
    fn schedule_next(&mut self) -> Option<u64>;
    fn update_process(&mut self, pid: u64, state: u32) -> Result<(), SchedulerError>;
    fn metadata(&self) -> &SchedulerMetadata;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_metadata() {
        let metadata = DriverMetadata {
            name: "TestDriver".to_string(),
            version: "1.0".to_string(),
            capabilities: 0x1234,
        };
        assert_eq!(metadata.name, "TestDriver");
        assert!(metadata.has_capability(0x1234));
    }

    impl DriverMetadata {
        fn has_capability(&self, capability: u64) -> bool {
            self.capabilities & capability != 0
        }
    }
}
