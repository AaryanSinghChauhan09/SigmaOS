// SigmaOS Virtualization Module
pub mod orchestration;
pub mod container;
pub mod namespaces;

pub use orchestration::{
    Container, KubernetesPod, LegacyVirtualizationStrategy, ModernVirtualizationStrategy,
    ResourcePool, VirtualMachine, VirtualizationError, VirtualizationOrchestrator,
    VirtualizationStrategy, VirtualizationStrategyFactory, VirtualizationTech, VmState,
};

pub use oci_pod::{
    ContainerConfig, OciPod, OciPodManager, PodState,
};
pub use container::{
    ContainerConfig as VirtContainerConfig, ContainerError, ContainerInfo, ContainerRuntime, ContainerRuntimeManager,
    ContainerState as VirtContainerState, ContainerStats, DockerRuntime, NetworkMode, PodmanRuntime, PortMapping,
    PortProtocol, ResourceLimits, RestartPolicy, VolumeMapping,
};
pub use namespaces::{Namespace, NamespaceData, NamespaceManager, NamespaceType};
