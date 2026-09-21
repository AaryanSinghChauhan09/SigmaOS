// Linux-inspired user/group database management
// Provides user and group account management

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
    pub gecos: String,
}

impl User {
    pub fn new(uid: u32, username: String, gid: u32, home_dir: String, shell: String) -> Self {
        Self {
            uid,
            username,
            gid,
            home_dir,
            shell,
            gecos: String::new(),
        }
    }

    /// Set GECOS field
    pub fn set_gecos(&mut self, gecos: String) {
        self.gecos = gecos;
    }
}

/// Group account
#[derive(Debug, Clone)]
pub struct Group {
    pub gid: u32,
    pub groupname: String,
    pub members: Vec<String>,
}

impl Group {
    pub fn new(gid: u32, groupname: String) -> Self {
        Self {
            gid,
            groupname,
            members: Vec::new(),
        }
    }

    /// Add member to group
    pub fn add_member(&mut self, username: String) {
        if !self.members.contains(&username) {
            self.members.push(username);
        }
    }

    /// Remove member from group
    pub fn remove_member(&mut self, username: &str) {
        self.members.retain(|m| m != username);
    }
}

/// User/group database
#[derive(Debug, Clone)]
pub struct UserGroupDatabase {
    pub users: HashMap<u32, User>,
    pub groups: HashMap<u32, Group>,
    pub username_to_uid: HashMap<String, u32>,
    pub groupname_to_gid: HashMap<String, u32>,
}

impl UserGroupDatabase {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            groups: HashMap::new(),
            username_to_uid: HashMap::new(),
            groupname_to_gid: HashMap::new(),
        }
    }

    /// Add user
    pub fn add_user(&mut self, user: User) -> Result<(), String> {
        if self.username_to_uid.contains_key(&user.username) {
            return Err(format!("User {} already exists", user.username));
        }

        self.username_to_uid.insert(user.username.clone(), user.uid);
        self.users.insert(user.uid, user);

        Ok(())
    }

    /// Get user by UID
    pub fn get_user_by_uid(&self, uid: u32) -> Option<&User> {
        self.users.get(&uid)
    }

    /// Get user by username
    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.username_to_uid.get(username).and_then(|uid| self.users.get(uid))
    }

    /// Remove user
    pub fn remove_user(&mut self, uid: u32) -> Result<(), String> {
        let user = self.users.remove(&uid).ok_or_else(|| format!("User {} not found", uid))?;
        self.username_to_uid.remove(&user.username);
        Ok(())
    }

    /// Add group
    pub fn add_group(&mut self, group: Group) -> Result<(), String> {
        if self.groupname_to_gid.contains_key(&group.groupname) {
            return Err(format!("Group {} already exists", group.groupname));
        }

        self.groupname_to_gid.insert(group.groupname.clone(), group.gid);
        self.groups.insert(group.gid, group);

        Ok(())
    }

    /// Get group by GID
    pub fn get_group_by_gid(&self, gid: u32) -> Option<&Group> {
        self.groups.get(&gid)
    }

    /// Get group by groupname
    pub fn get_group_by_groupname(&self, groupname: &str) -> Option<&Group> {
        self.groupname_to_gid.get(groupname).and_then(|gid| self.groups.get(gid))
    }

    /// Remove group
    pub fn remove_group(&mut self, gid: u32) -> Result<(), String> {
        let group = self.groups.remove(&gid).ok_or_else(|| format!("Group {} not found", gid))?;
        self.groupname_to_gid.remove(&group.groupname);
        Ok(())
    }

    /// Get user count
    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    /// Get group count
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}

impl Default for UserGroupDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// User/group database manager for system-wide user/group management
pub struct UserGroupManager {
    database: Arc<Mutex<UserGroupDatabase>>,
}

impl UserGroupManager {
    pub fn new() -> Self {
        Self {
            database: Arc::new(Mutex::new(UserGroupDatabase::new())),
        }
    }

    /// Add user
    pub fn add_user(&self, user: User) -> Result<(), String> {
        let mut db = self.database.lock().unwrap();
        db.add_user(user)
    }

    /// Get user by UID
    pub fn get_user_by_uid(&self, uid: u32) -> Option<User> {
        let db = self.database.lock().unwrap();
        db.get_user_by_uid(uid).cloned()
    }

    /// Get user by username
    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        let db = self.database.lock().unwrap();
        db.get_user_by_username(username).cloned()
    }

    /// Remove user
    pub fn remove_user(&self, uid: u32) -> Result<(), String> {
        let mut db = self.database.lock().unwrap();
        db.remove_user(uid)
    }

    /// Add group
    pub fn add_group(&self, group: Group) -> Result<(), String> {
        let mut db = self.database.lock().unwrap();
        db.add_group(group)
    }

    /// Get group by GID
    pub fn get_group_by_gid(&self, gid: u32) -> Option<Group> {
        let db = self.database.lock().unwrap();
        db.get_group_by_gid(gid).cloned()
    }

    /// Get group by groupname
    pub fn get_group_by_groupname(&self, groupname: &str) -> Option<Group> {
        let db = self.database.lock().unwrap();
        db.get_group_by_groupname(groupname).cloned()
    }

    /// Remove group
    pub fn remove_group(&self, gid: u32) -> Result<(), String> {
        let mut db = self.database.lock().unwrap();
        db.remove_group(gid)
    }

    /// Add user to group
    pub fn add_user_to_group(&self, username: String, gid: u32) -> Result<(), String> {
        let mut db = self.database.lock().unwrap();
        match db.groups.get_mut(&gid) {
            Some(group) => {
                group.add_member(username);
                Ok(())
            }
            None => Err(format!("Group {} not found", gid)),
        }
    }

    /// Remove user from group
    pub fn remove_user_from_group(&self, username: &str, gid: u32) -> Result<(), String> {
        let mut db = self.database.lock().unwrap();
        match db.groups.get_mut(&gid) {
            Some(group) => {
                group.remove_member(username);
                Ok(())
            }
            None => Err(format!("Group {} not found", gid)),
        }
    }

    /// Get user count
    pub fn user_count(&self) -> usize {
        let db = self.database.lock().unwrap();
        db.user_count()
    }

    /// Get group count
    pub fn group_count(&self) -> usize {
        let db = self.database.lock().unwrap();
        db.group_count()
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
    fn test_user() {
        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());
        assert_eq!(user.uid, 1000);
        assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_user_set_gecos() {
        let mut user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());
        user.set_gecos("Test User".to_string());
        assert_eq!(user.gecos, "Test User");
    }

    #[test]
    fn test_group() {
        let group = Group::new(1000, "testgroup".to_string());
        assert_eq!(group.gid, 1000);
        assert_eq!(group.groupname, "testgroup");
        assert!(group.members.is_empty());
    }

    #[test]
    fn test_group_add_member() {
        let mut group = Group::new(1000, "testgroup".to_string());
        group.add_member("user1".to_string());
        group.add_member("user2".to_string());

        assert_eq!(group.members.len(), 2);
    }

    #[test]
    fn test_group_add_duplicate_member() {
        let mut group = Group::new(1000, "testgroup".to_string());
        group.add_member("user1".to_string());
        group.add_member("user1".to_string());

        assert_eq!(group.members.len(), 1);
    }

    #[test]
    fn test_group_remove_member() {
        let mut group = Group::new(1000, "testgroup".to_string());
        group.add_member("user1".to_string());
        group.add_member("user2".to_string());
        group.remove_member("user1");

        assert_eq!(group.members.len(), 1);
        assert_eq!(group.members[0], "user2");
    }

    #[test]
    fn test_user_group_database() {
        let db = UserGroupDatabase::new();
        assert_eq!(db.user_count(), 0);
        assert_eq!(db.group_count(), 0);
    }

    #[test]
    fn test_user_group_database_add_user() {
        let mut db = UserGroupDatabase::new();
        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());

        db.add_user(user).unwrap();
        assert_eq!(db.user_count(), 1);
    }

    #[test]
    fn test_user_group_database_add_duplicate_user() {
        let mut db = UserGroupDatabase::new();
        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());

        db.add_user(user.clone()).unwrap();
        assert!(db.add_user(user).is_err());
    }

    #[test]
    fn test_user_group_database_get_user_by_uid() {
        let mut db = UserGroupDatabase::new();
        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());

        db.add_user(user).unwrap();
        let retrieved = db.get_user_by_uid(1000).unwrap();

        assert_eq!(retrieved.username, "testuser");
    }

    #[test]
    fn test_user_group_database_get_user_by_username() {
        let mut db = UserGroupDatabase::new();
        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());

        db.add_user(user).unwrap();
        let retrieved = db.get_user_by_username("testuser").unwrap();

        assert_eq!(retrieved.uid, 1000);
    }

    #[test]
    fn test_user_group_database_remove_user() {
        let mut db = UserGroupDatabase::new();
        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());

        db.add_user(user).unwrap();
        db.remove_user(1000).unwrap();

        assert_eq!(db.user_count(), 0);
    }

    #[test]
    fn test_user_group_database_add_group() {
        let mut db = UserGroupDatabase::new();
        let group = Group::new(1000, "testgroup".to_string());

        db.add_group(group).unwrap();
        assert_eq!(db.group_count(), 1);
    }

    #[test]
    fn test_user_group_manager() {
        let manager = UserGroupManager::new();

        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());
        manager.add_user(user).unwrap();

        assert_eq!(manager.user_count(), 1);
    }

    #[test]
    fn test_user_group_manager_add_user_to_group() {
        let manager = UserGroupManager::new();

        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());
        manager.add_user(user).unwrap();

        let group = Group::new(1000, "testgroup".to_string());
        manager.add_group(group).unwrap();

        manager.add_user_to_group("testuser".to_string(), 1000).unwrap();

        let retrieved_group = manager.get_group_by_gid(1000).unwrap();
        assert_eq!(retrieved_group.members.len(), 1);
    }

    #[test]
    fn test_user_group_manager_remove_user_from_group() {
        let manager = UserGroupManager::new();

        let user = User::new(1000, "testuser".to_string(), 1000, "/home/testuser".to_string(), "/bin/bash".to_string());
        manager.add_user(user).unwrap();

        let group = Group::new(1000, "testgroup".to_string());
        manager.add_group(group).unwrap();

        manager.add_user_to_group("testuser".to_string(), 1000).unwrap();
        manager.remove_user_from_group("testuser", 1000).unwrap();

        let retrieved_group = manager.get_group_by_gid(1000).unwrap();
        assert_eq!(retrieved_group.members.len(), 0);
    }

    #[test]
    fn test_user_group_manager_invalid() {
        let manager = UserGroupManager::new();
        assert!(manager.get_user_by_uid(999).is_none());
        assert!(manager.get_group_by_gid(999).is_none());
    }
}
