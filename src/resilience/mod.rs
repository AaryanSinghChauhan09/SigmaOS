// SigmaOS Resilience Module
<<<<<<< HEAD
pub mod backup;
=======
pub mod automated_fixer;
>>>>>>> origin/jules-15532892492441614180-73ce6847
pub mod self_healing;

pub use backup::{BackupError, BackupSnapshot, SigmaTimeshift};
pub use self_healing::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
