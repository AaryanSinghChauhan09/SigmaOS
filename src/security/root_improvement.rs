#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Linux & BSD Inspired Superuser / Root Improvements Suite
// Implements advanced privilege management:
// 1. Timed sudo/doas tokens
// 2. Polkit fine-grained action authorization
// 3. Linux LinuxCap capability splitting
// 4. Rootless user namespace UID/GID translation
// 5. Stackable PAM subsystem (pam_unix, pam_faillock, pam_time, pam_limits, pam_mfa)
// 6. BSD Securelevel Kernel Security Enforcement (OpenBSD/FreeBSD parity)
// 7. OpenBSD doas.conf Granular Rule Engine
// 8. Linux Subordinate UID/GID Mapper (subuid/subgid container parity)
// 9. Rootless Privileged Port Binding Manager (sysctl ip_unprivileged_port_start parity)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};

// ==========================================
// 1. sudo/doas Style Privilege Elevator
// ==========================================

#[derive(Debug, Clone)]
pub struct SudoToken {
    pub user_id: u32,
    pub session_ttl_secs: u64,
    pub generated_at_ms: u64,
}

pub struct SudoDoasElevator {
    pub active_tokens: Vec<SudoToken>,
    pub password_database: Vec<(String, String)>, // (username, password_hash)
}

impl SudoDoasElevator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            active_tokens: Vec::new(),
            password_database: Vec::new(),
        }
    }

    pub fn elevate_via_doas(
        &mut self,
        username: &str,
        password_hash: &str,
        current_time_ms: u64,
    ) -> Result<u32, &'static str> {
        let mut user_found = false;
        for (user, hash) in &self.password_database {
            if user == username {
                if hash == password_hash {
                    user_found = true;
                    break;
                } else {
                    return Err("doas: authentication failed: invalid password credentials");
                }
            }
        }

        if !user_found {
            return Err("doas: authentication failed: user not found in authorization database");
        }

        let user_id = if username == "admin" { 0 } else { 1000 };
        self.active_tokens.push(SudoToken {
            user_id,
            session_ttl_secs: 900, // 15 mins
            generated_at_ms: current_time_ms,
        });

        Ok(user_id)
    }

    pub fn verify_active_sudo_session(&self, user_id: u32, current_time_ms: u64) -> bool {
        for token in &self.active_tokens {
            if token.user_id == user_id {
                let diff_ms = current_time_ms.saturating_sub(token.generated_at_ms);
                let diff_secs = diff_ms / 1000;
                if diff_secs < token.session_ttl_secs {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for SudoDoasElevator {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. PolicyKit (Polkit) Fine-Grained Authorization Enforcer
// ==========================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolkitAuthorization {
    Authorized,
    Blocked,
    ChallengeMfa,
}

#[derive(Debug, Clone)]
pub struct PolkitRule {
    pub action_id: String,
    pub min_uid: u32,
    pub allow_any: bool,
    pub requires_active_session: bool,
}

pub struct PolkitEnforcer {
    pub rules: Vec<PolkitRule>,
}

impl PolkitEnforcer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            rules: vec![
                PolkitRule {
                    action_id: "org.sigmaos.network.control".to_string(),
                    min_uid: 0,
                    allow_any: false,
                    requires_active_session: true,
                },
                PolkitRule {
                    action_id: "org.sigmaos.system.power-off".to_string(),
                    min_uid: 1000,
                    allow_any: true,
                    requires_active_session: false,
                },
            ],
        }
    }

    pub fn evaluate_polkit_action(
        &self,
        action_id: &str,
        uid: u32,
        has_active_sudo: bool,
    ) -> PolkitAuthorization {
        for rule in &self.rules {
            if rule.action_id == action_id {
                if uid == 0 {
                    return PolkitAuthorization::Authorized;
                }
                if rule.allow_any && uid >= rule.min_uid {
                    return PolkitAuthorization::Authorized;
                }
                if rule.requires_active_session && !has_active_sudo {
                    return PolkitAuthorization::ChallengeMfa;
                }
                if uid >= rule.min_uid && has_active_sudo {
                    return PolkitAuthorization::Authorized;
                }
            }
        }
        PolkitAuthorization::Blocked
    }
}

impl Default for PolkitEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 3. CAP Capability Splitting Suite
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxCap {
    CapChown = 1 << 0,
    CapDacOverride = 1 << 1,
    CapKill = 1 << 2,
    CapNetAdmin = 1 << 3,
    CapSysChroot = 1 << 4,
    CapSysAdmin = 1 << 5,
}

pub struct CapSplitter {
    pub permitted_capabilities: u32,
    pub effective_capabilities: u32,
}

impl CapSplitter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            permitted_capabilities: 0,
            effective_capabilities: 0,
        }
    }

    pub fn assign_capability(&mut self, cap: LinuxCap) {
        self.permitted_capabilities |= cap as u32;
    }

    pub fn activate_capability(&mut self, cap: LinuxCap) -> bool {
        if (self.permitted_capabilities & (cap as u32)) != 0 {
            self.effective_capabilities |= cap as u32;
            true
        } else {
            false
        }
    }

    pub fn check_capability(&self, cap: LinuxCap) -> bool {
        (self.effective_capabilities & (cap as u32)) != 0
    }

    pub fn drop_capability(&mut self, cap: LinuxCap) {
        self.permitted_capabilities &= !(cap as u32);
        self.effective_capabilities &= !(cap as u32);
    }
}

impl Default for CapSplitter {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. Rootless Namespace Translation Manager
// ==========================================

#[derive(Debug, Clone)]
pub struct UidMapEntry {
    pub inside_uid: u32,
    pub outside_uid: u32,
    pub range_length: u32,
}

pub struct RootlessNamespaceManager {
    pub uid_maps: Vec<UidMapEntry>,
    pub gid_maps: Vec<UidMapEntry>,
}

impl RootlessNamespaceManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            uid_maps: vec![UidMapEntry {
                inside_uid: 0,
                outside_uid: 1000,
                range_length: 1,
            }],
            gid_maps: vec![UidMapEntry {
                inside_uid: 0,
                outside_uid: 1000,
                range_length: 1,
            }],
        }
    }

    pub fn translate_inside_to_outside_uid(&self, inside_uid: u32) -> u32 {
        for entry in &self.uid_maps {
            if inside_uid >= entry.inside_uid && inside_uid < entry.inside_uid + entry.range_length
            {
                let offset = inside_uid - entry.inside_uid;
                return entry.outside_uid + offset;
            }
        }
        inside_uid // Pass-through fallback
    }

    pub fn translate_outside_to_inside_uid(&self, outside_uid: u32) -> u32 {
        for entry in &self.uid_maps {
            if outside_uid >= entry.outside_uid
                && outside_uid < entry.outside_uid + entry.range_length
            {
                let offset = outside_uid - entry.outside_uid;
                return entry.inside_uid + offset;
            }
        }
        outside_uid // Pass-through fallback
    }
}

impl Default for RootlessNamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 5. OpenBSD doas.conf Rule Engine
// ==========================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoasAction {
    Permit,
    Deny,
}

#[derive(Debug, Clone)]
pub struct DoasRule {
    pub action: DoasAction,
    pub identity: String, // username or group (e.g. ":wheel" or "alice")
    pub target_user: String, // target user (e.g. "root")
    pub keepenv: bool,
    pub nopass: bool,
    pub command: Option<String>,
}

pub struct DoasRuleEngine {
    pub rules: Vec<DoasRule>,
}

impl DoasRuleEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: DoasRule) {
        self.rules.push(rule);
    }

    /// Evaluates rules using OpenBSD's last-matching-rule semantics.
    pub fn evaluate(&self, user: &str, is_wheel: bool, target: &str, cmd: &str) -> Option<&DoasRule> {
        let mut last_match = None;
        for rule in &self.rules {
            let id_match = if rule.identity.starts_with(':') {
                rule.identity == ":wheel" && is_wheel
            } else {
                rule.identity == user || rule.identity == "*"
            };

            let target_match = rule.target_user == "*" || rule.target_user == target;
            let cmd_match = match &rule.command {
                Some(c) => c == cmd,
                None => true,
            };

            if id_match && target_match && cmd_match {
                last_match = Some(rule);
            }
        }
        last_match
    }
}

impl Default for DoasRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 6. FreeBSD Kernel Securelevel Guard
// ==========================================

/// FreeBSD kernel securelevel states (-1 to 3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecureLevel {
    PermanentlyUnsecure = -1,
    Insecure = 0,
    Secure = 1,
    HighlySecure = 2,
    NetworkSecure = 3,
}

pub struct BsdSecurelevelGuard {
    current_level: SecureLevel,
}

impl BsdSecurelevelGuard {
    pub fn new(level: SecureLevel) -> Self {
        Self { current_level: level }
    }

    pub fn current_level(&self) -> SecureLevel {
        self.current_level
    }

    /// Securelevel can only be raised when level >= 0, never lowered!
    pub fn raise_level(&mut self, new_level: SecureLevel) -> Result<(), &'static str> {
        if self.current_level == SecureLevel::PermanentlyUnsecure {
            return Err("securelevel is permanently unsecure and cannot be raised");
        }
        if new_level > self.current_level {
            self.current_level = new_level;
            Ok(())
        } else {
            Err("securelevel can only be raised, not lowered")
        }
    }

    pub fn allow_module_loading(&self) -> bool {
        self.current_level < SecureLevel::Secure
    }

    pub fn allow_raw_disk_write(&self) -> bool {
        self.current_level < SecureLevel::Secure
    }

    pub fn allow_time_adjustment(&self) -> bool {
        self.current_level < SecureLevel::HighlySecure
    }

    pub fn allow_firewall_modification(&self) -> bool {
        self.current_level < SecureLevel::NetworkSecure
    }
}

// ==========================================
// 7. Linux SubUid / SubGid Multi-Range Mapper
// ==========================================

#[derive(Debug, Clone)]
pub struct SubUidGidRange {
    pub username: String,
    pub start_id: u32,
    pub count: u32,
}

pub struct SubUidGidMapper {
    pub subuid_ranges: Vec<SubUidGidRange>,
    pub subgid_ranges: Vec<SubUidGidRange>,
}

impl SubUidGidMapper {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            subuid_ranges: Vec::new(),
            subgid_ranges: Vec::new(),
        }
    }

    pub fn add_subuid_range(&mut self, username: &str, start_id: u32, count: u32) {
        self.subuid_ranges.push(SubUidGidRange {
            username: username.to_string(),
            start_id,
            count,
        });
    }

    pub fn add_subgid_range(&mut self, username: &str, start_id: u32, count: u32) {
        self.subgid_ranges.push(SubUidGidRange {
            username: username.to_string(),
            start_id,
            count,
        });
    }

    pub fn is_subuid_valid(&self, username: &str, mapped_uid: u32) -> bool {
        for range in &self.subuid_ranges {
            if range.username == username && mapped_uid >= range.start_id && mapped_uid < range.start_id + range.count {
                return true;
            }
        }
        false
    }

    pub fn map_container_uid(&self, username: &str, container_uid: u32) -> Option<u32> {
        for range in &self.subuid_ranges {
            if range.username == username {
                if container_uid < range.count {
                    return Some(range.start_id + container_uid);
                }
            }
        }
        None
    }
}

impl Default for SubUidGidMapper {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 8. PAM MFA Pluggable Authenticator
// ==========================================

pub struct PamMfaAuthenticator {
    pub correct_mfa_code: u32,
}

impl PamMfaAuthenticator {
    pub fn new(correct_mfa_code: u32) -> Self {
        Self { correct_mfa_code }
    }

    pub fn verify_mfa_code(&self, code: u32) -> bool {
        code == self.correct_mfa_code
    }
}

// ==========================================
// 6. Linux-Inspired Stackable PAM Subsystem
// ==========================================

/// Standard PAM service types (management groups)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PamGroup {
    Auth,
    Account,
    Session,
    Password,
}

/// Standard PAM module control flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PamControlFlag {
    Required,
    Requisite,
    Sufficient,
    Optional,
}

/// Structured PAM execution result codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PamResult {
    Success,
    AuthError,
    CredExpired,
    AcctExpired,
    MaxTries,
    UserUnknown,
    SessionErr,
    PermissionDenied,
    Ignore,
}

/// Execution Context passed through the PAM stack
#[derive(Debug, Clone)]
pub struct PamContext {
    pub current_time_hour: u32,
    pub failed_attempts: u32,
    pub max_failed_allowed: u32,
    pub mfa_provided: Option<u32>,
    pub correct_mfa_code: Option<u32>,
    pub account_expired: bool,
    pub session_opened: bool,
}

impl PamContext {
    pub fn new() -> Self {
        Self {
            current_time_hour: 12,
            failed_attempts: 0,
            max_failed_allowed: 3,
            mfa_provided: None,
            correct_mfa_code: None,
            account_expired: false,
            session_opened: false,
        }
    }
}

impl Default for PamContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Pluggable Authentication Module interface (Polymorphic OOP design)
pub trait PamModule: Send + Sync {
    fn name(&self) -> &str;

    fn authenticate(
        &self,
        _username: &str,
        _password_hash: &str,
        _context: &mut PamContext,
    ) -> PamResult {
        PamResult::Ignore
    }

    fn validate_account(&self, _username: &str, _context: &mut PamContext) -> PamResult {
        PamResult::Ignore
    }

    fn open_session(&self, _username: &str, _context: &mut PamContext) -> PamResult {
        PamResult::Ignore
    }

    fn close_session(&self, _username: &str, _context: &mut PamContext) -> PamResult {
        PamResult::Ignore
    }

    fn change_password(
        &self,
        _username: &str,
        _old_hash: &str,
        _new_hash: &str,
        _context: &mut PamContext,
    ) -> PamResult {
        PamResult::Ignore
    }
}

/// Concrete PAM Module: Unix authentication module (pam_unix)
pub struct PamUnixModule {
    pub password_database: Vec<(String, String)>,
}

impl PamUnixModule {
    pub fn new(db: Vec<(String, String)>) -> Self {
        Self {
            password_database: db,
        }
    }
}

impl PamModule for PamUnixModule {
    fn name(&self) -> &str {
        "pam_unix"
    }

    fn authenticate(
        &self,
        username: &str,
        password_hash: &str,
        _context: &mut PamContext,
    ) -> PamResult {
        for (user, hash) in &self.password_database {
            if user == username {
                if hash == password_hash {
                    return PamResult::Success;
                } else {
                    return PamResult::AuthError;
                }
            }
        }
        PamResult::UserUnknown
    }

    fn validate_account(&self, _username: &str, context: &mut PamContext) -> PamResult {
        if context.account_expired {
            PamResult::AcctExpired
        } else {
            PamResult::Success
        }
    }
}

/// Concrete PAM Module: Account failure lockout (pam_faillock / pam_tally2)
pub struct PamFaillockModule;

impl PamModule for PamFaillockModule {
    fn name(&self) -> &str {
        "pam_faillock"
    }

    fn authenticate(
        &self,
        _username: &str,
        _password_hash: &str,
        context: &mut PamContext,
    ) -> PamResult {
        if context.failed_attempts >= context.max_failed_allowed {
            PamResult::MaxTries
        } else {
            PamResult::Success
        }
    }
}

/// Concrete PAM Module: Time restriction module (pam_time)
pub struct PamTimeModule {
    pub allowed_start_hour: u32,
    pub allowed_end_hour: u32,
}

impl PamTimeModule {
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            allowed_start_hour: start,
            allowed_end_hour: end,
        }
    }
}

impl PamModule for PamTimeModule {
    fn name(&self) -> &str {
        "pam_time"
    }

    fn validate_account(&self, _username: &str, context: &mut PamContext) -> PamResult {
        if context.current_time_hour >= self.allowed_start_hour
            && context.current_time_hour <= self.allowed_end_hour
        {
            PamResult::Success
        } else {
            PamResult::PermissionDenied
        }
    }
}

/// Concrete PAM Module: Session management module (pam_limits)
pub struct PamLimitsModule;

impl PamModule for PamLimitsModule {
    fn name(&self) -> &str {
        "pam_limits"
    }

    fn open_session(&self, _username: &str, context: &mut PamContext) -> PamResult {
        context.session_opened = true;
        PamResult::Success
    }

    fn close_session(&self, _username: &str, context: &mut PamContext) -> PamResult {
        context.session_opened = false;
        PamResult::Success
    }
}

/// Concrete PAM Module: Multi-factor Authentication module (pam_mfa)
pub struct PamMfaPluggableModule;

impl PamModule for PamMfaPluggableModule {
    fn name(&self) -> &str {
        "pam_mfa"
    }

    fn authenticate(
        &self,
        _username: &str,
        _password_hash: &str,
        context: &mut PamContext,
    ) -> PamResult {
        match (context.mfa_provided, context.correct_mfa_code) {
            (Some(prov), Some(corr)) => {
                if prov == corr {
                    PamResult::Success
                } else {
                    PamResult::AuthError
                }
            }
            _ => PamResult::Ignore,
        }
    }
}

/// A single rule in a PAM configuration chain
pub struct PamRule {
    pub control_flag: PamControlFlag,
    pub module: alloc::sync::Arc<dyn PamModule>,
}

/// Central Pluggable Authentication Modules manager
pub struct PamEngine {
    pub chains: BTreeMap<PamGroup, Vec<PamRule>>,
    pub context: PamContext,
}

impl PamEngine {
    pub fn new() -> Self {
        Self {
            chains: BTreeMap::new(),
            context: PamContext::new(),
        }
    }

    pub fn add_rule(&mut self, group: PamGroup, rule: PamRule) {
        self.chains.entry(group).or_insert_with(Vec::new).push(rule);
    }

    /// Evaluates the complete PAM configuration stack for a specific management group.
    /// Follows the standard Linux PAM specification.
    pub fn execute_group(
        &mut self,
        group: PamGroup,
        username: &str,
        password_hash: &str,
    ) -> PamResult {
        let rules = match self.chains.get(&group) {
            Some(r) => r,
            None => return PamResult::Success,
        };

        let mut required_failed = false;
        let mut failure_status = PamResult::AuthError;
        let mut sufficient_passed = false;
        let mut optional_passed = false;

        for rule in rules {
            let res = match group {
                PamGroup::Auth => {
                    rule.module
                        .authenticate(username, password_hash, &mut self.context)
                }
                PamGroup::Account => rule.module.validate_account(username, &mut self.context),
                PamGroup::Session => rule.module.open_session(username, &mut self.context),
                PamGroup::Password => {
                    rule.module
                        .change_password(username, password_hash, "", &mut self.context)
                }
            };

            match (rule.control_flag, res) {
                (PamControlFlag::Required, PamResult::Success) => {}
                (PamControlFlag::Required, failed_res) => {
                    required_failed = true;
                    if failure_status == PamResult::AuthError {
                        failure_status = failed_res;
                    }
                }
                (PamControlFlag::Requisite, PamResult::Success) => {}
                (PamControlFlag::Requisite, failed_res) => {
                    return failed_res;
                }
                (PamControlFlag::Sufficient, PamResult::Success) => {
                    if !required_failed {
                        return PamResult::Success;
                    }
                    sufficient_passed = true;
                }
                (PamControlFlag::Sufficient, _) => {}
                (PamControlFlag::Optional, PamResult::Success) => {
                    optional_passed = true;
                }
                (PamControlFlag::Optional, _) => {}
            }
        }

        if required_failed {
            failure_status
        } else if sufficient_passed || optional_passed {
            PamResult::Success
        } else {
            PamResult::Success
        }
    }
}

impl Default for PamEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 7. BSD Securelevel Kernel Controller (OpenBSD / FreeBSD Parity)
// ==========================================

/// BSD Securelevels (-1, 0, 1, 2, 3) enforcing strict hardware/kernel restrictions even on root
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BsdSecurelevel {
    PermanentlyUnsecure = -1,
    Insecure = 0,
    Secure = 1,
    HighlySecure = 2,
    NetworkSecure = 3,
}

pub struct BsdSecurelevelController {
    level: AtomicI32,
}

impl BsdSecurelevelController {
    pub fn new(initial_level: BsdSecurelevel) -> Self {
        Self {
            level: AtomicI32::new(initial_level as i32),
        }
    }

    pub fn current_level(&self) -> BsdSecurelevel {
        match self.level.load(Ordering::SeqCst) {
            -1 => BsdSecurelevel::PermanentlyUnsecure,
            0 => BsdSecurelevel::Insecure,
            1 => BsdSecurelevel::Secure,
            2 => BsdSecurelevel::HighlySecure,
            _ => BsdSecurelevel::NetworkSecure,
        }
    }

    /// Raises the securelevel. Note: Once raised above 0, securelevel can NEVER be lowered without rebooting.
    pub fn raise_level(&self, target_level: BsdSecurelevel) -> Result<(), &'static str> {
        let cur = self.level.load(Ordering::SeqCst);
        let target = target_level as i32;

        if target < cur && cur > 0 {
            return Err("securelevel: cannot lower securelevel once raised above 0");
        }

        self.level.store(target, Ordering::SeqCst);
        Ok(())
    }

    /// Checks if raw disk write operations are permitted
    pub fn check_raw_disk_write_allowed(&self) -> bool {
        self.current_level() < BsdSecurelevel::Secure
    }

    /// Checks if kernel module loading/unloading is permitted
    pub fn check_module_loading_allowed(&self) -> bool {
        self.current_level() < BsdSecurelevel::Secure
    }

    /// Checks if firewall rule modifications are allowed
    pub fn check_firewall_modification_allowed(&self) -> bool {
        self.current_level() < BsdSecurelevel::NetworkSecure
    }
}

impl Default for BsdSecurelevelController {
    fn default() -> Self {
        Self::new(BsdSecurelevel::Insecure)
    }
}


// ==========================================
// 10. Rootless Privileged Port Binding Manager
// ==========================================

pub struct RootlessPortBindingManager {
    pub unprivileged_port_start: u16,
    pub explicitly_allowed_users: Vec<u32>,
}

impl RootlessPortBindingManager {
    pub fn new(port_start: u16) -> Self {
        Self {
            unprivileged_port_start: port_start,
            explicitly_allowed_users: Vec::new(),
        }
    }

    pub fn allow_user_port_access(&mut self, uid: u32) {
        if !self.explicitly_allowed_users.contains(&uid) {
            self.explicitly_allowed_users.push(uid);
        }
    }

    pub fn can_bind_port(&self, uid: u32, port: u16) -> bool {
        if uid == 0 {
            return true; // Root can bind any port
        }

        if port >= self.unprivileged_port_start {
            return true; // Above threshold, non-privileged port
        }

        self.explicitly_allowed_users.contains(&uid)
    }
}

impl Default for RootlessPortBindingManager {
    fn default() -> Self {
        Self::new(1024) // Linux default sysctl net.ipv4.ip_unprivileged_port_start = 1024
    }
}

// ==========================================
// Comprehensive Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudo_doas_privilege_elevation() {
        let mut elevator = SudoDoasElevator::new();
        elevator.password_database.push(("admin".to_string(), "secure_hash_123".to_string()));

        // Failed attempt with incorrect password hash
        assert!(elevator
            .elevate_via_doas("admin", "invalid_hash", 10000)
            .is_err());

        // Successful elevation
        let uid = elevator.elevate_via_doas("admin", "secure_hash_123", 10000).unwrap();
        assert_eq!(uid, 0);

        // Verification must confirm active session under TTL
        assert!(elevator.verify_active_sudo_session(0, 15000)); // 5 secs later
        assert!(!elevator.verify_active_sudo_session(0, 1000000)); // ~16 mins later (Expired!)
    }

    #[test]
    fn test_polkit_enforcer_rules() {
        let enforcer = PolkitEnforcer::new();

        // Admin action org.sigmaos.network.control requires root or active sudo session
        assert_eq!(
            enforcer.evaluate_polkit_action("org.sigmaos.network.control", 1000, false),
            PolkitAuthorization::ChallengeMfa
        );

        assert_eq!(
            enforcer.evaluate_polkit_action("org.sigmaos.network.control", 0, false),
            PolkitAuthorization::Authorized
        );

        // User action org.sigmaos.system.power-off allows non-root without active session
        assert_eq!(
            enforcer.evaluate_polkit_action("org.sigmaos.system.power-off", 1000, false),
            PolkitAuthorization::Authorized
        );
    }

    #[test]
    fn test_capability_splitter_assignments() {
        let mut splitter = CapSplitter::new();
        splitter.assign_capability(LinuxCap::CapNetAdmin);
        splitter.assign_capability(LinuxCap::CapSysAdmin);

        // Can activate assigned capabilities
        assert!(splitter.activate_capability(LinuxCap::CapNetAdmin));
        assert!(splitter.check_capability(LinuxCap::CapNetAdmin));

        // Cannot activate unassigned capability
        assert!(!splitter.activate_capability(LinuxCap::CapKill));
        assert!(!splitter.check_capability(LinuxCap::CapKill));

        // Dropping capabilities completely cleans the permission flags
        splitter.drop_capability(LinuxCap::CapNetAdmin);
        assert!(!splitter.check_capability(LinuxCap::CapNetAdmin));
    }

    #[test]
    fn test_rootless_user_namespaces() {
        let manager = RootlessNamespaceManager::new();

        // Maps inside container root user (0) to outside host user (1000)
        assert_eq!(manager.translate_inside_to_outside_uid(0), 1000);
        assert_eq!(manager.translate_outside_to_inside_uid(1000), 0);

        // Fallbacks remain identical
        assert_eq!(manager.translate_inside_to_outside_uid(500), 500);
    }

    #[test]
    fn test_doas_last_match_rules() {
        let mut engine = DoasRuleEngine::new();
        engine.add_rule(DoasRule {
            action: DoasAction::Deny,
            identity: "alice".to_string(),
            target_user: "root".to_string(),
            keepenv: false,
            nopass: false,
            command: None,
        });
        engine.add_rule(DoasRule {
            action: DoasAction::Permit,
            identity: "alice".to_string(),
            target_user: "root".to_string(),
            keepenv: true,
            nopass: true,
            command: Some("reboot".to_string()),
        });

        let res1 = engine.evaluate("alice", false, "root", "shutdown").unwrap();
        assert_eq!(res1.action, DoasAction::Deny);

        let res2 = engine.evaluate("alice", false, "root", "reboot").unwrap();
        assert_eq!(res2.action, DoasAction::Permit);
        assert!(res2.keepenv);
        assert!(res2.nopass);
    }

    #[test]
    fn test_bsd_securelevel_enforcement() {
        let mut guard = BsdSecurelevelGuard::new(SecureLevel::Insecure);
        assert!(guard.allow_module_loading());
        assert!(guard.allow_raw_disk_write());

        assert!(guard.raise_level(SecureLevel::Secure).is_ok());
        assert!(!guard.allow_module_loading());
        assert!(!guard.allow_raw_disk_write());
        assert!(guard.allow_time_adjustment());

        assert!(guard.raise_level(SecureLevel::HighlySecure).is_ok());
        assert!(!guard.allow_time_adjustment());

        assert!(guard.raise_level(SecureLevel::Insecure).is_err());
    }

    #[test]
    fn test_subuid_gid_mapping() {
        let mut mapper = SubUidGidMapper::new();
        mapper.add_subuid_range("bob", 100000, 65536);
        mapper.add_subgid_range("bob", 100000, 65536);

        assert!(mapper.is_subuid_valid("bob", 100000));
        assert!(mapper.is_subuid_valid("bob", 165535));
        assert!(!mapper.is_subuid_valid("bob", 165536));
        assert!(!mapper.is_subuid_valid("bob", 99999));
        assert!(!mapper.is_subuid_valid("alice", 100000));
    }

    #[test]
    fn test_pam_mfa_authenticator() {
        let auth = PamMfaAuthenticator::new(123456);
        assert!(auth.verify_mfa_code(123456));
        assert!(!auth.verify_mfa_code(999999));
    }

    #[test]
    fn test_linux_inspired_pam_stack() {
        let mut engine = PamEngine::new();

        let unix_db: Vec<(String, String)> = vec![
            ("alice".to_string(), "alice_pwd_hash".to_string()),
        ];
        let pam_unix = alloc::sync::Arc::new(PamUnixModule::new(unix_db));
        let pam_faillock = alloc::sync::Arc::new(PamFaillockModule);
        let pam_time = alloc::sync::Arc::new(PamTimeModule::new(9, 17)); // 9 AM to 5 PM
        let pam_mfa = alloc::sync::Arc::new(PamMfaPluggableModule);

        // Scenario 1: Configure stack: Required pam_faillock + Required pam_unix + Optional pam_mfa
        engine.add_rule(
            PamGroup::Auth,
            PamRule {
                control_flag: PamControlFlag::Required,
                module: pam_faillock.clone(),
            },
        );
        engine.add_rule(
            PamGroup::Auth,
            PamRule {
                control_flag: PamControlFlag::Required,
                module: pam_unix.clone(),
            },
        );
        engine.add_rule(
            PamGroup::Auth,
            PamRule {
                control_flag: PamControlFlag::Optional,
                module: pam_mfa.clone(),
            },
        );

        // Test valid authentication
        assert_eq!(
            engine.execute_group(PamGroup::Auth, "alice", "alice_pwd_hash"),
            PamResult::Success
        );

        // Test wrong credentials
        assert_eq!(
            engine.execute_group(PamGroup::Auth, "alice", "wrong_hash"),
            PamResult::AuthError
        );

        // Test authentication with unknown user
        assert_eq!(
            engine.execute_group(PamGroup::Auth, "bob", "any_hash"),
            PamResult::UserUnknown
        );

        // Test authentication with correct credentials
        assert_eq!(
            engine.execute_group(PamGroup::Auth, "alice", "alice_pwd_hash"),
            PamResult::Success
        );

        // Scenario 2: Test account lockout with pam_faillock
        engine.context.failed_attempts = 4; // Locked out!
        assert_eq!(
            engine.execute_group(PamGroup::Auth, "alice", "alice_pwd_hash"),
            PamResult::MaxTries
        );

        // Scenario 3: Test pam_time access restriction (outside allowed hours)
        let mut engine_acct = PamEngine::new();
        engine_acct.add_rule(
            PamGroup::Account,
            PamRule {
                control_flag: PamControlFlag::Required,
                module: pam_time.clone(),
            },
        );

        // Current time: 20:00 (8 PM), outside 9 AM - 5 PM window
        engine_acct.context.current_time_hour = 20;
        assert_eq!(
            engine_acct.execute_group(PamGroup::Account, "alice", ""),
            PamResult::PermissionDenied
        );

        // Current time: 10:00 (10 AM), within window
        engine_acct.context.current_time_hour = 10;
        assert_eq!(
            engine_acct.execute_group(PamGroup::Account, "alice", ""),
            PamResult::Success
        );
    }

    #[test]
    fn test_bsd_securelevels() {
        let controller = BsdSecurelevelController::new(BsdSecurelevel::Insecure);
        assert_eq!(controller.current_level(), BsdSecurelevel::Insecure);
        assert!(controller.check_raw_disk_write_allowed());
        assert!(controller.check_module_loading_allowed());

        // Raise level to Secure (1)
        assert!(controller.raise_level(BsdSecurelevel::Secure).is_ok());
        assert_eq!(controller.current_level(), BsdSecurelevel::Secure);
        assert!(!controller.check_raw_disk_write_allowed());
        assert!(!controller.check_module_loading_allowed());

        // Attempting to lower securelevel back to Insecure fails!
        assert!(controller.raise_level(BsdSecurelevel::Insecure).is_err());
        assert_eq!(controller.current_level(), BsdSecurelevel::Secure);
    }

    #[test]
    fn test_doas_rule_engine() {
        let mut engine = DoasRuleEngine::new();

        // In OpenBSD doas.conf, rules are evaluated in order and the last matching rule wins.
        // General rule for :wheel group:
        engine.add_rule(DoasRule {
            action: DoasAction::Permit,
            identity: ":wheel".to_string(),
            target_user: "root".to_string(),
            command: None,
            nopass: false,
            keepenv: true,
        });

        // Specific override rule for alice for /sbin/reboot with nopass:
        engine.add_rule(DoasRule {
            action: DoasAction::Permit,
            identity: "alice".to_string(),
            target_user: "root".to_string(),
            command: Some("/sbin/reboot".to_string()),
            nopass: true,
            keepenv: false,
        });

        // Evaluate reboot for alice
        let res1 = engine.evaluate("alice", true, "root", "/sbin/reboot").unwrap();
        assert_eq!(res1.action, DoasAction::Permit);
        assert!(res1.nopass);

        // Evaluate general command for bob in wheel group
        let res2 = engine.evaluate("bob", true, "root", "/usr/bin/htop").unwrap();
        assert_eq!(res2.action, DoasAction::Permit);
        assert!(!res2.nopass);
        assert!(res2.keepenv);

        // Evaluate unauthorized user charlie
        let res3 = engine.evaluate("charlie", false, "root", "/sbin/reboot");
        assert!(res3.is_none());
    }

    #[test]
    fn test_subuid_gid_mapping_container() {
        let mut mapper = SubUidGidMapper::new();
        mapper.add_subuid_range("alice", 100000, 65536);

        assert_eq!(mapper.map_container_uid("alice", 0), Some(100000));
        assert_eq!(mapper.map_container_uid("alice", 1000), Some(101000));
        assert_eq!(mapper.map_container_uid("alice", 70000), None); // Exceeds count
    }

    #[test]
    fn test_rootless_port_binding() {
        let mut manager = RootlessPortBindingManager::new(1024);

        // Root user can bind any port
        assert!(manager.can_bind_port(0, 80));

        // Regular user cannot bind privileged port (< 1024)
        assert!(!manager.can_bind_port(1000, 80));

        // Regular user can bind unprivileged port (>= 1024)
        assert!(manager.can_bind_port(1000, 8080));

        // Allow user 1000 explicitly
        manager.allow_user_port_access(1000);
        assert!(manager.can_bind_port(1000, 80));
    }
}
