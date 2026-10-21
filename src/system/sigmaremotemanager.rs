#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Remote manager for remote system management
/// Inspired by SSH, Ansible, and remote administration tools
#[derive(Debug, Clone)]
pub struct RemoteManager {
    connections: Vec<RemoteConnection>,
    commands: Vec<RemoteCommand>,
}

/// A remote connection
#[derive(Debug, Clone)]
pub struct RemoteConnection {
    id: String,
    host: String,
    port: u16,
    auth_type: AuthType,
    state: ConnectionState,
}

/// Authentication type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthType {
    Password,
    Key,
    Certificate,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Authenticating,
    Error,
}

/// A remote command
#[derive(Debug, Clone)]
pub struct RemoteCommand {
    id: String,
    connection_id: String,
    command: String,
    state: CommandState,
    output: String,
}

/// Command state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandState {
    Pending,
    Running,
    Completed,
    Failed,
}

impl RemoteManager {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            commands: Vec::new(),
        }
    }

    /// Add a remote connection
    pub fn add_connection(&mut self, id: String, host: String, port: u16, auth_type: AuthType) -> Result<String, String> {
        if self.connections.iter().any(|c| c.id == id) {
            return Err(format!("Connection with id {} already exists", id));
        }
        
        let connection = RemoteConnection {
            id: id.clone(),
            host,
            port,
            auth_type,
            state: ConnectionState::Disconnected,
        };
        self.connections.push(connection);
        Ok(id)
    }

    /// Remove a connection
    pub fn remove_connection(&mut self, id: &str) -> Result<(), String> {
        let pos = self.connections.iter().position(|c| c.id == id)
            .ok_or_else(|| format!("Connection with id {} not found", id))?;
        self.connections.remove(pos);
        Ok(())
    }

    /// Connect to a remote host
    pub fn connect(&mut self, id: &str) -> Result<(), String> {
        let connection = self.connections.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Connection with id {} not found", id))?;
        connection.state = ConnectionState::Authenticating;
        connection.state = ConnectionState::Connected;
        Ok(())
    }

    /// Disconnect from a remote host
    pub fn disconnect(&mut self, id: &str) -> Result<(), String> {
        let connection = self.connections.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Connection with id {} not found", id))?;
        connection.state = ConnectionState::Disconnected;
        Ok(())
    }

    /// Execute a remote command
    pub fn execute_command(&mut self, id: String, connection_id: String, command: String) -> Result<String, String> {
        if self.commands.iter().any(|c| c.id == id) {
            return Err(format!("Command with id {} already exists", id));
        }
        
        let remote_command = RemoteCommand {
            id: id.clone(),
            connection_id,
            command,
            state: CommandState::Pending,
            output: String::new(),
        };
        self.commands.push(remote_command);
        Ok(id)
    }

    /// Get all connections
    pub fn get_connections(&self) -> &[RemoteConnection] {
        &self.connections
    }

    /// Get connected hosts
    pub fn get_connected_hosts(&self) -> Vec<&RemoteConnection> {
        self.connections.iter().filter(|c| c.state == ConnectionState::Connected).collect()
    }

    /// Get all commands
    pub fn get_commands(&self) -> &[RemoteCommand] {
        &self.commands
    }
}

impl Default for RemoteManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_connection() {
        let mut manager = RemoteManager::new();
        let result = manager.add_connection("conn1".to_string(), "192.168.1.100".to_string(), 22, AuthType::Key);
        assert!(result.is_ok());
        assert_eq!(manager.get_connections().len(), 1);
    }

    #[test]
    fn test_connect_disconnect() {
        let mut manager = RemoteManager::new();
        manager.add_connection("conn1".to_string(), "192.168.1.100".to_string(), 22, AuthType::Key).unwrap();
        
        manager.connect("conn1").unwrap();
        assert_eq!(manager.get_connection("conn1").unwrap().state, ConnectionState::Connected);
        
        manager.disconnect("conn1").unwrap();
        assert_eq!(manager.get_connection("conn1").unwrap().state, ConnectionState::Disconnected);
    }

    #[test]
    fn test_execute_command() {
        let mut manager = RemoteManager::new();
        manager.add_connection("conn1".to_string(), "192.168.1.100".to_string(), 22, AuthType::Key).unwrap();
        
        let result = manager.execute_command("cmd1".to_string(), "conn1".to_string(), "ls -la".to_string());
        assert!(result.is_ok());
        assert_eq!(manager.get_commands().len(), 1);
    }
}
