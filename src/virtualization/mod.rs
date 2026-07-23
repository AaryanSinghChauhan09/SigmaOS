// SigmaOS Virtualization Module
pub mod container;
pub mod orchestration;
pub mod vm_manager;

pub use container::{
    ContainerConfig, ContainerError, ContainerInfo, ContainerRuntime, ContainerRuntimeManager,
    ContainerState, ContainerStats, DockerRuntime, NetworkMode, PodmanRuntime, PortMapping,
    PortProtocol, ResourceLimits, RestartPolicy, VolumeMapping,
};
pub use orchestration::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
};
pub use vm_manager::{
    HypervisorBackend, OsType, QemuBackend, VirtualBoxBackend, VmConfig, VmError, VmManager,
    VmResourceUsage, VmSnapshot, VmState as ManagerVmState,
};
