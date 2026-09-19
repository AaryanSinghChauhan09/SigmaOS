// SigmaOS Compatibility Module

use core::sync::atomic::{AtomicU64, Ordering};

pub mod abi_extended;
pub mod abi_translator;
pub mod absorb_tools;
pub mod advanced_ecosystem;
pub mod alpine_linux;
pub mod android_chromeos;
pub mod antix;
pub mod apache_ossie;
pub mod arch;
pub mod arch_aur;
pub mod arch_linux;
pub mod artix_linux;
pub mod atomic_distribution;
pub mod bodhi_moksha;
pub mod bsd;
pub mod cachy_os;
pub mod canonical;
pub mod chakra;
pub mod chimera_linux;
pub mod clear_linux;
pub mod community_foundation;
pub mod constellation;
pub mod constellation_mesh;
pub mod cross_platform;
pub mod cross_platform_kernel;
pub mod debian;
pub use debian::*;
pub mod distro_bridge;
pub mod dragonfly_bsd;
pub mod elf_execution;
pub mod endeavour;
pub mod federation;
pub mod fedora;
pub use fedora::*;
pub mod fedora_missing_components;
pub mod fedora_domination;

pub use fedora_missing_components::{
    Dnf5Advisory, FedoraAnacondaKickstartEngine, FedoraDnf5PackageEngine,
    FedoraMockChrootBuilder, FedoraSssdFreeIpaEngine, KickstartPartition, MockChrootConfig,
};
pub use fedora_domination::*;
pub use fedora_missing_components::{
    BodhiStatus, BodhiUpdateRecord, BodhiUpdateType, CoprRepository, CryptoPolicyProfile,
    FedoraBodhiUpdateEngine, FedoraContainerStackEngine, FedoraCoprBuildGatewayEngine,
    FedoraCryptoPoliciesEngine, FedoraGreenwaveDecisionEngine, FedoraKojiBuildSystemEngine,
    FedoraMockChrootBuilderEngine, FedoraOpenQaTestGatewayEngine, FedoraPagureForgeEngine,
    FedoraRpmostreeAtomicEngine, FedoraWaiverDbEngine, GreenwaveDecisionStatus,
    GreenwavePolicyRequirement, KojiBuildTask, KojiTaskState, MockChrootProfile, OciContainerImage,
    OpenQaJobStatus, OpenQaTestJob, PagurePullRequest, RpmOstreeDeployment,
    SovereignFedoraEcosystemSuite, WaiverRecord,
};
// pub mod freebsd_jails;
pub mod freedos;
pub mod gap_closure;
pub mod garuda_zen;
pub mod gentoo;
pub use gentoo::*;
pub mod gentoo_useflags;
pub mod historic_linux;
pub mod hopper_lab;
pub mod india_professional_tools;
pub mod india_stack;
pub mod india_stack_localization;
pub mod itsfoss_inspiration_suite;

pub use itsfoss_inspiration_suite::{
    ItsFossLocalSendTransferEngine, ItsFossStacerOptimizerEngine, ItsFossTimeshiftBackupEngine,
    ItsFossVentoyMultiBootUsbEngine, LocalSendPeer, SnapshotMode, StacerCleanCategory,
    TimeshiftRestorePoint, VentoyIsoEntry,
};
pub mod innovations;
pub mod installer;
pub mod interim;
pub mod jails;
pub mod jehanne;
pub mod kimi_code;
pub mod lattice;
pub mod lattice_grid;
pub mod legacy_adapters;
pub mod linux_adapter;
pub mod linux_bsd_ecosystem_bridge;
pub mod linux_compat;

pub use linux_bsd_ecosystem_bridge::{
    EcosystemAbi, LinuxBsdEcosystemBridge, MultiFormatPackageBridge, NativeSigmaPackageManifest,
    PackageSourceFormat, PosixSharedMemoryIpcBridge, SyscallTranslationResult,
    UniversalSyscallAbiShim,
};
pub mod linux_distro_parity;
pub use linux_distro_parity::{
    FstabEntry, LinuxCoreDumpFilterEngine, LinuxFstabEngine, LinuxLdSoLoader,
    LinuxModulesLoadEngine, LinuxPamAuthenticationEngine, LinuxRunlevel, LinuxRunlevelGovernor,
    LinuxSwapfileManagerEngine, LinuxSysctlGovernor, LinuxSystemdTmpfilesEngine,
    LinuxUdevRulesEngine, LsbReleaseGovernor, LsbReleaseInfo, SharedLibrary, SwapDevice,
    SwapKind, TmpfileItemType, TmpfileRule, UdevRule,
};
pub mod linux_init;
pub mod linux_network;
pub mod linux_security;
pub mod linux_standards;
pub mod linuxulator;
pub mod localsend;
pub mod lubuntu;
pub mod macos_darwin;
pub mod mate_betsy;
pub mod mesh_hub;
pub mod mint;
pub mod mint_ecosystem;
pub mod mint_linux;

pub use mint_ecosystem::{
    CaptainMintManager, CinnamonSpiceDesklet, CinnamonSpiceTheme, CinnamonSpicesEngine,
    LinuxMintEcosystemHub, MintRepoMirror, MintUpgradeSourcesEngine, RecentDocument,
    SlickGreeterXappPortal, WarpinatorLanShare, WarpinatorPeer, XappAppsSuite,
};
pub use mint::{
    LinuxMintIntegrationEngine, MintInstallManager, SoftwarePackage,
    UpdateLevel, UpdatePackage, CinnamonDesktopManager, CinnamonPanel, CinnamonPanelPosition,
    XAppPreferences, MintSystemConfig,
};
pub mod mobile_desktop_parity;
pub mod nixos;
// pub mod nixos_reproducible;
pub mod oldlinux;
pub mod open_source_dominance;
pub mod open_source_tier1;
pub mod opensuse_slackware;
pub use opensuse_slackware::*;
pub mod overtake;
pub mod penetration_assistant;
pub mod persona;
pub mod personality;
pub mod pop_os;
pub mod prism;
pub mod proxy;
pub mod reactos;
pub mod register_set;
pub mod relay_nexus;
pub mod scosmos;
pub mod sigmawin;
pub use reactos::RegistryHive;
pub use sigmawin::{
    D3dToVulkanTranslator, D3dVersion, NtNativeSyscallTranslator, PeHeaderInfo,
    User32MessageQueue, Win32Message, Win32PeExecutableParser, WinSockAdapter,
    WindowsPowerShellShimEngine,
};
pub mod solid_kernel;
pub mod sovereign_suite;
pub mod superiority;
pub mod tiny_core;
pub mod wsl;
pub mod zorin;

// ─── POSIX Compatibility Tiers ─────────────────────────────────────────────

/// POSIX compatibility tier level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PosixTier {
    /// Full POSIX.1-2008 compliance
    Full,
    /// POSIX.1-2001 compliance
    Standard,
    /// Basic POSIX (subset)
    Basic,
    /// Extended with Linux/BSD extensions
    Extended,
}

/// POSIX syscall compatibility translation
#[derive(Debug, Clone)]
pub struct PosixSyscallTranslation {
    pub original_syscall: u32,
    pub translated_syscall: u32,
    pub capability_required: bool,
}

/// POSIX compatibility layer
#[derive(Debug)]
pub struct PosixCompatibilityLayer {
    pub tier: PosixTier,
    pub translations: Vec<PosixSyscallTranslation>,
    pub syscalls_translated: AtomicU64,
}

impl PosixCompatibilityLayer {
    pub fn new(tier: PosixTier) -> Self {
        PosixCompatibilityLayer {
            tier,
            translations: Vec::new(),
            syscalls_translated: AtomicU64::new(0),
        }
    }

    pub fn add_translation(&mut self, original: u32, translated: u32, capability_required: bool) {
        self.translations.push(PosixSyscallTranslation {
            original_syscall: original,
            translated_syscall: translated,
            capability_required,
        });
    }

    pub fn translate_syscall(&self, syscall: u32) -> Option<u32> {
        for translation in &self.translations {
            if translation.original_syscall == syscall {
                self.syscalls_translated.fetch_add(1, Ordering::SeqCst);
                return Some(translation.translated_syscall);
            }
        }
        None
    }

    pub fn get_translation_count(&self) -> u64 {
        self.syscalls_translated.load(Ordering::SeqCst)
    }
}

impl Default for PosixCompatibilityLayer {
    fn default() -> Self {
        Self::new(PosixTier::Standard)
    }
}

// ─── FHS Overlay Management ─────────────────────────────────────────────────

/// FHS (Filesystem Hierarchy Standard) path
#[derive(Debug, Clone)]
pub struct FhsPath {
    pub original_path: String,
    pub overlay_path: String,
    pub capability_required: bool,
}

/// FHS overlay mount
#[derive(Debug)]
pub struct FhsOverlayMount {
    pub mounts: Vec<FhsPath>,
    pub mount_count: AtomicU64,
}

impl FhsOverlayMount {
    pub fn new() -> Self {
        FhsOverlayMount {
            mounts: Vec::new(),
            mount_count: AtomicU64::new(0),
        }
    }

    pub fn add_mount(&mut self, original: &str, overlay: &str, capability_required: bool) {
        self.mounts.push(FhsPath {
            original_path: String::from(original),
            overlay_path: String::from(overlay),
            capability_required,
        });
        self.mount_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn resolve_path(&self, path: &str) -> Option<String> {
        for mount in &self.mounts {
            if path.starts_with(&mount.original_path) {
                let relative = &path[mount.original_path.len()..];
                return Some(format!("{}{}", mount.overlay_path, relative));
            }
        }
        None
    }

    pub fn get_mount_count(&self) -> u64 {
        self.mount_count.load(Ordering::SeqCst)
    }

    /// Add standard FHS paths (/bin, /etc, /usr/lib, /var)
    pub fn add_standard_fhs(&mut self) {
        self.add_mount("/bin", "/sigma/bin", true);
        self.add_mount("/etc", "/sigma/etc", true);
        self.add_mount("/usr/lib", "/sigma/lib", true);
        self.add_mount("/var", "/sigma/var", true);
        self.add_mount("/usr/share", "/sigma/share", true);
        self.add_mount("/usr/local", "/sigma/local", true);
    }
}

impl Default for FhsOverlayMount {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod posix_tests {
    use super::*;

    #[test]
    fn test_posix_compatibility_layer() {
        let mut layer = PosixCompatibilityLayer::new(PosixTier::Full);
        layer.add_translation(1, 100, true);
        layer.add_translation(2, 200, false);

        assert_eq!(layer.translate_syscall(1), Some(100));
        assert_eq!(layer.translate_syscall(2), Some(200));
        assert_eq!(layer.translate_syscall(3), None);
        assert_eq!(layer.get_translation_count(), 2);
    }

    #[test]
    fn test_fhs_overlay_mount() {
        let mut fhs = FhsOverlayMount::new();
        fhs.add_standard_fhs();

        assert_eq!(fhs.resolve_path("/bin/bash"), Some("/sigma/bin/bash".to_string()));
        assert_eq!(fhs.resolve_path("/etc/passwd"), Some("/sigma/etc/passwd".to_string()));
        assert_eq!(fhs.resolve_path("/usr/lib/libc.so"), Some("/sigma/lib/libc.so".to_string()));
        assert_eq!(fhs.get_mount_count(), 6);
    }
}

pub use wsl::*;
pub use zorin::*;

pub use gap_closure::{
    AiTaskOrchestrator, BuildLedgerSystem,
    DriverRepositoryManager, FirmwareBridgeManager, HidGraphicsDriver,
    KernelModuleManager, NetworkStackGateway,
    OpenSourceCompetitorOrchestrator,
    PeripheralEmulationLibrary, SecurityPolicyManager, SovereignDistroAbsorptionEngine,
    SyscallCompatibilityRegistry, TargetDistroFamily, VirtualMemoryManager,
    ZorinAppearanceSwitcher,
};
pub use superiority::{
    LockFreeQueue, NumaCfsScheduler, ShardIgnitor, SovereignCloudFS, SovereignForensics,
    SovereignObjectBus, SovereignRecoverUtility, SovereignRegistry, SovereignSigLoader,
    SovereignThemeEngine, SovereignTimeMachine,
};

pub use arch_linux::{
    ArchFirewall, ArchInitSystem, ArchMirror, ArchNewsFeedParser, ArchPackage,
    ArchWikiSearchEngine, ArchinstallConfig, ArchinstallParity, ArtixInitBridge,
    ArtixInitSystemType, AurPatch, AurPatchEngine, AurRepoStatus, CachedPackage, DevFile,
    DevFileType, FirewallRule, KeyTrustLevel, LsmMode, LsmSentinel, MkinitcpioGenerator, NewsItem,
    PacmanDbCleaner, PacmanEngine, PacmanError, PacmanKey, PacmanKeyring, PamGate, PaneLayout,
    ProcFile, ProcFileType, ReflectorMirrorlist, RuleAction, RunlevelTarget, ServiceState,
    SovereignArchChrootVfsEngine, SovereignEnvRegistry, SovereignMakepkgConfEngine,
    SovereignPacmanConfEngine, SovereignSvntogitEngine, SubvolumeConfig, SystemdBootMetrics,
    TmuxMultiplexer, WikiPage, YayParuAdapter,
};

pub use open_source_tier1::{
    LibsodiumIntegration, SmolTcpIntegration, SqliteIntegration, WasmerIntegration,
};

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, TargetPlatform, TranslationLayer,
};
pub use interim::{InterimLispVM, LispVal, MntReformLpcDriver, ReformPowerStats};
pub use lubuntu::{
    CpuGovernor, LidCloseAction, LubuntuGlobalHotkeyManager, LubuntuHealthReport,
    LubuntuNetworkTrayAdapter, LubuntuNotificationDaemon, LubuntuPackageUpdateNotice,
    LubuntuPowerManager, LubuntuPpaRepository, LubuntuPpaRepositoryManager, LubuntuSystemManager,
    LxqtPackageUpdateNotifierEngine, SystemPressure, WifiAccessPointNode,
};

pub use cross_platform_kernel::{
    DeferredProcedureCall, IdtEntry, Idtr, Irql, IrqlController, Kpcr, Kpcrb, MemoryArch,
    PageAccessMode, PageDirectory as CrossPlatformPageDirectory, SovereignKernelInternals,
    SystemServiceTable, UmsContext, UmsThreadState,
};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};

pub use mate_betsy::{
    MateBetsyCategory, MateBetsyPackage, MatePackagesBetsyEngine,
};
pub use mint_linux::{
    CinnamonPreset, CinnamonThemeEngine, Mint4WinInstallationConfig, Mint4WinInstallerEngine,
    MintAppMetadata, MintBackupTool, MintCinnamonStyling, MintDriverInfo, MintDriverManager,
    MintReportAlert, MintReportAlertSeverity, MintReportSystem, MintSoftwareManager,
    MintTimeshiftEngine, MintUpdateLevel, MintUpdateManager, MintUpdatePackage,
    TimeshiftSystemRestorer,
};
pub use legacy_adapters::{
    KernelPersona, SyscallAbi, KernelPersonaVM, BinaryCompatMatrix, LibcVersion,
    LegacyDriverAdapter, LegacyFSAdapter, LegacyProtocolAdapter,
};

pub use chimera_linux::{ApkPackageMetadata, ApkPackageStore};
pub use community_foundation::{
    BountySeverity, FoundationRole, HackathonEvent, SecurityBounty, SovereignFoundationManager,
};

pub use relay_nexus::{
    BIOSNexus, BuildChronicle, BuildChronicleManager, CRTArchiveV2, CorebootNexus, DACNexus,
    DotMatrixArchiveV2, DriverVaultV2, DriverVaultV2Manager, FileEntry, FirmwareNexus,
    FirmwareNexusManager, FirmwareType, FloppyArchiveV2, GraphicsVaultV2, KernelRelay,
    LegacyAsmChronicle, LegacyCChronicle, LegacyCppChronicle, LegacyDriver, NetworkEntry,
    NetworkVaultV2, PeripheralArchiveV2, PeripheralArchiveV2Manager, PersonaType, ProcessEntry,
    SELinuxNexus, SecurityModelType, SecurityNexus, SecurityNexusManager, StorageVaultV2,
    SyscallEncyclopedia, SyscallEncyclopediaEntry, SyscallEntry, TapeArchiveV2, UEFINexus,
    ZeroTrustNexus,
};

pub use solid_kernel::{
    AuditBlock, ComplianceScheduler, IScheduler, PrioritySchedulerPort, RoundRobinSchedulerPort,
    SigmaFSPlusPlus, SolidKernelCore,
};

pub use absorb_tools::{
    CasObject, Clause, ContentAddressedStorage, DpllSatSolver, Literal, PledgePermission,
    PledgeUnveilSandbox, PqcSecureChannel,
};

pub use antix::*;
pub use bsd::{
    BsdJail, FreeBsdGeomManager, FreeBsdJailManager, GeomClassType, GeomProvider,
    NetBsdRumpKernelRouter, OpenBsdSandboxGuard, OpenBsdSysctlKernelMib, RumpHypercall,
};
pub use legacy_adapters::*;
pub use linux_compat::{
    AuxVector, BsdKevent, BsdKqueueFilter, BsdKqueueFilter as BsdCompatKqueueFilter,
    BsdKqueueMultiplexer, DistroTargetProfile, LinuxCompatSpec, LinuxElfLoaderShim,
    LinuxProcFsAdapter, LinuxSyscallNum, LinuxSyscallTranslator, OpenBsdPledgeUnveilFilter,
    TargetDistro,
};
pub use tiny_core::{FiletoolOverlay, FrugalLoader, TceLoader, TczExtension, TinyCoreBootConfig};

pub use apache_ossie::{
    MetricAggregation, OssieCatalog, OssieDimension, OssieInterpreter, OssieMetric, OssieOntology,
    OssieRelationship, SemanticRow,
};

pub use sovereign_suite::{
    CreativeMatrix, EverySearch, FancyZonesManager, ImageLayer, JoplinE2ee, LayoutZone,
    ProcMonitor, ProcessExplorerState, SpreadsheetCore, SysDiag,
};

pub use open_source_dominance::{
    InspirationFeature, InspirationFeatureMatrix, InspirationPackageIntegrator,
    InspirationSecurityGuard, OpenSourceDominanceEngine, OpenSourceInspirationTier,
};

pub use canonical::{
    AiResourceScheduler, AppSuiteBundle, AppSuiteType, BrailleMatrix, BsdJailSandbox,
    CloudOrchestrator, CloudProvider, CompatBinary, CompatBinaryFormat, CompatibilityLayer,
    ContinuityCoordinator, DesktopMode, DistroReleaseChannel, EcosystemSnapshot, FlatpakApp,
    HandoffTask, LanguageTranslationCatalog, LocaleManager, ReleaseGovernanceCouncil,
    ReproducibleBuildVerifier, SigmaContainer, SnapshotManager, SuiteRegistry, TtsSynthesizer,
    UnifiedAppStore,
};
pub use chakra::{
    AkabeiBundle, AkabeiPackageEngine, BundleType, DesktopTheme, InstallerStep, KapudanAssistant,
    TribeInstaller, GLOBAL_AKABEI, GLOBAL_KAPUDAN, GLOBAL_TRIBE,
};
pub use legacy_adapters::{
    APITimelineManager, DiscontinuedFS, DriverBridge, GraphicsBridge,
    LegacyBus, LegacyPluginManager, NetworkBridge,
    StorageBridge, WorkloadOptimizer, WorkloadProfile, GLOBAL_PERSONA_VM,
    GLOBAL_PLUGIN_MANAGER, GLOBAL_WORKLOAD_OPTIMIZER,
};
