// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod linux_adapter;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use linux_adapter::{
    LinuxKernelVersion, LegacyKernelAdapter, LegacyPackageAdapter, LegacySecurityAdapter, LegacyUIAdapter,
};
