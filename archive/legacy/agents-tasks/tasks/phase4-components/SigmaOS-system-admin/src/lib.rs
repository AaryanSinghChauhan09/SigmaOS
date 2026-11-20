//! SigmaOS System Admin
//!
//! System administration tools for SigmaOS. Provides a unified interface for
//! managing users, groups, services, and system configuration.
//!
//! # Modules
//!
//! - [`users`]: User and group management (create, modify, delete, list).
//! - [`services`]: Service inspection and control via the system service manager.

pub mod users;
pub mod services;

/// Initialize the system-admin subsystem.
///
/// Call once at startup to detect the service manager backend and
/// verify the necessary privileges for administrative operations.
pub fn init() {
    // TODO: detect service manager backend, verify privileges, load config
}
