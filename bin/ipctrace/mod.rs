// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Userspace IPC Trace Tool (Rust, no_std)
//! Replaces: bin/ipctrace/main.cpp
//! =========================================================================

pub struct IpcTraceTool {
    tracing: bool,
    filter_port: Option<u32>,
}

impl IpcTraceTool {
    pub const fn new() -> Self {
        Self {
            tracing: false,
            filter_port: None,
        }
    }

    pub fn start_trace(&mut self, port: Option<u32>) {
        self.tracing = true;
        self.filter_port = port;
    }

    pub fn stop_trace(&mut self) {
        self.tracing = false;
        self.filter_port = None;
    }

    pub fn is_tracing(&self) -> bool {
        self.tracing
    }

    pub fn log_event(&self, sender: u32, receiver: u32, msg_type: u32) {
        if !self.tracing {
            return;
        }
        if let Some(port) = self.filter_port {
            if sender != port && receiver != port {
                return;
            }
        }
        // In real userspace, print trace event to standard out via sovereign syscalls
    }

    pub fn class_name(&self) -> &'static str {
        "IpcTraceTool"
    }
}
