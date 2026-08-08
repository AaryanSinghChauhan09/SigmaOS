// SigmaOS Process Management Module
pub mod kernel_data;
pub mod spawn;

pub use kernel_data::{
    EProcess, EThread, KPrcb, KProcess, KThread, KernelDebuggerShim, Kpcr, MemoryDescriptorList,
    ThreadState, ThreadWaitMode, VasDescriptor,
};
pub use spawn::{
    Process, ProcessError, ProcessGroup, ProcessID, ProcessSpawner, ProcessState, ProcessWaiter,
    SimpleProcess, SimpleProcessGroup, SimpleProcessSpawner, SimpleProcessWaiter,
};
