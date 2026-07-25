// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod linux_adapter;
pub mod persona;
pub mod abi_translator;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use linux_adapter::{
    LinuxKernelVersion, LegacyKernelAdapter, LegacyPackageAdapter, LegacySecurityAdapter, LegacyUIAdapter,
};
pub use persona::{
    PersonaVersion, KernelPersonaContainer, SyscallCategory, SyscallNode, SyscallGraph,
};
pub use abi_translator::{
    CpuArchitecture, ABITranslator,
};
