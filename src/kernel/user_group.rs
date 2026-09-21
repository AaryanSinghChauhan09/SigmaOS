// Linux-inspired User and Group Management
// Provides user account and group management with permissions

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// User account
#[derive(Debug, Clone)]
pub struct User {
    pub uid: u32,
    pub username: String,
    pub gid: u32,
    pub home_dir: String,
    pub shell: String,
    pub full_name: String,
}

impl User {
    pub fn new(uid: u32, username: String, gid: u32) -> Self {
        let home_dir = format!("/home/{}", username);
        Self {
            uid,
            username,
            gid,
            home_dir,
            shell: "/bin/sh".to_string(),
            full_name: String::new(),
        }
    }

    pub fn with_home_dir(mut self, home_dir: String) -> Self {
        self.home_dir = home_dir;
        self
    }

    pub fn with_shell(mut self, shell: String) -> Self {
        self.shell = shell;
        self
    }

    pub fn with_full_name(mut self, full_name: String) -> Self {
        self.full_name = full_name;
        self
    }

    pub fn is_root(&self) -> bool {
        self.uid == 0
    }
}

/// Group account
#[derive(Debug, Clone)]
pub struct Group {
    pub gid: u32,
    pub groupname: String,
    pub members: Vec<u32>, // List of UIDs
}

impl Group {
    pub fn new(gid: u32, groupname: String) -> Self {
        Self {
            gid,
            groupname,
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, uid: u32) {
        if !self.members.contains(&uid) {
            self.members.push(uid);
        }
    }

    pub fn remove_member(&mut self, uid: u64) -> bool {
        if let Some(pos) = self.members.iter().position(|&u| u as u64 == uid) {
            self.members.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn has_member(&self, uid: u32) -> bool {
        self.members.contains(&uid)
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

/// File permissions (Unix-style)
#[derive(Debug, Clone, Copy)]
pub struct FilePermissions {
    pub mode: u32, // Unix mode bits
}

impl FilePermissions {
    pub fn new(mode: u32) -> Self {
        Self { mode }
    }

    pub fn from_octal(octal: u32) -> Self {
        Self { mode: octal }
    }

    pub fn user_read(&self) -> bool {
        (self.mode & 0o400) != 0
    }

    pub fn user_write(&self) -> bool {
        (self.mode & 0o200) != 0
    }

    pub fn user_exec(&self) -> bool {
        (self.mode & 0o100) != 0
    }

    pub fn group_read(&self) -> bool {
        (self.mode & 0o040) != 0
    }

    pub fn group_write(&self) -> bool {
        (self.mode & 0o020) != 0
    }

    pub fn group_exec(&self) -> bool {
        (self.mode & 0o010) != 0
    }

    pub fn other_read(&self) -> bool {
        (self.mode & 0o004) != 0
    }

    pub fn other_write(&self) -> bool {
        (self.mode & 0o002) != 0
    }

    pub fn other_exec(&self) -> bool {
        (self.mode & 0o001) != 0
    }

    pub fn set_user_read(&mut self, value: bool) {
        if value {
            self.mode |= 0o400;
        } else {
            self.mode &= !0o400;
        }
    }

    pub fn set_user_write(&mut self, value: bool) {
        if value {
            self.mode |= 0o200;
        } else {
            self.mode &= !0o200;
        }
    }

    pub fn set_user_exec(&mut self, value: bool) {
        if value {
            self.mode |= 0o100;
        } else {
            self.mode &= !0o100;
        }
    }

    pub fn set_group_read(&mut self, value: bool) {
        if value {
            self.mode |= 0o040;
        } else {
            self.mode &= !0o040;
        }
    }

    pub fn set_group_write(&mut self, value: bool) {
        if value {
            self.mode |= 0o020;
        } else {
            self.mode &= !0o020;
        }
    }

    pub fn set_group_exec(&mut self, value: bool) {
        if value {
            self.mode |= 0o010;
        } else {
            self.mode &= !0o010;
        }
    }

    pub fn set_other_read(&mut self, value: bool) {
        if value {
            self.mode |= 0o004;
        } else {
            self.mode &= !0o004;
        }
    }

    pub fn set_other_write(&mut self, value: bool) {
        if value {
            self.mode |= 0o002;
        } else {
            self.mode &= !0o002;
        }
    }

    pub fn set_other_exec(&mut self, value: bool) {
        if value {
            self.mode |= 0o001;
        } else {
            self.mode &= !0o001;
        }
    }

    pub fn as_octal(&self) -> u32 {
        self.mode & 0o777
    }
}

impl Default for FilePermissions {
    fn default() -> Self {
        Self::from_octal(0o644) // rw-r--r--
    }
}

/// User and group manager
pub struct UserGroupManager {
    users: Arc<Mutex<HashMap<u32, User>>>,
    groups: Arc<Mutex<HashMap<u32, Group>>>,
    next_uid: Arc<Mutex<u32>>,
    next_gid: Arc<Mutex<u32>>,
}

impl UserGroupManager {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        let mut groups = HashMap::new();

        // Create root user
        let root = User::new(0, "root".to_string(), 0)
            .with_home_dir("/root".to_string())
            .with_shell("/bin/sh".to_string());
        users.insert(0, root);

        // Create root group
        let root_group = Group::new(0, "root".to_string());
        groups.insert(0, root_group);

        Self {
            users: Arc::new(Mutex::new(users)),
            groups: Arc::new(Mutex::new(groups)),
            next_uid: Arc::new(Mutex::new(1000)),
            next_gid: Arc::new(Mutex::new(1000)),
        }
    }

    /// Create a new user
    pub fn create_user(&self, username: String, gid: u32) -> Result<u32, String> {
        let mut next_uid = self.next_uid.lock().unwrap();
        let uid = *next_uid;
        *next_uid += 1;
        drop(next_uid);

        let user = User::new(uid, username.clone(), gid);
        let mut users = self.users.lock().unwrap();

        // Check for duplicate username
        for existing_user in users.values() {
            if existing_user.username == username {
                return Err(format!("User {} already exists", username));
            }
        }

        users.insert(uid, user);
        Ok(uid)
    }

    /// Get a user by UID
    pub fn get_user(&self, uid: u32) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.get(&uid).cloned()
    }

    /// Get a user by username
    pub fn get_user_by_name(&self, username: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.values().find(|u| u.username == username).cloned()
    }

    /// Remove a user
    pub fn remove_user(&self, uid: u32) -> Result<(), String> {
        if uid == 0 {
            return Err("Cannot remove root user".to_string());
        }
        let mut users = self.users.lock().unwrap();
        match users.remove(&uid) {
            Some(_) => Ok(()),
            None => Err(format!("User {} not found", uid)),
        }
    }

    /// Create a new group
    pub fn create_group(&self, groupname: String) -> Result<u32, String> {
        let mut next_gid = self.next_gid.lock().unwrap();
        let gid = *next_gid;
        *next_gid += 1;
        drop(next_gid);

        let group = Group::new(gid, groupname.clone());
        let mut groups = self.groups.lock().unwrap();

        // Check for duplicate groupname
        for existing_group in groups.values() {
            if existing_group.groupname == groupname {
                return Err(format!("Group {} already exists", groupname));
            }
        }

        groups.insert(gid, group);
        Ok(gid)
    }

    /// Get a group by GID
    pub fn get_group(&self, gid: u32) -> Option<Group> {
        let groups = self.groups.lock().unwrap();
        groups.get(&gid).cloned()
    }

    /// Get a group by name
    pub fn get_group_by_name(&self, groupname: &str) -> Option<Group> {
        let groups = self.groups.lock().unwrap();
        groups.values().find(|g| g.groupname == groupname).cloned()
    }

    /// Remove a group
    pub fn remove_group(&self, gid: u32) -> Result<(), String> {
        if gid == 0 {
            return Err("Cannot remove root group".to_string());
        }
        let mut groups = self.groups.lock().unwrap();
        match groups.remove(&gid) {
            Some(_) => Ok(()),
            None => Err(format!("Group {} not found", gid)),
        }
    }

    /// Add user to group
    pub fn add_user_to_group(&self, uid: u32, gid: u32) -> Result<(), String> {
        let mut groups = self.groups.lock().unwrap();
        match groups.get_mut(&gid) {
            Some(group) => {
                group.add_member(uid);
                Ok(())
            }
            None => Err(format!("Group {} not found", gid)),
        }
    }

    /// Remove user from group
    pub fn remove_user_from_group(&self, uid: u32, gid: u32) -> Result<(), String> {
        let mut groups = self.groups.lock().unwrap();
        match groups.get_mut(&gid) {
            Some(group) => {
                if group.remove_member(uid as u64) {
                    Ok(())
                } else {
                    Err(format!("User {} not in group {}", uid, gid))
                }
            }
            None => Err(format!("Group {} not found", gid)),
        }
    }

    /// Check if user is in group
    pub fn user_in_group(&self, uid: u32, gid: u32) -> bool {
        match self.get_group(gid) {
            Some(group) => group.has_member(uid),
            None => false,
        }
    }

    /// Get user count
    pub fn user_count(&self) -> usize {
        let users = self.users.lock().unwrap();
        users.len()
    }

    /// Get group count
    pub fn group_count(&self) -> usize {
        let groups = self.groups.lock().unwrap();
        groups.len()
    }
}

impl Default for UserGroupManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1000, "testuser".to_string(), 1000);
        assert_eq!(user.uid, 1000);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.gid, 1000);
        assert!(!user.is_root());
    }

    #[test]
    fn test_user_root() {
        let user = User::new(0, "root".to_string(), 0);
        assert!(user.is_root());
    }

    #[test]
    fn test_user_builder() {
        let user = User::new(1000, "testuser".to_string(), 1000)
            .with_home_dir("/custom/home".to_string())
            .with_shell("/bin/bash".to_string())
            .with_full_name("Test User".to_string());

        assert_eq!(user.home_dir, "/custom/home");
        assert_eq!(user.shell, "/bin/bash");
        assert_eq!(user.full_name, "Test User");
    }

    #[test]
    fn test_group_creation() {
        let group = Group::new(1000, "testgroup".to_string());
        assert_eq!(group.gid, 1000);
        assert_eq!(group.groupname, "testgroup");
        assert_eq!(group.member_count(), 0);
    }

    #[test]
    fn test_group_members() {
        let mut group = Group::new(1000, "testgroup".to_string());
        group.add_member(1000);
        group.add_member(1001);

        assert_eq!(group.member_count(), 2);
        assert!(group.has_member(1000));
        assert!(group.has_member(1001));
        assert!(!group.has_member(1002));

        group.remove_member(1000);
        assert_eq!(group.member_count(), 1);
        assert!(!group.has_member(1000));
    }

    #[test]
    fn test_file_permissions() {
        let perms = FilePermissions::from_octal(0o755);
        assert!(perms.user_read());
        assert!(perms.user_write());
        assert!(perms.user_exec());
        assert!(perms.group_read());
        assert!(!perms.group_write());
        assert!(perms.group_exec());
        assert!(perms.other_read());
        assert!(!perms.other_write());
        assert!(perms.other_exec());
    }

    #[test]
    fn test_file_permissions_modify() {
        let mut perms = FilePermissions::from_octal(0o644);
        perms.set_user_exec(true);
        perms.set_group_write(true);

        assert!(perms.user_exec());
        assert!(perms.group_write());
        assert_eq!(perms.as_octal(), 0o764);
    }

    #[test]
    fn test_user_group_manager() {
        let manager = UserGroupManager::new();

        let uid = manager.create_user("testuser".to_string(), 1000).unwrap();
        assert_eq!(uid, 1000);

        let user = manager.get_user(uid).unwrap();
        assert_eq!(user.username, "testuser");

        let user_by_name = manager.get_user_by_name("testuser").unwrap();
        assert_eq!(user_by_name.uid, uid);

        assert_eq!(manager.user_count(), 2); // root + testuser
    }

    #[test]
    fn test_user_group_manager_groups() {
        let manager = UserGroupManager::new();

        let gid = manager.create_group("testgroup".to_string()).unwrap();
        assert_eq!(gid, 1000);

        let group = manager.get_group(gid).unwrap();
        assert_eq!(group.groupname, "testgroup");

        assert_eq!(manager.group_count(), 2); // root + testgroup
    }

    #[test]
    fn test_user_group_manager_group_membership() {
        let manager = UserGroupManager::new();

        let uid = manager.create_user("testuser".to_string(), 1000).unwrap();
        let gid = manager.create_group("testgroup".to_string()).unwrap();

        manager.add_user_to_group(uid, gid).unwrap();
        assert!(manager.user_in_group(uid, gid));

        manager.remove_user_from_group(uid, gid).unwrap();
        assert!(!manager.user_in_group(uid, gid));
    }

    #[test]
    fn test_user_group_manager_remove_root() {
        let manager = UserGroupManager::new();

        assert!(manager.remove_user(0).is_err());
        assert!(manager.remove_group(0).is_err());
    }

    #[test]
    fn test_user_group_manager_duplicate() {
        let manager = UserGroupManager::new();

        manager.create_user("testuser".to_string(), 1000).unwrap();
        assert!(manager.create_user("testuser".to_string(), 1001).is_err());

        manager.create_group("testgroup".to_string()).unwrap();
        assert!(manager.create_group("testgroup".to_string()).is_err());
    }
}
