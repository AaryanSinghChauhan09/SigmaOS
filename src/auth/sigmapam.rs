// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaPAM - Pluggable Authentication Modules
//!
//! SigmaPAM is a pluggable authentication framework for SigmaOS, inspired by Linux PAM.
//! It provides authentication, account management, session management, and password management.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// PAM service types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PamService {
    /// Authentication service
    Auth,
    /// Account management service
    Account,
    /// Session management service
    Session,
    /// Password management service
    Password,
}

impl fmt::Display for PamService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PamService::Auth => write!(f, "auth"),
            PamService::Account => write!(f, "account"),
            PamService::Session => write!(f, "session"),
            PamService::Password => write!(f, "password"),
        }
    }
}

/// PAM control flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PamFlag {
    /// Sufficient - module success is sufficient
    Sufficient,
    /// Required - module success is required
    Required,
    /// Requisite - module failure is fatal
    Requisite,
    /// Optional - module result is ignored
    Optional,
    /// Include - always execute module
    Include,
}

impl fmt::Display for PamFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PamFlag::Sufficient => write!(f, "sufficient"),
            PamFlag::Required => write!(f, "required"),
            PamFlag::Requisite => write!(f, "requisite"),
            PamFlag::Optional => write!(f, "optional"),
            PamFlag::Include => write!(f, "include"),
        }
    }
}

/// Authentication result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthResult {
    /// Authentication succeeded
    Success,
    /// Authentication failed
    AuthError,
    /// Service not available
    ServiceError,
    /// Module error
    ModuleError,
    /// Permission denied
    PermissionDenied,
    /// Account expired
    AccountExpired,
    /// Account locked
    AccountLocked,
    /// Password expired
    PasswordExpired,
}

impl fmt::Display for AuthResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthResult::Success => write!(f, "success"),
            AuthResult::AuthError => write!(f, "auth_error"),
            AuthResult::ServiceError => write!(f, "service_error"),
            AuthResult::ModuleError => write!(f, "module_error"),
            AuthResult::PermissionDenied => write!(f, "permission_denied"),
            AuthResult::AccountExpired => write!(f, "account_expired"),
            AuthResult::AccountLocked => write!(f, "account_locked"),
            AuthResult::PasswordExpired => write!(f, "password_expired"),
        }
    }
}

/// User account information
#[derive(Debug, Clone)]
pub struct UserAccount {
    /// Username
    pub username: String,
    /// User ID (UID)
    pub uid: u32,
    /// Group ID (GID)
    pub gid: u32,
    /// Full name
    pub full_name: String,
    /// Home directory
    pub home_dir: String,
    /// Shell
    pub shell: String,
    /// Account enabled
    pub enabled: bool,
    /// Account locked
    pub locked: bool,
    /// Password expiry time (timestamp)
    pub password_expiry: Option<u64>,
    /// Account expiry time (timestamp)
    pub account_expiry: Option<u64>,
    /// Last login time
    pub last_login: Option<u64>,
    /// Failed login attempts
    pub failed_attempts: u32,
    /// Maximum failed attempts before lock
    pub max_failed_attempts: u32,
}

impl UserAccount {
    /// Create a new user account
    pub fn new(username: String, uid: u32, gid: u32, home_dir: String, shell: String) -> Self {
        Self {
            username,
            uid,
            gid,
            full_name: String::new(),
            home_dir,
            shell,
            enabled: true,
            locked: false,
            password_expiry: None,
            account_expiry: None,
            last_login: None,
            failed_attempts: 0,
            max_failed_attempts: 5,
        }
    }

    /// Set full name
    pub fn with_full_name(mut self, name: String) -> Self {
        self.full_name = name;
        self
    }

    /// Set account enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set account locked
    pub fn with_locked(mut self, locked: bool) -> Self {
        self.locked = locked;
        self
    }

    /// Set password expiry
    pub fn with_password_expiry(mut self, expiry: u64) -> Self {
        self.password_expiry = Some(expiry);
        self
    }

    /// Set account expiry
    pub fn with_account_expiry(mut self, expiry: u64) -> Self {
        self.account_expiry = Some(expiry);
        self
    }

    /// Set max failed attempts
    pub fn with_max_failed_attempts(mut self, max: u32) -> Self {
        self.max_failed_attempts = max;
        self
    }

    /// Check if account is valid for authentication
    pub fn is_valid(&self) -> bool {
        if !self.enabled {
            return false;
        }
        if self.locked {
            return false;
        }
        if let Some(expiry) = self.account_expiry {
            // In a real implementation, check against current time
            if expiry < 1000 {
                // Placeholder check
                return false;
            }
        }
        true
    }

    /// Check if password needs change
    pub fn password_needs_change(&self) -> bool {
        if let Some(expiry) = self.password_expiry {
            // In a real implementation, check against current time
            expiry < 1000 // Placeholder check
        } else {
            false
        }
    }

    /// Increment failed login attempts
    pub fn increment_failed_attempts(&mut self) {
        self.failed_attempts += 1;
        if self.failed_attempts >= self.max_failed_attempts {
            self.locked = true;
        }
    }

    /// Reset failed login attempts
    pub fn reset_failed_attempts(&mut self) {
        self.failed_attempts = 0;
    }

    /// Update last login time
    pub fn update_last_login(&mut self, time: u64) {
        self.last_login = Some(time);
    }
}

/// Authentication request
#[derive(Debug, Clone)]
pub struct AuthRequest {
    /// Username
    pub username: String,
    /// Password (hashed)
    pub password_hash: String,
    /// Service being accessed
    pub service: PamService,
    /// Remote host
    pub remote_host: Option<String>,
    /// Terminal
    pub terminal: Option<String>,
    /// Additional data
    pub data: BTreeMap<String, String>,
}

impl AuthRequest {
    /// Create a new authentication request
    pub fn new(username: String, password_hash: String, service: PamService) -> Self {
        Self {
            username,
            password_hash,
            service,
            remote_host: None,
            terminal: None,
            data: BTreeMap::new(),
        }
    }

    /// Set remote host
    pub fn with_remote_host(mut self, host: String) -> Self {
        self.remote_host = Some(host);
        self
    }

    /// Set terminal
    pub fn with_terminal(mut self, terminal: String) -> Self {
        self.terminal = Some(terminal);
        self
    }

    /// Add data field
    pub fn with_data(mut self, key: String, value: String) -> Self {
        self.data.insert(key, value);
        self
    }
}

/// Authentication response
#[derive(Debug, Clone)]
pub struct AuthResponse {
    /// Authentication result
    pub result: AuthResult,
    /// User account (if authenticated)
    pub account: Option<UserAccount>,
    /// Additional data
    pub data: BTreeMap<String, String>,
}

impl AuthResponse {
    /// Create a new authentication response
    pub fn new(result: AuthResult) -> Self {
        Self {
            result,
            account: None,
            data: BTreeMap::new(),
        }
    }

    /// Set user account
    pub fn with_account(mut self, account: UserAccount) -> Self {
        self.account = Some(account);
        self
    }

    /// Add data field
    pub fn with_data(mut self, key: String, value: String) -> Self {
        self.data.insert(key, value);
        self
    }
}

/// PAM module interface
pub trait PamModule {
    /// Module name
    fn name(&self) -> &str;

    /// Authenticate user
    fn authenticate(&self, request: &AuthRequest) -> AuthResult;

    /// Check account validity
    fn account_check(&self, request: &AuthRequest) -> AuthResult;

    /// Open session
    fn session_open(&self, request: &AuthRequest) -> AuthResult;

    /// Close session
    fn session_close(&self, request: &AuthRequest) -> AuthResult;

    /// Change password
    fn password_change(&self, request: &AuthRequest) -> AuthResult;
}

/// Simple password-based authentication module
#[derive(Debug, Clone)]
pub struct PasswordAuthModule {
    /// User database (username -> password hash)
    user_db: BTreeMap<String, String>,
    /// Account database (username -> account info)
    account_db: BTreeMap<String, UserAccount>,
}

impl PasswordAuthModule {
    /// Create a new password authentication module
    pub fn new() -> Self {
        Self {
            user_db: BTreeMap::new(),
            account_db: BTreeMap::new(),
        }
    }

    /// Add a user
    pub fn add_user(&mut self, username: String, password_hash: String, account: UserAccount) {
        self.user_db.insert(username.clone(), password_hash);
        self.account_db.insert(username, account);
    }

    /// Remove a user
    pub fn remove_user(&mut self, username: &str) {
        self.user_db.remove(username);
        self.account_db.remove(username);
    }

    /// Get user account
    pub fn get_account(&self, username: &str) -> Option<&UserAccount> {
        self.account_db.get(username)
    }

    /// Get user account (mutable)
    pub fn get_account_mut(&mut self, username: &str) -> Option<&mut UserAccount> {
        self.account_db.get_mut(username)
    }
}

impl Default for PasswordAuthModule {
    fn default() -> Self {
        Self::new()
    }
}

impl PamModule for PasswordAuthModule {
    fn name(&self) -> &str {
        "pam_unix"
    }

    fn authenticate(&self, request: &AuthRequest) -> AuthResult {
        // Get stored password hash
        let stored_hash = self.user_db.get(&request.username);

        match stored_hash {
            Some(hash) => {
                // In a real implementation, use constant-time comparison
                if hash == &request.password_hash {
                    AuthResult::Success
                } else {
                    AuthResult::AuthError
                }
            }
            None => AuthResult::AuthError,
        }
    }

    fn account_check(&self, request: &AuthRequest) -> AuthResult {
        if let Some(account) = self.account_db.get(&request.username) {
            if account.is_valid() {
                AuthResult::Success
            } else if account.locked {
                AuthResult::AccountLocked
            } else if !account.enabled {
                AuthResult::PermissionDenied
            } else {
                AuthResult::AccountExpired
            }
        } else {
            AuthResult::AuthError
        }
    }

    fn session_open(&self, request: &AuthRequest) -> AuthResult {
        // In a real implementation, create session entry
        AuthResult::Success
    }

    fn session_close(&self, request: &AuthRequest) -> AuthResult {
        // In a real implementation, close session entry
        AuthResult::Success
    }

    fn password_change(&self, request: &AuthRequest) -> AuthResult {
        // In a real implementation, update password hash
        AuthResult::Success
    }
}

/// PAM configuration
#[derive(Debug, Clone)]
pub struct PamConfig {
    /// Authentication modules
    pub auth_modules: Vec<(PamFlag, String)>,
    /// Account modules
    pub account_modules: Vec<(PamFlag, String)>,
    /// Session modules
    pub session_modules: Vec<(PamFlag, String)>,
    /// Password modules
    pub password_modules: Vec<(PamFlag, String)>,
}

impl PamConfig {
    /// Create a new PAM configuration
    pub fn new() -> Self {
        Self {
            auth_modules: Vec::new(),
            account_modules: Vec::new(),
            session_modules: Vec::new(),
            password_modules: Vec::new(),
        }
    }

    /// Add authentication module
    pub fn add_auth_module(mut self, flag: PamFlag, module: String) -> Self {
        self.auth_modules.push((flag, module));
        self
    }

    /// Add account module
    pub fn add_account_module(mut self, flag: PamFlag, module: String) -> Self {
        self.account_modules.push((flag, module));
        self
    }

    /// Add session module
    pub fn add_session_module(mut self, flag: PamFlag, module: String) -> Self {
        self.session_modules.push((flag, module));
        self
    }

    /// Add password module
    pub fn add_password_module(mut self, flag: PamFlag, module: String) -> Self {
        self.password_modules.push((flag, module));
        self
    }
}

impl Default for PamConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaPAM - Pluggable Authentication Modules
pub struct SigmaPAM {
    /// PAM configuration
    config: PamConfig,
    /// Registered modules
    modules: BTreeMap<String, Box<dyn PamModule>>,
    /// Default password auth module
    password_auth: PasswordAuthModule,
    /// Running state
    running: bool,
}

impl SigmaPAM {
    /// Create a new SigmaPAM instance
    pub fn new(config: PamConfig) -> Self {
        let password_auth = PasswordAuthModule::new();

        Self {
            config,
            modules: BTreeMap::new(),
            password_auth,
            running: false,
        }
    }

    /// Create SigmaPAM with default configuration
    pub fn with_default_config() -> Self {
        let config = PamConfig::default()
            .add_auth_module(PamFlag::Required, "pam_unix".to_string())
            .add_account_module(PamFlag::Required, "pam_unix".to_string())
            .add_session_module(PamFlag::Required, "pam_unix".to_string())
            .add_password_module(PamFlag::Required, "pam_unix".to_string());

        Self::new(config)
    }

    /// Register a PAM module
    pub fn register_module(&mut self, name: String, module: Box<dyn PamModule>) {
        self.modules.insert(name, module);
    }

    /// Start the PAM daemon
    pub fn start(&mut self) {
        self.running = true;
        // In a real implementation, this would start a background thread
        // to handle authentication requests
    }

    /// Stop the PAM daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Authenticate a user
    pub fn authenticate(&self, request: &AuthRequest) -> AuthResponse {
        if !self.running {
            return AuthResponse::new(AuthResult::ServiceError);
        }

        // Use default password auth module
        let result = self.password_auth.authenticate(request);

        let mut response = AuthResponse::new(result);

        if result == AuthResult::Success {
            if let Some(account) = self.password_auth.get_account(&request.username) {
                response = response.with_account(account.clone());
            }
        }

        response
    }

    /// Check account validity
    pub fn account_check(&self, request: &AuthRequest) -> AuthResult {
        if !self.running {
            return AuthResult::ServiceError;
        }

        self.password_auth.account_check(request)
    }

    /// Open a session
    pub fn session_open(&self, request: &AuthRequest) -> AuthResult {
        if !self.running {
            return AuthResult::ServiceError;
        }

        self.password_auth.session_open(request)
    }

    /// Close a session
    pub fn session_close(&self, request: &AuthRequest) -> AuthResult {
        if !self.running {
            return AuthResult::ServiceError;
        }

        self.password_auth.session_close(request)
    }

    /// Change password
    pub fn password_change(&self, request: &AuthRequest) -> AuthResult {
        if !self.running {
            return AuthResult::ServiceError;
        }

        self.password_auth.password_change(request)
    }

    /// Get password auth module for user management
    pub fn get_password_auth(&mut self) -> &mut PasswordAuthModule {
        &mut self.password_auth
    }

    /// Get configuration
    pub fn config(&self) -> &PamConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: PamConfig) {
        self.config = config;
    }
}

impl Default for SigmaPAM {
    fn default() -> Self {
        Self::with_default_config()
    }
}

/// Global SigmaPAM instance (placeholder for actual global state)
static mut SIGMAPAM_INSTANCE: Option<SigmaPAM> = None;

/// Initialize the global SigmaPAM instance
pub fn init_sigmapam(config: PamConfig) {
    unsafe {
        SIGMAPAM_INSTANCE = Some(SigmaPAM::new(config));
    }
}

/// Initialize SigmaPAM with default configuration
pub fn init_sigmapam_default() {
    init_sigmapam(PamConfig::default());
}

/// Get the global SigmaPAM instance
pub fn get_sigmapam() -> Option<&'static mut SigmaPAM> {
    unsafe { SIGMAPAM_INSTANCE.as_mut() }
}

/// Convenience function to authenticate a user
pub fn authenticate_user(username: &str, password_hash: &str, service: PamService) -> AuthResponse {
    if let Some(pam) = get_sigmapam() {
        let request = AuthRequest::new(username.to_string(), password_hash.to_string(), service);
        pam.authenticate(&request)
    } else {
        AuthResponse::new(AuthResult::ServiceError)
    }
}

/// Convenience function to check account validity
pub fn check_account(username: &str, service: PamService) -> AuthResult {
    if let Some(pam) = get_sigmapam() {
        let request = AuthRequest::new(username.to_string(), String::new(), service);
        pam.account_check(&request)
    } else {
        AuthResult::ServiceError
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pam_service_display() {
        assert_eq!(PamService::Auth.to_string(), "auth");
        assert_eq!(PamService::Account.to_string(), "account");
    }

    #[test]
    fn test_pam_flag_display() {
        assert_eq!(PamFlag::Required.to_string(), "required");
        assert_eq!(PamFlag::Sufficient.to_string(), "sufficient");
    }

    #[test]
    fn test_auth_result_display() {
        assert_eq!(AuthResult::Success.to_string(), "success");
        assert_eq!(AuthResult::AuthError.to_string(), "auth_error");
    }

    #[test]
    fn test_user_account_creation() {
        let account = UserAccount::new(
            "alice".to_string(),
            1000,
            1000,
            "/home/alice".to_string(),
            "/bin/bash".to_string(),
        )
        .with_full_name("Alice Smith".to_string())
        .with_enabled(true);

        assert_eq!(account.username, "alice");
        assert_eq!(account.uid, 1000);
        assert_eq!(account.full_name, "Alice Smith");
        assert!(account.enabled);
    }

    #[test]
    fn test_user_account_validity() {
        let account = UserAccount::new(
            "alice".to_string(),
            1000,
            1000,
            "/home/alice".to_string(),
            "/bin/bash".to_string(),
        );

        assert!(account.is_valid());

        let disabled = account.clone().with_enabled(false);
        assert!(!disabled.is_valid());

        let locked = account.with_locked(true);
        assert!(!locked.is_valid());
    }

    #[test]
    fn test_failed_attempts() {
        let mut account = UserAccount::new(
            "alice".to_string(),
            1000,
            1000,
            "/home/alice".to_string(),
            "/bin/bash".to_string(),
        )
        .with_max_failed_attempts(3);

        account.increment_failed_attempts();
        assert_eq!(account.failed_attempts, 1);
        assert!(!account.locked);

        account.increment_failed_attempts();
        account.increment_failed_attempts();
        assert_eq!(account.failed_attempts, 3);
        assert!(account.locked);

        account.reset_failed_attempts();
        assert_eq!(account.failed_attempts, 0);
        assert!(!account.locked);
    }

    #[test]
    fn test_auth_request_creation() {
        let request =
            AuthRequest::new("alice".to_string(), "hash123".to_string(), PamService::Auth)
                .with_remote_host("192.168.1.100".to_string())
                .with_terminal("tty1".to_string());

        assert_eq!(request.username, "alice");
        assert_eq!(request.service, PamService::Auth);
        assert_eq!(request.remote_host, Some("192.168.1.100".to_string()));
        assert_eq!(request.terminal, Some("tty1".to_string()));
    }

    #[test]
    fn test_password_auth_module() {
        let mut module = PasswordAuthModule::new();

        let account = UserAccount::new(
            "alice".to_string(),
            1000,
            1000,
            "/home/alice".to_string(),
            "/bin/bash".to_string(),
        );

        module.add_user("alice".to_string(), "correct_hash".to_string(), account);

        let request = AuthRequest::new(
            "alice".to_string(),
            "correct_hash".to_string(),
            PamService::Auth,
        );

        let result = module.authenticate(&request);
        assert_eq!(result, AuthResult::Success);

        let request = AuthRequest::new(
            "alice".to_string(),
            "wrong_hash".to_string(),
            PamService::Auth,
        );

        let result = module.authenticate(&request);
        assert_eq!(result, AuthResult::AuthError);
    }

    #[test]
    fn test_sigmapam_creation() {
        let mut pam = SigmaPAM::with_default_config();
        assert!(!pam.is_running());
        pam.start();
        assert!(pam.is_running());
        pam.stop();
        assert!(!pam.is_running());
    }

    #[test]
    fn test_sigmapam_authenticate() {
        let mut pam = SigmaPAM::with_default_config();
        pam.start();

        let account = UserAccount::new(
            "alice".to_string(),
            1000,
            1000,
            "/home/alice".to_string(),
            "/bin/bash".to_string(),
        );

        pam.get_password_auth()
            .add_user("alice".to_string(), "hash123".to_string(), account);

        let request =
            AuthRequest::new("alice".to_string(), "hash123".to_string(), PamService::Auth);

        let response = pam.authenticate(&request);
        assert_eq!(response.result, AuthResult::Success);
        assert!(response.account.is_some());
    }

    #[test]
    fn test_authenticate_user_convenience() {
        init_sigmapam_default();

        let mut pam = get_sigmapam().unwrap();
        let account = UserAccount::new(
            "alice".to_string(),
            1000,
            1000,
            "/home/alice".to_string(),
            "/bin/bash".to_string(),
        );

        pam.get_password_auth()
            .add_user("alice".to_string(), "hash123".to_string(), account);

        let response = authenticate_user("alice", "hash123", PamService::Auth);
        assert_eq!(response.result, AuthResult::Success);
    }

    #[test]
    fn test_check_account_convenience() {
        init_sigmapam_default();

        let mut pam = get_sigmapam().unwrap();
        let account = UserAccount::new(
            "alice".to_string(),
            1000,
            1000,
            "/home/alice".to_string(),
            "/bin/bash".to_string(),
        );

        pam.get_password_auth()
            .add_user("alice".to_string(), "hash123".to_string(), account);

        let result = check_account("alice", PamService::Account);
        assert_eq!(result, AuthResult::Success);
    }
}
