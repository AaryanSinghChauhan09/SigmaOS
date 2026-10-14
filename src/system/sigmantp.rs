// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaNTP - System NTP Client
//!
//! SigmaNTP provides system NTP client functionality for SigmaOS, inspired by ntpd/chrony from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// NTP server state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtpServerState {
    /// Candidate
    Candidate,
    /// Peer
    Peer,
    /// Server
    Server,
}

impl fmt::Display for NtpServerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NtpServerState::Candidate => write!(f, "candidate"),
            NtpServerState::Peer => write!(f, "peer"),
            NtpServerState::Server => write!(f, "server"),
        }
    }
}

/// NTP server
#[derive(Debug, Clone)]
pub struct NtpServer {
    /// Server ID
    pub server_id: String,
    /// Server address
    pub address: String,
    /// Port
    pub port: u16,
    /// State
    pub state: NtpServerState,
    /// Offset in milliseconds
    pub offset: i64,
    /// Stratum level
    pub stratum: u8,
}

impl NtpServer {
    /// Create a new NTP server
    pub fn new(server_id: String, address: String, port: u16) -> Self {
        Self {
            server_id,
            address,
            port,
            state: NtpServerState::Candidate,
            offset: 0,
            stratum: 0,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: NtpServerState) -> Self {
        self.state = state;
        self
    }

    /// Set offset
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    /// Set stratum
    pub fn with_stratum(mut self, stratum: u8) -> Self {
        self.stratum = stratum;
        self
    }
}

/// SigmaNTP - System NTP Client
pub struct SigmaNTP {
    /// NTP servers (server_id -> server)
    servers: BTreeMap<String, NtpServer>,
    /// Running state
    running: bool,
    /// Server counter
    server_counter: u32,
}

impl SigmaNTP {
    /// Create a new SigmaNTP instance
    pub fn new() -> Self {
        Self {
            servers: BTreeMap::new(),
            running: false,
            server_counter: 0,
        }
    }

    /// Start the NTP client
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the NTP client
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the client is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add an NTP server
    pub fn add_server(&mut self, server: NtpServer) -> Result<(), String> {
        if self.servers.contains_key(&server.server_id) {
            return Err(format!("Server '{}' already exists", server.server_id));
        }
        
        self.servers.insert(server.server_id.clone(), server);
        self.server_counter += 1;
        Ok(())
    }

    /// Remove an NTP server
    pub fn remove_server(&mut self, server_id: &str) -> Result<(), String> {
        if self.servers.remove(server_id).is_none() {
            return Err(format!("Server '{}' not found", server_id));
        }
        Ok(())
    }

    /// Sync time with NTP
    pub fn sync_time(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaNTP not running".to_string());
        }
        
        // In a real implementation, this would actually sync with NTP servers
        Ok(())
    }

    /// Get an NTP server
    pub fn get_server(&self, server_id: &str) -> Option<&NtpServer> {
        self.servers.get(server_id)
    }

    /// List all servers
    pub fn list_servers(&self) -> Vec<&NtpServer> {
        self.servers.values().collect()
    }

    /// List servers by state
    pub fn list_by_state(&self, state: NtpServerState) -> Vec<&NtpServer> {
        self.servers.values()
            .filter(|s| s.state == state)
            .collect()
    }

    /// Get server count
    pub fn server_count(&self) -> usize {
        self.servers.len()
    }

    /// Get NTP client statistics
    pub fn ntp_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_servers".to_string(), self.server_count().to_string());
        stats.insert("peer_servers".to_string(), self.list_by_state(NtpServerState::Peer).len().to_string());
        stats.insert("server_servers".to_string(), self.list_by_state(NtpServerState::Server).len().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaNTP {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNTP instance (placeholder for actual global state)
static mut SIGMANTP_INSTANCE: Option<SigmaNTP> = None;

/// Initialize the global SigmaNTP instance
pub fn init_sigmantp() {
    unsafe {
        SIGMANTP_INSTANCE = Some(SigmaNTP::new());
    }
}

/// Get the global SigmaNTP instance
pub fn get_sigmantp() -> Option<&'static mut SigmaNTP> {
    unsafe { SIGMANTP_INSTANCE.as_mut() }
}

/// Convenience function to sync time
pub fn sync_ntp_time() -> Result<(), String> {
    if let Some(ntp) = get_sigmantp() {
        ntp.sync_time()
    } else {
        Err("SigmaNTP not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ntp_server_state_display() {
        assert_eq!(NtpServerState::Candidate.to_string(), "candidate");
        assert_eq!(NtpServerState::Server.to_string(), "server");
    }

    #[test]
    fn test_ntp_server() {
        let server = NtpServer::new(
            "server1".to_string(),
            "pool.ntp.org".to_string(),
            123
        )
        .with_state(NtpServerState::Server)
        .with_offset(5)
        .with_stratum(2);

        assert_eq!(server.server_id, "server1");
        assert_eq!(server.state, NtpServerState::Server);
        assert_eq!(server.offset, 5);
    }

    #[test]
    fn test_sigmantp() {
        let mut ntp = SigmaNTP::new();
        assert!(!ntp.is_running());
        ntp.start();
        assert!(ntp.is_running());
        ntp.stop();
        assert!(!ntp.is_running());
    }

    #[test]
    fn test_sigmantp_add_server() {
        let mut ntp = SigmaNTP::new();
        
        let server = NtpServer::new(
            "server1".to_string(),
            "pool.ntp.org".to_string(),
            123
        );
        
        assert!(ntp.add_server(server).is_ok());
        assert_eq!(ntp.server_count(), 1);
    }

    #[test]
    fn test_sigmantp_sync_time() {
        let mut ntp = SigmaNTP::new();
        ntp.start();
        
        let server = NtpServer::new(
            "server1".to_string(),
            "pool.ntp.org".to_string(),
            123
        );
        
        ntp.add_server(server).unwrap();
        assert!(ntp.sync_time().is_ok());
    }

    #[test]
    fn test_sigmantp_list_by_state() {
        let mut ntp = SigmaNTP::new();
        
        let server = NtpServer::new(
            "server1".to_string(),
            "pool.ntp.org".to_string(),
            123
        )
        .with_state(NtpServerState::Server);
        
        ntp.add_server(server).unwrap();
        
        let servers = ntp.list_by_state(NtpServerState::Server);
        assert_eq!(servers.len(), 1);
    }

    #[test]
    fn test_sigmantp_stats() {
        let mut ntp = SigmaNTP::new();
        
        let server = NtpServer::new(
            "server1".to_string(),
            "pool.ntp.org".to_string(),
            123
        )
        .with_state(NtpServerState::Server);
        
        ntp.add_server(server).unwrap();
        
        let stats = ntp.ntp_stats();
        assert_eq!(stats.get("total_servers"), Some(&"1".to_string()));
        assert_eq!(stats.get("server_servers"), Some(&"1".to_string()));
    }

    #[test]
    fn test_sync_ntp_time_convenience() {
        init_sigmantp();
        
        let mut ntp = get_sigmantp().unwrap();
        ntp.start();
        
        let server = NtpServer::new(
            "server1".to_string(),
            "pool.ntp.org".to_string(),
            123
        );
        
        ntp.add_server(server).unwrap();
        
        let result = sync_ntp_time();
        assert!(result.is_ok());
    }
}
