// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod defensive_audit;
pub mod parrot;
pub mod pledge;
pub mod vulnerability;
pub mod parrot_linux;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use defensive_audit::{
    DefensiveAuditSystem, ForensicBlock, MaliciousSignature, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};
pub use parrot::{
    AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, RoutingMode, SandboxPolicy,
    GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX,
};
pub use parrot_linux::{
    AnonsurfEngine, AnonymityMode, ForensicsAuditTool, RecoveredFile, KaliSniffer,
    SniffedPacket, PentestAssistant, SecureWipeTool, SigmaIDS, IntrusionSeverity, IntrusionAlert,
};
// NemoClaw Security Primitives
pub use nemoclaw::{DefaultDenyNetworkPolicy, NemoClawError, OpenShellAgentSandbox, PrivacyRouter};

// Placeholder stubs for standard types to satisfy lib.rs exports
pub struct CronDaemon;
pub struct CronJob;
pub struct DmesgLog;
pub struct FirewallRule;
pub struct IptablesFirewall;
pub struct KaliError;
pub struct PluggableAuthenticationModule;
pub struct SudoPrivilegeEscalation;
pub struct SwapSpaceManager;
pub struct TmuxMultiplexer;
pub struct TmuxPane;
