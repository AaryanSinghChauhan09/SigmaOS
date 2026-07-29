// SigmaOS Virtualization Module
pub mod container;
pub mod orchestration;
<<<<<<< HEAD
pub mod vm_manager;
=======
#[cfg(any())]
pub mod container;
#[cfg(any())]
pub mod namespaces;
>>>>>>> origin/jules-6565657164915217370-c04e8c01

pub use container::{
    ContainerConfig, ContainerError, ContainerInfo, ContainerRuntime, ContainerRuntimeManager,
    ContainerState, ContainerStats, DockerRuntime, NetworkMode, PodmanRuntime, PortMapping,
    PortProtocol, ResourceLimits, RestartPolicy, VolumeMapping,
};
pub use orchestration::{
    Container, KubernetesPod, LegacyVirtualizationStrategy, ModernVirtualizationStrategy,
    ResourcePool, VirtualMachine, VirtualizationError, VirtualizationOrchestrator,
    VirtualizationStrategy, VirtualizationStrategyFactory, VirtualizationTech, VmState,
};
pub use vm_manager::{
    HypervisorBackend, OsType, QemuBackend, VirtualBoxBackend, VmConfig, VmError, VmManager,
    VmResourceUsage, VmSnapshot, VmState as ManagerVmState,
};
