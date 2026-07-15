// SigmaOS Resilience Module
pub mod self_healing;

pub use self_healing::{SelfHealingModule, RecoveryRule, RecoveryAction, SystemSnapshot, RecoveryEventType, ResilienceError};
