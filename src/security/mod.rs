// SigmaOS Security Subsystem
pub mod capability;
pub mod kali_stack;
pub mod nemoclaw;
pub mod hardening;
pub mod pledge;
pub mod vulnerability;
pub mod deobfuscation;
pub mod defensive_audit;

pub use capability::{CapabilityGate, CapabilityToken, Permission};
pub use hardening::{
    secure_zeroize, AuditLogEntry, HardenedAuditTrail, IntrusionMonitor, IntrusionSeverity,
};
pub use pledge::{PledgeError, PledgeManager, PledgePromise};
pub use vulnerability::{
    ExploitPayload, PenetrationAssistant, SecurityScanner, VulnerabilityClass, VulnerabilityReport,
};
pub use deobfuscation::{
    ArchInstruction, CpuArch, InstructionType, AbstractValue, DisassemblerCallback, MetasmEmulator, DeobfuscationEngine,
};
pub use defensive_audit::{DefensiveAuditSystem, ForensicBlock, MaliciousSignature};
