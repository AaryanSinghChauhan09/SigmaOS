// SigmaOS Virtualization Module
pub mod orchestration;
pub mod vm_manager;

pub use orchestration::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use vm_manager::{
    HypervisorBackend, OsType, QemuBackend, VmConfig, VmError, VmManager, VmResourceUsage,
    VmSnapshot, VmState as ManagerVmState, VirtualBoxBackend,
};
