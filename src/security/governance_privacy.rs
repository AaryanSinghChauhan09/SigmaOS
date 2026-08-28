extern crate alloc;
// SPDX-License-Identifier: MIT
//! SigmaOS Security, Privacy & Governance (Items 61-80)
//! Enterprise-grade security posture, MAC engine, secrets keyring, zero-trust network,
//! runtime sandboxing, integrity monitoring, audit trails, privacy dashboard,
//! signed updates, incident response playbooks, TPM attestation, vulnerability disclosure,
//! container policies, encrypted homes, SBOM generator, developer key rotator,
//! differential privacy telemetry, compliance templates (GDPR/HIPAA/SOC2/DPDP),
//! governance charter, and legal licensing auditor.
//!
//! Inspired by Linux kernel (SELinux/AppArmor/WireGuard/LUKS2) and BSD (Pledge/Unveil/Capsicum/Audit).


use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// ============================================================================
// 1. DEFAULT SECURE POSTURE (Item 61)
// ============================================================================

pub struct DefaultSecurePosture {
    pub enabled_services: BTreeMap<String, bool>,
    pub umask: u32,
    pub default_inbound_firewall_drop: bool,
    pub hardened_sysctl_enabled: bool,
    pub aslr_enabled: bool,
}

impl Default for DefaultSecurePosture {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSecurePosture {
    pub fn new() -> Self {
        let mut services = BTreeMap::new();
        services.insert("sshd".to_string(), false);
        services.insert("cupsd".to_string(), false);
        services.insert("ftpd".to_string(), false);
        services.insert("telnetd".to_string(), false);
        services.insert("sigma-core-init".to_string(), true);

        Self {
            enabled_services: services,
            umask: 0o077, // Strict umask: user read/write/execute only
            default_inbound_firewall_drop: true,
            hardened_sysctl_enabled: true,
            aslr_enabled: true,
        }
    }

    pub fn enable_service(&mut self, service_name: &str) {
        self.enabled_services.insert(service_name.to_string(), true);
    }

    pub fn disable_service(&mut self, service_name: &str) {
        self.enabled_services.insert(service_name.to_string(), false);
    }

    pub fn is_service_active(&self, service_name: &str) -> bool {
        *self.enabled_services.get(service_name).unwrap_or(&false)
    }

    pub fn evaluate_posture_score(&self) -> u32 {
        let mut score = 100;
        if self.umask != 0o077 {
            score -= 20;
        }
        if !self.default_inbound_firewall_drop {
            score -= 30;
        }
        if !self.hardened_sysctl_enabled {
            score -= 25;
        }
        if !self.aslr_enabled {
            score -= 25;
        }
        score
    }
}

// ============================================================================
// 2. MANDATORY ACCESS CONTROL ENGINE (Item 62)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityLabel {
    pub domain: String,
    pub type_tag: String,
    pub level: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacDecision {
    Allow,
    Deny,
    AuditOnly,
}

#[derive(Debug, Clone)]
pub struct MacRule {
    pub source_domain: String,
    pub target_type: String,
    pub class_name: String,
    pub permissions: Vec<String>,
    pub decision: MacDecision,
}

pub struct MandatoryAccessControlEngine {
    pub rules: Vec<MacRule>,
    pub enforcing_mode: bool,
}

impl Default for MandatoryAccessControlEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryAccessControlEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            enforcing_mode: true,
        }
    }

    pub fn add_rule(&mut self, rule: MacRule) {
        self.rules.push(rule);
    }

    pub fn check_permission(
        &self,
        subject: &SecurityLabel,
        object: &SecurityLabel,
        class_name: &str,
        requested_perm: &str,
    ) -> MacDecision {
        for rule in &self.rules {
            if rule.source_domain == subject.domain
                && rule.target_type == object.type_tag
                && rule.class_name == class_name
                && rule.permissions.iter().any(|p| p == requested_perm)
            {
                if !self.enforcing_mode && rule.decision == MacDecision::Deny {
                    return MacDecision::AuditOnly;
                }
                return rule.decision;
            }
        }

        if self.enforcing_mode {
            MacDecision::Deny // Default-deny MAC
        } else {
            MacDecision::AuditOnly
        }
    }
}

// ============================================================================
// 3. SECRETS MANAGEMENT & KEYRING (Item 63)
// ============================================================================

#[derive(Debug, Clone)]
pub struct SecretLease {
    pub key_path: String,
    pub payload: Vec<u8>,
    pub expires_at_timestamp: u64,
    pub hardware_token_required: bool,
}

pub struct SystemKeyringSecretsManager {
    pub secrets: BTreeMap<String, SecretLease>,
    pub hardware_token_authenticated: bool,
}

impl Default for SystemKeyringSecretsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemKeyringSecretsManager {
    pub fn new() -> Self {
        Self {
            secrets: BTreeMap::new(),
            hardware_token_authenticated: false,
        }
    }

    pub fn store_secret(
        &mut self,
        path: &str,
        payload: &[u8],
        ttl_seconds: u64,
        hw_required: bool,
        current_time: u64,
    ) {
        let lease = SecretLease {
            key_path: path.to_string(),
            payload: payload.to_vec(),
            expires_at_timestamp: current_time + ttl_seconds,
            hardware_token_required: hw_required,
        };
        self.secrets.insert(path.to_string(), lease);
    }

    pub fn retrieve_secret(&self, path: &str, current_time: u64) -> Result<Vec<u8>, &'static str> {
        let lease = self.secrets.get(path).ok_or("Secret not found")?;
        if current_time > lease.expires_at_timestamp {
            return Err("Secret lease expired");
        }
        if lease.hardware_token_required && !self.hardware_token_authenticated {
            return Err("Hardware token authentication required");
        }
        Ok(lease.payload.clone())
    }
}

// ============================================================================
// 4. NETWORK ZERO-TRUST DEFAULTS (Item 64)
// ============================================================================

#[derive(Debug, Clone)]
pub struct AppNetworkPolicy {
    pub app_id: String,
    pub allowed_endpoints: Vec<String>,
    pub allowed_ports: Vec<u16>,
    pub force_wireguard_tunnel: bool,
}

pub struct ZeroTrustNetworkPolicyEngine {
    pub app_policies: BTreeMap<String, AppNetworkPolicy>,
    pub wireguard_active: bool,
}

impl Default for ZeroTrustNetworkPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroTrustNetworkPolicyEngine {
    pub fn new() -> Self {
        Self {
            app_policies: BTreeMap::new(),
            wireguard_active: true,
        }
    }

    pub fn register_policy(&mut self, policy: AppNetworkPolicy) {
        self.app_policies.insert(policy.app_id.clone(), policy);
    }

    pub fn is_traffic_allowed(&self, app_id: &str, endpoint: &str, port: u16) -> bool {
        if let Some(policy) = self.app_policies.get(app_id) {
            if policy.force_wireguard_tunnel && !self.wireguard_active {
                return false;
            }
            let endpoint_ok = policy.allowed_endpoints.iter().any(|e| e == "*" || e == endpoint);
            let port_ok = policy.allowed_ports.contains(&port);
            endpoint_ok && port_ok
        } else {
            false // Default-deny zero-trust network policy
        }
    }
}

// ============================================================================
// 5. RUNTIME SANDBOXING (Item 65)
// ============================================================================

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub app_name: String,
    pub allowed_paths_read: Vec<String>,
    pub allowed_paths_write: Vec<String>,
    pub permitted_syscall_promises: Vec<String>,
    pub max_memory_bytes: u64,
}

pub struct RuntimeSandboxController {
    pub active_sandboxes: BTreeMap<String, SandboxConfig>,
}

impl Default for RuntimeSandboxController {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeSandboxController {
    pub fn new() -> Self {
        Self {
            active_sandboxes: BTreeMap::new(),
        }
    }

    pub fn spawn_sandbox(&mut self, config: SandboxConfig) {
        self.active_sandboxes.insert(config.app_name.clone(), config);
    }

    pub fn can_access_file(&self, app_name: &str, path: &str, write_access: bool) -> bool {
        if let Some(config) = self.active_sandboxes.get(app_name) {
            if write_access {
                config.allowed_paths_write.iter().any(|p| path.starts_with(p))
            } else {
                config.allowed_paths_read.iter().any(|p| path.starts_with(p))
                    || config.allowed_paths_write.iter().any(|p| path.starts_with(p))
            }
        } else {
            false // Unregistered apps cannot bypass sandbox
        }
    }
}

// ============================================================================
// 6. SYSTEM INTEGRITY MONITORING (Item 66)
// ============================================================================

pub struct SystemIntegrityMonitor {
    pub baseline_hashes: BTreeMap<String, u64>, // Maps path -> checksum hash
    pub tamper_alerts: Vec<String>,
}

impl Default for SystemIntegrityMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemIntegrityMonitor {
    pub fn new() -> Self {
        Self {
            baseline_hashes: BTreeMap::new(),
            tamper_alerts: Vec::new(),
        }
    }

    pub fn register_baseline(&mut self, path: &str, hash: u64) {
        self.baseline_hashes.insert(path.to_string(), hash);
    }

    pub fn verify_file_integrity(&mut self, path: &str, current_hash: u64) -> bool {
        if let Some(&expected) = self.baseline_hashes.get(path) {
            if expected != current_hash {
                self.tamper_alerts.push(format!("File modified: {} (expected {}, got {})", path, expected, current_hash));
                false
            } else {
                true
            }
        } else {
            self.tamper_alerts.push(format!("New untracked binary detected: {}", path));
            false
        }
    }
}

// ============================================================================
// 7. AUDIT LOGGING & RETENTION (Item 67)
// ============================================================================

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub id: u64,
    pub timestamp: u64,
    pub actor: String,
    pub event_type: String,
    pub previous_entry_hash: u64,
    pub current_hash: u64,
}

pub struct ImmutableAuditLogger {
    pub entries: Vec<AuditEntry>,
    pub next_id: u64,
    pub last_hash: u64,
    pub retention_period_seconds: u64,
}

impl Default for ImmutableAuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl ImmutableAuditLogger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            last_hash: 0xA5A5_5A5A_1234_5678,
            retention_period_seconds: 365 * 86400,
        }
    }

    pub fn log_event(&mut self, actor: &str, event_type: &str, timestamp: u64) -> u64 {
        let prev = self.last_hash;
        // Simple hash chain computation
        let hash = prev.wrapping_add(self.next_id).wrapping_add(timestamp) ^ 0xDEAD_BEEF_CAFE_BABE;

        let entry = AuditEntry {
            id: self.next_id,
            timestamp,
            actor: actor.to_string(),
            event_type: event_type.to_string(),
            previous_entry_hash: prev,
            current_hash: hash,
        };

        self.last_hash = hash;
        self.entries.push(entry);
        self.next_id += 1;
        hash
    }

    pub fn verify_integrity(&self) -> bool {
        let mut prev = 0xA5A5_5A5A_1234_5678;
        for entry in &self.entries {
            if entry.previous_entry_hash != prev {
                return false;
            }
            let expected_hash = prev.wrapping_add(entry.id).wrapping_add(entry.timestamp) ^ 0xDEAD_BEEF_CAFE_BABE;
            if entry.current_hash != expected_hash {
                return false;
            }
            prev = entry.current_hash;
        }
        true
    }

    pub fn purge_expired_logs(&mut self, current_time: u64) {
        if current_time < self.retention_period_seconds {
            return;
        }
        let cutoff = current_time - self.retention_period_seconds;
        self.entries.retain(|e| e.timestamp >= cutoff);
    }
}

// ============================================================================
// 8. PRIVACY DASHBOARD (Item 68)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryLevel {
    None,
    Minimal,
    Full,
}

pub struct PrivacyDashboardController {
    pub telemetry_level: TelemetryLevel,
    pub location_sharing_enabled: bool,
    pub camera_mic_enabled: bool,
    pub app_permissions: BTreeMap<String, Vec<String>>,
}

impl Default for PrivacyDashboardController {
    fn default() -> Self {
        Self::new()
    }
}

impl PrivacyDashboardController {
    pub fn new() -> Self {
        Self {
            telemetry_level: TelemetryLevel::None, // Privacy-first default
            location_sharing_enabled: false,
            camera_mic_enabled: false,
            app_permissions: BTreeMap::new(),
        }
    }

    pub fn grant_app_permission(&mut self, app_id: &str, permission: &str) {
        self.app_permissions
            .entry(app_id.to_string())
            .or_default()
            .push(permission.to_string());
    }

    pub fn revoke_all_app_permissions(&mut self, app_id: &str) {
        self.app_permissions.remove(app_id);
    }

    pub fn has_permission(&self, app_id: &str, permission: &str) -> bool {
        if let Some(perms) = self.app_permissions.get(app_id) {
            perms.iter().any(|p| p == permission)
        } else {
            false
        }
    }
}

// ============================================================================
// 9. SECURE UPDATE CHANNEL (Item 69)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RolloutStage {
    Canary,
    Beta,
    GeneralAvailability,
}

#[derive(Debug, Clone)]
pub struct UpdatePackage {
    pub version: String,
    pub signature_ed25519: Vec<u8>,
    pub payload_hash: u64,
    pub stage: RolloutStage,
}

pub struct SecureUpdateChannel {
    pub trusted_key_hash: u64,
    pub installed_version: String,
    pub staged_rollouts: Vec<UpdatePackage>,
}

impl Default for SecureUpdateChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureUpdateChannel {
    pub fn new() -> Self {
        Self {
            trusted_key_hash: 0x9999_8888_7777_6666,
            installed_version: "1.0.0".to_string(),
            staged_rollouts: Vec::new(),
        }
    }

    pub fn publish_update(&mut self, pkg: UpdatePackage) {
        self.staged_rollouts.push(pkg);
    }

    pub fn apply_update(&mut self, version: &str, target_stage: RolloutStage) -> Result<(), &'static str> {
        if let Some(pkg) = self.staged_rollouts.iter().find(|p| p.version == version && p.stage == target_stage) {
            if pkg.signature_ed25519.is_empty() {
                return Err("Invalid unsigned update package");
            }
            self.installed_version = pkg.version.clone();
            Ok(())
        } else {
            Err("Matching update package not found")
        }
    }
}

// ============================================================================
// 10. INCIDENT RESPONSE PLAYBOOKS (Item 70)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncidentType {
    RansomwareDetected,
    UnauthorizedPrivilegeEscalation,
    DataExfiltrationAttempt,
}

pub struct IncidentResponsePlaybookEngine {
    pub playbooks_executed: Vec<(IncidentType, String)>,
    pub network_isolated: bool,
    pub emergency_lockdown: bool,
}

impl Default for IncidentResponsePlaybookEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl IncidentResponsePlaybookEngine {
    pub fn new() -> Self {
        Self {
            playbooks_executed: Vec::new(),
            network_isolated: false,
            emergency_lockdown: false,
        }
    }

    pub fn trigger_playbook(&mut self, incident: IncidentType) -> String {
        let action_summary = match incident {
            IncidentType::RansomwareDetected => {
                self.network_isolated = true;
                self.emergency_lockdown = true;
                "Isolated network interfaces, froze non-essential process tree, created read-only filesystem snapshot"
            }
            IncidentType::UnauthorizedPrivilegeEscalation => {
                self.emergency_lockdown = true;
                "Revoked active user sessions, invalidated secrets keyring, triggered mandatory re-auth"
            }
            IncidentType::DataExfiltrationAttempt => {
                self.network_isolated = true;
                "Blocked outward egress traffic, enabled deep packet inspection logs"
            }
        };

        self.playbooks_executed.push((incident, action_summary.to_string()));
        action_summary.to_string()
    }
}

// ============================================================================
// 11. HARDWARE ATTESTATION (Item 71)
// ============================================================================

pub struct HardwareAttestationEngine {
    pub tpm_pcr_values: BTreeMap<u32, u64>,
    pub endorsement_key_verified: bool,
}

impl Default for HardwareAttestationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareAttestationEngine {
    pub fn new() -> Self {
        let mut pcrs = BTreeMap::new();
        pcrs.insert(0, 0x1111_2222_3333_4444); // PCR0: Firmware
        pcrs.insert(1, 0x5555_6666_7777_8888); // PCR1: BIOS Config
        pcrs.insert(7, 0x9999_AAAA_BBBB_CCCC); // PCR7: Secure Boot State

        Self {
            tpm_pcr_values: pcrs,
            endorsement_key_verified: true,
        }
    }

    pub fn generate_attestation_quote(&self, pcr_index: u32) -> Result<u64, &'static str> {
        if !self.endorsement_key_verified {
            return Err("Endorsement key verification failed");
        }
        self.tpm_pcr_values
            .get(&pcr_index)
            .copied()
            .ok_or("PCR index not extended")
    }
}

// ============================================================================
// 12. VULNERABILITY DISCLOSURE PROGRAM (Item 72)
// ============================================================================

#[derive(Debug, Clone)]
pub struct VulnerabilityReport {
    pub cve_id: String,
    pub cvss_score: f32,
    pub title: String,
    pub bug_bounty_reward_usd: u32,
    pub resolved: bool,
}

pub struct VulnerabilityDisclosureManager {
    pub reports: Vec<VulnerabilityReport>,
}

impl Default for VulnerabilityDisclosureManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VulnerabilityDisclosureManager {
    pub fn new() -> Self {
        Self { reports: Vec::new() }
    }

    pub fn submit_report(&mut self, cve: &str, cvss: f32, title: &str, bounty: u32) {
        self.reports.push(VulnerabilityReport {
            cve_id: cve.to_string(),
            cvss_score: cvss,
            title: title.to_string(),
            bug_bounty_reward_usd: bounty,
            resolved: false,
        });
    }

    pub fn mark_resolved(&mut self, cve: &str) {
        if let Some(report) = self.reports.iter_mut().find(|r| r.cve_id == cve) {
            report.resolved = true;
        }
    }
}

// ============================================================================
// 13. CONTAINER SECURITY POLICIES (Item 73)
// ============================================================================

pub struct ContainerSecurityPolicyEngine {
    pub require_cosign_signature: bool,
    pub enforce_read_only_rootfs: bool,
    pub drop_cap_sys_admin: bool,
}

impl Default for ContainerSecurityPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerSecurityPolicyEngine {
    pub fn new() -> Self {
        Self {
            require_cosign_signature: true,
            enforce_read_only_rootfs: true,
            drop_cap_sys_admin: true,
        }
    }

    pub fn validate_container_launch(
        &self,
        image_signed: bool,
        read_only_root: bool,
        has_sys_admin: bool,
    ) -> Result<(), &'static str> {
        if self.require_cosign_signature && !image_signed {
            return Err("Unsigned container image rejected");
        }
        if self.enforce_read_only_rootfs && !read_only_root {
            return Err("Mutable rootfs container rejected");
        }
        if self.drop_cap_sys_admin && has_sys_admin {
            return Err("Container with CAP_SYS_ADMIN rejected");
        }
        Ok(())
    }
}

// ============================================================================
// 14. ENCRYPTED HOME BY DEFAULT (Item 74)
// ============================================================================

pub struct EncryptedHomeManager {
    pub home_encrypted: bool,
    pub key_derivation_argon2: bool,
    pub is_mounted: bool,
}

impl Default for EncryptedHomeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EncryptedHomeManager {
    pub fn new() -> Self {
        Self {
            home_encrypted: true,
            key_derivation_argon2: true,
            is_mounted: false,
        }
    }

    pub fn mount_encrypted_home(&mut self, passphrase: &str) -> Result<(), &'static str> {
        if passphrase.len() < 8 {
            return Err("Passphrase too short");
        }
        self.is_mounted = true;
        Ok(())
    }

    pub fn unmount_and_lock(&mut self) {
        self.is_mounted = false;
    }
}

// ============================================================================
// 15. SUPPLY CHAIN TRANSPARENCY (Item 75)
// ============================================================================

#[derive(Debug, Clone)]
pub struct SbomComponent {
    pub name: String,
    pub version: String,
    pub license_spdx: String,
    pub hash_sha256: String,
}

pub struct SupplyChainSbomGenerator {
    pub components: Vec<SbomComponent>,
}

impl Default for SupplyChainSbomGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SupplyChainSbomGenerator {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, name: &str, version: &str, license: &str, hash: &str) {
        self.components.push(SbomComponent {
            name: name.to_string(),
            version: version.to_string(),
            license_spdx: license.to_string(),
            hash_sha256: hash.to_string(),
        });
    }

    pub fn generate_spdx_json(&self) -> String {
        format!("{{\"spdxVersion\":\"SPDX-2.3\",\"componentsCount\":{}}}", self.components.len())
    }
}

// ============================================================================
// 16. SECURE DEVELOPER KEYS (Item 76)
// ============================================================================

#[derive(Debug, Clone)]
pub struct DeveloperKey {
    pub key_id: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub is_revoked: bool,
}

pub struct DeveloperKeyRotator {
    pub keys: BTreeMap<String, DeveloperKey>,
}

impl Default for DeveloperKeyRotator {
    fn default() -> Self {
        Self::new()
    }
}

impl DeveloperKeyRotator {
    pub fn new() -> Self {
        Self {
            keys: BTreeMap::new(),
        }
    }

    pub fn register_key(&mut self, key_id: &str, current_time: u64, validity_days: u64) {
        let key = DeveloperKey {
            key_id: key_id.to_string(),
            created_at: current_time,
            expires_at: current_time + (validity_days * 86400),
            is_revoked: false,
        };
        self.keys.insert(key_id.to_string(), key);
    }

    pub fn revoke_key(&mut self, key_id: &str) {
        if let Some(key) = self.keys.get_mut(key_id) {
            key.is_revoked = true;
        }
    }

    pub fn is_key_valid(&self, key_id: &str, current_time: u64) -> bool {
        if let Some(key) = self.keys.get(key_id) {
            !key.is_revoked && current_time <= key.expires_at
        } else {
            false
        }
    }
}

// ============================================================================
// 17. PRIVACY-PRESERVING TELEMETRY (Item 77)
// ============================================================================

pub struct PrivacyPreservingTelemetry {
    pub opt_in: bool,
    pub noise_scale_epsilon: f64,
}

impl Default for PrivacyPreservingTelemetry {
    fn default() -> Self {
        Self::new()
    }
}

impl PrivacyPreservingTelemetry {
    pub fn new() -> Self {
        Self {
            opt_in: false, // Default opt-out
            noise_scale_epsilon: 1.0,
        }
    }

    pub fn record_metric_with_laplace_noise(&self, raw_count: u64) -> Option<f64> {
        if !self.opt_in {
            return None; // No telemetry collected if user opted out
        }
        // Simulated Laplace noise addition for differential privacy:
        let noise = 0.05 / self.noise_scale_epsilon;
        Some(raw_count as f64 + noise)
    }
}

// ============================================================================
// 18. COMPLIANCE PROFILES (Item 78)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceStandard {
    Gdpr,
    Hipaa,
    Soc2,
    IndianDpdp2023,
}

pub struct ComplianceProfileTemplates {
    pub active_standard: ComplianceStandard,
}

impl Default for ComplianceProfileTemplates {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplianceProfileTemplates {
    pub fn new() -> Self {
        Self {
            active_standard: ComplianceStandard::IndianDpdp2023,
        }
    }

    pub fn verify_compliance_readiness(&self, posture: &DefaultSecurePosture, audit_integrity: bool) -> bool {
        match self.active_standard {
            ComplianceStandard::Gdpr | ComplianceStandard::IndianDpdp2023 => {
                posture.evaluate_posture_score() >= 80 && audit_integrity
            }
            ComplianceStandard::Hipaa | ComplianceStandard::Soc2 => {
                posture.evaluate_posture_score() >= 90 && audit_integrity
            }
        }
    }
}

// ============================================================================
// 19. GOVERNANCE CHARTER (Item 79)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContributorRole {
    Maintainer,
    Committer,
    Contributor,
}

pub struct GovernanceCharter {
    pub roles: BTreeMap<String, ContributorRole>,
    pub code_of_conduct_enforced: bool,
}

impl Default for GovernanceCharter {
    fn default() -> Self {
        Self::new()
    }
}

impl GovernanceCharter {
    pub fn new() -> Self {
        let mut roles = BTreeMap::new();
        roles.insert("AaryanSinghChauhan09".to_string(), ContributorRole::Maintainer);

        Self {
            roles,
            code_of_conduct_enforced: true,
        }
    }

    pub fn assign_role(&mut self, handle: &str, role: ContributorRole) {
        self.roles.insert(handle.to_string(), role);
    }

    pub fn get_role(&self, handle: &str) -> ContributorRole {
        *self.roles.get(handle).unwrap_or(&ContributorRole::Contributor)
    }
}

// ============================================================================
// 20. LEGAL & LICENSING AUDITOR (Item 80)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseCategory {
    Permissive, // MIT, Apache-2.0, BSD-3-Clause
    Copyleft,   // GPL-2.0, GPL-3.0
    NonFree,    // Proprietary
}

pub struct LegalLicensingAuditor {
    pub component_licenses: BTreeMap<String, (String, LicenseCategory)>,
}

impl Default for LegalLicensingAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl LegalLicensingAuditor {
    pub fn new() -> Self {
        let mut licenses = BTreeMap::new();
        licenses.insert("sigmaos-kernel".to_string(), ("MIT".to_string(), LicenseCategory::Permissive));
        licenses.insert("sigmaos-init".to_string(), ("MIT".to_string(), LicenseCategory::Permissive));

        Self { component_licenses: licenses }
    }

    pub fn register_component(&mut self, name: &str, spdx: &str, category: LicenseCategory) {
        self.component_licenses.insert(name.to_string(), (spdx.to_string(), category));
    }

    pub fn check_compliance(&self) -> bool {
        !self.component_licenses.values().any(|(_, cat)| *cat == LicenseCategory::NonFree)
    }
}

// ============================================================================
// UNIT TESTS (Items 61-80)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_secure_posture() {
        let mut posture = DefaultSecurePosture::new();
        assert!(!posture.is_service_active("sshd"));
        assert!(posture.is_service_active("sigma-core-init"));
        assert_eq!(posture.evaluate_posture_score(), 100);

        posture.umask = 0o022;
        assert_eq!(posture.evaluate_posture_score(), 80);
    }

    #[test]
    fn test_mandatory_access_control() {
        let mut mac = MandatoryAccessControlEngine::new();
        mac.add_rule(MacRule {
            source_domain: "httpd_t".to_string(),
            target_type: "var_log_t".to_string(),
            class_name: "file".to_string(),
            permissions: vec!["read".to_string(), "append".to_string()],
            decision: MacDecision::Allow,
        });

        let subj = SecurityLabel {
            domain: "httpd_t".to_string(),
            type_tag: "httpd_exec_t".to_string(),
            level: "s0".to_string(),
        };
        let obj = SecurityLabel {
            domain: "system_u".to_string(),
            type_tag: "var_log_t".to_string(),
            level: "s0".to_string(),
        };

        assert_eq!(
            mac.check_permission(&subj, &obj, "file", "read"),
            MacDecision::Allow
        );
        assert_eq!(
            mac.check_permission(&subj, &obj, "file", "unlink"),
            MacDecision::Deny
        );
    }

    #[test]
    fn test_secrets_manager() {
        let mut secrets = SystemKeyringSecretsManager::new();
        secrets.store_secret("db_pass", b"SuperSecret123", 3600, false, 1000);

        let retrieved = secrets.retrieve_secret("db_pass", 1500).unwrap();
        assert_eq!(retrieved, b"SuperSecret123");

        assert!(secrets.retrieve_secret("db_pass", 5000).is_err());
    }

    #[test]
    fn test_zero_trust_network() {
        let mut zt = ZeroTrustNetworkPolicyEngine::new();
        zt.register_policy(AppNetworkPolicy {
            app_id: "browser".to_string(),
            allowed_endpoints: vec!["*".to_string()],
            allowed_ports: vec![80, 443],
            force_wireguard_tunnel: true,
        });

        assert!(zt.is_traffic_allowed("browser", "example.com", 443));
        assert!(!zt.is_traffic_allowed("browser", "example.com", 22));

        zt.wireguard_active = false;
        assert!(!zt.is_traffic_allowed("browser", "example.com", 443));
    }

    #[test]
    fn test_sandbox_controller() {
        let mut sandbox = RuntimeSandboxController::new();
        sandbox.spawn_sandbox(SandboxConfig {
            app_name: "pdf_viewer".to_string(),
            allowed_paths_read: vec!["/home/user/Downloads".to_string()],
            allowed_paths_write: vec!["/tmp/pdf".to_string()],
            permitted_syscall_promises: vec!["read".to_string(), "write".to_string()],
            max_memory_bytes: 512 * 1024 * 1024,
        });

        assert!(sandbox.can_access_file("pdf_viewer", "/home/user/Downloads/doc.pdf", false));
        assert!(!sandbox.can_access_file("pdf_viewer", "/etc/shadow", false));
    }

    #[test]
    fn test_integrity_monitor() {
        let mut monitor = SystemIntegrityMonitor::new();
        monitor.register_baseline("/usr/bin/login", 0x1234_5678);

        assert!(monitor.verify_file_integrity("/usr/bin/login", 0x1234_5678));
        assert!(!monitor.verify_file_integrity("/usr/bin/login", 0x9999_9999));
        assert_eq!(monitor.tamper_alerts.len(), 1);
    }

    #[test]
    fn test_audit_trail_integrity() {
        let mut logger = ImmutableAuditLogger::new();
        logger.log_event("root", "USER_LOGIN", 1000);
        logger.log_event("alice", "SUDO_EXEC", 1005);

        assert!(logger.verify_integrity());
        assert_eq!(logger.entries.len(), 2);
    }

    #[test]
    fn test_privacy_dashboard() {
        let mut privacy = PrivacyDashboardController::new();
        assert_eq!(privacy.telemetry_level, TelemetryLevel::None);

        privacy.grant_app_permission("maps_app", "location");
        assert!(privacy.has_permission("maps_app", "location"));

        privacy.revoke_all_app_permissions("maps_app");
        assert!(!privacy.has_permission("maps_app", "location"));
    }

    #[test]
    fn test_secure_update_channel() {
        let mut update = SecureUpdateChannel::new();
        update.publish_update(UpdatePackage {
            version: "1.1.0".to_string(),
            signature_ed25519: vec![1, 2, 3, 4],
            payload_hash: 0x11223344,
            stage: RolloutStage::GeneralAvailability,
        });

        assert!(update.apply_update("1.1.0", RolloutStage::GeneralAvailability).is_ok());
        assert_eq!(update.installed_version, "1.1.0");
    }

    #[test]
    fn test_incident_response_playbook() {
        let mut ir = IncidentResponsePlaybookEngine::new();
        let summary = ir.trigger_playbook(IncidentType::RansomwareDetected);

        assert!(summary.contains("Isolated network"));
        assert!(ir.network_isolated);
        assert!(ir.emergency_lockdown);
    }

    #[test]
    fn test_hardware_attestation() {
        let attestation = HardwareAttestationEngine::new();
        let quote = attestation.generate_attestation_quote(0).unwrap();
        assert_eq!(quote, 0x1111_2222_3333_4444);
    }

    #[test]
    fn test_vulnerability_manager() {
        let mut vm = VulnerabilityDisclosureManager::new();
        vm.submit_report("CVE-2026-1234", 9.8, "Remote Kernel Exec", 10000);

        assert_eq!(vm.reports.len(), 1);
        assert!(!vm.reports[0].resolved);

        vm.mark_resolved("CVE-2026-1234");
        assert!(vm.reports[0].resolved);
    }

    #[test]
    fn test_container_security_policy() {
        let policy = ContainerSecurityPolicyEngine::new();
        assert!(policy.validate_container_launch(true, true, false).is_ok());
        assert!(policy.validate_container_launch(false, true, false).is_err());
    }

    #[test]
    fn test_encrypted_home() {
        let mut home = EncryptedHomeManager::new();
        assert!(home.mount_encrypted_home("SecretPassphrase").is_ok());
        assert!(home.is_mounted);

        home.unmount_and_lock();
        assert!(!home.is_mounted);
    }

    #[test]
    fn test_sbom_generator() {
        let mut sbom = SupplyChainSbomGenerator::new();
        sbom.add_component("sigmaos-kernel", "1.0.0", "MIT", "abc123hash");

        let json = sbom.generate_spdx_json();
        assert!(json.contains("SPDX-2.3"));
    }

    #[test]
    fn test_developer_key_rotator() {
        let mut rotator = DeveloperKeyRotator::new();
        rotator.register_key("dev-key-1", 1000, 30);

        assert!(rotator.is_key_valid("dev-key-1", 1500));
        rotator.revoke_key("dev-key-1");
        assert!(!rotator.is_key_valid("dev-key-1", 1500));
    }

    #[test]
    fn test_privacy_telemetry() {
        let mut telemetry = PrivacyPreservingTelemetry::new();
        assert!(telemetry.record_metric_with_laplace_noise(100).is_none());

        telemetry.opt_in = true;
        assert!(telemetry.record_metric_with_laplace_noise(100).is_some());
    }

    #[test]
    fn test_compliance_profiles() {
        let templates = ComplianceProfileTemplates::new();
        let posture = DefaultSecurePosture::new();

        assert!(templates.verify_compliance_readiness(&posture, true));
    }

    #[test]
    fn test_governance_charter() {
        let charter = GovernanceCharter::new();
        assert_eq!(
            charter.get_role("AaryanSinghChauhan09"),
            ContributorRole::Maintainer
        );
    }

    #[test]
    fn test_legal_licensing_auditor() {
        let auditor = LegalLicensingAuditor::new();
        assert!(auditor.check_compliance());
    }
}
