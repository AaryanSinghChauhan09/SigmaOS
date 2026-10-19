// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaDB - System Database Server
//!
//! SigmaDB provides system database server functionality for SigmaOS, inspired by PostgreSQL/MySQL from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Database connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbConnectionState {
    /// Connected
    Connected,
    /// Authenticating
    Authenticating,
    /// Authenticated
    Authenticated,
    /// Querying
    Querying,
    /// Closed
    Closed,
;

impl fmt::Display for DbConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbConnectionState::Connected => write!(f, "connected"),
            DbConnectionState::Authenticating => write!(f, "authenticating"),
            DbConnectionState::Authenticated => write!(f, "authenticated"),
            DbConnectionState::Querying => write!(f, "querying"),
            DbConnectionState::Closed => write!(f, "closed"),
        }
    }
;

/// Database connection
#[derive(Debug, Clone)]
pub struct DbConnection {
    /// Connection ID
    pub connection_id: String,
    /// Client address
    pub client_address: String,
    /// Client port
    pub client_port: u16,
    /// Username
    pub username: String,
    /// Database name
    pub database: String,
    /// State
    pub state: DbConnectionState,
;

impl DbConnection {
    /// Create a new database connection
    pub fn new(
        connection_id: String,
        client_address: String,
        client_port: u16,
        username: String,
        database: String,
    ) -> Self {
        Self {
            connection_id,
            client_address,
            client_port,
            username,
            database,
            state: DbConnectionState::Connected,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: DbConnectionState) -> Self {
        self.state = state;
        self
    }
;

/// SigmaDB - System Database Server
pub struct SigmaDB {
    /// Database connections (connection_id -> connection)
    connections: BTreeMap<String, DbConnection>,
    /// Running state
    running: bool,
    /// Connection counter
    connection_counter: u32,
;

impl SigmaDB {
    /// Create a new SigmaDB instance
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            running: false,
            connection_counter: 0,
        }
    }

    /// Start the database server
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the database server
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a database connection
    pub fn add_connection(&mut self, connection: DbConnection) -> Result<(), String> {
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

    /// Remove a database connection
    pub fn remove_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if self.connections.remove(connection_id).is_none() {
            return Err(format!("Connection '{}' not found", connection_id));
        }
        Ok(())
    }

    /// Authenticate a connection
    pub fn authenticate_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDB not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = DbConnectionState::Authenticated;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Execute a query
    pub fn execute_query(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDB not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = DbConnectionState::Querying;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Close a connection
    pub fn close_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDB not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = DbConnectionState::Closed;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Get a database connection
    pub fn get_connection(&self, connection_id: &str) -> Option<&DbConnection> {
        self.connections.get(connection_id)
    }

    /// List all connections
    pub fn list_connections(&self) -> Vec<&DbConnection> {
        self.connections.values().collect()
    }

    /// List connections by state
    pub fn list_by_state(&self, state: DbConnectionState) -> Vec<&DbConnection> {
        self.connections
            .values()
            .filter(|c| c.state == state)
            .collect()
    }

    /// List connections by database
    pub fn list_by_database(&self, database: &str) -> Vec<&DbConnection> {
        self.connections
            .values()
            .filter(|c| c.database == database)
            .collect()
    }

    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Get database server statistics
    pub fn db_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_connections".to_string(),
            self.connection_count().to_string(),
        );
        stats.insert(
            "authenticated_connections".to_string(),
            self.list_by_state(DbConnectionState::Authenticated)
                .len()
                .to_string(),
        );
        stats.insert(
            "querying_connections".to_string(),
            self.list_by_state(DbConnectionState::Querying)
                .len()
                .to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaDB {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaDB instance (placeholder for actual global state)
static mut SIGMADB_INSTANCE: Option<SigmaDB> = None;

/// Initialize the global SigmaDB instance
pub fn init_sigmadb() {
    unsafe {
        SIGMADB_INSTANCE = Some(SigmaDB::new());
    }
;

/// Get the global SigmaDB instance
pub fn get_sigmadb() -> Option<&'static mut SigmaDB> {
    unsafe { SIGMADB_INSTANCE.as_mut() }
;

/// Convenience function to execute a query
pub fn execute_db_query(connection_id: &str) -> Result<(), String> {
    if let Some(db) = get_sigmadb() {
        db.execute_query(connection_id)
    } else {
        Err("SigmaDB not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_connection_state_display() {
        assert_eq!(DbConnectionState::Connected.to_string(), "connected");
        assert_eq!(DbConnectionState::Querying.to_string(), "querying");
    }

    #[test]
    fn test_db_connection() {
        let connection = DbConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            5432,
            "user".to_string(),
            "testdb".to_string(),
        )
        .with_state(DbConnectionState::Authenticated);

        assert_eq!(connection.connection_id, "conn1");
        assert_eq!(connection.database, "testdb");
        assert_eq!(connection.state, DbConnectionState::Authenticated);
    }

    #[test]
    fn test_sigmadb() {
        let mut db = SigmaDB::new();
        assert!(!db.is_running());
        db.start();
        assert!(db.is_running());
        db.stop();
        assert!(!db.is_running());
    }

    #[test]
    fn test_sigmadb_add_connection() {
        let mut db = SigmaDB::new();

        let connection = DbConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            5432,
            "user".to_string(),
            "testdb".to_string(),
        );

        assert!(db.add_connection(connection).is_ok());
        assert_eq!(db.connection_count(), 1);
    }

    #[test]
    fn test_sigmadb_authenticate_connection() {
        let mut db = SigmaDB::new();
        db.start();

        let connection = DbConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            5432,
            "user".to_string(),
            "testdb".to_string(),
        );

        db.add_connection(connection).unwrap();
        assert!(db.authenticate_connection("conn1").is_ok());

        let conn = db.get_connection("conn1").unwrap();
        assert_eq!(conn.state, DbConnectionState::Authenticated);
    }

    #[test]
    fn test_sigmadb_execute_query() {
        let mut db = SigmaDB::new();
        db.start();

        let connection = DbConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            5432,
            "user".to_string(),
            "testdb".to_string(),
        )
        .with_state(DbConnectionState::Authenticated);

        db.add_connection(connection).unwrap();
        assert!(db.execute_query("conn1").is_ok());

        let conn = db.get_connection("conn1").unwrap();
        assert_eq!(conn.state, DbConnectionState::Querying);
    }

    #[test]
    fn test_sigmadb_list_by_state() {
        let mut db = SigmaDB::new();

        let connection = DbConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            5432,
            "user".to_string(),
            "testdb".to_string(),
        )
        .with_state(DbConnectionState::Authenticated);

        db.add_connection(connection).unwrap();

        let connections = db.list_by_state(DbConnectionState::Authenticated);
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_sigmadb_list_by_database() {
        let mut db = SigmaDB::new();

        let connection = DbConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            5432,
            "user".to_string(),
            "testdb".to_string(),
        );

        db.add_connection(connection).unwrap();

        let connections = db.list_by_database("testdb");
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_sigmadb_stats() {
        let mut db = SigmaDB::new();

        let connection = DbConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            5432,
            "user".to_string(),
            "testdb".to_string(),
        )
        .with_state(DbConnectionState::Authenticated);

        db.add_connection(connection).unwrap();

        let stats = db.db_stats();
        assert_eq!(stats.get("total_connections"), Some(&"1".to_string()));
        assert_eq!(
            stats.get("authenticated_connections"),
            Some(&"1".to_string())
        );
    }

    #[test]
    fn test_execute_db_query_convenience() {
        init_sigmadb();

        let mut db = get_sigmadb().unwrap();
        db.start();

        let connection = DbConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            5432,
            "user".to_string(),
            "testdb".to_string(),
        )
        .with_state(DbConnectionState::Authenticated);

        db.add_connection(connection).unwrap();

        let result = execute_db_query("conn1");
        assert!(result.is_ok());
    }
;
