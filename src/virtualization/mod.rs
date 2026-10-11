// SigmaOS Virtualization Module
pub mod kvm;
pub mod kvm_vcpu;
pub mod kvm_vmcs;
pub mod oci_pod;
pub mod orchestration;
pub mod rancher;
pub mod virtiofsd;
pub mod firecracker;
pub mod virtualization_pr_gateway;

pub use virtiofsd::{VirtioFsConfig, VirtioFsDaemon, VirtioFsInodeMap};
pub use virtualization_pr_gateway::{
    SovereignVirtualizationPrGatewayEngine, VirtualizationPrCategory, VirtualizationPrStatus,
    VirtualizationPrSubmission, VirtualizationTechnologyFormat,
};


pub use kvm_vcpu::{
    KvmExitCode, KvmMemoryRegion, KvmVcpu, KvmVcpuRegisters, KvmVcpuSregs, VirtioDeviceBackend,
    VirtioDeviceType,
};

pub use orchestration::{
    Container, KubernetesPod, LegacyVirtualizationStrategy, ModernVirtualizationStrategy,
    ResourcePool, VirtualMachine, VirtualizationError, VirtualizationOrchestrator,
    VirtualizationStrategy, VirtualizationStrategyFactory, VirtualizationTech, VmState,
};

pub use oci_pod::{ContainerConfig, OciPod, OciPodManager, PodState};
pub use rancher::{
    DaemonlessContainer, K3osOrchestrator, RancherError, RancherHarvesterVirtualMachineGovernor,
    RancherK3sEmbeddedClusterController, RancherSystemDockerEngine,
};
pub mod vendor_hardware;
pub use vendor_hardware::{
    AmdVmcbExecutionBlock, IntelVmcsExecutionState, MultiVendorVirtualizationEngine,
    NvidiaVgpuMediatedInstance, VirtualizationVendorType,
};
