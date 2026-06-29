// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Socket (Rust, no_std)
//! =========================================================================

type U32 = u32;

pub const SIGMA_PROTO_TCP: U32 = 1;
pub const SIGMA_PROTO_UDP: U32 = 2;
pub const SIGMA_PROTO_RAW: U32 = 3;

#[derive(Clone, Copy)]
pub struct SovereignSocket {
    pub id: U32,
    pub protocol: U32,
    pub active: bool,
}

impl SovereignSocket {
    pub const fn empty() -> Self {
        SovereignSocket {
            id: 0,
            protocol: 0,
            active: false,
        }
    }
}

pub struct SocketManager {
    sockets: [SovereignSocket; 128],
    next_id: U32,
}

impl SocketManager {
    pub const fn new() -> Self {
        SocketManager {
            sockets: [SovereignSocket::empty(); 128],
            next_id: 1,
        }
    }

    pub fn create_socket(&mut self, protocol: U32) -> U32 {
        let mut i = 0;
        while i < 128 {
            if !self.sockets[i].active {
                self.sockets[i].id = self.next_id;
                self.sockets[i].protocol = protocol;
                self.sockets[i].active = true;
                self.next_id += 1;
                return self.sockets[i].id;
            }
            i += 1;
        }
        0 // Error: no sockets available
    }

    pub fn close_socket(&mut self, id: U32) {
        let mut i = 0;
        while i < 128 {
            if self.sockets[i].id == id && self.sockets[i].active {
                self.sockets[i].active = false;
                return;
            }
            i += 1;
        }
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_SOCKET_MGR: SocketManager = SocketManager::new();

// ── C-ABI Exports (Replacing sigma_net_socket.cpp) ────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_net_socket_create(protocol: U32) -> U32 {
    G_SOCKET_MGR.create_socket(protocol)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_net_socket_close(id: U32) {
    G_SOCKET_MGR.close_socket(id);
}
