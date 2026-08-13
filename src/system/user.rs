// SigmaOS User Management System
// Linux distro-inspired user and group management
// Handles user accounts, authentication, and permissions

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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
}

/// Group information
#[derive(Debug, Clone)]
pub struct Group {
    pub groupname: String,
    pub gid: u32,
    pub members: Vec<String>,
}

/// User manager
pub struct UserManager {
    pub users: HashMap<String, User>,
    pub groups: HashMap<String, Group>,
    pub etc_dir: PathBuf,
    pub next_uid: u32,
    pub next_gid: u32,
}

impl UserManager {
    pub fn new(etc_dir: &str) -> Self {
        let mut manager = Self {
            users: HashMap::new(),
            groups: HashMap::new(),
            etc_dir: PathBuf::from(etc_dir),
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
        };
        manager.users.insert("root".to_string(), root_user);

        // Initialize with root group
        let root_group = Group {
            groupname: "root".to_string(),
            gid: 0,
            members: vec!["root".to_string()],
        };
        manager.groups.insert("root".to_string(), root_group);

        manager
    }

    /// Initialize user management system
    pub fn initialize(&self) -> Result<(), UserError> {
        fs::create_dir_all(&self.etc_dir)
            .map_err(|e| UserError::InitError(self.etc_dir.clone(), e))?;
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

    /// Save users to passwd file
    pub fn save_passwd(&self) -> Result<(), UserError> {
        let passwd_path = self.etc_dir.join("passwd");
        let mut content = String::new();

        let mut users: Vec<_> = self.users.values().collect();
        users.sort_by_key(|u| u.uid);

        for user in users {
            let password = user.password_hash.as_deref().unwrap_or("x");
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

        fs::write(&passwd_path, content)
            .map_err(|e| UserError::WriteError(passwd_path, e))?;

        Ok(())
    }

    /// Save groups to group file
    pub fn save_group(&self) -> Result<(), UserError> {
        let group_path = self.etc_dir.join("group");
        let mut content = String::new();

        let mut groups: Vec<_> = self.groups.values().collect();
        groups.sort_by_key(|g| g.gid);

        for group in groups {
            content.push_str(&format!(
                "{}:{}:{}\n",
                group.groupname,
                "x",
                group.gid
            ));
        }

        fs::write(&group_path, content)
            .map_err(|e| UserError::WriteError(group_path, e))?;

        Ok(())
    }

    /// Load users from passwd file
    pub fn load_passwd(&mut self) -> Result<(), UserError> {
        let passwd_path = self.etc_dir.join("passwd");
        
        if !passwd_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&passwd_path)
            .map_err(|e| UserError::ReadError(passwd_path, e))?;

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
                    password_hash: if parts[1] == "x" { None } else { Some(parts[1].to_string()) },
                    is_root: parts[0] == "root",
                };
                self.users.insert(user.username.clone(), user);
            }
        }

        Ok(())
    }

    /// Load groups from group file
    pub fn load_group(&mut self) -> Result<(), UserError> {
        let group_path = self.etc_dir.join("group");
        
        if !group_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&group_path)
            .map_err(|e| UserError::ReadError(group_path, e))?;

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

    /// Set user password (simple hash for demonstration)
    pub fn set_password(&mut self, username: &str, password: &str) -> Result<(), UserError> {
        if let Some(user) = self.users.get_mut(username) {
            // Simple hash - in production use proper password hashing
            let hash = Self::simple_hash(password);
            user.password_hash = Some(hash);
            Ok(())
        } else {
            Err(UserError::UserNotFound(username.to_string()))
        }
    }

    /// Verify user password
    pub fn verify_password(&self, username: &str, password: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            if let Some(ref hash) = user.password_hash {
                let computed_hash = Self::simple_hash(password);
                return hash == &computed_hash;
            }
        }
        false
    }

    /// Simple hash function for demonstration (use proper crypto in production)
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
    InitError(PathBuf, std::io::Error),
    ReadError(PathBuf, std::io::Error),
    WriteError(PathBuf, std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_manager() {
        let mut manager = UserManager::new("/tmp/test_etc");
        manager.initialize().unwrap();
        
        let user = manager.create_user("testuser", "Test User").unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.uid, 1000);
        
        let retrieved = manager.get_user("testuser");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().username, "testuser");
    }

    #[test]
    fn test_group_management() {
        let mut manager = UserManager::new("/tmp/test_etc");
        manager.initialize().unwrap();
        
        manager.create_user("testuser", "Test User").unwrap();
        manager.create_group("testgroup").unwrap();
        
        manager.add_user_to_group("testuser", "testgroup").unwrap();
        
        let group = manager.get_group("testgroup");
        assert!(group.is_some());
        assert!(group.unwrap().members.contains(&"testuser".to_string()));
    }

    #[test]
    fn test_password_management() {
        let mut manager = UserManager::new("/tmp/test_etc");
        manager.initialize().unwrap();
        
        manager.create_user("testuser", "Test User").unwrap();
        manager.set_password("testuser", "password123").unwrap();
        
        assert!(manager.verify_password("testuser", "password123"));
        assert!(!manager.verify_password("testuser", "wrongpassword"));
    }

    #[test]
    fn test_cannot_delete_root() {
        let mut manager = UserManager::new("/tmp/test_etc");
        manager.initialize().unwrap();
        
        let result = manager.delete_user("root");
        assert!(matches!(result, Err(UserError::CannotDeleteRoot)));
    }
}
