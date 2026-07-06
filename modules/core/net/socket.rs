/// SigmaOS — modules/core/net/socket.rs
/// Unified Socket API (similar to BSD sockets but simplified for no_std).
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaUsize = usize;

pub const AF_INET:     SigmaI32 = 2;
pub const SOCK_STREAM: SigmaI32 = 1; // TCP
pub const SOCK_DGRAM:  SigmaI32 = 2; // UDP

// External links to TCP / UDP specific functions
extern "C" {
    fn tcp_alloc() -> SigmaI32;
    fn tcp_set_state(idx: SigmaI32, state: u8);
}

// ─── Socket Dispatch ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_net_socket(domain: SigmaI32, kind: SigmaI32, proto: SigmaI32) -> SigmaI32 {
    if domain != AF_INET { return -97; } // EAFNOSUPPORT
    
    if kind == SOCK_STREAM {
        // Allocate TCP TCB
        let idx = tcp_alloc();
        if idx >= 0 {
            // Map the TCP TCB index to a global Socket Descriptor table (simplified here)
            // In full integration this returns an FD that VFS also understands.
            return idx; 
        }
        return -12; // ENOMEM
    } else if kind == SOCK_DGRAM {
        return -38; // UDP not implemented in this phase
    }
    
    -94 // ESOCKTNOSUPPORT
}

#[no_mangle]
pub unsafe extern "C" fn sigma_net_bind(sock: SigmaI32, addr: *const u8, addrlen: SigmaUsize) -> SigmaI32 {
    // Requires CAP_NET_BIND if port < 1024
    if sock < 0 { return -9; } // EBADF
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_net_connect(sock: SigmaI32, addr: *const u8, addrlen: SigmaUsize) -> SigmaI32 {
    if sock < 0 { return -9; }
    
    // Transition TCP state to SynSent
    tcp_set_state(sock, 2 /* SynSent */);
    
    0
}
