// SigmaOS Virtualization Module
pub mod advanced_virt;
pub mod cgroups;
pub mod container;
pub mod deterministic;
pub mod kvm_vcpu;
pub mod namespaces;
pub mod oci_pod;
pub mod orchestration;
pub mod rancher;
pub mod vm_manager;
pub mod vendor_hardware;

pub use kvm_vcpu::{
    FirecrackerMicroVmSupervisor, FreeBsdBhyveVirtioEngine, KvmExitCode, KvmMemoryRegion,
    KvmVcpu, KvmVcpuRegisters, KvmVcpuSregs, MicroVmConfig, OpenBsdVmmMicroHypervisorEngine,
    VirtioDeviceBackend, VirtioDeviceType, VmmGuestMode,
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
pub use vendor_hardware::{
    AmdVmcbExecutionBlock, IntelVmcsExecutionState, MultiVendorVirtualizationEngine,
    NvidiaVgpuMediatedInstance, VirtualizationVendorType,
};
