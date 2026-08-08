// SigmaOS Resilience Module
pub mod self_healing;
pub mod automated_fixer;

pub use self_healing::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use automated_fixer::{
    ProblemType as SovereignProblemType, RemediationAction as SovereignRemediationAction,
    FixerStats as SovereignFixerStats, AutomatedFixerDaemon,
};
