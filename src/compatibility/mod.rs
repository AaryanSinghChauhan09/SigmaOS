// SigmaOS Compatibility Module
pub mod cross_platform;
pub mod india_stack_localization;
pub mod scosmos;
pub mod standards;
pub mod proxy;
pub mod oldlinux;

pub use oldlinux::{
    OldLinuxRelease, OldLinuxCompatManager,
};

pub use proxy::{
    KernelPersonality, KernelProxy, SyscallLedgerEntry, LedgerManager, LegacyDriver,
    StorageProxy, NetworkProxy, GraphicsProxy, DriverProxy, FirmwareInterface,
    BIOSProxy, UEFIProxy, CorebootProxy, FirmwareProxy, CompilerBackend, LegacyCProxy,
    LegacyCppProxy, LegacyAsmProxy, BuildProxy, SecurityModel, DACProxy, SELinuxProxy,
    ZeroTrustProxy, SecurityProxy, ObsoleteDevice, FloppyProxy, TapeProxy, CRTProxy,
    DotMatrixProxy, PeripheralProxy,
};

pub use cross_platform::{
    ApplicationBinary, BinaryFormat as CrossPlatformBinaryFormat,
    CompatibilityError as CrossPlatformError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};
pub use scosmos::{
    ApkLoader, BinaryFormat as ScosmosBinaryFormat, BinderCallType,
    CompatibilityError as ScosmosError, MachoLoader, PeBinaryLoader, ScosmosManager,
};
pub use standards::{
    FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
};
