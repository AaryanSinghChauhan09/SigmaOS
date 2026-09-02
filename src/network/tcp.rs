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
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// SigmaOS Network Stack - TCP Implementation
// Sovereign TCP/IP stack with capability-based access control

/// TCP state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    Closing,
    TimeWait,
    CloseWait,
    LastAck,
}

/// TCP connection
#[derive(Debug)]
pub struct TcpConnection {
    pub local_port: u16,
    pub remote_port: u16,
    pub remote_ip: [u8; 4],
    pub state: TcpState,
    pub sequence_number: u32,
    pub acknowledgment_number: u32,
    pub window_size: u16,
    pub capabilities: u64,
}

impl TcpConnection {
    pub fn new(local_port: u16, remote_port: u16, remote_ip: [u8; 4]) -> Self {
        Self {
            local_port,
            remote_port,
            remote_ip,
            state: TcpState::Closed,
            sequence_number: 0,
            acknowledgment_number: 0,
            window_size: 65535,
            capabilities: 0,
        }
    }

    pub fn set_capabilities(&mut self, capabilities: u64) {
        self.capabilities = capabilities;
    }

    pub fn has_capability(&self, capability: u64) -> bool {
        (self.capabilities & capability) != 0
    }
}

/// TCP segment
#[derive(Debug, Clone)]
pub struct TcpSegment {
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence_number: u32,
    pub acknowledgment_number: u32,
    pub flags: u8,
    pub window_size: u16,
    pub data: Vec<u8>,
}

impl TcpSegment {
    pub fn new(source_port: u16, destination_port: u16) -> Self {
        Self {
            source_port,
            destination_port,
            sequence_number: 0,
            acknowledgment_number: 0,
            flags: 0,
            window_size: 65535,
            data: Vec::new(),
        }
    }

    pub fn set_syn(&mut self) {
        self.flags |= 0x02;
    }

    pub fn set_ack(&mut self) {
        self.flags |= 0x10;
    }

    pub fn set_fin(&mut self) {
        self.flags |= 0x01;
    }

    pub fn set_rst(&mut self) {
        self.flags |= 0x04;
    }

    pub fn is_syn(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    pub fn is_ack(&self) -> bool {
        (self.flags & 0x10) != 0
    }

    pub fn is_fin(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    pub fn is_rst(&self) -> bool {
        (self.flags & 0x04) != 0
    }
}

/// TCP stack
pub struct TcpStack {
    connections: Vec<TcpConnection>,
    next_port: u16,
}

impl TcpStack {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            next_port: 1024,
        }
    }

    pub fn allocate_port(&mut self) -> u16 {
        let port = self.next_port;
        self.next_port = self.next_port.wrapping_add(1);
        if self.next_port < 1024 {
            self.next_port = 1024;
        }
        port
    }

    pub fn create_connection(&mut self, remote_port: u16, remote_ip: [u8; 4]) -> u16 {
        let local_port = self.allocate_port();
        let connection = TcpConnection::new(local_port, remote_port, remote_ip);
        self.connections.push(connection);
        local_port
    }

    pub fn get_connection(&self, local_port: u16) -> Option<&TcpConnection> {
        self.connections.iter().find(|c| c.local_port == local_port)
    }

    pub fn get_connection_mut(&mut self, local_port: u16) -> Option<&mut TcpConnection> {
        self.connections
            .iter_mut()
            .find(|c| c.local_port == local_port)
    }

    pub fn close_connection(&mut self, local_port: u16) {
        self.connections.retain(|c| c.local_port != local_port);
    }

    pub fn process_segment(&mut self, segment: TcpSegment) -> Result<(), TcpError> {
        let connection = self
            .get_connection_mut(segment.destination_port)
            .ok_or(TcpError::ConnectionNotFound)?;

        // Process based on current state
        match connection.state {
            TcpState::Listen => {
                if segment.is_syn() {
                    connection.state = TcpState::SynReceived;
                    connection.sequence_number = segment.sequence_number + 1;
                }
            }
            TcpState::SynSent => {
                if segment.is_syn() && segment.is_ack() {
                    connection.state = TcpState::Established;
                    connection.acknowledgment_number = segment.sequence_number + 1;
                }
            }
            TcpState::Established => {
                if segment.is_fin() {
                    connection.state = TcpState::CloseWait;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl Default for TcpStack {
    fn default() -> Self {
        Self::new()
    }
}

/// TCP errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpError {
    ConnectionNotFound,
    InvalidState,
    InvalidSegment,
    ConnectionRefused,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_stack_creation() {
        let stack = TcpStack::new();
        assert!(stack.connections.is_empty());
    }

    #[test]
    fn test_port_allocation() {
        let mut stack = TcpStack::new();
        let port1 = stack.allocate_port();
        let port2 = stack.allocate_port();
        assert_ne!(port1, port2);
    }

    #[test]
    fn test_connection_creation() {
        let mut stack = TcpStack::new();
        let local_port = stack.create_connection(80, [127, 0, 0, 1]);
        assert!(stack.get_connection(local_port).is_some());
    }

    #[test]
    fn test_segment_flags() {
        let mut segment = TcpSegment::new(1234, 80);
        segment.set_syn();
        segment.set_ack();
        assert!(segment.is_syn());
        assert!(segment.is_ack());
    }
}
