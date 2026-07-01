// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Networking Stack (Rust, no_std)
//! Replaces: include/net/*.h, include/network/*.h, include/kernel/sigma_network.h
//! =========================================================================

const MAX_SOCKETS: usize = 512;

#[derive(Copy, Clone, PartialEq)]
pub enum Protocol {
    Tcp,
    Udp,
    Raw,
}

#[derive(Copy, Clone, PartialEq)]
pub enum SocketState {
    Closed,
    Listening,
    Connected,
    Closing,
}

#[derive(Copy, Clone)]
pub struct Socket {
    pub id: u32,
    pub proto: Protocol,
    pub state: SocketState,
    pub local_port: u16,
    pub remote_port: u16,
    pub local_ip: [u8; 4],
    pub remote_ip: [u8; 4],
}

impl Socket {
    pub const fn new(id: u32, proto: Protocol) -> Self {
        Self {
            id, proto, state: SocketState::Closed,
            local_port: 0, remote_port: 0,
            local_ip: [0; 4], remote_ip: [0; 4],
        }
    }

    pub fn class_name(&self) -> &'static str { "Socket" }
}

pub struct TcpIpStack {
    sockets: [Option<Socket>; MAX_SOCKETS],
    count: usize,
}

impl TcpIpStack {
    pub const fn new() -> Self {
        Self { sockets: [None; MAX_SOCKETS], count: 0 }
    }

    pub fn create_socket(&mut self, proto: Protocol) -> Option<u32> {
        if self.count >= MAX_SOCKETS { return None; }
        let id = self.count as u32;
        self.sockets[self.count] = Some(Socket::new(id, proto));
        self.count += 1;
        Some(id)
    }

    pub fn bind(&mut self, id: u32, port: u16) -> bool {
        for i in 0..self.count {
            if let Some(ref mut s) = self.sockets[i] {
                if s.id == id { s.local_port = port; return true; }
            }
        }
        false
    }

    pub fn class_name(&self) -> &'static str { "TcpIpStack" }
}

pub struct AetherFirewall {
    default_deny: bool,
}

impl AetherFirewall {
    pub const fn new(default_deny: bool) -> Self {
        Self { default_deny }
    }

    pub fn check_packet(&self, _src_port: u16, _dst_port: u16) -> bool {
        !self.default_deny
    }

    pub fn class_name(&self) -> &'static str { "AetherFirewall" }
}
