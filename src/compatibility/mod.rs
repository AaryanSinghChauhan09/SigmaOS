// SigmaOS Compatibility Module
pub mod chimera_linux;
pub mod cross_platform;
pub mod linux_adapter;
pub mod persona;
pub mod abi_translator;
pub mod lattice;
pub mod prism;
pub mod canonical;

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
pub use standards::{
    FhsConventionStatus, LsbProfile, PosixComplianceLevel, StandardsComplianceManager,
};
pub use legacy_adapters::{
    LegacyKernelAdapter, LegacyDriverAdapter, LegacyPackageAdapter, LegacyFSAdapter,
    LegacyProtocolAdapter, LegacySecurityAdapter, LegacyUIAdapter,
};
pub use constellation_mesh::{
    KernelConstellationGrid, ConstellationNode, SyscallAlmanacHub, FileAlmanacHub, NetworkAlmanacHub,
    ProcessAlmanacHub, DriverArchiveGridV2, StorageArchiveGridV2, NetworkArchiveGridV2,
    GraphicsArchiveGridV2, FirmwareGatewayMesh, BIOSGatewayMesh, UEFIGatewayMesh,
    CorebootGatewayMesh, BuildCodexGrid, LegacyCCodexGrid, LegacyCppCodexGrid, LegacyAsmCodexGrid,
    SecurityConstellation, DACConstellation, SELinuxConstellation, ZeroTrustConstellation,
    PeripheralArchiveMesh, FloppyMesh, TapeMesh, CRTMesh, DotMatrixMesh,
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
pub use lattice::{
    LatticeFeature, KernelLattice, SyscallLifecycle, SyscallHistory, SyscallTracker,
};
pub use prism::{
    PrismFacet, KernelPrism, LedgerEntry, SyscallLedgerbook,
};
pub use canonical::{
    SigmaSubiquity, SigmaNetplan, SigmaCloudInit, SigmaMultipass, SigmaCurtin,
};
