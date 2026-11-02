//! SigmaOS System Layer
//!
//! Provides the init system, service manager, orchestration, and IPC
//! subsystems for SigmaOS.  All cross-crate types that would normally
//! come from `sigmaos-kernel`, `sigmaos-filesystem`, or
//! `sigmaos-security` are supplied as inline stubs so that this crate
//! compiles standalone during the Phase-2 scaffold.
//!
//! Design principles followed:
//! - Single Responsibility  : init / service-manager / IPC are separate modules
//! - Separation of Concerns : each module owns only its domain
//! - Fail-Fast              : errors are returned immediately, not silenced
//! - Race-Free Design       : no shared mutable global state; callers own instances
//! - SOLID / Open-Closed    : public traits allow extension without modification
//! - Efficient Scheduling   : dependency-ordered startup (topological sort)
//! - Deadlock Avoidance     : dependency graph is validated for cycles before use

// ─── lint policy (mirrors monorepo style) ────────────────────────────────────
#![allow(dead_code)]
#![allow(clippy::new_without_default)]

// ─── re-exports ──────────────────────────────────────────────────────────────
pub mod init;
pub mod service;
pub mod orchestration;
pub mod ipc;

pub use init::{InitSystem, SystemState};
pub use service::{Service, ServiceConfig, ServiceManager, ServiceState, ServiceStatus};
pub use orchestration::Orchestrator;
pub use ipc::{IpcChannel, IpcError, IpcMessage, IpcMessageKind};

// ─── inline stubs for cross-crate kernel / filesystem / security types ───────
//
// These stubs will be replaced with path-dependencies on sigmaos-kernel,
// sigmaos-filesystem, and sigmaos-security during Phase-3 workspace integration.

/// Minimal kernel process identifier (stub for sigmaos-kernel::Pid).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pid(pub u32);

/// Minimal filesystem error stub (for sigmaos-filesystem::FsError).
#[derive(Debug)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    IoError,
}

impl core::fmt::Display for FsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FsError::NotFound => write!(f, "filesystem: not found"),
            FsError::PermissionDenied => write!(f, "filesystem: permission denied"),
            FsError::IoError => write!(f, "filesystem: I/O error"),
        }
    }
}

/// Minimal security capability stub (for sigmaos-security::Capability).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    /// May manage system services.
    ServiceControl,
    /// May send IPC messages to any process.
    IpcUnrestricted,
    /// May change the system run-level.
    SystemAdmin,
}
