// SigmaOS Virtualization Module
pub mod oci_pod;
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
    ContainerConfig as VirtContainerConfig, ContainerError as VirtContainerError, ContainerInfo as VirtContainerInfo, ContainerRuntime as VirtContainerRuntime, ContainerRuntimeManager as VirtContainerRuntimeManager,
    ContainerState as VirtContainerState, ContainerStats as VirtContainerStats, DockerRuntime, NetworkMode, PortMapping,
    PortProtocol, ResourceLimits, RestartPolicy, VolumeMapping,
};
pub use namespaces::{Namespace, NamespaceData, NamespaceManager, NamespaceType};
