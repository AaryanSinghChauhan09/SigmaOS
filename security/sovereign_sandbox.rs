/// SigmaOS: Sovereign Capability-based Sandboxing Framework
/// Built in Rust — #![no_std], no alloc, no external dependencies.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaBool = bool;
type SigmaI32 = i32;

pub const SIGMA_OK: SigmaI32 = 0;
pub const SIGMA_ERR_DENIED: SigmaI32 = -1;

#[derive(Clone, Copy, PartialEq)]
pub enum SandboxCapability {
    ReadFilesystem = 1 << 0,
    WriteFilesystem = 1 << 1,
    NetworkSend = 1 << 2,
    NetworkReceive = 1 << 3,
    ProcessSpawn = 1 << 4,
    SyscallOverride = 1 << 5,
}

pub struct Sandbox {
    pub process_id: SigmaU32,
    pub capabilities: SigmaU32, // Bitmask of enabled capabilities
    pub hardened: SigmaBool,
}

impl Sandbox {
    pub const fn new(pid: SigmaU32) -> Self {
        Sandbox {
            process_id: pid,
            capabilities: 0,
            hardened: true,
        }
    }

    pub fn grant(&mut self, cap: SandboxCapability) {
        self.capabilities |= cap as SigmaU32;
    }

    pub fn revoke(&mut self, cap: SandboxCapability) {
        self.capabilities &= !(cap as SigmaU32);
    }

    pub fn has_capability(&self, cap: SandboxCapability) -> SigmaBool {
        (self.capabilities & (cap as SigmaU32)) != 0
    }

    pub fn enforce(&self, cap: SandboxCapability) -> SigmaI32 {
        if self.has_capability(cap) {
            SIGMA_OK
        } else {
            SIGMA_ERR_DENIED
        }
    }
}

pub trait SecurityPolicy {
    fn validate_action(&self, sandbox: &Sandbox, action: SandboxCapability) -> SigmaBool;
}

pub struct StrictPolicy;

impl SecurityPolicy for StrictPolicy {
    fn validate_action(&self, sandbox: &Sandbox, action: SandboxCapability) -> SigmaBool {
        if sandbox.hardened {
            sandbox.has_capability(action)
        } else {
            true // Hardening disabled (warning state)
        }
    }
}
