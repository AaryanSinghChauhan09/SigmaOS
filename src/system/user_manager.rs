//! User Manager
//!
//! User account management inspired by Linux Mint's user accounts and Omarchy's
//! user utilities, supporting user creation, deletion, and management.

use std::collections::HashMap;

/// User account type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserType {
    System,
    Normal,
    Administrator,
}

impl UserType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "system" => Some(UserType::System),
            "normal" | "user" => Some(UserType::Normal),
            "administrator" | "admin" | "root" => Some(UserType::Administrator),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            UserType::System => "System",
            UserType::Normal => "Normal",
            UserType::Administrator => "Administrator",
        }
    }
}

/// User account
#[derive(Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub user_id: u32,
    pub group_id: u32,
    pub full_name: String,
    pub home_directory: String,
    pub shell: String,
    pub user_type: UserType,
    pub is_active: bool,
    pub created_at: u64,
}

impl UserAccount {
    pub fn new(username: String, user_id: u32, group_id: u32, user_type: UserType) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            username: username.clone(),
            user_id,
            group_id,
            full_name: String::new(),
            home_directory: format!("/home/{}", username),
            shell: "/bin/bash".to_string(),
            user_type,
            is_active: true,
            created_at,
        }
    }

    pub fn set_full_name(&mut self, name: String) {
        self.full_name = name;
    }

    pub fn set_home(&mut self, home: String) {
        self.home_directory = home;
    }

    pub fn set_shell(&mut self, shell: String) {
        self.shell = shell;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn is_admin(&self) -> bool {
        self.user_type == UserType::Administrator
    }
}

/// User manager
#[derive(Debug)]
pub struct UserManager {
    users: HashMap<String, UserAccount>,
    next_uid: u32,
}

impl UserManager {
    pub fn new() -> Self {
        let mut manager = Self {
            users: HashMap::new(),
            next_uid: 1000,
        };

        // Add root user
        let mut root = UserAccount::new("root".to_string(), 0, 0, UserType::Administrator);
        root.set_full_name("System Administrator".to_string());
        root.set_home("/root".to_string());
        root.set_shell("/bin/sh".to_string());
        manager.users.insert("root".to_string(), root);

        manager
    }

    /// Create a new user
    pub fn create_user(
        &mut self,
        username: String,
        user_type: UserType,
    ) -> Result<UserAccount, String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        if self.users.contains_key(&username) {
            return Err(format!("User {} already exists", username));
        }

        let uid = self.next_uid;
        self.next_uid += 1;

        let mut user = UserAccount::new(username.clone(), uid, uid, user_type);
        self.users.insert(username.clone(), user.clone());

        Ok(user)
    }

    /// Get a user
    pub fn get_user(&self, username: &str) -> Option<&UserAccount> {
        self.users.get(username)
    }

    /// Get a user mutably
    pub fn get_user_mut(&mut self, username: &str) -> Option<&mut UserAccount> {
        self.users.get_mut(username)
    }

    /// List all users
    pub fn list_users(&self) -> Vec<&UserAccount> {
        self.users.values().collect()
    }

    /// List users by type
    pub fn list_by_type(&self, user_type: UserType) -> Vec<&UserAccount> {
        self.users
            .values()
            .filter(|u| u.user_type == user_type)
            .collect()
    }

    /// List active users
    pub fn list_active(&self) -> Vec<&UserAccount> {
        self.users.values().filter(|u| u.is_active).collect()
    }

    /// Delete a user
    pub fn delete_user(&mut self, username: &str) -> Result<(), String> {
        let user = self
            .users
            .get(username)
            .ok_or_else(|| format!("User {} not found", username))?;

        if user.username == "root" {
            return Err("Cannot delete root user".to_string());
        }

        self.users.remove(username);
        Ok(())
    }

    /// Update user
    pub fn update_user(
        &mut self,
        username: &str,
        full_name: Option<String>,
        home: Option<String>,
        shell: Option<String>,
    ) -> Result<(), String> {
        let user = self
            .users
            .get_mut(username)
            .ok_or_else(|| format!("User {} not found", username))?;

        if let Some(name) = full_name {
            user.set_full_name(name);
        }
        if let Some(h) = home {
            user.set_home(h);
        }
        if let Some(s) = shell {
            user.set_shell(s);
        }

        Ok(())
    }

    /// Activate user
    pub fn activate_user(&mut self, username: &str) -> Result<(), String> {
        let user = self
            .users
            .get_mut(username)
            .ok_or_else(|| format!("User {} not found", username))?;

        user.set_active(true);
        Ok(())
    }

    /// Deactivate user
    pub fn deactivate_user(&mut self, username: &str) -> Result<(), String> {
        let user = self
            .users
            .get_mut(username)
            .ok_or_else(|| format!("User {} not found", username))?;

        if user.username == "root" {
            return Err("Cannot deactivate root user".to_string());
        }

        user.set_active(false);
        Ok(())
    }

    /// Get user by UID
    pub fn get_user_by_uid(&self, uid: u32) -> Option<&UserAccount> {
        self.users.values().find(|u| u.user_id == uid)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> UserStatistics {
        let total_users = self.users.len();
        let active_users = self.users.values().filter(|u| u.is_active).count();
        let admin_users = self.users.values().filter(|u| u.is_admin()).count();
        let normal_users = self
            .users
            .values()
            .filter(|u| u.user_type == UserType::Normal)
            .count();
        let system_users = self
            .users
            .values()
            .filter(|u| u.user_type == UserType::System)
            .count();

        UserStatistics {
            total_users,
            active_users,
            admin_users,
            normal_users,
            system_users,
            next_uid: self.next_uid,
        }
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

/// User statistics
#[derive(Debug, Clone)]
pub struct UserStatistics {
    pub total_users: usize,
    pub active_users: usize,
    pub admin_users: usize,
    pub normal_users: usize,
    pub system_users: usize,
    pub next_uid: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_type_from_str() {
        assert_eq!(UserType::from_str("admin"), Some(UserType::Administrator));
        assert_eq!(UserType::from_str("normal"), Some(UserType::Normal));
    }

    #[test]
    fn test_user_account_creation() {
        let user = UserAccount::new("test".to_string(), 1000, 1000, UserType::Normal);
        assert_eq!(user.username, "test");
        assert_eq!(user.user_id, 1000);
    }

    #[test]
    fn test_user_manager_creation() {
        let manager = UserManager::new();
        assert!(manager.get_user("root").is_some());
    }

    #[test]
    fn test_create_user() {
        let mut manager = UserManager::new();
        let user = manager
            .create_user("testuser".to_string(), UserType::Normal)
            .unwrap();
        assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_delete_user() {
        let mut manager = UserManager::new();
        manager
            .create_user("testuser".to_string(), UserType::Normal)
            .ok();
        assert!(manager.delete_user("testuser").is_ok());
        assert!(manager.get_user("testuser").is_none());
    }

    #[test]
    fn test_delete_root_fails() {
        let mut manager = UserManager::new();
        assert!(manager.delete_user("root").is_err());
    }

    #[test]
    fn test_update_user() {
        let mut manager = UserManager::new();
        manager
            .create_user("testuser".to_string(), UserType::Normal)
            .ok();
        assert!(manager
            .update_user("testuser", Some("Test User".to_string()), None, None)
            .is_ok());
    }

    #[test]
    fn test_deactivate_user() {
        let mut manager = UserManager::new();
        manager
            .create_user("testuser".to_string(), UserType::Normal)
            .ok();
        assert!(manager.deactivate_user("testuser").is_ok());
    }

    #[test]
    fn test_deactivate_root_fails() {
        let mut manager = UserManager::new();
        assert!(manager.deactivate_user("root").is_err());
    }

    #[test]
    fn test_statistics() {
        let manager = UserManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_users >= 1);
    }
}
