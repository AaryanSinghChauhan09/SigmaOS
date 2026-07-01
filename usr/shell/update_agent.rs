// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Update Agent (Rust, no_std)
//! =========================================================================
//! Replaces: usr/update-agent.cpp
//!
//! OOP Design:
//!   - UpdateAgent struct: manages channel, version, and update state.
//!   - Communicates with sigpkg over the Sovereign Syscall IPC.
//!   - No network stdlib: raw frame dispatch via Sovereign net ABI.
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK:    SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

type U32 = u32;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum UpdateChannel {
    Stable  = 0,
    Testing = 1,
    Nightly = 2,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum AgentState {
    Idle      = 0,
    Checking  = 1,
    Upgrading = 2,
    Done      = 3,
    Error     = 4,
}

// ── UpdateAgent Struct ─────────────────────────────────────────────────────

pub struct UpdateAgent {
    channel:          UpdateChannel,
    state:            AgentState,
    current_version:  U32,   // packed major.minor.patch
    pending_version:  U32,
}

impl UpdateAgent {
    pub const fn new() -> Self {
        UpdateAgent {
            channel:         UpdateChannel::Stable,
            state:           AgentState::Idle,
            current_version: 0x000F_0000, // v15.0.0
            pending_version: 0,
        }
    }

    pub fn set_channel(&mut self, ch: UpdateChannel) {
        self.channel = ch;
    }

    /// Trigger an update check (stubs IPC to sigpkg manifests).
    pub fn check_updates(&mut self) -> SigmaStatus {
        if self.state != AgentState::Idle { return SIGMA_ERROR; }
        self.state = AgentState::Checking;
        // TODO: dispatch IPC to sovereign net manifest endpoint
        // Simulate: no update found → return to Idle
        self.state = AgentState::Idle;
        SIGMA_OK
    }

    /// Apply a pending update.
    pub fn apply_update(&mut self) -> SigmaStatus {
        if self.pending_version == 0 { return SIGMA_ERROR; }
        self.state = AgentState::Upgrading;
        // TODO: invoke sigpkg_install via IPC
        self.current_version = self.pending_version;
        self.pending_version = 0;
        self.state = AgentState::Done;
        SIGMA_OK
    }

    pub fn current_state(&self) -> AgentState { self.state }
    pub fn current_ver(&self) -> U32 { self.current_version }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_UPDATE_AGENT: UpdateAgent = UpdateAgent::new();

// ── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn update_agent_set_channel(ch: u8) -> SigmaStatus {
    let channel = match ch {
        0 => UpdateChannel::Stable,
        1 => UpdateChannel::Testing,
        _ => UpdateChannel::Nightly,
    };
    G_UPDATE_AGENT.set_channel(channel);
    SIGMA_OK
}

#[no_mangle]
pub unsafe extern "C" fn update_agent_check() -> SigmaStatus {
    G_UPDATE_AGENT.check_updates()
}

#[no_mangle]
pub unsafe extern "C" fn update_agent_apply() -> SigmaStatus {
    G_UPDATE_AGENT.apply_update()
}

#[no_mangle]
pub unsafe extern "C" fn update_agent_version() -> U32 {
    G_UPDATE_AGENT.current_ver()
}
