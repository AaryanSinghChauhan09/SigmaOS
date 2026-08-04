// SigmaOS Process Spawning & Signal Execution Subsystem Mod

pub mod spawn;

pub use spawn::{
    Process, ProcessError, ProcessGroup, ProcessID, ProcessSpawner, ProcessState, ProcessWaiter,
    SignalHandlerFn, SimpleProcess, SimpleProcessGroup, SimpleProcessSpawner, SimpleProcessWaiter,
    SIGINT, SIGKILL, SIGTERM, SIGUSR1,
};
