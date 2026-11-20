// SPDX-License-Identifier: MIT
// SigmaOS User Manager - Arch Linux Inspired
// Implements user and group management similar to Arch's useradd/usermod/groupadd

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// User information
#[derive(Debug, Clone, PartialEq)]
pub struct UserInfo {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub gecos: String,
    pub home_dir: PathBuf,
    pub shell: String,
    pub password_hash: Option<String>,
    pub password_last_changed: Option<u64>,
    pub account_expiry: Option<u64>,
    pub password_expiry: Option<u64>,
    pub groups: Vec<String>,
    pub is_locked: bool,
    pub is_system_user: bool,
    pub created_at: u64,
    pub modified_at: u64,
}

impl UserInfo {
    /// Create a new user info
    pub fn new(username: impl Into<String>, uid: u32, gid: u32) -> Self {
        Self {
            username: username.into(),
            uid,
            gid,
            gecos: String::new(),
            home_dir: PathBuf::from(format!("/home/{}", username.into())),
            shell: "/bin/bash".to_string(),
            password_hash: None,
            password_last_changed: None,
            account_expiry: None,
            password_expiry: None,
            groups: Vec::new(),
            is_locked: false,
            is_system_user: false,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            modified_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        }
    }

    /// Set GECOS (user information)
    pub fn gecos(mut self, gecos: impl Into<String>) -> Self {
        self.gecos = gecos.into();
        self
    }

    /// Set home directory
    pub fn home_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.home_dir = dir.into();
        self
    }

    /// Set shell
    pub fn shell(mut self, shell: impl Into<String>) -> Self {
        self.shell = shell.into();
        self
    }

    /// Set password hash
    pub fn password_hash(mut self, hash: impl Into<String>) -> Self {
        self.password_hash = Some(hash.into());
        self
    }

    /// Set password last changed
    pub fn password_last_changed(mut self, time: u64) -> Self {
        self.password_last_changed = Some(time);
        self
    }

    /// Set account expiry
    pub fn account_expiry(mut self, time: u64) -> Self {
        self.account_expiry = Some(time);
        self
    }

    /// Set password expiry
    pub fn password_expiry(mut self, time: u64) -> Self {
        self.password_expiry = Some(time);
        self
    }

    /// Add group
    pub fn add_group(mut self, group: impl Into<String>) -> Self {
        self.groups.push(group.into());
        self
    }

    /// Set locked status
    pub fn locked(mut self, locked: bool) -> Self {
        self.is_locked = locked;
        self
    }

    /// Set system user
    pub fn system_user(mut self, system: bool) -> Self {
        self.is_system_user = system;
        self
    }

    /// Check if user is locked
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    /// Check if user is system user
    pub fn is_system_user(&self) -> bool {
        self.is_system_user
    }

    /// Check if user can login
    pub fn can_login(&self) -> bool {
        !self.is_locked && self.shell != "/usr/sbin/nologin" && self.shell != "/bin/false"
    }

    /// Get user age in days
    pub fn get_age_days(&self) -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        (now - self.created_at) / 86400
    }

    /// Get days until password expiry
    pub fn get_days_until_password_expiry(&self) -> Option<i64> {
        if let Some(expiry) = self.password_expiry {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
            Some((expiry as i64 - now as i64) / 86400)
        } else {
            None
        }
    }

    /// Get days until account expiry
    pub fn get_days_until_account_expiry(&self) -> Option<i64> {
        if let Some(expiry) = self.account_expiry {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
            Some((expiry as i64 - now as i64) / 86400)
        } else {
            None
        }
    }
}

/// Group information
#[derive(Debug, Clone, PartialEq)]
pub struct GroupInfo {
    pub groupname: String,
    pub gid: u32,
    pub members: Vec<String>,
    pub is_system_group: bool,
    pub created_at: u64,
    pub modified_at: u64,
}

impl GroupInfo {
    /// Create a new group info
    pub fn new(groupname: impl Into<String>, gid: u32) -> Self {
        Self {
            groupname: groupname.into(),
            gid,
            members: Vec::new(),
            is_system_group: false,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            modified_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        }
    }

    /// Add member
    pub fn add_member(mut self, member: impl Into<String>) -> Self {
        self.members.push(member.into());
        self
    }

    /// Set system group
    pub fn system_group(mut self, system: bool) -> Self {
        self.is_system_group = system;
        self
    }

    /// Check if group is system group
    pub fn is_system_group(&self) -> bool {
        self.is_system_group
    }

    /// Check if user is member
    pub fn has_member(&self, username: &str) -> bool {
        self.members.contains(&username.to_string())
    }

    /// Get member count
    pub fn get_member_count(&self) -> usize {
        self.members.len()
    }
}

/// User manager
#[derive(Debug, Clone)]
pub struct UserManager {
    pub users: BTreeMap<String, UserInfo>,
    pub groups: BTreeMap<String, GroupInfo>,
    pub passwd_file: PathBuf,
    pub shadow_file: PathBuf,
    pub group_file: PathBuf,
    pub gshadow_file: PathBuf,
    pub last_sync: Option<u64>,
}

impl UserManager {
    /// Create a new user manager
    pub fn new() -> Self {
        Self {
            users: BTreeMap::new(),
            groups: BTreeMap::new(),
            passwd_file: PathBuf::from("/etc/passwd"),
            shadow_file: PathBuf::from("/etc/shadow"),
            group_file: PathBuf::from("/etc/group"),
            gshadow_file: PathBuf::from("/etc/gshadow"),
            last_sync: None,
        }
    }

    /// Create user manager with custom paths
    pub fn with_paths(passwd: PathBuf, shadow: PathBuf, group: PathBuf, gshadow: PathBuf) -> Self {
        Self {
            users: BTreeMap::new(),
            groups: BTreeMap::new(),
            passwd_file: passwd,
            shadow_file: shadow,
            group_file: group,
            gshadow_file: gshadow,
            last_sync: None,
        }
    }

    /// Load users and groups from system files
    pub fn load_system(&mut self) -> Result<(), String> {
        self.load_passwd()?;
        self.load_shadow()?;
        self.load_group()?;
        self.load_gshadow()?;

        self.last_sync = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        Ok(())
    }

    /// Load /etc/passwd
    fn load_passwd(&mut self) -> Result<(), String> {
        if !self.passwd_file.exists() {
            return Ok(());
        }

        let file = fs::File::open(&self.passwd_file)
            .map_err(|e| format!("Failed to open passwd file: {}", e))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read passwd line: {}", e))?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse passwd entry: username:password:uid:gid:gecos:home:shell
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 7 {
                let username = parts[0].to_string();
                let uid: u32 = parts[2].parse().unwrap_or(1000);
                let gid: u32 = parts[3].parse().unwrap_or(1000);
                let gecos = parts[4].to_string();
                let home_dir = PathBuf::from(parts[5]);
                let shell = parts[6].to_string();

                let mut user = UserInfo::new(username.clone(), uid, gid)
                    .gecos(gecos)
                    .home_dir(home_dir)
                    .shell(shell);

                // Check if system user
                if uid < 1000 {
                    user = user.system_user(true);
                }

                self.users.insert(username, user);
            }
        }

        Ok(())
    }

    /// Load /etc/shadow
    fn load_shadow(&mut self) -> Result<(), String> {
        if !self.shadow_file.exists() {
            return Ok(());
        }

        let file = fs::File::open(&self.shadow_file)
            .map_err(|e| format!("Failed to open shadow file: {}", e))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read shadow line: {}", e))?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse shadow entry: username:password:lastchanged:min:max:warn:inactive:expire:flag
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let username = parts[0].to_string();
                let password_hash = if parts[1].is_empty() || parts[1] == "!" || parts[1] == "*" {
                    None
                } else {
                    Some(parts[1].to_string())
                };

                if let Some(user) = self.users.get_mut(&username) {
                    user.password_hash = password_hash;

                    // Parse password last changed
                    if parts.len() > 2 {
                        if let Ok(last_changed) = parts[2].parse::<u64>() {
                            user.password_last_changed = Some(last_changed);
                        }
                    }

                    // Parse password expiry
                    if parts.len() > 4 {
                        if let Ok(expiry) = parts[4].parse::<u64>() {
                            user.password_expiry = Some(expiry);
                        }
                    }

                    // Parse account expiry
                    if parts.len() > 7 {
                        if let Ok(expiry) = parts[7].parse::<u64>() {
                            user.account_expiry = Some(expiry);
                        }
                    }

                    // Check if locked
                    if parts[1].starts_with('!') {
                        user.is_locked = true;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load /etc/group
    fn load_group(&mut self) -> Result<(), String> {
        if !self.group_file.exists() {
            return Ok(());
        }

        let file = fs::File::open(&self.group_file)
            .map_err(|e| format!("Failed to open group file: {}", e))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read group line: {}", e))?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse group entry: groupname:password:gid:members
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 4 {
                let groupname = parts[0].to_string();
                let gid: u32 = parts[2].parse().unwrap_or(1000);
                let members: Vec<String> = parts[3].split(',')
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();

                let mut group = GroupInfo::new(groupname.clone(), gid);

                // Add members
                for member in members {
                    group.members.push(member);
                }

                // Check if system group
                if gid < 1000 {
                    group.is_system_group = true;
                }

                self.groups.insert(groupname, group);
            }
        }

        Ok(())
    }

    /// Load /etc/gshadow
    fn load_gshadow(&mut self) -> Result<(), String> {
        if !self.gshadow_file.exists() {
            return Ok(());
        }

        // gshadow contains group passwords and admin members
        // For now, just load it to ensure it exists
        Ok(())
    }

    /// Save users and groups to system files
    pub fn save_system(&self) -> Result<(), String> {
        self.save_passwd()?;
        self.save_shadow()?;
        self.save_group()?;
        self.save_gshadow()?;
        Ok(())
    }

    /// Save /etc/passwd
    fn save_passwd(&self) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# SigmaOS /etc/passwd\n");
        content.push_str("# Generated by SigmaOS User Manager\n\n");

        for user in self.users.values() {
            content.push_str(&format!("{}:x:{}:{}:{}:{}:{}\n",
                user.username,
                user.uid,
                user.gid,
                user.gecos,
                user.home_dir.display(),
                user.shell
            ));
        }

        fs::write(&self.passwd_file, content)
            .map_err(|e| format!("Failed to save passwd file: {}", e))
    }

    /// Save /etc/shadow
    fn save_shadow(&self) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# SigmaOS /etc/shadow\n");
        content.push_str("# Generated by SigmaOS User Manager\n\n");

        for user in self.users.values() {
            let password = user.password_hash.as_deref().unwrap_or("!");
            let last_changed = user.password_last_changed.unwrap_or(0);
            let min_days = "0";
            let max_days = "99999";
            let warn_days = "7";
            let inactive_days = "";
            let expire_date = user.account_expiry.unwrap_or(0);

            content.push_str(&format!("{}:{}:{}:{}:{}:{}:{}:{}\n",
                user.username,
                password,
                last_changed,
                min_days,
                max_days,
                warn_days,
                inactive_days,
                expire_date
            ));
        }

        fs::write(&self.shadow_file, content)
            .map_err(|e| format!("Failed to save shadow file: {}", e))
    }

    /// Save /etc/group
    fn save_group(&self) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# SigmaOS /etc/group\n");
        content.push_str("# Generated by SigmaOS User Manager\n\n");

        for group in self.groups.values() {
            let members = group.members.join(",");
            content.push_str(&format!("{}:x:{}:{}\n",
                group.groupname,
                group.gid,
                members
            ));
        }

        fs::write(&self.group_file, content)
            .map_err(|e| format!("Failed to save group file: {}", e))
    }

    /// Save /etc/gshadow
    fn save_gshadow(&self) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# SigmaOS /etc/gshadow\n");
        content.push_str("# Generated by SigmaOS User Manager\n\n");

        for group in self.groups.values() {
            let members = group.members.join(",");
            content.push_str(&format!("{}:!::{}\n",
                group.groupname,
                members
            ));
        }

        fs::write(&self.gshadow_file, content)
            .map_err(|e| format!("Failed to save gshadow file: {}", e))
    }

    /// Add user
    pub fn add_user(&mut self, user: UserInfo) -> Result<(), String> {
        if self.users.contains_key(&user.username) {
            return Err(format!("User '{}' already exists", user.username));
        }

        // Check for UID conflicts
        if self.users.values().any(|u| u.uid == user.uid) {
            return Err(format!("UID {} already in use", user.uid));
        }

        // Create home directory
        if !user.home_dir.exists() {
            fs::create_dir_all(&user.home_dir)
                .map_err(|e| format!("Failed to create home directory: {}", e))?;
        }

        // Set ownership
        // In real implementation, would use chown
        // For now, just log it
        println!("Setting ownership of {} to {}:{}", user.home_dir.display(), user.uid, user.gid);

        self.users.insert(user.username.clone(), user);
        Ok(())
    }

    /// Remove user
    pub fn remove_user(&mut self, username: &str, remove_home: bool) -> Result<(), String> {
        if !self.users.contains_key(username) {
            return Err(format!("User '{}' not found", username));
        }

        // Remove from groups
        for group in self.groups.values_mut() {
            group.members.retain(|m| m != username);
        }

        // Remove user
        self.users.remove(username);

        // Remove home directory if requested
        if remove_home {
            if let Some(user) = self.users.get(username) {
                if user.home_dir.exists() {
                    fs::remove_dir_all(&user.home_dir)
                        .map_err(|e| format!("Failed to remove home directory: {}", e))?;
                }
            }
        }

        Ok(())
    }

    /// Get user
    pub fn get_user(&self, username: &str) -> Option<&UserInfo> {
        self.users.get(username)
    }

    /// Get user mutably
    pub fn get_user_mut(&mut self, username: &str) -> Option<&mut UserInfo> {
        self.users.get_mut(username)
    }

    /// Get all users
    pub fn get_all_users(&self) -> Vec<&UserInfo> {
        self.users.values().collect()
    }

    /// Get users by UID range
    pub fn get_users_by_uid_range(&self, min_uid: u32, max_uid: u32) -> Vec<&UserInfo> {
        self.users.values()
            .filter(|u| u.uid >= min_uid && u.uid <= max_uid)
            .collect()
    }

    /// Get system users
    pub fn get_system_users(&self) -> Vec<&UserInfo> {
        self.users.values()
            .filter(|u| u.is_system_user)
            .collect()
    }

    /// Get regular users
    pub fn get_regular_users(&self) -> Vec<&UserInfo> {
        self.users.values()
            .filter(|u| !u.is_system_user)
            .collect()
    }

    /// Get users with expired passwords
    pub fn get_users_with_expired_passwords(&self) -> Vec<&UserInfo> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        self.users.values()
            .filter(|u| {
                if let Some(expiry) = u.password_expiry {
                    expiry < now
                } else {
                    false
                }
            })
            .collect()
    }

    /// Get users with expired accounts
    pub fn get_users_with_expired_accounts(&self) -> Vec<&UserInfo> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        self.users.values()
            .filter(|u| {
                if let Some(expiry) = u.account_expiry {
                    expiry < now
                } else {
                    false
                }
            })
            .collect()
    }

    /// Get users that need password change
    pub fn get_users_needing_password_change(&self) -> Vec<&UserInfo> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        self.users.values()
            .filter(|u| {
                if let Some(last_changed) = u.password_last_changed {
                    // Assume password expires after 90 days
                    (now - last_changed) > 90 * 86400
                } else {
                    true // Never changed password
                }
            })
            .collect()
    }

    /// Add group
    pub fn add_group(&mut self, group: GroupInfo) -> Result<(), String> {
        if self.groups.contains_key(&group.groupname) {
            return Err(format!("Group '{}' already exists", group.groupname));
        }

        // Check for GID conflicts
        if self.groups.values().any(|g| g.gid == group.gid) {
            return Err(format!("GID {} already in use", group.gid));
        }

        self.groups.insert(group.groupname.clone(), group);
        Ok(())
    }

    /// Remove group
    pub fn remove_group(&mut self, groupname: &str) -> Result<(), String> {
        if !self.groups.contains_key(groupname) {
            return Err(format!("Group '{}' not found", groupname));
        }

        self.groups.remove(groupname);
        Ok(())
    }

    /// Get group
    pub fn get_group(&self, groupname: &str) -> Option<&GroupInfo> {
        self.groups.get(groupname)
    }

    /// Get group mutably
    pub fn get_group_mut(&mut self, groupname: &str) -> Option<&mut GroupInfo> {
        self.groups.get_mut(groupname)
    }

    /// Get all groups
    pub fn get_all_groups(&self) -> Vec<&GroupInfo> {
        self.groups.values().collect()
    }

    /// Get groups by GID range
    pub fn get_groups_by_gid_range(&self, min_gid: u32, max_gid: u32) -> Vec<&GroupInfo> {
        self.groups.values()
            .filter(|g| g.gid >= min_gid && g.gid <= max_gid)
            .collect()
    }

    /// Get system groups
    pub fn get_system_groups(&self) -> Vec<&GroupInfo> {
        self.groups.values()
            .filter(|g| g.is_system_group)
            .collect()
    }

    /// Get regular groups
    pub fn get_regular_groups(&self) -> Vec<&GroupInfo> {
        self.groups.values()
            .filter(|g| !g.is_system_group)
            .collect()
    }

    /// Add user to group
    pub fn add_user_to_group(&mut self, username: &str, groupname: &str) -> Result<(), String> {
        // Check if user exists
        if !self.users.contains_key(username) {
            return Err(format!("User '{}' not found", username));
        }

        // Check if group exists
        if !self.groups.contains_key(groupname) {
            return Err(format!("Group '{}' not found", groupname));
        }

        // Add user to group
        if let Some(group) = self.groups.get_mut(groupname) {
            if !group.members.contains(&username.to_string()) {
                group.members.push(username.to_string());
            }
        }

        // Add group to user
        if let Some(user) = self.users.get_mut(username) {
            if !user.groups.contains(&groupname.to_string()) {
                user.groups.push(groupname.to_string());
            }
        }

        Ok(())
    }

    /// Remove user from group
    pub fn remove_user_from_group(&mut self, username: &str, groupname: &str) -> Result<(), String> {
        // Check if group exists
        if let Some(group) = self.groups.get_mut(groupname) {
            group.members.retain(|m| m != username);
        }

        // Remove group from user
        if let Some(user) = self.users.get_mut(username) {
            user.groups.retain(|g| g != groupname);
        }

        Ok(())
    }

    /// Get user's groups
    pub fn get_user_groups(&self, username: &str) -> Vec<String> {
        self.users.get(username)
            .map(|u| u.groups.clone())
            .unwrap_or_default()
    }

    /// Get group's members
    pub fn get_group_members(&self, groupname: &str) -> Vec<String> {
        self.groups.get(groupname)
            .map(|g| g.members.clone())
            .unwrap_or_default()
    }

    /// Change user password
    pub fn change_password(&mut self, username: &str, new_password: &str) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(username) {
            // In real implementation, would hash the password
            // For now, just store it
            user.password_hash = Some(format!("$6${}", new_password));
            user.password_last_changed = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs());
            user.modified_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            Ok(())
        } else {
            Err(format!("User '{}' not found", username))
        }
    }

    /// Lock user account
    pub fn lock_user(&mut self, username: &str) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(username) {
            user.is_locked = true;
            user.modified_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            Ok(())
        } else {
            Err(format!("User '{}' not found", username))
        }
    }

    /// Unlock user account
    pub fn unlock_user(&mut self, username: &str) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(username) {
            user.is_locked = false;
            user.modified_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            Ok(())
        } else {
            Err(format!("User '{}' not found", username))
        }
    }

    /// Change user shell
    pub fn change_shell(&mut self, username: &str, shell: &str) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(username) {
            user.shell = shell.to_string();
            user.modified_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            Ok(())
        } else {
            Err(format!("User '{}' not found", username))
        }
    }

    /// Change user home directory
    pub fn change_home_dir(&mut self, username: &str, home_dir: &str) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(username) {
            user.home_dir = PathBuf::from(home_dir);
            user.modified_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            Ok(())
        } else {
            Err(format!("User '{}' not found", username))
        }
    }

    /// Change user GECOS
    pub fn change_gecos(&mut self, username: &str, gecos: &str) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(username) {
            user.gecos = gecos.to_string();
            user.modified_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            Ok(())
        } else {
            Err(format!("User '{}' not found", username))
        }
    }

    /// Set user expiry
    pub fn set_user_expiry(&mut self, username: &str, expiry: Option<u64>) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(username) {
            user.account_expiry = expiry;
            user.modified_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            Ok(())
        } else {
            Err(format!("User '{}' not found", username))
        }
    }

    /// Set password expiry
    pub fn set_password_expiry(&mut self, username: &str, expiry: Option<u64>) -> Result<(), String> {
        if let Some(user) = self.users.get_mut(username) {
            user.password_expiry = expiry;
            user.modified_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            Ok(())
        } else {
            Err(format!("User '{}' not found", username))
        }
    }

    /// Get next available UID
    pub fn get_next_uid(&self) -> u32 {
        self.users.values()
            .map(|u| u.uid)
            .max()
            .unwrap_or(1000) + 1
    }

    /// Get next available GID
    pub fn get_next_gid(&self) -> u32 {
        self.groups.values()
            .map(|g| g.gid)
            .max()
            .unwrap_or(1000) + 1
    }

    /// Create user with automatic UID/GID
    pub fn create_user(&mut self, username: &str, password: &str, gecos: &str, groups: &[&str]) -> Result<UserInfo, String> {
        let uid = self.get_next_uid();
        let gid = self.get_next_gid();

        let mut user = UserInfo::new(username, uid, gid)
            .gecos(gecos)
            .shell("/bin/bash")
            .password_hash(format!("$6${}", password));

        // Add groups
        for group in groups {
            user = user.add_group(group.to_string());
            self.add_user_to_group(username, group)?;
        }

        // Add primary group
        let primary_group = format!("{}", username);
        let group = GroupInfo::new(&primary_group, gid);
        self.add_group(group)?;
        user = user.add_group(primary_group.clone());
        self.add_user_to_group(username, &primary_group)?;

        self.add_user(user.clone())?;
        Ok(user)
    }

    /// Export user information
    pub fn export_users(&self) -> String {
        let mut content = String::new();
        content.push_str("# SigmaOS Users Export\n");
        content.push_str("# Generated by SigmaOS User Manager\n\n");

        for user in self.users.values() {
            content.push_str(&format!("user={} uid={} gid={} gecos=\"{}\" home={} shell={} groups={} locked={} system={}\n",
                user.username,
                user.uid,
                user.gid,
                user.gecos,
                user.home_dir.display(),
                user.shell,
                user.groups.join(","),
                user.is_locked,
                user.is_system_user
            ));
        }

        content.push_str("\n## Groups\n");
        for group in self.groups.values() {
            content.push_str(&format!("group={} gid={} members={} system={}\n",
                group.groupname,
                group.gid,
                group.members.join(","),
                group.is_system_group
            ));
        }

        content
    }

    /// Import user information
    pub fn import_users(&mut self, content: &str) -> Result<(), String> {
        self.users.clear();
        self.groups.clear();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("user=") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let mut username = "";
                let mut uid = 1000;
                let mut gid = 1000;
                let mut gecos = "";
                let mut home = PathBuf::new();
                let mut shell = "/bin/bash".to_string();
                let mut groups = Vec::new();
                let mut locked = false;
                let mut system = false;

                for part in parts {
                    if let Some((key, value)) = part.split_once('=') {
                        match key {
                            "user" => username = value,
                            "uid" => uid = value.parse().unwrap_or(1000),
                            "gid" => gid = value.parse().unwrap_or(1000),
                            "gecos" => gecos = value.trim_matches('"'),
                            "home" => home = PathBuf::from(value),
                            "shell" => shell = value.to_string(),
                            "groups" => groups = value.split(',').map(String::from).collect(),
                            "locked" => locked = value == "true",
                            "system" => system = value == "true",
                            _ => {}
                        }
                    }
                }

                let mut user = UserInfo::new(username, uid, gid)
                    .gecos(gecos)
                    .home_dir(home)
                    .shell(shell)
                    .locked(locked)
                    .system_user(system);

                for group in groups {
                    user = user.add_group(group);
                }

                self.users.insert(username.to_string(), user);
            } else if line.starts_with("group=") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let mut groupname = "";
                let mut gid = 1000;
                let mut members = Vec::new();
                let mut system = false;

                for part in parts {
                    if let Some((key, value)) = part.split_once('=') {
                        match key {
                            "group" => groupname = value,
                            "gid" => gid = value.parse().unwrap_or(1000),
                            "members" => members = value.split(',').map(String::from).collect(),
                            "system" => system = value == "true",
                            _ => {}
                        }
                    }
                }

                let mut group = GroupInfo::new(groupname, gid)
                    .system_group(system);

                for member in members {
                    group.members.push(member);
                }

                self.groups.insert(groupname.to_string(), group);
            }
        }

        Ok(())
    }

    /// Get user statistics
    pub fn get_statistics(&self) -> UserStatistics {
        let total_users = self.users.len();
        let system_users = self.users.values().filter(|u| u.is_system_user).count();
        let regular_users = total_users - system_users;
        let locked_users = self.users.values().filter(|u| u.is_locked).count();
        let expired_users = self.get_users_with_expired_accounts().len();
        let password_expired_users = self.get_users_with_expired_passwords().len();

        let total_groups = self.groups.len();
        let system_groups = self.groups.values().filter(|g| g.is_system_group).count();
        let regular_groups = total_groups - system_groups;

        UserStatistics {
            total_users,
            system_users,
            regular_users,
            locked_users,
            expired_users,
            password_expired_users,
            total_groups,
            system_groups,
            regular_groups,
            last_sync: self.last_sync,
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
    pub system_users: usize,
    pub regular_users: usize,
    pub locked_users: usize,
    pub expired_users: usize,
    pub password_expired_users: usize,
    pub total_groups: usize,
    pub system_groups: usize,
    pub regular_groups: usize,
    pub last_sync: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_info() {
        let user = UserInfo::new("testuser", 1000, 1000)
            .gecos("Test User")
            .shell("/bin/bash")
            .password_hash("$6$hash")
            .add_group("wheel")
            .add_group("users");

        assert_eq!(user.username, "testuser");
        assert_eq!(user.uid, 1000);
        assert_eq!(user.gid, 1000);
        assert_eq!(user.gecos, "Test User");
        assert_eq!(user.shell, "/bin/bash");
        assert_eq!(user.groups.len(), 2);
        assert!(!user.is_locked);
        assert!(user.can_login());
    }

    #[test]
    fn test_group_info() {
        let group = GroupInfo::new("testgroup", 1000)
            .add_member("user1")
            .add_member("user2")
            .system_group(false);

        assert_eq!(group.groupname, "testgroup");
        assert_eq!(group.gid, 1000);
        assert_eq!(group.members.len(), 2);
        assert!(!group.is_system_group);
        assert!(group.has_member("user1"));
        assert!(!group.has_member("user3"));
    }

    #[test]
    fn test_user_manager() {
        let mut manager = UserManager::new();

        let user = UserInfo::new("testuser", 1000, 1000)
            .gecos("Test User")
            .shell("/bin/bash");

        assert!(manager.add_user(user).is_ok());
        assert!(manager.get_user("testuser").is_some());
        assert_eq!(manager.get_all_users().len(), 1);
    }

    #[test]
    fn test_group_management() {
        let mut manager = UserManager::new();

        let group = GroupInfo::new("testgroup", 1000);
        assert!(manager.add_group(group).is_ok());
        assert!(manager.get_group("testgroup").is_some());
        assert_eq!(manager.get_all_groups().len(), 1);
    }

    #[test]
    fn test_user_group_membership() {
        let mut manager = UserManager::new();

        let user = UserInfo::new("testuser", 1000, 1000);
        let group = GroupInfo::new("testgroup", 1000);

        manager.add_user(user).unwrap();
        manager.add_group(group).unwrap();

        assert!(manager.add_user_to_group("testuser", "testgroup").is_ok());
        assert!(manager.get_user_groups("testuser").contains(&"testgroup".to_string()));
        assert!(manager.get_group_members("testgroup").contains(&"testuser".to_string()));
    }

    #[test]
    fn test_password_management() {
        let mut manager = UserManager::new();

        let user = UserInfo::new("testuser", 1000, 1000);
        manager.add_user(user).unwrap();

        assert!(manager.change_password("testuser", "newpassword").is_ok());
        let user = manager.get_user("testuser").unwrap();
        assert!(user.password_hash.is_some());
        assert!(user.password_last_changed.is_some());
    }

    #[test]
    fn test_user_locking() {
        let mut manager = UserManager::new();

        let user = UserInfo::new("testuser", 1000, 1000);
        manager.add_user(user).unwrap();

        assert!(manager.lock_user("testuser").is_ok());
        assert!(manager.get_user("testuser").unwrap().is_locked);

        assert!(manager.unlock_user("testuser").is_ok());
        assert!(!manager.get_user("testuser").unwrap().is_locked);
    }

    #[test]
    fn test_user_expiry() {
        let mut manager = UserManager::new();

        let user = UserInfo::new("testuser", 1000, 1000);
        manager.add_user(user).unwrap();

        let expiry = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 86400; // 1 day from now
        assert!(manager.set_user_expiry("testuser", Some(expiry)).is_ok());
        assert_eq!(manager.get_user("testuser").unwrap().account_expiry, Some(expiry));
    }

    #[test]
    fn test_create_user() {
        let mut manager = UserManager::new();

        let result = manager.create_user("newuser", "password", "New User", &["wheel", "users"]);
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.username, "newuser");
        assert_eq!(user.uid, 1000);
        assert_eq!(user.gid, 1000);
        assert!(user.groups.contains(&"wheel".to_string()));
        assert!(user.groups.contains(&"users".to_string()));
    }

    #[test]
    fn test_user_export_import() {
        let mut manager = UserManager::new();

        let user = UserInfo::new("testuser", 1000, 1000)
            .gecos("Test User")
            .shell("/bin/bash")
            .add_group("wheel");

        manager.add_user(user).unwrap();

        let group = GroupInfo::new("wheel", 10)
            .add_member("testuser");
        manager.add_group(group).unwrap();

        let export = manager.export_users();
        assert!(export.contains("testuser"));
        assert!(export.contains("wheel"));

        let mut manager2 = UserManager::new();
        assert!(manager2.import_users(&export).is_ok());
        assert!(manager2.get_user("testuser").is_some());
    }

    #[test]
    fn test_user_statistics() {
        let mut manager = UserManager::new();

        let user1 = UserInfo::new("user1", 1000, 1000).system_user(false);
        let user2 = UserInfo::new("user2", 1001, 1001).system_user(false).locked(true);
        let user3 = UserInfo::new("system", 0, 0).system_user(true);

        manager.add_user(user1).unwrap();
        manager.add_user(user2).unwrap();
        manager.add_user(user3).unwrap();

        let group = GroupInfo::new("users", 100);
        manager.add_group(group).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_users, 3);
        assert_eq!(stats.system_users, 1);
        assert_eq!(stats.regular_users, 2);
        assert_eq!(stats.locked_users, 1);
        assert_eq!(stats.total_groups, 1);
    }

    #[test]
    fn test_get_next_uid_gid() {
        let mut manager = UserManager::new();

        let user1 = UserInfo::new("user1", 1000, 1000);
        let user2 = UserInfo::new("user2", 1001, 1001);

        manager.add_user(user1).unwrap();
        manager.add_user(user2).unwrap();

        let group1 = GroupInfo::new("group1", 1000);
        let group2 = GroupInfo::new("group2", 1001);

        manager.add_group(group1).unwrap();
        manager.add_group(group2).unwrap();

        assert_eq!(manager.get_next_uid(), 1002);
        assert_eq!(manager.get_next_gid(), 1002);
    }
}
