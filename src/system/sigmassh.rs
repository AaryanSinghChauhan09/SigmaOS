// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaSSH - System SSH Server
//!
//! SigmaSSH provides system SSH server functionality for SigmaOS, inspired by OpenSSH from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// SSH connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SshConnectionState {
    /// Authenticating
    Authenticating,
    /// Authenticated
    Authenticated,
    /// Established
    Established,
    /// Closed
    Closed,
}

impl fmt::Display for SshConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SshConnectionState::Authenticating => write!(f, "authenticating"),
            SshConnectionState::Authenticated => write!(f, "authenticated"),
            SshConnectionState::Established => write!(f, "established"),
            SshConnectionState::Closed => write!(f, "closed"),
        }
    }
}

/// SSH authentication method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SshAuthMethod {
    /// Password
    Password,
    /// Public key
    PublicKey,
    /// Keyboard-interactive
    KeyboardInteractive,
}

impl fmt::Display for SshAuthMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SshAuthMethod::Password => write!(f, "password"),
            SshAuthMethod::PublicKey => write!(f, "publickey"),
            SshAuthMethod::KeyboardInteractive => write!(f, "keyboard-interactive"),
        }
    }
}

/// SSH connection
#[derive(Debug, Clone)]
pub struct SshConnection {
    /// Connection ID
    pub connection_id: String,
    /// Client address
    pub client_address: String,
    /// Client port
    pub client_port: u16,
    /// Username
    pub username: String,
    /// Authentication method
    pub auth_method: SshAuthMethod,
    /// State
    pub state: SshConnectionState,
}

impl SshConnection {
    /// Create a new SSH connection
    pub fn new(
        connection_id: String,
        client_address: String,
        client_port: u16,
        username: String,
    ) -> Self {
        Self {
            connection_id,
            client_address,
            client_port,
            username,
            auth_method: SshAuthMethod::Password,
            state: SshConnectionState::Authenticating,
        }
    }

    /// Set authentication method
    pub fn with_auth_method(mut self, auth_method: SshAuthMethod) -> Self {
        self.auth_method = auth_method;
        self
    }

    /// Set state
    pub fn with_state(mut self, state: SshConnectionState) -> Self {
        self.state = state;
        self
    }
}

/// SigmaSSH - System SSH Server
pub struct SigmaSSH {
    /// SSH connections (connection_id -> connection)
    connections: BTreeMap<String, SshConnection>,
    /// Running state
    running: bool,
    /// Connection counter
    connection_counter: u32,
}

impl SigmaSSH {
    /// Create a new SigmaSSH instance
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            running: false,
            connection_counter: 0,
        }
    }

    /// Start the SSH server
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the SSH server
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add an SSH connection
    pub fn add_connection(&mut self, connection: SshConnection) -> Result<(), String> {
        if self.connections.contains_key(&connection.connection_id) {
            return Err(format!(
                "Connection '{}' already exists",
                connection.connection_id
            ));
        }

        self.connections
            .insert(connection.connection_id.clone(), connection);
        self.connection_counter += 1;
        Ok(())
    }

    /// Remove an SSH connection
    pub fn remove_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if self.connections.remove(connection_id).is_none() {
            return Err(format!("Connection '{}' not found", connection_id));
        }
        Ok(())
    }

    /// Authenticate a connection
    pub fn authenticate_connection(
        &mut self,
        connection_id: &str,
        auth_method: SshAuthMethod,
    ) -> Result<(), String> {
        if !self.running {
            return Err("SigmaSSH not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.auth_method = auth_method;
                connection.state = SshConnectionState::Authenticated;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Establish a connection
    pub fn establish_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaSSH not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = SshConnectionState::Established;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Close a connection
    pub fn close_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaSSH not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = SshConnectionState::Closed;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Get an SSH connection
    pub fn get_connection(&self, connection_id: &str) -> Option<&SshConnection> {
        self.connections.get(connection_id)
    }

    /// List all connections
    pub fn list_connections(&self) -> Vec<&SshConnection> {
        self.connections.values().collect()
    }

    /// List connections by state
    pub fn list_by_state(&self, state: SshConnectionState) -> Vec<&SshConnection> {
        self.connections
            .values()
            .filter(|c| c.state == state)
            .collect()
    }

    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Get SSH server statistics
    pub fn ssh_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_connections".to_string(),
            self.connection_count().to_string(),
        );
        stats.insert(
            "established_connections".to_string(),
            self.list_by_state(SshConnectionState::Established)
                .len()
                .to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaSSH {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaSSH instance (placeholder for actual global state)
static mut SIGMASSH_INSTANCE: Option<SigmaSSH> = None;

/// Initialize the global SigmaSSH instance
pub fn init_sigmassh() {
    unsafe {
        SIGMASSH_INSTANCE = Some(SigmaSSH::new());
    }
}

/// Get the global SigmaSSH instance
pub fn get_sigmassh() -> Option<&'static mut SigmaSSH> {
    unsafe { SIGMASSH_INSTANCE.as_mut() }
}

/// Convenience function to authenticate a connection
pub fn authenticate_ssh_connection(
    connection_id: &str,
    auth_method: SshAuthMethod,
) -> Result<(), String> {
    if let Some(ssh) = get_sigmassh() {
        ssh.authenticate_connection(connection_id, auth_method)
    } else {
        Err("SigmaSSH not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_connection_state_display() {
        assert_eq!(
            SshConnectionState::Authenticating.to_string(),
            "authenticating"
        );
        assert_eq!(SshConnectionState::Established.to_string(), "established");
    }

    #[test]
    fn test_ssh_auth_method_display() {
        assert_eq!(SshAuthMethod::Password.to_string(), "password");
        assert_eq!(SshAuthMethod::PublicKey.to_string(), "publickey");
    }

    #[test]
    fn test_ssh_connection() {
        let connection = SshConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        )
        .with_auth_method(SshAuthMethod::PublicKey)
        .with_state(SshConnectionState::Established);

        assert_eq!(connection.connection_id, "conn1");
        assert_eq!(connection.auth_method, SshAuthMethod::PublicKey);
        assert_eq!(connection.state, SshConnectionState::Established);
    }

    #[test]
    fn test_sigmassh() {
        let mut ssh = SigmaSSH::new();
        assert!(!ssh.is_running());
        ssh.start();
        assert!(ssh.is_running());
        ssh.stop();
        assert!(!ssh.is_running());
    }

    #[test]
    fn test_sigmassh_add_connection() {
        let mut ssh = SigmaSSH::new();

        let connection = SshConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        );

        assert!(ssh.add_connection(connection).is_ok());
        assert_eq!(ssh.connection_count(), 1);
    }

    #[test]
    fn test_sigmassh_authenticate_connection() {
        let mut ssh = SigmaSSH::new();
        ssh.start();

        let connection = SshConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        );

        ssh.add_connection(connection).unwrap();
        assert!(ssh
            .authenticate_connection("conn1", SshAuthMethod::PublicKey)
            .is_ok());

        let conn = ssh.get_connection("conn1").unwrap();
        assert_eq!(conn.state, SshConnectionState::Authenticated);
    }

    #[test]
    fn test_sigmassh_establish_connection() {
        let mut ssh = SigmaSSH::new();
        ssh.start();

        let connection = SshConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        )
        .with_state(SshConnectionState::Authenticated);

        ssh.add_connection(connection).unwrap();
        assert!(ssh.establish_connection("conn1").is_ok());

        let conn = ssh.get_connection("conn1").unwrap();
        assert_eq!(conn.state, SshConnectionState::Established);
    }

    #[test]
    fn test_sigmassh_list_by_state() {
        let mut ssh = SigmaSSH::new();

        let connection = SshConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        )
        .with_state(SshConnectionState::Established);

        ssh.add_connection(connection).unwrap();

        let connections = ssh.list_by_state(SshConnectionState::Established);
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_sigmassh_stats() {
        let mut ssh = SigmaSSH::new();

        let connection = SshConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        )
        .with_state(SshConnectionState::Established);

        ssh.add_connection(connection).unwrap();

        let stats = ssh.ssh_stats();
        assert_eq!(stats.get("total_connections"), Some(&"1".to_string()));
        assert_eq!(stats.get("established_connections"), Some(&"1".to_string()));
    }

    #[test]
    fn test_authenticate_ssh_connection_convenience() {
        init_sigmassh();

        let mut ssh = get_sigmassh().unwrap();
        ssh.start();

        let connection = SshConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        );

        ssh.add_connection(connection).unwrap();

        let result = authenticate_ssh_connection("conn1", SshAuthMethod::PublicKey);
        assert!(result.is_ok());
    }
}
