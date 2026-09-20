//! SigmaOS Login Service
//!
//! This is the login service for SigmaOS, responsible for:
//! - User authentication
//! - Session management
//! - Shell spawning
//!
//! Inspired by Linux login, BSD login, and getty.

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// User account
#[derive(Debug, Clone)]
pub struct User {
    /// Username
    pub username: String,
    /// User ID
    pub uid: u32,
    /// Group ID
    pub gid: u32,
    /// Home directory
    pub home: String,
    /// Shell
    pub shell: String,
    /// Password hash (placeholder)
    pub password_hash: String,
}

/// Session information
#[derive(Debug, Clone)]
pub struct Session {
    /// Session ID
    pub session_id: u32,
    /// User
    pub user: User,
    /// TTY
    pub tty: String,
    /// Login time
    pub login_time: String,
}

/// Login service
pub struct LoginService {
    /// User database
    users: BTreeMap<String, User>,
    /// Current sessions
    sessions: Vec<Session>,
    /// Next session ID
    next_session_id: u32,
}

impl LoginService {
    /// Create new login service
    pub fn new() -> Self {
        let mut users = BTreeMap::new();
        
        // Add default root user
        users.insert(
            String::from("root"),
            User {
                username: String::from("root"),
                uid: 0,
                gid: 0,
                home: String::from("/root"),
                shell: String::from("/bin/sh"),
                password_hash: String::from("$6$placeholder"), // Placeholder hash
            },
        );
        
        // Add default user
        users.insert(
            String::from("sigma"),
            User {
                username: String::from("sigma"),
                uid: 1000,
                gid: 1000,
                home: String::from("/home/sigma"),
                shell: String::from("/bin/sh"),
                password_hash: String::from("$6$placeholder"), // Placeholder hash
            },
        );
        
        LoginService {
            users,
            sessions: Vec::new(),
            next_session_id: 1,
        }
    }
    
    /// Start login prompt
    pub fn prompt_login(&mut self, tty: &str) -> Result<Session, String> {
        println!("SigmaOS v0.1.0");
        println!("{} login: ", tty);
        
        // Read username
        let username = self.read_username()?;
        
        // Authenticate user
        let user = self.authenticate_user(&username)?;
        
        // Create session
        let session = Session {
            session_id: self.next_session_id,
            user: user.clone(),
            tty: tty.to_string(),
            login_time: String::from("now"), // Placeholder timestamp
        };
        
        self.sessions.push(session.clone());
        self.next_session_id += 1;
        
        println!("Login successful: {}", username);
        println!("Last login: {}", session.login_time);
        
        // Spawn shell
        self.spawn_shell(&user, tty)?;
        
        Ok(session)
    }
    
    /// Read username from input
    fn read_username(&self) -> Result<String, String> {
        // Placeholder: In a real implementation, this would:
        // - Read from TTY
        // - Validate username format
        // - Handle EOF
        
        Ok(String::from("sigma")) // Placeholder
    }
    
    /// Authenticate user
    fn authenticate_user(&self, username: &str) -> Result<User, String> {
        if let Some(user) = self.users.get(username) {
            // Placeholder: In a real implementation, this would:
            // - Prompt for password
            // - Verify password hash
            // - Check account status (locked, expired, etc.)
            
            Ok(user.clone())
        } else {
            Err(format!("User not found: {}", username))
        }
    }
    
    /// Spawn user shell
    fn spawn_shell(&self, user: &User, tty: &str) -> Result<(), String> {
        println!("Starting shell for {}...", user.username);
        println!("Shell: {}", user.shell);
        println!("TTY: {}", tty);
        
        // Placeholder: In a real implementation, this would:
        // - Set user ID and group ID
        // - Set up environment variables
        // - Change to home directory
        // - Execute the shell
        // - Set up controlling terminal
        
        println!("Shell spawned successfully");
        
        Ok(())
    }
    
    /// Add user to database
    pub fn add_user(&mut self, user: User) -> Result<(), String> {
        if self.users.contains_key(&user.username) {
            Err(format!("User already exists: {}", user.username))
        } else {
            self.users.insert(user.username.clone(), user);
            Ok(())
        }
    }
    
    /// Remove user from database
    pub fn remove_user(&mut self, username: &str) -> Result<(), String> {
        if self.users.remove(username).is_some() {
            Ok(())
        } else {
            Err(format!("User not found: {}", username))
        }
    }
    
    /// List all users
    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
    
    /// List all active sessions
    pub fn list_sessions(&self) -> &[Session] {
        &self.sessions
    }
}

impl Default for LoginService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_service_creation() {
        let service = LoginService::new();
        assert_eq!(service.users.len(), 2);
        assert!(service.users.contains_key("root"));
        assert!(service.users.contains_key("sigma"));
    }

    #[test]
    fn test_authenticate_user() {
        let service = LoginService::new();
        let result = service.authenticate_user("root");
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.username, "root");
        assert_eq!(user.uid, 0);
    }

    #[test]
    fn test_authenticate_invalid_user() {
        let service = LoginService::new();
        let result = service.authenticate_user("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_user() {
        let mut service = LoginService::new();
        let new_user = User {
            username: String::from("test"),
            uid: 1001,
            gid: 1001,
            home: String::from("/home/test"),
            shell: String::from("/bin/sh"),
            password_hash: String::from("$6$placeholder"),
        };
        
        let result = service.add_user(new_user);
        assert!(result.is_ok());
        assert_eq!(service.users.len(), 3);
    }

    #[test]
    fn test_remove_user() {
        let mut service = LoginService::new();
        let result = service.remove_user("sigma");
        assert!(result.is_ok());
        assert_eq!(service.users.len(), 1);
    }
}
