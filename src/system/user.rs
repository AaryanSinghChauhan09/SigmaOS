extern crate alloc;
use alloc::vec;
use alloc::string::String;
use alloc::vec::Vec;
// SigmaOS User Management System
// Linux distro-inspired user and group management
// Handles user accounts, authentication, shadow passwords, sudo policies, usermod, and groupmod

#[cfg(not(test))]
use crate::klib::HashMap;
#[cfg(test)]
use crate::klib::HashMap;
use crate::klib::path::PathBuf;
use std::fs;

/// User account information
#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub home_dir: String,
    pub shell: String,
    pub full_name: String,
    pub password_hash: Option<String>,
    pub is_root: bool,
    pub is_locked: bool,
}

/// Shadow password entry (/etc/shadow compatibility)
#[derive(Debug, Clone)]
pub struct ShadowEntry {
    pub username: String,
    pub password_hash: String,
    pub last_change_days: u32,
    pub min_days: u32,
    pub max_days: u32,
    pub warn_days: u32,
    pub inactive_days: i32,
    pub expire_days: i32,
}

/// Group information
#[derive(Debug, Clone)]
pub struct Group {
    pub groupname: String,
    pub gid: u32,
    pub members: Vec<String>,
}

/// Sudoers rule specification (/etc/sudoers and BSD doas.conf parity)
#[derive(Debug, Clone)]
pub struct SudoersRule {
    pub entity: String,  // "username" or "%groupname"
    pub host: String,    // "ALL"
    pub run_as: String,  // "ALL" or "root"
    pub command: String, // "ALL" or "/usr/bin/apt"
    pub nopasswd: bool,
}

/// Sudo Policy Engine
#[derive(Debug, Clone)]
pub struct SudoPolicyEngine {
    pub rules: Vec<SudoersRule>,
}

impl SudoPolicyEngine {
    pub fn new() -> Self {
        let mut engine = Self { rules: Vec::new() };
        // Default rule: root ALL=(ALL:ALL) ALL
        engine.rules.push(SudoersRule {
            entity: "root".to_string(),
            host: "ALL".to_string(),
            run_as: "ALL".to_string(),
            command: "ALL".to_string(),
            nopasswd: true,
        });
        // Default rule: %wheel ALL=(ALL:ALL) ALL
        engine.rules.push(SudoersRule {
            entity: "%wheel".to_string(),
            host: "ALL".to_string(),
            run_as: "ALL".to_string(),
            command: "ALL".to_string(),
            nopasswd: false,
        });
        engine
    }

    pub fn add_rule(&mut self, rule: SudoersRule) {
        self.rules.push(rule);
    }

    pub fn evaluate_sudo_privilege(
        &self,
        username: &str,
        user_groups: &[String],
        target_cmd: &str,
    ) -> Result<bool, UserError> {
        if username == "root" {
            return Ok(true); // Root bypasses password and rule checks
        }

        for rule in &self.rules {
            let is_user_match = rule.entity == username;
            let is_group_match =
                rule.entity.starts_with('%') && user_groups.contains(&rule.entity[1..].to_string());

            if is_user_match || is_group_match {
                let cmd_matches = rule.command == "ALL" || rule.command == target_cmd;
                if cmd_matches {
                    return Ok(rule.nopasswd);
                }
            }
        }

        Err(UserError::SudoPermissionDenied(
            username.to_string(),
            target_cmd.to_string(),
        ))
    }
}

impl Default for SudoPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// User manager
pub struct UserManager {
    pub users: HashMap<String, User>,
    pub shadow_entries: HashMap<String, ShadowEntry>,
    pub groups: HashMap<String, Group>,
    pub sudo_engine: SudoPolicyEngine,
    pub etc_dir: String,
    pub next_uid: u32,
    pub next_gid: u32,
}

impl UserManager {
    pub fn new(etc_dir: &str) -> Self {
        let mut manager = Self {
            users: HashMap::new(),
            shadow_entries: HashMap::new(),
            groups: HashMap::new(),
            sudo_engine: SudoPolicyEngine::new(),
            etc_dir: etc_dir.to_string(),
            next_uid: 1000,
            next_gid: 1000,
        };

        // Initialize with root user
        let root_user = User {
            username: "root".to_string(),
            uid: 0,
            gid: 0,
            home_dir: "/root".to_string(),
            shell: "/bin/sh".to_string(),
            full_name: "Super User".to_string(),
            password_hash: None,
            is_root: true,
            is_locked: false,
        };
        manager.users.insert("root".to_string(), root_user);

        // Initialize with root group and wheel administrative group
        let root_group = Group {
            groupname: "root".to_string(),
            gid: 0,
            members: vec!["root".to_string()],
        };
        manager.groups.insert("root".to_string(), root_group);

        let wheel_group = Group {
            groupname: "wheel".to_string(),
            gid: 10,
            members: vec!["root".to_string()],
        };
        manager.groups.insert("wheel".to_string(), wheel_group);

        manager
    }

    /// Initialize user management system
    pub fn initialize(&self) -> Result<(), UserError> {
        let std_path = std::path::Path::new(self.etc_dir.as_str());
        fs::create_dir_all(std_path)
            .map_err(|_| UserError::InitError(self.etc_dir.clone(), "Failed to create directory"))?;
        Ok(())
    }

    /// Create a new user
    pub fn create_user(&mut self, username: &str, full_name: &str) -> Result<User, UserError> {
        if self.users.contains_key(username) {
            return Err(UserError::UserExists(username.to_string()));
        }

        let uid = self.next_uid;
        let gid = self.next_gid;
        self.next_uid += 1;
        self.next_gid += 1;

        let user = User {
            username: username.to_string(),
            uid,
            gid,
            home_dir: format!("/home/{}", username),
            shell: "/bin/sh".to_string(),
            full_name: full_name.to_string(),
            password_hash: None,
            is_root: false,
            is_locked: false,
        };

        // Create user's primary group
        let group = Group {
            groupname: username.to_string(),
            gid,
            members: vec![username.to_string()],
        };
        self.groups.insert(username.to_string(), group);

        self.users.insert(username.to_string(), user.clone());
        Ok(user)
    }

    /// usermod - modify user account options (shell, home directory, primary GID, lock status)
    pub fn usermod(
        &mut self,
        username: &str,
        new_shell: Option<&str>,
        new_home: Option<&str>,
        new_gid: Option<u32>,
        lock: Option<bool>,
    ) -> Result<(), UserError> {
        if let Some(user) = self.users.get_mut(username) {
            if let Some(sh) = new_shell {
                user.shell = sh.to_string();
            }
            if let Some(home) = new_home {
                user.home_dir = home.to_string();
            }
            if let Some(gid) = new_gid {
                user.gid = gid;
            }
            if let Some(lock_val) = lock {
                user.is_locked = lock_val;
            }
            Ok(())
        } else {
            Err(UserError::UserNotFound(username.to_string()))
        }
    }

    /// groupmod - modify group attributes (rename group or update GID)
    pub fn groupmod(
        &mut self,
        old_name: &str,
        new_name: Option<&str>,
        new_gid: Option<u32>,
    ) -> Result<(), UserError> {
        if !self.groups.contains_key(old_name) {
            return Err(UserError::GroupNotFound(old_name.to_string()));
        }

        let mut group = self.groups.remove(old_name).unwrap();

        if let Some(gid) = new_gid {
            group.gid = gid;
        }

        let target_name = if let Some(name) = new_name {
            group.groupname = name.to_string();
            name.to_string()
        } else {
            old_name.to_string()
        };

        self.groups.insert(target_name, group);
        Ok(())
    }

    /// Get user by username
    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }

    /// Get user by UID
    pub fn get_user_by_uid(&self, uid: u32) -> Option<&User> {
        self.users.values().find(|u| u.uid == uid)
    }

    /// Delete user
    pub fn delete_user(&mut self, username: &str) -> Result<(), UserError> {
        if username == "root" {
            return Err(UserError::CannotDeleteRoot);
        }

        if !self.users.contains_key(username) {
            return Err(UserError::UserNotFound(username.to_string()));
        }

        self.users.remove(username);
        self.shadow_entries.remove(username);
        self.groups.remove(username);
        Ok(())
    }

    /// Add user to group
    pub fn add_user_to_group(&mut self, username: &str, groupname: &str) -> Result<(), UserError> {
        if !self.users.contains_key(username) {
            return Err(UserError::UserNotFound(username.to_string()));
        }

        if !self.groups.contains_key(groupname) {
            return Err(UserError::GroupNotFound(groupname.to_string()));
        }

        if let Some(group) = self.groups.get_mut(groupname) {
            if !group.members.contains(&username.to_string()) {
                group.members.push(username.to_string());
            }
        }

        Ok(())
    }

    /// Get secondary groups for a user
    pub fn get_user_groups(&self, username: &str) -> Vec<String> {
        self.groups
            .values()
            .filter(|g| g.members.contains(&username.to_string()))
            .map(|g| g.groupname.clone())
            .collect()
    }

    /// Create group
    pub fn create_group(&mut self, groupname: &str) -> Result<Group, UserError> {
        if self.groups.contains_key(groupname) {
            return Err(UserError::GroupExists(groupname.to_string()));
        }

        let gid = self.next_gid;
        self.next_gid += 1;

        let group = Group {
            groupname: groupname.to_string(),
            gid,
            members: Vec::new(),
        };

        self.groups.insert(groupname.to_string(), group.clone());
        Ok(group)
    }

    /// Get group by name
    pub fn get_group(&self, groupname: &str) -> Option<&Group> {
        self.groups.get(groupname)
    }

    /// Get group by GID
    pub fn get_group_by_gid(&self, gid: u32) -> Option<&Group> {
        self.groups.values().find(|g| g.gid == gid)
    }

    /// Set user password and create/update shadow entry
    pub fn set_password(&mut self, username: &str, password: &str) -> Result<(), UserError> {
        if let Some(user) = self.users.get_mut(username) {
            let hash = Self::simple_hash(password);
            user.password_hash = Some(hash.clone());

            let shadow = ShadowEntry {
                username: username.to_string(),
                password_hash: hash,
                last_change_days: 19500,
                min_days: 0,
                max_days: 99999,
                warn_days: 7,
                inactive_days: -1,
                expire_days: -1,
            };
            self.shadow_entries.insert(username.to_string(), shadow);

            Ok(())
        } else {
            Err(UserError::UserNotFound(username.to_string()))
        }
    }

    /// Verify user password against shadow password database
    pub fn verify_password(&self, username: &str, password: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            if user.is_locked {
                return false; // Account locked via usermod
            }
            if let Some(shadow) = self.shadow_entries.get(username) {
                let computed_hash = Self::simple_hash(password);
                return shadow.password_hash == computed_hash;
            } else if let Some(ref hash) = user.password_hash {
                let computed_hash = Self::simple_hash(password);
                return hash == &computed_hash;
            }
        }
        false
    }

    /// Save shadow entries to /etc/shadow
    pub fn save_shadow(&self) -> Result<(), UserError> {
        let shadow_path = format!("{}/shadow", self.etc_dir);
        let mut content = String::new();

        for shadow in self.shadow_entries.values() {
            content.push_str(&format!(
                "{}:{}:{}:{}:{}:{}:{}:{}\n",
                shadow.username,
                shadow.password_hash,
                shadow.last_change_days,
                shadow.min_days,
                shadow.max_days,
                shadow.warn_days,
                shadow.inactive_days,
                shadow.expire_days
            ));
        }

        fs::write(&shadow_path, content).map_err(|_| UserError::WriteError(shadow_path.clone(), "Failed to write shadow"))?;

        Ok(())
    }

    /// Load shadow entries from /etc/shadow
    pub fn load_shadow(&mut self) -> Result<(), UserError> {
        let shadow_path_str = format!("{}/shadow", self.etc_dir);
        let std_path = std::path::Path::new(shadow_path_str.as_str());
        if !std_path.exists() {
            return Ok(());
        }

        let content =
            fs::read_to_string(std_path).map_err(|_| UserError::ReadError(shadow_path_str, "Failed to read shadow"))?;

        for line in content.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 8 {
                let shadow = ShadowEntry {
                    username: parts[0].to_string(),
                    password_hash: parts[1].to_string(),
                    last_change_days: parts[2].parse().unwrap_or(0),
                    min_days: parts[3].parse().unwrap_or(0),
                    max_days: parts[4].parse().unwrap_or(99999),
                    warn_days: parts[5].parse().unwrap_or(7),
                    inactive_days: parts[6].parse().unwrap_or(-1),
                    expire_days: parts[7].parse().unwrap_or(-1),
                };
                self.shadow_entries.insert(shadow.username.clone(), shadow);
            }
        }

        Ok(())
    }

    /// Save users to passwd file
    pub fn save_passwd(&self) -> Result<(), UserError> {
        let passwd_path = format!("{}/passwd", self.etc_dir);
        let mut content = String::new();

        let mut users: Vec<_> = self.users.values().collect();
        users.sort_by_key(|u| u.uid);

        for user in users {
            let password = if self.shadow_entries.contains_key(&user.username) {
                "x"
            } else {
                user.password_hash.as_deref().unwrap_or("x")
            };

            content.push_str(&format!(
                "{}:{}:{}:{}:{}:{}:{}\n",
                user.username,
                password,
                user.uid,
                user.gid,
                user.full_name,
                user.home_dir,
                user.shell
            ));
        }

        fs::write(&passwd_path, content).map_err(|_| UserError::WriteError(passwd_path.clone(), "Failed to write passwd"))?;

        Ok(())
    }

    /// Save groups to group file
    pub fn save_group(&self) -> Result<(), UserError> {
        let group_path = format!("{}/group", self.etc_dir);
        let mut content = String::new();

        let mut groups: Vec<_> = self.groups.values().collect();
        groups.sort_by_key(|g| g.gid);

        for group in groups {
            let members_str = group.members.join(",");
            content.push_str(&format!(
                "{}:{}:{}:{}\n",
                group.groupname, "x", group.gid, members_str
            ));
        }

        fs::write(&group_path, content).map_err(|_| UserError::WriteError(group_path.clone(), "Failed to write group"))?;

        Ok(())
    }

    /// Load users from passwd file
    pub fn load_passwd(&mut self) -> Result<(), UserError> {
        let passwd_path_str = format!("{}/passwd", self.etc_dir);
        let std_path = std::path::Path::new(passwd_path_str.as_str());

        if !std_path.exists() {
            return Ok(());
        }

        let content =
            fs::read_to_string(std_path).map_err(|_| UserError::ReadError(passwd_path_str, "Failed to read passwd"))?;

        for line in content.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 7 {
                let user = User {
                    username: parts[0].to_string(),
                    uid: parts[2].parse().unwrap_or(0),
                    gid: parts[3].parse().unwrap_or(0),
                    full_name: parts[4].to_string(),
                    home_dir: parts[5].to_string(),
                    shell: parts[6].to_string(),
                    password_hash: if parts[1] == "x" {
                        None
                    } else {
                        Some(parts[1].to_string())
                    },
                    is_root: parts[0] == "root",
                    is_locked: false,
                };
                self.users.insert(user.username.clone(), user);
            }
        }

        Ok(())
    }

    /// Load groups from group file
    pub fn load_group(&mut self) -> Result<(), UserError> {
        let group_path_str = format!("{}/group", self.etc_dir);
        let std_path = std::path::Path::new(group_path_str.as_str());

        if !std_path.exists() {
            return Ok(());
        }

        let content =
            fs::read_to_string(std_path).map_err(|_| UserError::ReadError(group_path_str, "Failed to read group"))?;

        for line in content.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let group = Group {
                    groupname: parts[0].to_string(),
                    gid: parts[2].parse().unwrap_or(0),
                    members: if parts.len() > 3 && !parts[3].is_empty() {
                        parts[3].split(',').map(|s| s.to_string()).collect()
                    } else {
                        Vec::new()
                    },
                };
                self.groups.insert(group.groupname.clone(), group);
            }
        }

        Ok(())
    }

    /// Simple hash function for demonstration
    fn simple_hash(input: &str) -> String {
        let mut hash: u64 = 5381;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        format!("{:x}", hash)
    }
}

/// User management errors
#[derive(Debug)]
pub enum UserError {
    UserExists(String),
    UserNotFound(String),
    GroupExists(String),
    GroupNotFound(String),
    CannotDeleteRoot,
    SudoPermissionDenied(String, String),
    InitError(String, &'static str),
    ReadError(String, &'static str),
    WriteError(String, &'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_manager() {
        let mut manager = UserManager::new("/tmp/test_etc_user");
        manager.initialize().unwrap();

        let user = manager.create_user("testuser", "Test User").unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.uid, 1000);

        let retrieved = manager.get_user("testuser");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().username, "testuser");
    }

    #[test]
    fn test_usermod_and_groupmod() {
        let mut manager = UserManager::new("/tmp/test_etc_mod");
        manager.initialize().unwrap();

        manager.create_user("alice", "Alice User").unwrap();
        manager
            .usermod(
                "alice",
                Some("/bin/zsh"),
                Some("/var/home/alice"),
                Some(2000),
                Some(true),
            )
            .unwrap();

        let updated = manager.get_user("alice").unwrap();
        assert_eq!(updated.shell, "/bin/zsh");
        assert_eq!(updated.home_dir, "/var/home/alice");
        assert_eq!(updated.gid, 2000);
        assert!(updated.is_locked);

        manager
            .groupmod("alice", Some("alice_group"), Some(2000))
            .unwrap();
        assert!(manager.get_group("alice_group").is_some());
    }

    #[test]
    fn test_shadow_and_sudoers_policy() {
        let mut manager = UserManager::new("/tmp/test_etc_shadow");
        manager.initialize().unwrap();

        manager.create_user("bob", "Bob Developer").unwrap();
        manager.set_password("bob", "secret_pass_123").unwrap();
        assert!(manager.verify_password("bob", "secret_pass_123"));

        manager.add_user_to_group("bob", "wheel").unwrap();
        let groups = manager.get_user_groups("bob");
        assert!(groups.contains(&"wheel".to_string()));

        let sudo_res =
            manager
                .sudo_engine
                .evaluate_sudo_privilege("bob", &groups, "/usr/bin/systemctl");
        assert!(sudo_res.is_ok());
    }
}
