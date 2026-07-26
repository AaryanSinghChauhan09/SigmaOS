// SigmaOS Compatibility Module
pub mod constellation;
pub mod cross_platform;
pub mod historic_linux;
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
    ApkLoader as ScosmosApkLoader, BinaryFormat as ScosmosBinaryFormat, BinderCallType,
    CompatibilityError as ScosmosCompatibilityError, MachoLoader as ScosmosMachoLoader,
    PeBinaryLoader as ScosmosPeBinaryLoader, ScosmosManager,
};
pub use standards::{
    FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
};
