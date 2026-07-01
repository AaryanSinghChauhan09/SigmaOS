// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Network Stack Engine (Rust, no_std)
//! =========================================================================

use super::firewall::SovereignFirewall;

type U32 = u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NetConfig {
    pub enable_ipv6: bool,
    pub enable_firewall: bool,
    pub enable_ssl: bool,
}

pub struct SovereignNetStack {
    initialized: bool,
    packets_sent: U32,
    packets_received: U32,
    config: NetConfig,
    firewall: SovereignFirewall,
}

impl SovereignNetStack {
    pub const fn new() -> Self {
        SovereignNetStack {
            initialized: false,
            packets_sent: 0,
            packets_received: 0,
            config: NetConfig {
                enable_ipv6: true,
                enable_firewall: true,
                enable_ssl: true,
            },
            firewall: SovereignFirewall::new(),
        }
    }

    pub fn init(&mut self, config_ptr: *const NetConfig) {
        if !config_ptr.is_null() {
            unsafe {
                self.config = *config_ptr;
            }
        }
        self.initialized = true;
    }

    pub fn send_packet(&mut self, data: *const u8, len: U32) {
        if !self.initialized || data.is_null() || len == 0 {
            return;
        }

        // Simulating packet transmission logic (IPv4 / IPv6 headers)
        self.packets_sent += 1;
    }

    pub fn receive_packet(&mut self, buffer: *mut u8, len: *mut U32) {
        if !self.initialized || buffer.is_null() || len.is_null() {
            return;
        }

        let packet_len = unsafe { *len };

        if self.config.enable_firewall {
            if !self.firewall.inspect(buffer, packet_len) {
                // Drop packet
                unsafe { *len = 0; }
                return;
            }
        }

        // Process incoming datagram logic
        self.packets_received += 1;
    }

    pub fn report_stats(&self) {
        // Stats reporting logic placeholder for C-ABI
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_NETSTACK: SovereignNetStack = SovereignNetStack::new();

// ── C-ABI Exports (Replacing SovereignNetStack.cpp) ────────────────────────

#[no_mangle]
pub unsafe extern "C" fn net_stack_init(config: *const NetConfig) {
    G_NETSTACK.init(config);
}

#[no_mangle]
pub unsafe extern "C" fn net_send_packet(data: *const u8, len: U32) {
    G_NETSTACK.send_packet(data, len);
}

#[no_mangle]
pub unsafe extern "C" fn net_receive_packet(buffer: *mut u8, len: *mut U32) {
    G_NETSTACK.receive_packet(buffer, len);
}

#[no_mangle]
pub unsafe extern "C" fn net_report_stats() {
    G_NETSTACK.report_stats();
}
