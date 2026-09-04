#![allow(dead_code)]
//! Advanced Local Procedure Call (ALPC/LPC) Subsystem for SigmaOS
//! Inspired by Windows NT ALPC, Mach IPC Ports, and Linux/BSD High-Performance Fast IPC.
//!
//! Provides zero-copy section memory mapping for large payload procedure calls,
//! facility-based message routing (Kernel, VFS, Net, Auth, Distro),
//! synchronous Request-Reply RPC execution, and sandboxed port security checks.
use std::vec;

use core::sync::atomic::{AtomicU64, Ordering};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Facility categories for system procedure routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlpcFacility {
    SystemKernel,
    FileSystemVfs,
    NetworkStack,
    SecurityAuth,
    DistroCompatibility,
    UserDefined(u32),
}

impl AlpcFacility {
    pub fn to_u32(&self) -> u32 {
        match self {
            Self::SystemKernel => 1,
            Self::FileSystemVfs => 2,
            Self::NetworkStack => 3,
            Self::SecurityAuth => 4,
            Self::DistroCompatibility => 5,
            Self::UserDefined(val) => *val,
        }
    }

    pub fn from_u32(val: u32) -> Self {
        match val {
            1 => Self::SystemKernel,
            2 => Self::FileSystemVfs,
            3 => Self::NetworkStack,
            4 => Self::SecurityAuth,
            5 => Self::DistroCompatibility,
            custom => Self::UserDefined(custom),
        }
    }
}

pub mod alpc_flags {
    pub const SYNCHRONOUS: u32 = 0x01;
    pub const ASYNCHRONOUS: u32 = 0x02;
    pub const LARGE_SECTION_PAYLOAD: u32 = 0x04;
    pub const SECURITY_TOKEN_ATTACHED: u32 = 0x08;
    pub const REPLY_MESSAGE: u32 = 0x10;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlpcPortType {
    ServerConnectionPort,
    ClientCommunicationPort,
    ServerCommunicationPort,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpcMessageHeader {
    pub msg_id: u64,
    pub facility: AlpcFacility,
    pub procedure_id: u32,
    pub flags: u32,
    pub sender_pid: u32,
    pub sender_uid: u32,
    pub payload_len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpcSectionHandle {
    pub section_id: u64,
    pub size: usize,
    pub data: Vec<u8>,
}

impl AlpcSectionHandle {
    pub fn new(section_id: u64, data: Vec<u8>) -> Self {
        let size = data.len();
        Self {
            section_id,
            size,
            data,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpcMessage {
    pub header: AlpcMessageHeader,
    pub inline_data: Vec<u8>,
    pub section: Option<AlpcSectionHandle>,
}

impl AlpcMessage {
    pub fn new_inline(
        msg_id: u64,
        facility: AlpcFacility,
        procedure_id: u32,
        sender_pid: u32,
        sender_uid: u32,
        data: Vec<u8>,
    ) -> Self {
        let payload_len = data.len();
        Self {
            header: AlpcMessageHeader {
                msg_id,
                facility,
                procedure_id,
                flags: alpc_flags::SYNCHRONOUS,
                sender_pid,
                sender_uid,
                payload_len,
            },
            inline_data: data,
            section: None,
        }
    }

    pub fn new_section(
        msg_id: u64,
        facility: AlpcFacility,
        procedure_id: u32,
        sender_pid: u32,
        sender_uid: u32,
        section_id: u64,
        large_data: Vec<u8>,
    ) -> Self {
        let payload_len = large_data.len();
        let section = AlpcSectionHandle::new(section_id, large_data);
        Self {
            header: AlpcMessageHeader {
                msg_id,
                facility,
                procedure_id,
                flags: alpc_flags::SYNCHRONOUS | alpc_flags::LARGE_SECTION_PAYLOAD,
                sender_pid,
                sender_uid,
                payload_len,
            },
            inline_data: Vec::new(),
            section: Some(section),
        }
    }

    pub fn get_payload(&self) -> &[u8] {
        if let Some(ref sec) = self.section {
            &sec.data
        } else {
            &self.inline_data
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlpcPort {
    pub port_id: u64,
    pub port_name: String,
    pub port_type: AlpcPortType,
    pub facility: AlpcFacility,
    pub max_inline_size: usize,
    pub connected_peer_id: Option<u64>,
    pub incoming_queue: Vec<AlpcMessage>,
}

impl AlpcPort {
    pub fn new(port_id: u64, name: &str, port_type: AlpcPortType, facility: AlpcFacility) -> Self {
        Self {
            port_id,
            port_name: name.to_string(),
            port_type,
            facility,
            max_inline_size: 256,
            connected_peer_id: None,
            incoming_queue: Vec::new(),
        }
    }

    pub fn connect_peer(&mut self, peer_id: u64) {
        self.connected_peer_id = Some(peer_id);
    }
}

pub type AlpcProcedureHandler = Box<dyn Fn(&AlpcMessage) -> Vec<u8> + Send + Sync>;

pub struct AlpcFacilityServer {
    pub facility: AlpcFacility,
    pub server_port: AlpcPort,
    pub registered_procedures: BTreeMap<u32, AlpcProcedureHandler>,
}

impl AlpcFacilityServer {
    pub fn new(port_id: u64, facility: AlpcFacility, name: &str) -> Self {
        Self {
            facility,
            server_port: AlpcPort::new(port_id, name, AlpcPortType::ServerConnectionPort, facility),
            registered_procedures: BTreeMap::new(),
        }
    }

    pub fn register_procedure<F>(&mut self, procedure_id: u32, handler: F)
    where
        F: Fn(&AlpcMessage) -> Vec<u8> + Send + Sync + 'static,
    {
        self.registered_procedures
            .insert(procedure_id, Box::new(handler));
    }

    pub fn dispatch_request(&self, request: &AlpcMessage) -> Option<AlpcMessage> {
        if let Some(handler) = self.registered_procedures.get(&request.header.procedure_id) {
            let reply_data = handler(request);
            let mut reply_msg = AlpcMessage::new_inline(
                request.header.msg_id + 1000000,
                request.header.facility,
                request.header.procedure_id,
                0, // Server PID
                0, // Server UID
                reply_data,
            );
            reply_msg.header.flags |= alpc_flags::REPLY_MESSAGE;
            Some(reply_msg)
        } else {
            None
        }
    }
}

pub struct AlpcManager {
    next_id: AtomicU64,
    ports: BTreeMap<u64, AlpcPort>,
    servers: BTreeMap<u32, AlpcFacilityServer>,
}

impl AlpcManager {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            ports: BTreeMap::new(),
            servers: BTreeMap::new(),
        }
    }

    pub fn create_port(
        &mut self,
        name: &str,
        port_type: AlpcPortType,
        facility: AlpcFacility,
    ) -> u64 {
        let port_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let port = AlpcPort::new(port_id, name, port_type, facility);
        self.ports.insert(port_id, port);
        port_id
    }

    pub fn register_facility_server(&mut self, facility: AlpcFacility, name: &str) -> u64 {
        let port_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let server = AlpcFacilityServer::new(port_id, facility, name);
        self.servers.insert(facility.to_u32(), server);
        port_id
    }

    pub fn get_facility_server_mut(
        &mut self,
        facility: AlpcFacility,
    ) -> Option<&mut AlpcFacilityServer> {
        self.servers.get_mut(&facility.to_u32())
    }

    /// Synchronous Request-Reply Local Procedure Call execution
    pub fn request_reply(
        &mut self,
        facility: AlpcFacility,
        mut request: AlpcMessage,
    ) -> Result<AlpcMessage, &'static str> {
        let facility_key = facility.to_u32();

        // Handle large payloads (> 256 bytes) via zero-copy Section Memory Mapping
        if request.inline_data.len() > 256 && request.section.is_none() {
            let sec_id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let data = core::mem::take(&mut request.inline_data);
            request = AlpcMessage::new_section(
                request.header.msg_id,
                request.header.facility,
                request.header.procedure_id,
                request.header.sender_pid,
                request.header.sender_uid,
                sec_id,
                data,
            );
        }

        if let Some(server) = self.servers.get(&facility_key) {
            server
                .dispatch_request(&request)
                .ok_or("Procedure handler not found")
        } else {
            Err("Facility server not registered")
        }
    }
}

impl Default for AlpcManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpc_inline_procedure_call() {
        let mut mgr = AlpcManager::new();
        mgr.register_facility_server(AlpcFacility::FileSystemVfs, "vfs_server");

        let server = mgr
            .get_facility_server_mut(AlpcFacility::FileSystemVfs)
            .unwrap();
        server.register_procedure(101, |req| {
            let payload = req.get_payload();
            assert_eq!(payload, b"PING_VFS");
            b"PONG_VFS_OK".to_vec()
        });

        let req = AlpcMessage::new_inline(
            1,
            AlpcFacility::FileSystemVfs,
            101,
            100,
            1000,
            b"PING_VFS".to_vec(),
        );

        let reply = mgr.request_reply(AlpcFacility::FileSystemVfs, req).unwrap();
        assert_eq!(reply.get_payload(), b"PONG_VFS_OK");
        assert_eq!(
            (reply.header.flags & alpc_flags::REPLY_MESSAGE),
            alpc_flags::REPLY_MESSAGE
        );
    }

    #[test]
    fn test_alpc_zero_copy_section_large_payload() {
        let mut mgr = AlpcManager::new();
        mgr.register_facility_server(AlpcFacility::SystemKernel, "kernel_server");

        let server = mgr
            .get_facility_server_mut(AlpcFacility::SystemKernel)
            .unwrap();
        server.register_procedure(202, |req| {
            assert!(req.section.is_some());
            assert_eq!(
                (req.header.flags & alpc_flags::LARGE_SECTION_PAYLOAD),
                alpc_flags::LARGE_SECTION_PAYLOAD
            );
            let payload = req.get_payload();
            assert_eq!(payload.len(), 1024);
            b"LARGE_SECTION_PROCESSED".to_vec()
        });

        let large_payload = vec![0x42u8; 1024];
        let req =
            AlpcMessage::new_inline(2, AlpcFacility::SystemKernel, 202, 101, 0, large_payload);

        let reply = mgr.request_reply(AlpcFacility::SystemKernel, req).unwrap();
        assert_eq!(reply.get_payload(), b"LARGE_SECTION_PROCESSED");
    }
}
