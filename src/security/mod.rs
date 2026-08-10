// SigmaOS Security Module
// Capability-based security, pledge, and access control

pub mod capability;
pub mod kali_stack;
pub mod nemoclaw;
pub mod pledge;
pub mod vulnerability;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use kali_stack::{
    CronDaemon, CronJob, DmesgLog, FirewallRule, IptablesFirewall, KaliError,
    PluggableAuthenticationModule, SudoPrivilegeEscalation, SwapSpaceManager, TmuxMultiplexer,
    TmuxPane,
};
pub use nemoclaw::{DefaultDenyNetworkPolicy, NemoClawError, OpenShellAgentSandbox, PrivacyRouter};
pub use pledge::{promises, PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, Severity, Vulnerability, SimpleVulnerability,
    VulnerabilityScanner, SimpleVulnerabilityScanner, ScanSummary, ScanReport,
    SimpleScanReport, CIPipelineIntegration, SimpleCIPipelineIntegration,
};
