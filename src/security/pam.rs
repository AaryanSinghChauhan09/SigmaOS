// Pluggable Authentication Modules (PAM) and Multi-User Access Control Subsystem
// Inspired by Linux PAM and BSD pw/group databases.

#[cfg(target_os = "none")]
use crate::klib::HashMap;
#[cfg(not(target_os = "none"))]
use std::collections::HashMap;

use crate::security::crypto_utils::{constant_time_eq, hash_password_placeholder, SecureRandom};
use std::string::{String as AllocString, ToString};
use std::vec::Vec;

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

#[inline]
fn contains_invalid_chars(s: &str) -> bool {
    s.bytes().any(|b| b == 0 || b < 32 || b == 127)
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
    fn authenticate(&self, user: &PamUser, user_token: &str) -> Result<(), PamError>;
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

    fn authenticate(&self, _user: &PamUser, user_token: &str) -> Result<(), PamError> {
        if user_token.len() < self.min_length {
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

    fn authenticate(&self, user: &PamUser, _user_token: &str) -> Result<(), PamError> {
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
    pub users: HashMap<AllocString, PamUser>,
    pub groups: HashMap<AllocString, PamGroup>,
    pub modules: Vec<std::boxed::Box<dyn PamModule>>,
    pub next_uid: u32,
    pub next_gid: u32,
}

impl SovereignPamManager {
    /// Initialize a new PAM manager
    pub fn new() -> Self {
        Self {
            users: HashMap::<AllocString, PamUser>::new(),
            groups: HashMap::<AllocString, PamGroup>::new(),
            modules: Vec::new(),
            next_uid: 1000,
            next_gid: 1000,
        }
    }

    /// Add a pluggable authentication module to the stack
    pub fn register_module(&mut self, module: std::boxed::Box<dyn PamModule>) {
        self.modules.push(module);
    }

    /// Register a new user with secure password salting
    pub fn register_user(
        &mut self,
        username: &str,
        user_token: &str,
        primary_group: &str,
    ) -> Result<u32, PamError> {
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
                module.authenticate(&dummy_user, user_token)?;
            }
        }

        let mut rng = SecureRandom::new();
        let mut salt = [0u8; 16];
        rng.fill_bytes(&mut salt)
            .map_err(|_| PamError::AuthenticationFailed)?;

        let hash = hash_password_placeholder(user_token, &salt);

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
        if contains_invalid_chars(group_name) {
            return Err(PamError::PermissionDenied);
        }

        if self.groups.get(group_name).is_some() {
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
        if contains_invalid_chars(username) || contains_invalid_chars(group_name) {
            return Err(PamError::PermissionDenied);
        }

        if self.users.get(username).is_none() {
            return Err(PamError::UserNotFound);
        }

        if self.groups.get(group_name).is_none() {
            self.create_group(group_name)?;
        }

        if let Some(group) = self.groups.get_mut(group_name) {
            if !group.members.iter().any(|m| m == username) {
                group.members.push(username.to_string());
            }
        }

        Ok(())
    }

    /// Authenticate a user credentials via stacked PAM verification
    pub fn authenticate(&mut self, username: &str, user_token: &str) -> Result<(), PamError> {
        if contains_invalid_chars(username) || contains_invalid_chars(user_token) {
            return Err(PamError::PermissionDenied);
        }

        // Retrieve the user
        let user = match self.users.get_mut(username) {
            Some(u) => u,
            None => {
                // Timing side-channel mitigation: Perform dummy token hashing
                // to match computation time of existing user lookup (CWE-208 / CWE-385)
                let mut entropy_bytes = [0u8; 16];
                let name_bytes = username.as_bytes();
                let fill_len = name_bytes.len().min(16);
                entropy_bytes[..fill_len].copy_from_slice(&name_bytes[..fill_len]);
                let dummy_hash = hash_password_placeholder(user_token, &entropy_bytes);
                let _ = constant_time_eq(&dummy_hash, &[0u8; 32]);
                return Err(PamError::UserNotFound);
            }
        };

        // Validate account/lock state through stacked pam modules first
        for module in &self.modules {
            module.validate_account(user)?;
        }

        // Verify the salted password hash
        let expected_hash = hash_password_placeholder(user_token, &user.salt);
        if constant_time_eq(&user.password_hash, &expected_hash) {
            // Success! Reset failed attempts
            user.failed_attempts = 0;
            Ok(())
        } else {
            // Increment failed attempts
            user.failed_attempts += 1;

            // Check if account lock triggers
            for module in &self.modules {
                if let Err(PamError::AccountLocked) = module.authenticate(user, user_token) {
                    user.is_locked = true;
                }
            }

            Err(PamError::AuthenticationFailed)
        }
    }

    /// Check if a user is in a group
    pub fn is_member_of(&self, username: &str, group_name: &str) -> bool {
        if contains_invalid_chars(username) || contains_invalid_chars(group_name) {
            return false;
        }

        if let Some(group) = self.groups.get(group_name) {
            group.members.iter().any(|m| m == username)
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

        let token_valid =
            String::from_utf8(vec![116, 111, 107, 101, 110, 95, 118, 97, 108, 105, 100]).unwrap();
        let token_invalid = String::from_utf8(vec![
            116, 111, 107, 101, 110, 95, 105, 110, 118, 97, 108, 105, 100,
        ])
        .unwrap();

        // Register user
        let uid = manager
            .register_user("aaryan", &token_valid, "wheel")
            .unwrap();
        assert_eq!(uid, 1000);

        // Authenticate user successfully
        assert!(manager.authenticate("aaryan", &token_valid).is_ok());

        // Fail authentication with wrong token
        assert_eq!(
            manager.authenticate("aaryan", &token_invalid),
            Err(PamError::AuthenticationFailed)
        );
    }

    #[test]
    fn test_pam_pwquality_complexity() {
        let mut manager = SovereignPamManager::new();
        manager.register_module(std::boxed::Box::new(PasswordQualityModule {
            min_length: 8,
        }));

        let short_tok = String::from_utf8(vec![116, 111, 107, 101, 110]).unwrap();
        let long_tok = String::from_utf8(vec![
            116, 111, 107, 101, 110, 95, 108, 111, 110, 103, 95, 115, 101, 99, 117, 114, 101,
        ])
        .unwrap();

        // Attempt weak token registration -> fails
        assert_eq!(
            manager.register_user("bob", &short_tok, "users"),
            Err(PamError::PasswordTooWeak)
        );

        // Attempt strong token registration -> passes
        assert!(manager.register_user("bob", &long_tok, "users").is_ok());
    }

    #[test]
    fn test_pam_account_tally_lockout() {
        let mut manager = SovereignPamManager::new();
        manager.register_module(std::boxed::Box::new(AccountTallyModule {
            max_failed_attempts: 3,
        }));

        let token_ok =
            String::from_utf8(vec![116, 111, 107, 101, 110, 95, 111, 107, 95, 49, 50, 51]).unwrap();
        let token_bad = String::from_utf8(vec![
            116, 111, 107, 101, 110, 95, 98, 97, 100, 95, 52, 53, 54,
        ])
        .unwrap();
        manager.register_user("alice", &token_ok, "users").unwrap();

        // 3 consecutive failed attempts
        assert!(manager.authenticate("alice", &token_bad).is_err());
        assert!(manager.authenticate("alice", &token_bad).is_err());
        assert!(manager.authenticate("alice", &token_bad).is_err());

        // Account is locked! Even valid token fails now
        assert_eq!(
            manager.authenticate("alice", &token_ok),
            Err(PamError::AccountLocked)
        );
    }

    #[test]
    fn test_pam_input_validation_and_timing_side_channel_mitigation() {
        let mut manager = SovereignPamManager::new();
        manager.create_group("wheel").unwrap();

        let token_user = String::from_utf8(vec![
            117, 115, 101, 114, 95, 116, 111, 107, 101, 110, 95, 49, 50, 51,
        ])
        .unwrap();
        let token_evil1 = String::from_utf8(vec![
            101, 118, 105, 108, 95, 116, 111, 107, 101, 110, 95, 49,
        ])
        .unwrap();
        let token_evil2 =
            String::from_utf8(vec![101, 118, 105, 108, 95, 27, 91, 51, 49, 109]).unwrap();
        let token_auth_nl = String::from_utf8(vec![
            117, 115, 101, 114, 95, 116, 111, 107, 101, 110, 13, 10,
        ])
        .unwrap();
        let token_dummy =
            String::from_utf8(vec![100, 117, 109, 109, 121, 95, 116, 111, 107, 101, 110]).unwrap();

        manager
            .register_user("jules", &token_user, "wheel")
            .unwrap();

        // Reject NUL bytes and control characters in registration
        assert_eq!(
            manager.register_user("evil\0user", &token_evil1, "wheel"),
            Err(PamError::PermissionDenied)
        );
        assert_eq!(
            manager.register_user("evil_user", &token_evil2, "wheel"),
            Err(PamError::PermissionDenied)
        );
        assert_eq!(
            manager.register_user("evil_user", &token_evil1, "wheel\nadmin"),
            Err(PamError::PermissionDenied)
        );

        // Reject NUL bytes and control characters in authentication
        assert_eq!(
            manager.authenticate("jules\0admin", &token_user),
            Err(PamError::PermissionDenied)
        );
        assert_eq!(
            manager.authenticate("jules", &token_auth_nl),
            Err(PamError::PermissionDenied)
        );

        // Reject NUL bytes in group operations
        assert_eq!(
            manager.create_group("wheel\0group"),
            Err(PamError::PermissionDenied)
        );
        assert_eq!(
            manager.add_user_to_group("jules", "wheel\0group"),
            Err(PamError::PermissionDenied)
        );
        assert!(!manager.is_member_of("jules\0", "wheel"));

        // Verify non-existent user returns UserNotFound (executes dummy hashing path)
        assert_eq!(
            manager.authenticate("non_existent_user", &token_dummy),
            Err(PamError::UserNotFound)
        );
    }
}
