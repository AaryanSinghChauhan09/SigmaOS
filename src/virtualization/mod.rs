// SigmaOS Virtualization Module
pub mod kvm_vcpu;
pub mod oci_pod;
pub mod orchestration;
pub mod rancher;

pub use kvm_vcpu::{
    KvmExitCode, KvmMemoryRegion, KvmVcpu, KvmVcpuRegisters, KvmVcpuSregs,
    VirtioDeviceBackend, VirtioDeviceType,
};

pub use kvm_vcpu::{
    KvmExitCode, KvmMemoryRegion, KvmVcpu, KvmVcpuRegisters, KvmVcpuSregs,
    VirtioDeviceBackend, VirtioDeviceType,
};

pub use orchestration::{
    Container, KubernetesPod, LegacyVirtualizationStrategy, ModernVirtualizationStrategy,
    ResourcePool, VirtualMachine, VirtualizationError, VirtualizationOrchestrator,
    VirtualizationStrategy, VirtualizationStrategyFactory, VirtualizationTech, VmState,
};

pub use oci_pod::{ContainerConfig, OciPod, OciPodManager, PodState};
pub use rancher::{
    DaemonlessContainer, K3osOrchestrator, RancherError,
    RancherHarvesterVirtualMachineGovernor, RancherK3sEmbeddedClusterController,
    RancherSystemDockerEngine,
};
