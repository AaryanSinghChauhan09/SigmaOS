//! Σ SigmaOS Core Orchestrator
//!
//! The lightweight shard manager and plugin loader at the heart of SigmaOS.
//! Responsibilities:
//!   - Register and discover shard plugins via a static dispatch table
//!   - Load shards in dependency order
//!   - Provide the Sovereign Event Bus for inter-shard messaging
//!   - Expose a C FFI surface for the C11 kernel to call into

#![cfg_attr(feature = "bare_metal", no_std)]

/// Shard lifecycle states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShardState {
    Unloaded,
    Loading,
    Active,
    Error,
    Suspended,
}

/// Every shard must implement this trait
pub trait SigmaShard: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str { "1.0.0" }
    fn init(&mut self) -> Result<(), &'static str>;
    fn shutdown(&mut self) -> Result<(), &'static str> { Ok(()) }
    fn health(&self) -> ShardState { ShardState::Active }
}

/// Event types on the Sovereign Event Bus
#[derive(Debug, Clone)]
pub enum SovereignEvent {
    ShardLoaded    { name: &'static str },
    ShardFailed    { name: &'static str, reason: &'static str },
    ConfigChanged  { key: &'static str },
    AutomationFire { action: &'static str },
    Custom         { tag: &'static str },
}

/// Core Orchestrator — owns the shard registry
pub struct SovereignOrchestrator {
    shards: heapless_shard_vec::ShardVec,
    event_log: [Option<SovereignEvent>; 64],
    event_head: usize,
}

/// Minimal Vec-like container avoiding heap allocation in bare-metal mode
mod heapless_shard_vec {
    use super::*;

    const MAX_SHARDS: usize = 64;

    pub struct ShardVec {
        items: [Option<ShardEntry>; MAX_SHARDS],
        count: usize,
    }

    pub struct ShardEntry {
        pub name:  &'static str,
        pub state: ShardState,
    }

    impl ShardVec {
        pub const fn new() -> Self {
            const NONE: Option<ShardEntry> = None;
            Self { items: [NONE; MAX_SHARDS], count: 0 }
        }
        pub fn register(&mut self, name: &'static str) -> bool {
            if self.count >= MAX_SHARDS { return false; }
            self.items[self.count] = Some(ShardEntry { name, state: ShardState::Unloaded });
            self.count += 1;
            true
        }
        pub fn set_state(&mut self, name: &'static str, state: ShardState) {
            for slot in self.items.iter_mut().flatten() {
                if slot.name == name { slot.state = state; return; }
            }
        }
        pub fn count(&self) -> usize { self.count }
        pub fn iter(&self) -> impl Iterator<Item = &ShardEntry> {
            self.items.iter().flatten()
        }
    }
}

impl SovereignOrchestrator {
    pub const fn new() -> Self {
        const NONE_EV: Option<SovereignEvent> = None;
        Self {
            shards: heapless_shard_vec::ShardVec::new(),
            event_log: [NONE_EV; 64],
            event_head: 0,
        }
    }

    /// Register a shard by name — called during boot
    pub fn register(&mut self, name: &'static str) -> bool {
        self.shards.register(name)
    }

    /// Mark a shard as active and publish a ShardLoaded event
    pub fn activate(&mut self, name: &'static str) {
        self.shards.set_state(name, ShardState::Active);
        self.publish(SovereignEvent::ShardLoaded { name });
    }

    /// Publish an event into the ring buffer
    pub fn publish(&mut self, event: SovereignEvent) {
        let idx = self.event_head % 64;
        self.event_log[idx] = Some(event);
        self.event_head = self.event_head.wrapping_add(1);
    }

    pub fn shard_count(&self) -> usize { self.shards.count() }

    /// Iterate registered shards for telemetry
    pub fn shard_names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.shards.iter().map(|e| e.name)
    }
}

// ── C FFI Surface ─────────────────────────────────────────────────────────────

static mut ORCHESTRATOR: SovereignOrchestrator = SovereignOrchestrator::new();

#[no_mangle]
pub extern "C" fn sigma_orchestrator_register(name: *const u8, len: usize) -> i32 {
    let name_str: &'static str = unsafe {
        let slice = core::slice::from_raw_parts(name, len);
        core::str::from_utf8_unchecked(slice)
    };
    if unsafe { ORCHESTRATOR.register(name_str) } { 0 } else { -1 }
}

#[no_mangle]
pub extern "C" fn sigma_orchestrator_shard_count() -> usize {
    unsafe { ORCHESTRATOR.shard_count() }
}

#[no_mangle]
pub extern "C" fn sigma_orchestrator_activate(name: *const u8, len: usize) {
    let name_str: &'static str = unsafe {
        let slice = core::slice::from_raw_parts(name, len);
        core::str::from_utf8_unchecked(slice)
    };
    unsafe { ORCHESTRATOR.activate(name_str) };
}

#[cfg(feature = "hosted")]
pub mod tests {
    use super::*;
    #[test]
    fn test_register_and_count() {
        let mut orc = SovereignOrchestrator::new();
        assert!(orc.register("S01_Genesis"));
        assert!(orc.register("S04_HAL"));
        assert_eq!(orc.shard_count(), 2);
    }
    #[test]
    fn test_activate_publishes_event() {
        let mut orc = SovereignOrchestrator::new();
        orc.register("S05_Memory");
        orc.activate("S05_Memory");
        assert_eq!(orc.event_head, 1);
    }
}
