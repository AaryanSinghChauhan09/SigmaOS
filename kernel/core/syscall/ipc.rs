// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign IPC Router (Rust, no_std)
//! =========================================================================

type U32 = u32;

pub struct IpcRouter {
    messages_routed: U32,
}

impl IpcRouter {
    pub const fn new() -> Self {
        IpcRouter { messages_routed: 0 }
    }

    pub fn send_message(&mut self, dest_pid: U32, msg_ptr: *const u8, size: U32) -> i32 {
        self.messages_routed += 1;
        
        let _pid = dest_pid;
        let _ptr = msg_ptr;
        let _sz = size;

        // Stub for WFAE IPC message queue insertion
        0 // Success
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_IPC: IpcRouter = IpcRouter::new();

// ── C-ABI Exports (Replacing ipc.c and signal.c) ───────────────────────────

#[no_mangle]
pub unsafe extern "C" fn ipc_init_shard() {
    // Init if needed
}

#[no_mangle]
pub unsafe extern "C" fn ipc_send_message_shard(dest_pid: U32, msg_ptr: *const u8, size: U32) -> i32 {
    G_IPC.send_message(dest_pid, msg_ptr, size)
}

#[no_mangle]
pub unsafe extern "C" fn signal_send_shard(dest_pid: U32, signal: U32) -> i32 {
    // Treat signals as short IPC messages
    let sig_val = signal;
    G_IPC.send_message(dest_pid, &sig_val as *const U32 as *const u8, 4)
}
