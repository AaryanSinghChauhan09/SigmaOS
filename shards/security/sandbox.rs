/// shards/security/sandbox.rs — Rust sandbox enforcement
/// Zero-trust capability model: each shard runs in an isolated namespace
/// with explicitly granted capabilities. No capability = no access.

use core::sync::atomic::{AtomicU32, Ordering};

// ── Capability bitfield ────────────────────────────────────────────────────
pub const CAP_NONE:         u32 = 0;
pub const CAP_READ_MEM:     u32 = 1 << 0;
pub const CAP_WRITE_MEM:    u32 = 1 << 1;
pub const CAP_EXEC:         u32 = 1 << 2;
pub const CAP_NET:          u32 = 1 << 3;
pub const CAP_FS_READ:      u32 = 1 << 4;
pub const CAP_FS_WRITE:     u32 = 1 << 5;
pub const CAP_IPC:          u32 = 1 << 6;
pub const CAP_SYSCALL:      u32 = 1 << 7;
pub const CAP_SOVEREIGN:    u32 = 0xFFFF_FFFF; // root — all capabilities

// ── Violation counter (lock-free) ─────────────────────────────────────────
static VIOLATIONS: AtomicU32 = AtomicU32::new(0);

// ── Sandbox ───────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Sandbox {
    pub id:           u32,
    pub name:         &'static str,
    pub capabilities: u32,
    pub trust_level:  u8,    // 0=untrusted, 128=system, 255=sovereign
    pub max_mem_kb:   u32,   // memory ceiling in KB
    pub max_cycles:   u64,   // instruction budget
}

#[derive(Debug)]
pub enum SandboxError {
    CapabilityDenied { required: u32, granted: u32 },
    MemoryExceeded   { limit_kb: u32 },
    CyclesExceeded   { limit: u64 },
    Untrusted,
}

impl Sandbox {
    pub const fn new(id: u32, name: &'static str) -> Self {
        Self {
            id, name,
            capabilities: CAP_NONE,
            trust_level:  0,
            max_mem_kb:   4096,
            max_cycles:   1_000_000,
        }
    }

    pub const fn with_caps(mut self, caps: u32) -> Self { self.capabilities = caps; self }
    pub const fn with_trust(mut self, t: u8) -> Self { self.trust_level = t; self }
    pub const fn with_mem(mut self, kb: u32) -> Self { self.max_mem_kb = kb; self }

    /// Check if a capability is granted
    pub fn check(&self, required: u32) -> Result<(), SandboxError> {
        if self.capabilities & required == required {
            Ok(())
        } else {
            VIOLATIONS.fetch_add(1, Ordering::Relaxed);
            Err(SandboxError::CapabilityDenied {
                required,
                granted: self.capabilities,
            })
        }
    }

    /// Enforce memory ceiling
    pub fn check_mem(&self, requested_kb: u32) -> Result<(), SandboxError> {
        if requested_kb <= self.max_mem_kb { Ok(()) }
        else {
            VIOLATIONS.fetch_add(1, Ordering::Relaxed);
            Err(SandboxError::MemoryExceeded { limit_kb: self.max_mem_kb })
        }
    }

    /// Enforce cycle budget
    pub fn check_cycles(&self, used: u64) -> Result<(), SandboxError> {
        if used <= self.max_cycles { Ok(()) }
        else {
            VIOLATIONS.fetch_add(1, Ordering::Relaxed);
            Err(SandboxError::CyclesExceeded { limit: self.max_cycles })
        }
    }

    pub fn grant(&mut self, cap: u32) { self.capabilities |= cap; }
    pub fn revoke(&mut self, cap: u32) { self.capabilities &= !cap; }
    pub fn has(&self, cap: u32) -> bool { self.capabilities & cap == cap }
}

// ── Built-in sandboxes ────────────────────────────────────────────────────
pub fn sandbox_wasm() -> Sandbox {
    Sandbox::new(1, "wasm-runtime")
        .with_caps(CAP_READ_MEM | CAP_WRITE_MEM | CAP_EXEC)
        .with_trust(64)
        .with_mem(65536)
}

pub fn sandbox_plugin() -> Sandbox {
    Sandbox::new(2, "plugin")
        .with_caps(CAP_READ_MEM | CAP_IPC)
        .with_trust(32)
        .with_mem(1024)
}

pub fn sandbox_kernel() -> Sandbox {
    Sandbox::new(0, "kernel")
        .with_caps(CAP_SOVEREIGN)
        .with_trust(255)
        .with_mem(u32::MAX)
}

// ── Global violation stats (C FFI) ────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_sandbox_violations() -> u32 {
    VIOLATIONS.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_check() {
        let mut sb = Sandbox::new(99, "test");
        sb.grant(CAP_NET);
        assert!(sb.check(CAP_NET).is_ok());
        assert!(sb.check(CAP_FS_WRITE).is_err());
    }

    #[test]
    fn test_revoke() {
        let mut sb = Sandbox::new(99, "test");
        sb.grant(CAP_NET | CAP_IPC);
        sb.revoke(CAP_NET);
        assert!(!sb.has(CAP_NET));
        assert!(sb.has(CAP_IPC));
    }

    #[test]
    fn test_mem_ceiling() {
        let sb = Sandbox::new(1, "t").with_mem(64);
        assert!(sb.check_mem(60).is_ok());
        assert!(sb.check_mem(100).is_err());
    }

    #[test]
    fn test_sovereign_has_all() {
        let sb = sandbox_kernel();
        assert!(sb.check(CAP_NET | CAP_FS_WRITE | CAP_SYSCALL).is_ok());
    }
}
