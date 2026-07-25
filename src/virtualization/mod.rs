// SigmaOS Virtualization Module
pub mod cgroups;
pub mod container;
pub mod namespaces;
pub mod orchestration;
pub mod oci_pod;

pub use cgroups::{
    Cgroup, CgroupController, CgroupManager, CgroupState, CgroupSubsystem, CpuController,
    MemoryController, PidController,
};
pub use container::{
    ContainerConfig, ContainerError, ContainerInfo, ContainerRuntime, ContainerRuntimeManager,
    ContainerState, ContainerStats, DockerRuntime, NetworkMode, PodmanRuntime, PortMapping,
    PortProtocol, ResourceLimits, RestartPolicy, VolumeMapping,
};
pub use namespaces::{Namespace, NamespaceData, NamespaceManager, NamespaceType};
pub use orchestration::{
    Container, KubernetesPod, LegacyVirtualizationStrategy, ModernVirtualizationStrategy,
    ResourcePool, VirtualMachine, VirtualizationError, VirtualizationOrchestrator,
    VirtualizationStrategy, VirtualizationStrategyFactory, VirtualizationTech, VmState,
};

pub use oci_pod::{
    ContainerConfig, OciPod, OciPodManager, PodState,
};

pub use oci_pod::{ContainerConfig, OciPod, OciPodManager, PodState};
