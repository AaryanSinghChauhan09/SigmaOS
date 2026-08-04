pub mod spawn;
pub mod linux_proc;

pub use spawn::{Process, ProcessID, ProcessState, ProcessError, SimpleProcess, ProcessSpawner, SimpleProcessSpawner, ProcessWaiter, SimpleProcessWaiter, ProcessGroup, SimpleProcessGroup};
pub use linux_proc::{NiceValue, CGroup, PidNamespace, LinuxProcessEntry, LinuxProcessState, LinuxSignal, ProcFileSystem};
||||||| 43be3a7e8
// SigmaOS Process Spawning & Signal Execution Subsystem Mod

pub mod spawn;

pub use spawn::{
    Process, ProcessError, ProcessGroup, ProcessID, ProcessSpawner, ProcessState, ProcessWaiter,
    SignalHandlerFn, SimpleProcess, SimpleProcessGroup, SimpleProcessSpawner, SimpleProcessWaiter,
    SIGINT, SIGKILL, SIGTERM, SIGUSR1,
};
