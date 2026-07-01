/// SigmaOS: Sovereign Linux Security Module (LSM) Parity Hooks
/// Built in Rust — #![no_std], no alloc, no external dependencies.
/// Implements hooks for mandatory access control (MAC) over processes and filesystem operations.

#![no_std]
#![allow(dead_code)]

use crate::sovereign_sandbox::{Sandbox, SandboxCapability};

#[path = "sovereign_sandbox.rs"]
mod sovereign_sandbox;

type SigmaU32 = u32;
type SigmaBool = bool;
type SigmaI32 = i32;

pub const SIGMA_OK: SigmaI32 = 0;
pub const SIGMA_ERR_DENIED: SigmaI32 = -1;

pub trait SecurityModule {
    fn file_open(&self, sandbox: &Sandbox, flags: SigmaU32) -> SigmaI32;
    fn process_spawn(&self, sandbox: &Sandbox, target_pid: SigmaU32) -> SigmaI32;
    fn socket_connect(&self, sandbox: &Sandbox) -> SigmaI32;
}

pub struct SovereignMac;

impl SecurityModule for SovereignMac {
    fn file_open(&self, sandbox: &Sandbox, flags: SigmaU32) -> SigmaI32 {
        let is_write = (flags & 2) != 0;
        if is_write {
            sandbox.enforce(SandboxCapability::WriteFilesystem)
        } else {
            sandbox.enforce(SandboxCapability::ReadFilesystem)
        }
    }

    fn process_spawn(&self, sandbox: &Sandbox, _target_pid: SigmaU32) -> SigmaI32 {
        sandbox.enforce(SandboxCapability::ProcessSpawn)
    }

    fn socket_connect(&self, sandbox: &Sandbox) -> SigmaI32 {
        sandbox.enforce(SandboxCapability::NetworkSend)
    }
}
