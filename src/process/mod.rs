<<<<<<< HEAD
// SigmaOS Process Management Module
pub mod spawn;
pub mod kernel_data;

pub use spawn::{
    Process, ProcessID, ProcessState, ProcessError, SimpleProcess, ProcessSpawner, SimpleProcessSpawner, ProcessWaiter, SimpleProcessWaiter, ProcessGroup, SimpleProcessGroup,
};
pub use kernel_data::{
    ThreadWaitMode, ThreadState, MemoryDescriptorList, KThread, EThread, VasDescriptor, KProcess, EProcess, KPrcb, Kpcr, KernelDebuggerShim,
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
