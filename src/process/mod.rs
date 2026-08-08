<<<<<<< HEAD
<<<<<<< HEAD
// SigmaOS Process Management Module
||||||| 23ef22a4a
// SigmaOS Process Management Module
=======
// SigmaOS Process Spawning & Signal Execution Subsystem Mod

>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub mod spawn;

pub use spawn::{
    Process, ProcessError, ProcessGroup, ProcessID, ProcessSpawner, ProcessState, ProcessWaiter,
    SignalHandlerFn, SimpleProcess, SimpleProcessGroup, SimpleProcessSpawner, SimpleProcessWaiter,
    SIGINT, SIGKILL, SIGTERM, SIGUSR1,
};
||||||| 43be3a7e8
=======
pub mod spawn;
pub mod linux_proc;
pub mod linux_sysfs;

pub use spawn::{Process, ProcessID, ProcessState, ProcessError, SimpleProcess, ProcessSpawner, SimpleProcessSpawner, ProcessWaiter, SimpleProcessWaiter, ProcessGroup, SimpleProcessGroup};
pub use linux_proc::{NiceValue, CGroup, PidNamespace, LinuxProcessEntry, LinuxProcessState, LinuxSignal, ProcFileSystem};
pub use linux_sysfs::{SysfsAttribute, LoopDevice, SysfsRegistry};
>>>>>>> origin/fix/mem-leak-custom-vec-drop-7188808108065826003
