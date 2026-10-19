// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaFTP - System FTP Server
//!
//! SigmaFTP provides system FTP server functionality for SigmaOS, inspired by vsftpd/proftpd from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// FTP connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtpConnectionState {
    /// Connected
    Connected,
    /// Authenticating
    Authenticating,
    /// Authenticated
    Authenticated,
    /// Active
    Active,
    /// Closed
    Closed,
;

impl fmt::Display for FtpConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FtpConnectionState::Connected => write!(f, "connected"),
            FtpConnectionState::Authenticating => write!(f, "authenticating"),
            FtpConnectionState::Authenticated => write!(f, "authenticated"),
            FtpConnectionState::Active => write!(f, "active"),
            FtpConnectionState::Closed => write!(f, "closed"),
        }
    }
;

/// FTP transfer mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtpTransferMode {
    /// ASCII mode
    Ascii,
    /// Binary mode
    Binary,
;

impl fmt::Display for FtpTransferMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FtpTransferMode::Ascii => write!(f, "ascii"),
            FtpTransferMode::Binary => write!(f, "binary"),
        }
    }
;

/// FTP connection
#[derive(Debug, Clone)]
pub struct FtpConnection {
    /// Connection ID
    pub connection_id: String,
    /// Client address
    pub client_address: String,
    /// Client port
    pub client_port: u16,
    /// Username
    pub username: String,
    /// Current directory
    pub current_directory: String,
    /// Transfer mode
    pub transfer_mode: FtpTransferMode,
    /// State
    pub state: FtpConnectionState,
;

impl FtpConnection {
    /// Create a new FTP connection
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
            current_directory: "/".to_string(),
            transfer_mode: FtpTransferMode::Binary,
            state: FtpConnectionState::Connected,
        }
    }

    /// Set transfer mode
    pub fn with_transfer_mode(mut self, transfer_mode: FtpTransferMode) -> Self {
        self.transfer_mode = transfer_mode;
        self
    }

    /// Set state
    pub fn with_state(mut self, state: FtpConnectionState) -> Self {
        self.state = state;
        self
    }

    /// Set current directory
    pub fn with_current_directory(mut self, directory: String) -> Self {
        self.current_directory = directory;
        self
    }
;

/// SigmaFTP - System FTP Server
pub struct SigmaFTP {
    /// FTP connections (connection_id -> connection)
    connections: BTreeMap<String, FtpConnection>,
    /// Running state
    running: bool,
    /// Connection counter
    connection_counter: u32,
;

impl SigmaFTP {
    /// Create a new SigmaFTP instance
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            running: false,
            connection_counter: 0,
        }
    }

    /// Start the FTP server
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the FTP server
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add an FTP connection
    pub fn add_connection(&mut self, connection: FtpConnection) -> Result<(), String> {
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

    /// Remove an FTP connection
    pub fn remove_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if self.connections.remove(connection_id).is_none() {
            return Err(format!("Connection '{}' not found", connection_id));
        }
        Ok(())
    }

    /// Authenticate a connection
    pub fn authenticate_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaFTP not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = FtpConnectionState::Authenticated;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Activate a connection
    pub fn activate_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaFTP not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = FtpConnectionState::Active;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Close a connection
    pub fn close_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaFTP not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = FtpConnectionState::Closed;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Change directory for a connection
    pub fn change_directory(
        &mut self,
        connection_id: &str,
        directory: String,
    ) -> Result<(), String> {
        if !self.running {
            return Err("SigmaFTP not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.current_directory = directory;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Set transfer mode for a connection
    pub fn set_transfer_mode(
        &mut self,
        connection_id: &str,
        mode: FtpTransferMode,
    ) -> Result<(), String> {
        if !self.running {
            return Err("SigmaFTP not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.transfer_mode = mode;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Get an FTP connection
    pub fn get_connection(&self, connection_id: &str) -> Option<&FtpConnection> {
        self.connections.get(connection_id)
    }

    /// List all connections
    pub fn list_connections(&self) -> Vec<&FtpConnection> {
        self.connections.values().collect()
    }

    /// List connections by state
    pub fn list_by_state(&self, state: FtpConnectionState) -> Vec<&FtpConnection> {
        self.connections
            .values()
            .filter(|c| c.state == state)
            .collect()
    }

    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Get FTP server statistics
    pub fn ftp_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_connections".to_string(),
            self.connection_count().to_string(),
        );
        stats.insert(
            "active_connections".to_string(),
            self.list_by_state(FtpConnectionState::Active)
                .len()
                .to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaFTP {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaFTP instance (placeholder for actual global state)
static mut SIGMAFTP_INSTANCE: Option<SigmaFTP> = None;

/// Initialize the global SigmaFTP instance
pub fn init_sigmaftp() {
    unsafe {
        SIGMAFTP_INSTANCE = Some(SigmaFTP::new());
    }
;

/// Get the global SigmaFTP instance
pub fn get_sigmaftp() -> Option<&'static mut SigmaFTP> {
    unsafe { SIGMAFTP_INSTANCE.as_mut() }
;

/// Convenience function to authenticate a connection
pub fn authenticate_ftp_connection(connection_id: &str) -> Result<(), String> {
    if let Some(ftp) = get_sigmaftp() {
        ftp.authenticate_connection(connection_id)
    } else {
        Err("SigmaFTP not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftp_connection_state_display() {
        assert_eq!(FtpConnectionState::Connected.to_string(), "connected");
        assert_eq!(FtpConnectionState::Active.to_string(), "active");
    }

    #[test]
    fn test_ftp_transfer_mode_display() {
        assert_eq!(FtpTransferMode::Ascii.to_string(), "ascii");
        assert_eq!(FtpTransferMode::Binary.to_string(), "binary");
    }

    #[test]
    fn test_ftp_connection() {
        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        )
        .with_transfer_mode(FtpTransferMode::Ascii)
        .with_state(FtpConnectionState::Active)
        .with_current_directory("/home/user".to_string());

        assert_eq!(connection.connection_id, "conn1");
        assert_eq!(connection.transfer_mode, FtpTransferMode::Ascii);
        assert_eq!(connection.state, FtpConnectionState::Active);
        assert_eq!(connection.current_directory, "/home/user");
    }

    #[test]
    fn test_sigmaftp() {
        let mut ftp = SigmaFTP::new();
        assert!(!ftp.is_running());
        ftp.start();
        assert!(ftp.is_running());
        ftp.stop();
        assert!(!ftp.is_running());
    }

    #[test]
    fn test_sigmaftp_add_connection() {
        let mut ftp = SigmaFTP::new();

        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        );

        assert!(ftp.add_connection(connection).is_ok());
        assert_eq!(ftp.connection_count(), 1);
    }

    #[test]
    fn test_sigmaftp_authenticate_connection() {
        let mut ftp = SigmaFTP::new();
        ftp.start();

        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        );

        ftp.add_connection(connection).unwrap();
        assert!(ftp.authenticate_connection("conn1").is_ok());

        let conn = ftp.get_connection("conn1").unwrap();
        assert_eq!(conn.state, FtpConnectionState::Authenticated);
    }

    #[test]
    fn test_sigmaftp_activate_connection() {
        let mut ftp = SigmaFTP::new();
        ftp.start();

        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        )
        .with_state(FtpConnectionState::Authenticated);

        ftp.add_connection(connection).unwrap();
        assert!(ftp.activate_connection("conn1").is_ok());

        let conn = ftp.get_connection("conn1").unwrap();
        assert_eq!(conn.state, FtpConnectionState::Active);
    }

    #[test]
    fn test_sigmaftp_change_directory() {
        let mut ftp = SigmaFTP::new();
        ftp.start();

        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        );

        ftp.add_connection(connection).unwrap();
        assert!(ftp
            .change_directory("conn1", "/home/user".to_string())
            .is_ok());

        let conn = ftp.get_connection("conn1").unwrap();
        assert_eq!(conn.current_directory, "/home/user");
    }

    #[test]
    fn test_sigmaftp_set_transfer_mode() {
        let mut ftp = SigmaFTP::new();
        ftp.start();

        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        );

        ftp.add_connection(connection).unwrap();
        assert!(ftp
            .set_transfer_mode("conn1", FtpTransferMode::Ascii)
            .is_ok());

        let conn = ftp.get_connection("conn1").unwrap();
        assert_eq!(conn.transfer_mode, FtpTransferMode::Ascii);
    }

    #[test]
    fn test_sigmaftp_list_by_state() {
        let mut ftp = SigmaFTP::new();

        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        )
        .with_state(FtpConnectionState::Active);

        ftp.add_connection(connection).unwrap();

        let connections = ftp.list_by_state(FtpConnectionState::Active);
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_sigmaftp_stats() {
        let mut ftp = SigmaFTP::new();

        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        )
        .with_state(FtpConnectionState::Active);

        ftp.add_connection(connection).unwrap();

        let stats = ftp.ftp_stats();
        assert_eq!(stats.get("total_connections"), Some(&"1".to_string()));
        assert_eq!(stats.get("active_connections"), Some(&"1".to_string()));
    }

    #[test]
    fn test_authenticate_ftp_connection_convenience() {
        init_sigmaftp();

        let mut ftp = get_sigmaftp().unwrap();
        ftp.start();

        let connection = FtpConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345,
            "user".to_string(),
        );

        ftp.add_connection(connection).unwrap();

        let result = authenticate_ftp_connection("conn1");
        assert!(result.is_ok());
    }
;
