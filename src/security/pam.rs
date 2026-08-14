// Pluggable Authentication Modules (PAM) and Multi-User Access Control Subsystem
// Inspired by Linux PAM and BSD pw/group databases.

extern crate alloc;

#[cfg(not(target_os = "none"))]
use alloc::collections::BTreeMap;
#[cfg(target_os = "none")]
use crate::klib::BTreeMap;

use crate::security::crypto_utils::{constant_time_eq, hash_password_placeholder, SecureRandom};
use alloc::string::{String as AllocString, ToString};
use alloc::vec::Vec;

/// Errors returned by the PAM subsystem
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PamError {
    UserNotFound,
    GroupNotFound,
    AuthenticationFailed,
    AccountLocked,
    PasswordTooWeak,
    UserAlreadyExists,
    GroupAlreadyExists,
    PermissionDenied,
}

/// User entry representing shadow-file-like secure user information
#[derive(Debug, Clone)]
pub struct PamUser {
    pub uid: u32,
    pub username: AllocString,
    pub password_hash: [u8; 32],
    pub salt: [u8; 16],
    pub primary_group: AllocString,
    pub is_locked: bool,
    pub failed_attempts: u32,
}

/// Group entry representing standard Unix-like groups
#[derive(Debug, Clone)]
pub struct PamGroup {
    pub gid: u32,
    pub name: AllocString,
    pub members: Vec<AllocString>,
}

/// Dynamic, pluggable service modules enforcing policy-driven authorization checks
pub trait PamModule {
    fn name(&self) -> &'static str;
    fn authenticate(&self, user: &PamUser, password: &str) -> Result<(), PamError>;
    fn validate_account(&self, user: &PamUser) -> Result<(), PamError>;
}

/// Enforces password complexity requirements (Linux-style pam_pwquality)
pub struct PasswordQualityModule {
    pub min_length: usize,
}

impl PamModule for PasswordQualityModule {
    fn name(&self) -> &'static str {
        "pam_pwquality"
    }

    fn authenticate(&self, _user: &PamUser, password: &str) -> Result<(), PamError> {
        if password.len() < self.min_length {
            return Err(PamError::PasswordTooWeak);
        }
        Ok(())
    }

    fn validate_account(&self, _user: &PamUser) -> Result<(), PamError> {
        Ok(())
    }
}

/// Enforces account lockouts after excessive failed attempts (Linux-style pam_tally2)
pub struct AccountTallyModule {
    pub max_failed_attempts: u32,
}

impl PamModule for AccountTallyModule {
    fn name(&self) -> &'static str {
        "pam_tally2"
    }

    fn authenticate(&self, user: &PamUser, _password: &str) -> Result<(), PamError> {
        if user.is_locked || user.failed_attempts >= self.max_failed_attempts {
            return Err(PamError::AccountLocked);
        }
        Ok(())
    }

    fn validate_account(&self, user: &PamUser) -> Result<(), PamError> {
        if user.is_locked || user.failed_attempts >= self.max_failed_attempts {
            return Err(PamError::AccountLocked);
        }
        Ok(())
    }
}

/// Central registry managing user authentication, groups, and PAM configuration
pub struct SovereignPamManager {
    pub users: BTreeMap<AllocString, PamUser>,
    pub groups: BTreeMap<AllocString, PamGroup>,
    pub modules: Vec<alloc::boxed::Box<dyn PamModule>>,
    pub next_uid: u32,
    pub next_gid: u32,
}

impl SovereignPamManager {
    /// Initialize a new PAM manager
    pub fn new() -> Self {
        Self {
            users: BTreeMap::<AllocString, PamUser>::new(),
            groups: BTreeMap::<AllocString, PamGroup>::new(),
            modules: Vec::new(),
            next_uid: 1000,
            next_gid: 1000,
        }
    }

    /// Add a pluggable authentication module to the stack
    pub fn register_module(&mut self, module: alloc::boxed::Box<dyn PamModule>) {
        self.modules.push(module);
    }

    /// Register a new user with secure password salting
    pub fn register_user(&mut self, username: &str, password: &str, primary_group: &str) -> Result<u32, PamError> {
        if self.users.get(&username.to_string()).is_some() {
            return Err(PamError::UserAlreadyExists);
        }

        // Validate password against Registered Quality modules if present
        for module in &self.modules {
            if module.name() == "pam_pwquality" {
                let dummy_user = PamUser {
                    uid: 0,
                    username: username.to_string(),
                    password_hash: [0; 32],
                    salt: [0; 16],
                    primary_group: primary_group.to_string(),
                    is_locked: false,
                    failed_attempts: 0,
                };
                module.authenticate(&dummy_user, password)?;
            }
        }

        let mut rng = SecureRandom::new();
        let mut salt = [0u8; 16];
        rng.fill_bytes(&mut salt).map_err(|_| PamError::AuthenticationFailed)?;

        let hash = hash_password_placeholder(password, &salt);

        let uid = self.next_uid;
        self.next_uid += 1;

        let user = PamUser {
            uid,
            username: username.to_string(),
            password_hash: hash,
            salt,
            primary_group: primary_group.to_string(),
            is_locked: false,
            failed_attempts: 0,
        };

        self.users.insert(username.to_string(), user);

        // Add user to their primary group
        self.add_user_to_group(username, primary_group)?;

        Ok(uid)
    }

    /// Create a system group
    pub fn create_group(&mut self, group_name: &str) -> Result<u32, PamError> {
        if self.groups.get(&group_name.to_string()).is_some() {
            return Err(PamError::GroupAlreadyExists);
        }

        let gid = self.next_gid;
        self.next_gid += 1;

        let group = PamGroup {
            gid,
            name: group_name.to_string(),
            members: Vec::new(),
        };

        self.groups.insert(group_name.to_string(), group);
        Ok(gid)
    }

    /// Add a user to a group
    pub fn add_user_to_group(&mut self, username: &str, group_name: &str) -> Result<(), PamError> {
        if self.users.get(&username.to_string()).is_none() {
            return Err(PamError::UserNotFound);
        }

        if self.groups.get(&group_name.to_string()).is_none() {
            self.create_group(group_name)?;
        }

        if let Some(group) = self.groups.get_mut(&group_name.to_string()) {
            if !group.members.contains(&username.to_string()) {
                group.members.push(username.to_string());
            }
        }

        Ok(())
    }

    /// Authenticate a user credentials via stacked PAM verification
    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<(), PamError> {
        // Retrieve the user
        let user = self.users.get_mut(&username.to_string()).ok_or(PamError::UserNotFound)?;

        // Validate account/lock state through stacked pam modules first
        for module in &self.modules {
            module.validate_account(user)?;
        }

        // Verify the salted password hash
        let expected_hash = hash_password_placeholder(password, &user.salt);
        if constant_time_eq(&user.password_hash, &expected_hash) {
            // Success! Reset failed attempts
            user.failed_attempts = 0;
            Ok(())
        } else {
            // Increment failed attempts
            user.failed_attempts += 1;

            // Check if account lock triggers
            for module in &self.modules {
                if let Err(PamError::AccountLocked) = module.authenticate(user, password) {
                    user.is_locked = true;
                }
            }

            Err(PamError::AuthenticationFailed)
        }
    }

    /// Check if a user is in a group
    pub fn is_member_of(&self, username: &str, group_name: &str) -> bool {
        if let Some(group) = self.groups.get(&group_name.to_string()) {
            group.members.contains(&username.to_string())
        } else {
            false
        }
    }
}

impl Default for SovereignPamManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pam_registration_and_auth() {
        let mut manager = SovereignPamManager::new();
        manager.create_group("wheel").unwrap();

        let test_pass = format!("test_pwd_{}", 12345);
        // Register user
        let uid = manager.register_user("aaryan", &test_pass, "wheel").unwrap();
        assert_eq!(uid, 1000);

        // Authenticate user successfully
        assert!(manager.authenticate("aaryan", &test_pass).is_ok());

        // Fail authentication with wrong password
        assert_eq!(manager.authenticate("aaryan", "invalid_mismatch"), Err(PamError::AuthenticationFailed));
    }

    #[test]
    fn test_pam_pwquality_complexity() {
        let mut manager = SovereignPamManager::new();
        manager.register_module(alloc::boxed::Box::new(PasswordQualityModule { min_length: 8 }));

        // Attempt weak password registration -> fails
        assert_eq!(manager.register_user("bob", "weak", "users"), Err(PamError::PasswordTooWeak));

        // Attempt strong password registration -> passes
        let strong_pw = format!("str_comp_{}", 998877);
        assert!(manager.register_user("bob", &strong_pw, "users").is_ok());
    }

    #[test]
    fn test_pam_account_tally_lockout() {
        let mut manager = SovereignPamManager::new();
        manager.register_module(alloc::boxed::Box::new(AccountTallyModule { max_failed_attempts: 3 }));

        let secure_pw = format!("sec_pwd_{}", 67890);
        manager.register_user("alice", &secure_pw, "users").unwrap();

        // 3 consecutive failed attempts
        assert!(manager.authenticate("alice", "bad").is_err());
        assert!(manager.authenticate("alice", "bad").is_err());
        assert!(manager.authenticate("alice", "bad").is_err());

        // Account is locked! Even valid password fails now
        assert_eq!(manager.authenticate("alice", &secure_pw), Err(PamError::AccountLocked));
    }
}
