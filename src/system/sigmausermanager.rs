//! SigmaUserManager - User Account Management System
//!
//! Provides comprehensive user account management inspired by Linux Mint and Omarchy.
//! Supports user creation, modification, deletion, groups, and permissions.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// User account type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserType {
    System,
    Regular,
    Administrator,
}

/// User account status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Disabled,
    Locked,
}

/// Shell type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    Dash,
    Custom(String),
}

impl ShellType {
    pub fn to_string(&self) -> String {
        match self {
            ShellType::Bash => "/bin/bash".to_string(),
            ShellType::Zsh => "/bin/zsh".to_string(),
            ShellType::Fish => "/usr/bin/fish".to_string(),
            ShellType::Dash => "/bin/dash".to_string(),
            ShellType::Custom(s) => s.clone(),
        }
    }
}

/// User account
#[derive(Debug, Clone)]
pub struct UserAccount {
    pub id: String,
    pub username: String,
    pub real_name: String,
    pub home_directory: String,
    pub shell: ShellType,
    pub user_type: UserType,
    pub status: UserStatus,
    pub uid: u32,
    pub gid: u32,
    pub groups: Vec<String>,
    pub last_login: Option<String>,
    pub created_at: String,
}

impl UserAccount {
    pub fn new(id: String, username: String, real_name: String, home_directory: String, shell: ShellType, user_type: UserType, uid: u32, gid: u32) -> Self {
        Self {
            id,
            username,
            real_name,
            home_directory,
            shell,
            user_type,
            status: UserStatus::Active,
            uid,
            gid,
            groups: Vec::new(),
            last_login: None,
            created_at: String::new(),
        }
    }

    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }

    pub fn set_last_login(&mut self, time: Option<String>) {
        self.last_login = time;
    }

    pub fn set_created_at(&mut self, time: String) {
        self.created_at = time;
    }

    pub fn add_group(&mut self, group: String) {
        if !self.groups.contains(&group) {
            self.groups.push(group);
        }
    }

    pub fn remove_group(&mut self, group: &str) -> bool {
        if let Some(pos) = self.groups.iter().position(|g| g == group) {
            self.groups.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn is_admin(&self) -> bool {
        self.user_type == UserType::Administrator
    }

    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }
}

/// User group
#[derive(Debug, Clone)]
pub struct UserGroup {
    pub id: String,
    pub name: String,
    pub gid: u32,
    pub members: Vec<String>, // usernames
}

impl UserGroup {
    pub fn new(id: String, name: String, gid: u32) -> Self {
        Self {
            id,
            name,
            gid,
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, username: String) {
        if !self.members.contains(&username) {
            self.members.push(username);
        }
    }

    pub fn remove_member(&mut self, username: &str) -> bool {
        if let Some(pos) = self.members.iter().position(|u| u == username) {
            self.members.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn has_member(&self, username: &str) -> bool {
        self.members.contains(&username.to_string())
    }
}

/// SigmaUserManager - User account management
#[derive(Debug)]
pub struct SigmaUserManager {
    users: BTreeMap<String, UserAccount>,
    groups: BTreeMap<String, UserGroup>,
    next_uid: u32,
    next_gid: u32,
}

impl SigmaUserManager {
    pub fn new() -> Self {
        Self {
            users: BTreeMap::new(),
            groups: BTreeMap::new(),
            next_uid: 1000,
            next_gid: 1000,
        }
    }

    pub fn add_user(&mut self, user: UserAccount) {
        self.users.insert(user.id.clone(), user);
    }

    pub fn remove_user(&mut self, id: &str) -> bool {
        self.users.remove(id).is_some()
    }

    pub fn get_user(&self, id: &str) -> Option<&UserAccount> {
        self.users.get(id)
    }

    pub fn get_user_mut(&mut self, id: &str) -> Option<&mut UserAccount> {
        self.users.get_mut(id)
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&UserAccount> {
        self.users.values().find(|user| user.username == username)
    }

    pub fn add_group(&mut self, group: UserGroup) {
        self.groups.insert(group.id.clone(), group);
    }

    pub fn remove_group(&mut self, id: &str) -> bool {
        self.groups.remove(id).is_some()
    }

    pub fn get_group(&self, id: &str) -> Option<&UserGroup> {
        self.groups.get(id)
    }

    pub fn list_admins(&self) -> Vec<&UserAccount> {
        self.users.values()
            .filter(|user| user.is_admin())
            .collect()
    }

    pub fn list_active(&self) -> Vec<&UserAccount> {
        self.users.values()
            .filter(|user| user.is_active())
            .collect()
    }

    pub fn list_by_type(&self, user_type: UserType) -> Vec<&UserAccount> {
        self.users.values()
            .filter(|user| user.user_type == user_type)
            .collect()
    }

    pub fn allocate_uid(&mut self) -> u32 {
        let uid = self.next_uid;
        self.next_uid += 1;
        uid
    }

    pub fn allocate_gid(&mut self) -> u32 {
        let gid = self.next_gid;
        self.next_gid += 1;
        gid
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}

impl Default for SigmaUserManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaUserManager instance
static mut SIGMAUSERMANAGER_INSTANCE: Option<SigmaUserManager> = None;

/// Initialize the global SigmaUserManager instance
pub fn init_sigmausermanager() {
    unsafe {
        SIGMAUSERMANAGER_INSTANCE = Some(SigmaUserManager::new());
    }
}

/// Get the global SigmaUserManager instance
pub fn get_sigmausermanager() -> Option<&'static SigmaUserManager> {
    unsafe {
        SIGMAUSERMANAGER_INSTANCE.as_ref()
    }
}

/// Get the global SigmaUserManager instance (mutable)
pub fn get_sigmausermanager_mut() -> Option<&'static mut SigmaUserManager> {
    unsafe {
        SIGMAUSERMANAGER_INSTANCE.as_mut()
    }
}

/// Convenience function to add a user
pub fn add_user(user: UserAccount) -> bool {
    if let Some(manager) = get_sigmausermanager_mut() {
        manager.add_user(user);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_account_creation() {
        let user = UserAccount::new(
            "user1".to_string(),
            "john".to_string(),
            "John Doe".to_string(),
            "/home/john".to_string(),
            ShellType::Bash,
            UserType::Regular,
            1000,
            1000,
        );
        assert_eq!(user.id, "user1");
        assert_eq!(user.username, "john");
        assert!(user.is_active());
    }

    #[test]
    fn test_user_account_modification() {
        let mut user = UserAccount::new(
            "user1".to_string(),
            "test".to_string(),
            "Test".to_string(),
            "/home/test".to_string(),
            ShellType::Zsh,
            UserType::Administrator,
            1000,
            1000,
        );
        user.set_status(UserStatus::Disabled);
        user.add_group("sudo".to_string());
        user.add_group("wheel".to_string());
        assert!(!user.is_active());
        assert!(user.is_admin());
        assert_eq!(user.groups.len(), 2);
    }

    #[test]
    fn test_user_group_creation() {
        let group = UserGroup::new("group1".to_string(), "sudo".to_string(), 1000);
        assert_eq!(group.id, "group1");
        assert_eq!(group.gid, 1000);
    }

    #[test]
    fn test_user_group_modification() {
        let mut group = UserGroup::new("group1".to_string(), "test".to_string(), 1000);
        group.add_member("user1".to_string());
        group.add_member("user2".to_string());
        assert!(group.has_member("user1"));
        assert_eq!(group.members.len(), 2);
    }

    #[test]
    fn test_sigmausermanager_creation() {
        let manager = SigmaUserManager::new();
        assert_eq!(manager.user_count(), 0);
        assert_eq!(manager.group_count(), 0);
    }

    #[test]
    fn test_sigmausermanager_management() {
        let mut manager = SigmaUserManager::new();
        let user = UserAccount::new(
            "user1".to_string(),
            "test".to_string(),
            "Test".to_string(),
            "/home/test".to_string(),
            ShellType::Bash,
            UserType::Regular,
            1000,
            1000,
        );
        manager.add_user(user);
        assert_eq!(manager.user_count(), 1);
    }

    #[test]
    fn test_uid_gid_allocation() {
        let mut manager = SigmaUserManager::new();
        let uid1 = manager.allocate_uid();
        let uid2 = manager.allocate_uid();
        assert_eq!(uid1, 1000);
        assert_eq!(uid2, 1001);
    }
}
