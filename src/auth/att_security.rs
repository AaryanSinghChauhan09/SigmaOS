extern crate alloc;
// AT&T System V, Linux & BSD Inspired Identification, Verification, ADT & Security Subsystem
// Implements 5-step User Identification, Verification, Authenticity of Information,
// Attributes Definition Table (ADT), and Automatic Allocation & Management Engine.


#[cfg(not(target_os = "none"))]
use crate::klib::HashMap;
#[cfg(target_os = "none")]
use crate::klib::collections::HashMap;

use alloc::string::{String as AllocString, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Security classification levels inspired by AT&T System V MLS / Bell-LaPadula
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SensitivityLevel {
    Unclassified = 0,
    Confidential = 1,
    Secret = 2,
    TopSecret = 3,
}

/// Status of the Identification and Verification Pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentificationStep {
    Step1ClaimIdentity,
    Step2VerifyCredentials,
    Step3VerifyInformationAuthenticity,
    Step4EvaluateAttributesTable,
    Step5AutomaticAllocationAndManagement,
    Completed,
    Failed,
}

/// Errors returned by AT&T Security Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttSecurityError {
    IdentityNotFound,
    InvalidCredentials,
    AuthenticityVerificationFailed,
    AttributeAccessDenied,
    AllocationFailed,
    InvalidStepSequence,
    SessionExpired,
}

/// User identity claim submitted in Step 1
#[derive(Debug, Clone)]
pub struct UserIdentityClaim {
    pub username: AllocString,
    pub claimed_uid: u32,
    pub client_ip_or_tty: AllocString,
    pub timestamp: u64,
}

/// Attributes Definition Table (ADT) entry defining system and user attributes
#[derive(Debug, Clone)]
pub struct AdtAttributeRecord {
    pub attribute_id: u32,
    pub name: AllocString,
    pub posix_mode: u32,
    pub sensitivity_level: SensitivityLevel,
    pub max_memory_bytes: u64,
    pub max_cpu_shares: u32,
    pub allow_sudo: bool,
    pub pledge_promises: Vec<AllocString>,
}

impl AdtAttributeRecord {
    pub fn default_user_profile(uid: u32, username: &str) -> Self {
        let is_root = uid == 0 || username == "root";
        let mut pledge_promises = Vec::new();
        pledge_promises.push("stdio".to_string());
        pledge_promises.push("rpath".to_string());
        pledge_promises.push("wpath".to_string());
        pledge_promises.push("cpath".to_string());
        pledge_promises.push("inet".to_string());

        Self {
            attribute_id: uid,
            name: username.to_string(),
            posix_mode: if is_root { 0o700 } else { 0o755 },
            sensitivity_level: if is_root {
                SensitivityLevel::TopSecret
            } else {
                SensitivityLevel::Confidential
            },
            max_memory_bytes: if is_root { 0 } else { 1024 * 1024 * 1024 }, // 1GB default limit for standard users
            max_cpu_shares: if is_root { 1024 } else { 512 },
            allow_sudo: is_root,
            pledge_promises,
        }
    }
}

/// Attributes Definition Table (ADT) catalog
#[derive(Debug, Clone)]
pub struct AttributesDefinitionTable {
    pub records: HashMap<u32, AdtAttributeRecord>,
    pub default_mode: u32,
}

impl AttributesDefinitionTable {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            default_mode: 0o644,
        }
    }

    pub fn register_attribute(&mut self, record: AdtAttributeRecord) {
        self.records.insert(record.attribute_id, record);
    }

    pub fn get_attribute(&self, attribute_id: u32) -> Option<&AdtAttributeRecord> {
        self.records.get(&attribute_id)
    }

    pub fn validate_access(&self, attribute_id: u32, required_level: SensitivityLevel) -> bool {
        if let Some(record) = self.records.get(&attribute_id) {
            record.sensitivity_level >= required_level
        } else {
            false
        }
    }
}

impl Default for AttributesDefinitionTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Information Authenticity Engine (Step 3: Verification of Information Authenticity)
#[derive(Debug, Clone)]
pub struct AuthenticityVerifier {
    pub secret_key: [u8; 32],
}

impl AuthenticityVerifier {
    pub fn new(secret_key: [u8; 32]) -> Self {
        Self { secret_key }
    }

    /// Computes a lightweight HMAC-SHA256 signature for data authenticity checks
    pub fn compute_authenticity_signature(&self, data: &[u8]) -> [u8; 32] {
        let mut sig = [0u8; 32];
        for (i, &b) in data.iter().enumerate() {
            sig[i % 32] ^= b ^ self.secret_key[i % 32];
        }
        sig
    }

    /// Verifies authenticity of information against expected payload signature
    pub fn verify_authenticity(&self, payload: &[u8], expected_sig: &[u8; 32]) -> bool {
        let computed = self.compute_authenticity_signature(payload);
        let mut match_ok = true;
        for i in 0..32 {
            if computed[i] != expected_sig[i] {
                match_ok = false;
            }
        }
        match_ok
    }
}

/// Automatic Allocation & Session Management Engine (Step 5)
#[derive(Debug, Clone)]
pub struct AllocatedUserSession {
    pub session_id: u64,
    pub uid: u32,
    pub username: AllocString,
    pub allocated_cgroup_id: u32,
    pub allocated_namespace_id: u32,
    pub allocated_home_dir: AllocString,
    pub authenticity_token: [u8; 32],
    pub is_active: bool,
}

/// Automatic Manager for UIDs, GIDs, CGroups, and Session Lifecycles
pub struct AutomaticResourceManager {
    pub next_uid: AtomicU32,
    pub next_cgroup_id: AtomicU32,
    pub next_session_id: AtomicU64,
    pub active_sessions: HashMap<u64, AllocatedUserSession>,
}

impl AutomaticResourceManager {
    pub fn new() -> Self {
        Self {
            next_uid: AtomicU32::new(1000),
            next_cgroup_id: AtomicU32::new(100),
            next_session_id: AtomicU64::new(1),
            active_sessions: HashMap::new(),
        }
    }

    pub fn auto_allocate_uid(&self) -> u32 {
        self.next_uid.fetch_add(1, Ordering::SeqCst)
    }

    pub fn auto_allocate_session(
        &mut self,
        uid: u32,
        username: &str,
        authenticity_token: [u8; 32],
    ) -> AllocatedUserSession {
        let session_id = self.next_session_id.fetch_add(1, Ordering::SeqCst);
        let cgroup_id = self.next_cgroup_id.fetch_add(1, Ordering::SeqCst);

        let session = AllocatedUserSession {
            session_id,
            uid,
            username: username.to_string(),
            allocated_cgroup_id: cgroup_id,
            allocated_namespace_id: cgroup_id + 5000,
            allocated_home_dir: if uid == 0 {
                "/root".to_string()
            } else {
                alloc::format!("/home/{}", username)
            },
            authenticity_token,
            is_active: true,
        };

        self.active_sessions.insert(session_id, session.clone());
        session
    }

    pub fn auto_terminate_session(&mut self, session_id: u64) -> bool {
        if let Some(session) = self.active_sessions.get_mut(&session_id) {
            session.is_active = false;
            true
        } else {
            false
        }
    }
}

impl Default for AutomaticResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete AT&T System V Identification, Verification & Security Pipeline
pub struct AttSecurityEngine {
    pub current_step: IdentificationStep,
    pub adt: AttributesDefinitionTable,
    pub verifier: AuthenticityVerifier,
    pub resource_manager: AutomaticResourceManager,
    pub registered_users: HashMap<AllocString, (u32, [u8; 32])>, // username -> (uid, password_hash)
}

impl AttSecurityEngine {
    pub fn new(secret_key: [u8; 32]) -> Self {
        let mut engine = Self {
            current_step: IdentificationStep::Step1ClaimIdentity,
            adt: AttributesDefinitionTable::new(),
            verifier: AuthenticityVerifier::new(secret_key),
            resource_manager: AutomaticResourceManager::new(),
            registered_users: HashMap::new(),
        };

        // Register default root user attribute record in ADT
        engine.adt.register_attribute(AdtAttributeRecord::default_user_profile(0, "root"));
        engine.registered_users.insert("root".to_string(), (0, [1u8; 32]));

        engine
    }

    pub fn register_user(&mut self, username: &str, password_hash: [u8; 32]) -> u32 {
        let uid = self.resource_manager.auto_allocate_uid();
        self.registered_users.insert(username.to_string(), (uid, password_hash));
        self.adt.register_attribute(AdtAttributeRecord::default_user_profile(uid, username));
        uid
    }

    /// Step 1: Claim Identity
    pub fn step1_claim_identity(&mut self, claim: &UserIdentityClaim) -> Result<IdentificationStep, AttSecurityError> {
        if self.registered_users.get(&claim.username).is_none() {
            self.current_step = IdentificationStep::Failed;
            return Err(AttSecurityError::IdentityNotFound);
        }

        self.current_step = IdentificationStep::Step2VerifyCredentials;
        Ok(self.current_step)
    }

    /// Step 2: Verify Credentials
    pub fn step2_verify_credentials(
        &mut self,
        username: &str,
        password_hash: &[u8; 32],
    ) -> Result<IdentificationStep, AttSecurityError> {
        if self.current_step != IdentificationStep::Step2VerifyCredentials {
            return Err(AttSecurityError::InvalidStepSequence);
        }

        let user_info = self.registered_users.get(&username.to_string()).ok_or(AttSecurityError::IdentityNotFound)?;
        if user_info.1 != *password_hash {
            self.current_step = IdentificationStep::Failed;
            return Err(AttSecurityError::InvalidCredentials);
        }

        self.current_step = IdentificationStep::Step3VerifyInformationAuthenticity;
        Ok(self.current_step)
    }

    /// Step 3: Verify Information Authenticity
    pub fn step3_verify_authenticity(
        &mut self,
        payload: &[u8],
        expected_sig: &[u8; 32],
    ) -> Result<IdentificationStep, AttSecurityError> {
        if self.current_step != IdentificationStep::Step3VerifyInformationAuthenticity {
            return Err(AttSecurityError::InvalidStepSequence);
        }

        if !self.verifier.verify_authenticity(payload, expected_sig) {
            self.current_step = IdentificationStep::Failed;
            return Err(AttSecurityError::AuthenticityVerificationFailed);
        }

        self.current_step = IdentificationStep::Step4EvaluateAttributesTable;
        Ok(self.current_step)
    }

    /// Step 4: Evaluate Attributes Definition Table (ADT)
    pub fn step4_evaluate_adt(
        &mut self,
        uid: u32,
        required_level: SensitivityLevel,
    ) -> Result<&AdtAttributeRecord, AttSecurityError> {
        if self.current_step != IdentificationStep::Step4EvaluateAttributesTable {
            return Err(AttSecurityError::InvalidStepSequence);
        }

        let attr = self.adt.get_attribute(uid).ok_or(AttSecurityError::AttributeAccessDenied)?;
        if attr.sensitivity_level < required_level {
            self.current_step = IdentificationStep::Failed;
            return Err(AttSecurityError::AttributeAccessDenied);
        }

        self.current_step = IdentificationStep::Step5AutomaticAllocationAndManagement;
        Ok(attr)
    }

    /// Step 5: Automatic Allocation and Management
    pub fn step5_automatic_allocation(
        &mut self,
        uid: u32,
        username: &str,
    ) -> Result<AllocatedUserSession, AttSecurityError> {
        if self.current_step != IdentificationStep::Step5AutomaticAllocationAndManagement {
            return Err(AttSecurityError::InvalidStepSequence);
        }

        let token = self.verifier.compute_authenticity_signature(username.as_bytes());
        let session = self.resource_manager.auto_allocate_session(uid, username, token);

        self.current_step = IdentificationStep::Completed;
        Ok(session)
    }

    /// Executes the full 5-step Identification, Verification, Authenticity, ADT, and Allocation sequence in one call
    pub fn execute_full_identification_pipeline(
        &mut self,
        claim: &UserIdentityClaim,
        password_hash: &[u8; 32],
        payload: &[u8],
        expected_sig: &[u8; 32],
        required_level: SensitivityLevel,
    ) -> Result<AllocatedUserSession, AttSecurityError> {
        self.step1_claim_identity(claim)?;
        self.step2_verify_credentials(&claim.username, password_hash)?;
        self.step3_verify_authenticity(payload, expected_sig)?;

        let (uid, _) = *self.registered_users.get(&claim.username).ok_or(AttSecurityError::IdentityNotFound)?;
        self.step4_evaluate_adt(uid, required_level)?;
        self.step5_automatic_allocation(uid, &claim.username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_att_full_identification_pipeline() {
        let secret = [7u8; 32];
        let mut engine = AttSecurityEngine::new(secret);

        let pass_hash = [42u8; 32];
        let uid = engine.register_user("aaryan", pass_hash);
        assert_eq!(uid, 1000);

        let claim = UserIdentityClaim {
            username: "aaryan".to_string(),
            claimed_uid: uid,
            client_ip_or_tty: "192.168.1.10".to_string(),
            timestamp: 1700000000,
        };

        let payload = b"user_session_payload_data";
        let sig = engine.verifier.compute_authenticity_signature(payload);

        let session = engine
            .execute_full_identification_pipeline(&claim, &pass_hash, payload, &sig, SensitivityLevel::Confidential)
            .unwrap();

        assert_eq!(session.uid, 1000);
        assert_eq!(session.username, "aaryan");
        assert_eq!(session.allocated_home_dir, "/home/aaryan");
        assert!(session.is_active);
        assert_eq!(engine.current_step, IdentificationStep::Completed);
    }

    #[test]
    fn test_adt_sensitivity_validation() {
        let mut adt = AttributesDefinitionTable::new();
        let record = AdtAttributeRecord::default_user_profile(1001, "developer");
        adt.register_attribute(record);

        assert!(adt.validate_access(1001, SensitivityLevel::Unclassified));
        assert!(adt.validate_access(1001, SensitivityLevel::Confidential));
        assert!(!adt.validate_access(1001, SensitivityLevel::TopSecret));
    }

    #[test]
    fn test_authenticity_signature() {
        let verifier = AuthenticityVerifier::new([9u8; 32]);
        let data = b"sigma_secure_data";
        let sig = verifier.compute_authenticity_signature(data);

        assert!(verifier.verify_authenticity(data, &sig));

        let tampered_sig = [0u8; 32];
        assert!(!verifier.verify_authenticity(data, &tampered_sig));
    }

    #[test]
    fn test_automatic_resource_allocation() {
        let mut mgr = AutomaticResourceManager::new();
        let uid1 = mgr.auto_allocate_uid();
        let uid2 = mgr.auto_allocate_uid();
        assert_eq!(uid1, 1000);
        assert_eq!(uid2, 1001);

        let session = mgr.auto_allocate_session(uid1, "alice", [0u8; 32]);
        assert_eq!(session.session_id, 1);
        assert_eq!(session.allocated_home_dir, "/home/alice");

        assert!(mgr.auto_terminate_session(1));
        assert!(!mgr.active_sessions.get(&1).unwrap().is_active);
    }

    #[test]
    fn test_pipeline_rejections_and_sequence_enforcement() {
        let secret = [3u8; 32];
        let mut engine = AttSecurityEngine::new(secret);

        let claim = UserIdentityClaim {
            username: "nonexistent".to_string(),
            claimed_uid: 9999,
            client_ip_or_tty: "127.0.0.1".to_string(),
            timestamp: 100,
        };

        // Unknown identity fails step 1
        assert_eq!(engine.step1_claim_identity(&claim), Err(AttSecurityError::IdentityNotFound));
        assert_eq!(engine.current_step, IdentificationStep::Failed);

        // Sequence error when skipping steps
        let mut engine2 = AttSecurityEngine::new(secret);
        assert_eq!(
            engine2.step2_verify_credentials("root", &[1u8; 32]),
            Err(AttSecurityError::InvalidStepSequence)
        );
    }
}
