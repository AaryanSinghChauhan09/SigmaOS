// SigmaOS Security Subsystem
pub mod capability;
pub mod parrot_parity;
pub mod pledge;
pub mod vulnerability;
pub mod hardening;
pub mod deobfuscation;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use parrot_parity::{AnonSurfShunt, AppSandboxEngine, ForensicStorageFilter, SandboxPolicy, RoutingMode};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{SecurityScanner, VulnerabilityClass, VulnerabilityReport, ExploitPayload, PenetrationAssistant};
pub use hardening::{
    secure_zeroize, IntrusionSeverity, IntrusionMonitor, AuditLogEntry, HardenedAuditTrail,
};
pub use deobfuscation::{
    ArchInstruction, CpuArch, InstructionType, AbstractValue, DisassemblerCallback, MetasmEmulator, DeobfuscationEngine,
};
