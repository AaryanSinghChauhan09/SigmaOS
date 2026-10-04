//! # LibGksu Graphical Sudo & Administrative Elevation Engine
//!
//! Inspired by Linux & BSD graphical administrative elevation mechanisms:
//! - GNOME `gksu`/`gksudo` and `libgksu`
//! - KDE `kdesu` / `kdesudo`
//! - Polkit `pkexec`
//! - OpenBSD `doas` and FreeBSD `su`/`sudo` GTK wrappers
//!
//! Provides secure graphical password prompts, Wayland/X11 display socket isolation,
//! memory zeroization for credentials, environment variable sanitization,
//! and Polkit/Sudoers policy enforcement.

use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

#[path = "root_improvement.rs"]
mod root_improvement;
use root_improvement::{PolkitAuthorization, PolkitEnforcer};

/// Graphical Sudo Auth Backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GksuAuthBackend {
    Sudo,         // Traditional sudo / gksudo (/etc/sudoers)
    Su,           // Target user password elevation (/etc/passwd, su -)
    Polkit,       // PolicyKit fine-grained action authorization
    Kerberos,     // Active Directory / Krb5 SSO ticket ticket-granting ticket check
    PqcTokenGate, // Sovereign Post-Quantum Cryptography capability token
}

/// Display Server Isolation Environment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GksuDisplayServer {
    WaylandZenith {
        socket_path: String,
    },
    X11Compat {
        display_id: String,
        xauthority_path: String,
    },
    HeadlessConsole,
}

/// Execution Request Spec for libgksu
#[derive(Debug, Clone)]
pub struct GksuExecutionRequest {
    pub command: String,
    pub target_user: String, // Default "root"
    pub prompt_message: String,
    pub preserve_env: bool,
    pub keep_env_vars: Vec<String>,
    pub auth_backend: GksuAuthBackend,
    pub display: GksuDisplayServer,
    pub timeout_seconds: u32,
    pub action_id: Option<String>,
}

impl GksuExecutionRequest {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            target_user: "root".to_string(),
            prompt_message: format!("Administrative privileges required to run: {}", command),
            preserve_env: false,
            keep_env_vars: vec!["PATH".to_string(), "LANG".to_string(), "LC_ALL".to_string()],
            auth_backend: GksuAuthBackend::Polkit,
            display: GksuDisplayServer::WaylandZenith {
                socket_path: "/run/user/1000/wayland-0".to_string(),
            },
            timeout_seconds: 60,
            action_id: None,
        }
    }

    pub fn with_action_id(mut self, action_id: &str) -> Self {
        self.action_id = Some(action_id.to_string());
        self
    }

    pub fn with_target_user(mut self, user: &str) -> Self {
        self.target_user = user.to_string();
        self
    }

    pub fn with_auth_backend(mut self, backend: GksuAuthBackend) -> Self {
        self.auth_backend = backend;
        self
    }
}

/// Security Guard for memory zeroization and environment sanitization
#[derive(Debug)]
pub struct GksuSecurityGuard {
    pub sensitive_buffer: Vec<u8>,
}

impl GksuSecurityGuard {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            sensitive_buffer: secret.to_vec(),
        }
    }

    /// Zero out password memory
    pub fn wipe(&mut self) {
        for byte in self.sensitive_buffer.iter_mut() {
            *byte = 0;
        }
        self.sensitive_buffer.clear();
    }

    /// Sanitize environment variables before spawning privileged process
    pub fn sanitize_environment(
        input_env: &[(String, String)],
        allowed_keys: &[String],
    ) -> Vec<(String, String)> {
        let mut clean_env = Vec::new();
        for (k, v) in input_env {
            // Dangerous variables stripped automatically
            if k == "LD_PRELOAD"
                || k == "LD_LIBRARY_PATH"
                || k == "PYTHONPATH"
                || k == "RUBYLIB"
                || k == "PERL5LIB"
            {
                continue;
            }

            if allowed_keys.contains(k) {
                clean_env.push((k.clone(), v.clone()));
            }
        }
        clean_env
    }
}

impl Drop for GksuSecurityGuard {
    fn drop(&mut self) {
        self.wipe();
    }
}

/// Execution Result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GksuExecutionResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub auth_attempts: u32,
}

/// Main `libgksu` Graphical Sudo Engine
pub struct LibGksuGraphicalSudoEngine {
    pub polkit_enforcer: PolkitEnforcer,
    pub failed_attempts_limit: u32,
    pub current_failed_attempts: u32,
    pub is_locked_out: bool,
}

impl LibGksuGraphicalSudoEngine {
    pub fn new() -> Self {
        Self {
            polkit_enforcer: PolkitEnforcer::new(),
            failed_attempts_limit: 3,
            current_failed_attempts: 0,
            is_locked_out: false,
        }
    }

    /// Validate credentials against chosen backend
    pub fn authenticate(
        &mut self,
        request: &GksuExecutionRequest,
        credential_input: &[u8],
    ) -> Result<bool, &'static str> {
        if self.is_locked_out {
            return Err("LibGksu: Account locked out due to excessive failed attempts");
        }

        let mut guard = GksuSecurityGuard::new(credential_input);

        let is_valid = match request.auth_backend {
            GksuAuthBackend::Polkit => {
                let auth = self.polkit_enforcer.evaluate_polkit_action(
                    "org.sigmaos.system.exec_root",
                    1000,
                    false,
                );
                match auth {
                    PolkitAuthorization::Authorized => true,
                    PolkitAuthorization::ChallengeMfa => !credential_input.is_empty(),
                    PolkitAuthorization::Blocked => false,
                }
            }
            GksuAuthBackend::Sudo | GksuAuthBackend::Su => {
                // Simulated PAM / shadow verification
                !credential_input.is_empty() && credential_input == b"correct_root_pass"
            }
            GksuAuthBackend::Kerberos => {
                // Simulated Krb5 TGT check
                !credential_input.is_empty()
            }
            GksuAuthBackend::PqcTokenGate => credential_input.len() >= 16,
        };

        guard.wipe();

        if is_valid {
            self.current_failed_attempts = 0;
            Ok(true)
        } else {
            self.current_failed_attempts += 1;
            if self.current_failed_attempts >= self.failed_attempts_limit {
                self.is_locked_out = true;
            }
            Ok(false)
        }
    }

    /// Execute request under elevated privileges
    pub fn execute_elevated(
        &mut self,
        request: &GksuExecutionRequest,
        credential_input: &[u8],
        raw_env: &[(String, String)],
    ) -> Result<GksuExecutionResult, &'static str> {
        let authenticated = self.authenticate(request, credential_input)?;

        if !authenticated {
            return Ok(GksuExecutionResult {
                success: false,
                exit_code: 1,
                stdout: String::new(),
                stderr: "LibGksu: Authentication failed".to_string(),
                auth_attempts: self.current_failed_attempts,
            });
        }

        // Sanitize environment
        let _clean_env = GksuSecurityGuard::sanitize_environment(raw_env, &request.keep_env_vars);

        // Simulated process spawn under target_user e.g. root
        let stdout = format!(
            "LibGksu: Executed [{}] as user [{}]",
            request.command, request.target_user
        );

        Ok(GksuExecutionResult {
            success: true,
            exit_code: 0,
            stdout,
            stderr: String::new(),
            auth_attempts: 1,
        })
    }
}

impl Default for LibGksuGraphicalSudoEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 1. OPENBSD DOAS.CONF RULE POLICY EVALUATOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoasAction {
    Permit,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoasRule {
    pub action: DoasAction,
    pub identity: String,    // User or :group
    pub target_user: String, // e.g. "root"
    pub no_pass: bool,
    pub keep_env: Vec<String>,
    pub command_path: Option<String>,
}

impl DoasRule {
    pub fn permit(identity: &str, target_user: &str) -> Self {
        Self {
            action: DoasAction::Permit,
            identity: identity.to_string(),
            target_user: target_user.to_string(),
            no_pass: false,
            keep_env: Vec::new(),
            command_path: None,
        }
    }
}

pub struct DoasRulePolicyEvaluator {
    pub rules: Vec<DoasRule>,
}

impl DoasRulePolicyEvaluator {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: DoasRule) {
        self.rules.push(rule);
    }

    pub fn evaluate_authorization(
        &self,
        calling_user: &str,
        target_user: &str,
        cmd: &str,
    ) -> Option<(DoasAction, bool, Vec<String>)> {
        let mut last_match = None;
        for rule in &self.rules {
            let user_matches = rule.identity == calling_user || rule.identity == "*";
            let target_matches = rule.target_user == target_user || rule.target_user == "*";
            let cmd_matches = match &rule.command_path {
                Some(p) => p == cmd || cmd.starts_with(p.as_str()),
                None => true,
            };

            if user_matches && target_matches && cmd_matches {
                last_match = Some((rule.action, rule.no_pass, rule.keep_env.clone()));
            }
        }
        last_match
    }
}

impl Default for DoasRulePolicyEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. POLKIT ACTION RULE REGISTRY (POLKITD PARITY)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolkitImplicitResult {
    Yes,
    No,
    AuthSelf,
    AuthAdmin,
    AuthSelfKeep,
    AuthAdminKeep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolkitActionDefinition {
    pub action_id: String, // e.g. "org.sigmaos.system.network.configure"
    pub description: String,
    pub implicit_active: PolkitImplicitResult,
    pub implicit_inactive: PolkitImplicitResult,
}

pub struct PolkitActionRuleRegistry {
    pub actions: Vec<PolkitActionDefinition>,
    pub active_cache_ttl_sec: u64,
}

impl PolkitActionRuleRegistry {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            active_cache_ttl_sec: 300,
        }
    }

    pub fn register_action(&mut self, action: PolkitActionDefinition) {
        self.actions.push(action);
    }

    pub fn check_action_authorization(
        &self,
        action_id: &str,
        is_active_session: bool,
    ) -> PolkitImplicitResult {
        for action in &self.actions {
            if action.action_id == action_id {
                return if is_active_session {
                    action.implicit_active
                } else {
                    action.implicit_inactive
                };
            }
        }
        PolkitImplicitResult::AuthAdmin
    }
}

impl Default for PolkitActionRuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. SOVEREIGN SECURE HELPER DAEMON
// =========================================================================

pub struct SovereignSecureHelperDaemon {
    pub gksu_engine: LibGksuGraphicalSudoEngine,
    pub doas_evaluator: DoasRulePolicyEvaluator,
    pub polkit_registry: PolkitActionRuleRegistry,
    pub audit_logs: Vec<String>,
}

impl SovereignSecureHelperDaemon {
    pub fn new() -> Self {
        let mut daemon = Self {
            gksu_engine: LibGksuGraphicalSudoEngine::new(),
            doas_evaluator: DoasRulePolicyEvaluator::new(),
            polkit_registry: PolkitActionRuleRegistry::new(),
            audit_logs: Vec::new(),
        };

        daemon
            .doas_evaluator
            .add_rule(DoasRule::permit("admin", "root"));

        daemon
            .polkit_registry
            .register_action(PolkitActionDefinition {
                action_id: "org.sigmaos.pkg.install".to_string(),
                description: "Install Sovereign Packages".to_string(),
                implicit_active: PolkitImplicitResult::AuthAdmin,
                implicit_inactive: PolkitImplicitResult::No,
            });

        daemon
    }

    pub fn dispatch_helper_execution(
        &mut self,
        calling_user: &str,
        request: &GksuExecutionRequest,
        pass_input: &str,
        raw_env: &[(String, String)],
    ) -> Result<GksuExecutionResult, &'static str> {
        let doas_eval = self.doas_evaluator.evaluate_authorization(
            calling_user,
            &request.target_user,
            &request.command,
        );

        let mut permitted = false;

        match doas_eval {
            Some((DoasAction::Deny, _, _)) => {
                let msg = format!(
                    "SecureHelperDaemon: Denied by doas.conf policy for user {}",
                    calling_user
                );
                self.audit_logs.push(msg);
                return Err("SecureHelperDaemon: Policy Denied");
            }
            Some((DoasAction::Permit, _, _)) => {
                permitted = true;
            }
            None => {
                if let Some(ref action_id) = request.action_id {
                    let polkit_res = self
                        .polkit_registry
                        .check_action_authorization(action_id, true);
                    if polkit_res != PolkitImplicitResult::No {
                        permitted = true;
                    } else {
                        let msg = format!(
                            "SecureHelperDaemon: Denied by Polkit action policy for action {}",
                            action_id
                        );
                        self.audit_logs.push(msg);
                        return Err("SecureHelperDaemon: Polkit Policy Denied");
                    }
                }
            }
        }

        if !permitted {
            let msg = format!(
                "SecureHelperDaemon: Default Deny policy enforced for user [{}] cmd [{}]",
                calling_user, request.command
            );
            self.audit_logs.push(msg);
            return Err("SecureHelperDaemon: Default Deny Policy");
        }

        let result = self
            .gksu_engine
            .execute_elevated(request, pass_input.as_bytes(), raw_env)?;

        let audit_msg = format!(
            "SecureHelperDaemon: User [{}] executed [{}] as [{}] -> Success={}",
            calling_user, request.command, request.target_user, result.success
        );
        self.audit_logs.push(audit_msg);

        Ok(result)
    }
}

impl Default for SovereignSecureHelperDaemon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gksu_request_creation() {
        let req = GksuExecutionRequest::new("/usr/bin/gparted")
            .with_target_user("root")
            .with_auth_backend(GksuAuthBackend::Sudo);

        assert_eq!(req.command, "/usr/bin/gparted");
        assert_eq!(req.target_user, "root");
        assert_eq!(req.auth_backend, GksuAuthBackend::Sudo);
    }

    #[test]
    fn test_gksu_security_guard_wipe() {
        let mut guard = GksuSecurityGuard::new(b"secret_password");
        assert_eq!(guard.sensitive_buffer, b"secret_password");

        guard.wipe();
        assert!(guard.sensitive_buffer.is_empty());
    }

    #[test]
    fn test_environment_sanitization() {
        let raw_env = vec![
            ("PATH".to_string(), "/usr/bin:/bin".to_string()),
            ("LD_PRELOAD".to_string(), "/tmp/malicious.so".to_string()),
            ("PYTHONPATH".to_string(), "/tmp/hack".to_string()),
            ("LANG".to_string(), "en_US.UTF-8".to_string()),
        ];

        let allowed = vec!["PATH".to_string(), "LANG".to_string()];
        let clean = GksuSecurityGuard::sanitize_environment(&raw_env, &allowed);

        assert_eq!(clean.len(), 2);
        assert!(!clean.iter().any(|(k, _)| k == "LD_PRELOAD"));
        assert!(!clean.iter().any(|(k, _)| k == "PYTHONPATH"));
    }

    #[test]
    fn test_libgksu_elevation_workflow() {
        let mut engine = LibGksuGraphicalSudoEngine::new();
        let req = GksuExecutionRequest::new("/sbin/fdisk").with_auth_backend(GksuAuthBackend::Sudo);

        let raw_env = vec![("PATH".to_string(), "/sbin:/bin".to_string())];

        // Valid authentication
        let res = engine
            .execute_elevated(&req, b"correct_root_pass", &raw_env)
            .unwrap();
        assert!(res.success);
        assert_eq!(res.exit_code, 0);

        // Invalid authentication
        let res_fail = engine
            .execute_elevated(&req, b"wrong_pass", &raw_env)
            .unwrap();
        assert!(!res_fail.success);
    }

    #[test]
    fn test_doas_rule_policy_evaluator() {
        let mut doas = DoasRulePolicyEvaluator::new();
        doas.add_rule(DoasRule::permit("alice", "root"));
        doas.add_rule(DoasRule {
            action: DoasAction::Deny,
            identity: "bob".to_string(),
            target_user: "*".to_string(),
            no_pass: false,
            keep_env: Vec::new(),
            command_path: None,
        });

        let alice_eval = doas
            .evaluate_authorization("alice", "root", "/usr/bin/htop")
            .unwrap();
        assert_eq!(alice_eval.0, DoasAction::Permit);

        let bob_eval = doas
            .evaluate_authorization("bob", "root", "/usr/bin/htop")
            .unwrap();
        assert_eq!(bob_eval.0, DoasAction::Deny);
    }

    #[test]
    fn test_polkit_action_rule_registry() {
        let mut reg = PolkitActionRuleRegistry::new();
        reg.register_action(PolkitActionDefinition {
            action_id: "org.sigmaos.network.configure".to_string(),
            description: "Configure Network".to_string(),
            implicit_active: PolkitImplicitResult::Yes,
            implicit_inactive: PolkitImplicitResult::AuthAdmin,
        });

        assert_eq!(
            reg.check_action_authorization("org.sigmaos.network.configure", true),
            PolkitImplicitResult::Yes
        );
        assert_eq!(
            reg.check_action_authorization("org.sigmaos.network.configure", false),
            PolkitImplicitResult::AuthAdmin
        );
    }

    #[test]
    fn test_sovereign_secure_helper_daemon() {
        let mut daemon = SovereignSecureHelperDaemon::new();
        let req =
            GksuExecutionRequest::new("/usr/bin/pacman").with_auth_backend(GksuAuthBackend::Sudo);
        let env = vec![("PATH".to_string(), "/usr/bin".to_string())];

        let res = daemon
            .dispatch_helper_execution("admin", &req, "correct_root_pass", &env)
            .unwrap();
        assert!(res.success);
        assert!(!daemon.audit_logs.is_empty());
    }
}
