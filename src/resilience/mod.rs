// SigmaOS Resilience Module
<<<<<<< HEAD
pub mod backup;
pub mod self_healing;
||||||| 23ef22a4a
pub mod self_healing;
=======
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub mod automated_fixer;
pub mod self_healing;
pub mod backup;

<<<<<<< HEAD
pub use backup::{BackupError, BackupSnapshot, SigmaTimeshift, FsSnapshot, GLOBAL_TIMESHIFT};
||||||| 23ef22a4a
=======
pub use backup::{FsSnapshot, SigmaTimeshift, GLOBAL_TIMESHIFT};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub use self_healing::{
    RecoveryAction, RecoveryEventType, RecoveryRule, ResilienceError, SelfHealingModule,
    SystemSnapshot,
};
pub use backup::{
    BackupError, BackupSnapshot, SigmaTimeshift,
};
