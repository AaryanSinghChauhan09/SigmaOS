pub mod lsm;

pub mod audit;
pub mod capability;
pub mod cleaner;
pub mod forensics;
pub mod integrity;
pub mod mac;
pub mod phantom;
pub mod pki;
pub mod pledge;
pub mod vulnerability;
<<<<<<< HEAD
pub mod clipboard;
pub mod intrusion;
pub mod password;
||||||| 0ddf2eac7
=======
pub mod parrot_parity;
>>>>>>> origin/jules-523778995335499834-002b2189

pub use audit::{AuditEvent, AuditLogger, SimpleAuditEvent, SimpleAuditLogger};
pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use clipboard::{
    ClipboardEntry, ClipboardError, ClipboardSecurity, ClipboardType, NoEncryption,
    SecureClipboardManager, SecurityLevel, XorEncryption,
};
pub use integrity::{File, IntegrityError, IntegrityMonitor, IntegrityStatus, SimpleIntegrityMonitor};
pub use intrusion::{
    AnomalyDetection, DetectionResult, DetectionRule, DetectionStrategy, EventType, IdsError,
    IntrusionDetectionSystem, RuleAction, SecurityEvent, Severity, SignatureDetection,
};
pub use mac::{MACEngine, MACPolicy, SecurityContext as MacSecurityContext, SimpleMACEngine};
pub use password::{
    BiometricAuth, BiometricResult, BiometricType, FaceIdAuth, FingerprintAuth, PasswordCategory,
    PasswordEntry, PasswordError, PasswordManager, PasswordManagerResult,
};
pub use pki::{Certificate, PKIError, PKIManager};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    VulnerabilityClass, VulnerabilityReport, SecurityScanner, ExploitPayload,
    PenetrationAssistant,
};
pub use parrot_parity::{
    RoutingMode, AnonSurfShunt, SandboxPolicy, AppSandboxEngine, ForensicStorageFilter,
    GLOBAL_ANONSURF, GLOBAL_SANDBOX, GLOBAL_FORENSIC,
};
