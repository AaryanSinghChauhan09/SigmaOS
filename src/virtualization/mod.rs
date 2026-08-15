// SigmaOS Virtualization Module
pub mod deterministic;
pub mod orchestration;

pub use deterministic::{
    DeterministicError, DeterministicHypervisor, DeterministicVirtualMachine, VirtualCpuContext,
    VmExecutionSnapshot,
};
pub use orchestration::{
    Container, KubernetesPod, LegacyVirtualizationStrategy, ModernVirtualizationStrategy,
    ResourcePool, VirtualMachine, VirtualizationError, VirtualizationOrchestrator,
    VirtualizationStrategy, VirtualizationStrategyFactory, VirtualizationTech, VmState,
};

pub use container::{
    ContainerConfig as VirtContainerConfig, ContainerError, ContainerInfo, ContainerRuntime,
    ContainerRuntimeManager, ContainerState as VirtContainerState, ContainerStats, DockerRuntime,
    NetworkMode, PodmanRuntime, PortMapping, PortProtocol, ResourceLimits, RestartPolicy,
    VolumeMapping,
};
pub use namespaces::{Namespace, NamespaceData, NamespaceManager, NamespaceType};
