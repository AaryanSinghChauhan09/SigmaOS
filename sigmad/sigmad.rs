//! sigmad — SigmaOS System Daemon (Pure Rust, no_std-compatible subset)
//! Replaces the Go microservice stubs (api-gateway, watchdog, telemetry, etc.)
//! Provides: service watchdog, health monitoring, power management stubs.
//! Uses no external crates — pure core Rust primitives.

#![allow(dead_code)]

// ── Daemon Service Registry ──────────────────────────────────────────────

pub const MAX_SERVICES: usize = 32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceState {
    Stopped,
    Running,
    Crashed,
    Restarting,
}

#[derive(Clone, Copy)]
pub struct ServiceEntry {
    pub name: [u8; 32],
    pub state: ServiceState,
    pub pid: u32,
    pub restart_count: u8,
}

impl ServiceEntry {
    pub const fn empty() -> Self {
        Self {
            name: [0u8; 32],
            state: ServiceState::Stopped,
            pid: 0,
            restart_count: 0,
        }
    }
}

pub struct ServiceRegistry {
    pub entries: [ServiceEntry; MAX_SERVICES],
    pub count: usize,
}

impl ServiceRegistry {
    pub const fn new() -> Self {
        Self {
            entries: [ServiceEntry::empty(); MAX_SERVICES],
            count: 0,
        }
    }

    pub fn register(&mut self, name: &[u8]) -> Result<usize, &'static str> {
        if self.count >= MAX_SERVICES {
            return Err("Registry full");
        }
        let idx = self.count;
        let copy_len = name.len().min(31);
        self.entries[idx].name[..copy_len].copy_from_slice(&name[..copy_len]);
        self.entries[idx].state = ServiceState::Stopped;
        self.count += 1;
        Ok(idx)
    }

    pub fn set_state(&mut self, idx: usize, state: ServiceState) -> Result<(), &'static str> {
        if idx >= self.count {
            return Err("Invalid service index");
        }
        if state == ServiceState::Crashed {
            self.entries[idx].restart_count =
                self.entries[idx].restart_count.saturating_add(1);
        }
        self.entries[idx].state = state;
        Ok(())
    }

    /// Watchdog: inspect all services and flag crashed ones for restart.
    pub fn watchdog_tick(&mut self) -> usize {
        let mut flagged = 0usize;
        for i in 0..self.count {
            if self.entries[i].state == ServiceState::Crashed
                && self.entries[i].restart_count < 5
            {
                self.entries[i].state = ServiceState::Restarting;
                flagged += 1;
            }
        }
        flagged
    }
}

// ── Power Management Stub ────────────────────────────────────────────────

#[repr(u8)]
pub enum PowerEvent {
    Suspend  = 0,
    Resume   = 1,
    Shutdown = 2,
    Reboot   = 3,
}

pub fn handle_power_event(event: PowerEvent) -> &'static str {
    match event {
        PowerEvent::Suspend  => "[sigmad/power] System suspending...",
        PowerEvent::Resume   => "[sigmad/power] System resuming...",
        PowerEvent::Shutdown => "[sigmad/power] Initiating shutdown sequence...",
        PowerEvent::Reboot   => "[sigmad/power] Rebooting system...",
    }
}

// ── Telemetry Stub (Opt-in, Zero-allocation ring buffer) ────────────────

pub const TELEMETRY_BUF_SIZE: usize = 128;

pub struct TelemetryRing {
    buf: [[u8; 64]; TELEMETRY_BUF_SIZE],
    head: usize,
    count: usize,
}

impl TelemetryRing {
    pub const fn new() -> Self {
        Self {
            buf: [[0u8; 64]; TELEMETRY_BUF_SIZE],
            head: 0,
            count: 0,
        }
    }

    pub fn record(&mut self, event: &[u8]) {
        let idx = self.head % TELEMETRY_BUF_SIZE;
        let copy_len = event.len().min(64);
        self.buf[idx][..copy_len].copy_from_slice(&event[..copy_len]);
        self.head = self.head.wrapping_add(1);
        if self.count < TELEMETRY_BUF_SIZE {
            self.count += 1;
        }
    }
}
