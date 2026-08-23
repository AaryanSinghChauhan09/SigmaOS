// SPDX-License-Identifier: MIT
// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model, delegation, auditing, and time-limited tokens.

#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityToken {
    pub id: usize,
    pub permissions: u64,
}

impl CapabilityToken {
    pub fn from_bits(bits: u64) -> Self {
        CapabilityToken {
            id: bits as usize,
            permissions: bits,
        }
    }

    pub fn new() -> Self {
        CapabilityToken {
            id: 0,
            permissions: 0,
        }
    }

    pub fn with_permission(mut self, permission: u64) -> Self {
        self.permissions |= permission;
        self
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn has_permission(&self, permission: Permission) -> bool {
        if self.is_revoked {
            return false;
        }
        (self.bits & (1 << permission as u64)) != 0
    }

    pub fn allow_network(mut self, _proto: &str, port: u16) -> Self {
        self.bits |= 1 << (Permission::NetworkTcp as u64);
        if port != 0 {
            self.allowed_ports.push(port);
        }
        self
    }

    pub fn allow_read(mut self, path: &str) -> Self {
        self.bits |= 1 << (Permission::FileRead as u64);
        self.allowed_paths.push(String::from(path));
        self
    }

    pub fn allow_write(mut self, path: &str) -> Self {
        self.bits |= 1 << (Permission::FileWrite as u64);
        self.allowed_paths.push(String::from(path));
        self
    }

    pub fn allow_exec(mut self) -> Self {
        self.bits |= 1 << (Permission::ProcessExec as u64);
        self
    }

    pub fn allow_ipc(mut self) -> Self {
        self.bits |= 1 << (Permission::Ipc as u64);
        self
    }

    pub fn allow_network(mut self, _protocol: &str, _port: u16) -> Self {
        self.permissions |= Permission::NetworkTcp as u64;
        self
    }

    pub fn allow_read(mut self) -> Self {
        self.permissions |= Permission::Read as u64 | Permission::FileRead as u64;
        self
    }

    pub fn allow_write(mut self) -> Self {
        self.permissions |= Permission::Write as u64 | Permission::FileWrite as u64;
        self
    }

    pub fn allow_exec(mut self) -> Self {
        self.permissions |= Permission::Execute as u64 | Permission::ProcessExec as u64;
        self
    }

    pub fn allow_ipc(mut self) -> Self {
        self.permissions |= Permission::Ipc as u64;
        self
    }

    pub fn grant_permission(&mut self, perm: Permission) {
        self.permissions |= perm as u64;
    }

    pub fn revoke_permission(&mut self, perm: Permission) {
        self.permissions &= !(perm as u64);
    }

    pub fn has_permission(&self, perm: Permission) -> bool {
        (self.permissions & (perm as u64)) != 0
    }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Read = 1,
    Write = 2,
    Execute = 4,
    Admin = 8,
    NetworkTcp = 16,
    NetworkUdp = 32,
    FileRead = 64,
    FileWrite = 128,
    ProcessExec = 256,
    Ipc = 512,
}

pub struct CapabilityGate {
    pub required_permissions: u64,
}

impl CapabilityGate {
    pub fn new(required_permissions: u64) -> Self {
        Self { required_permissions }
    }

    pub fn set_capability(&mut self, perm: u64) {
        self.required_permissions |= perm;
    }

    pub fn check(&self, token: &CapabilityToken) -> bool {
        (token.permissions & self.required_permissions) == self.required_permissions
    }
}
