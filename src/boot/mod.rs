//! Boot System (GRUB2/systemd-boot/refind Inspiration)
//! Advanced boot manager with themes, secure boot, and boot environments

extern crate alloc;

pub mod boot_snapshot;
pub mod bootloader;
pub mod bridge_grid;
pub mod firmware;
pub mod firmware_bridge;
pub mod multiboot2;
pub mod optimization;
pub mod pci;
pub mod plymouth;
pub mod post;
pub mod secure;
pub mod sigma_boot;
pub mod sigma_bootloader;
pub mod sovereign_multi_distro_bootloader;
pub mod uefi;
pub mod verified;

pub use sigma_bootloader::{
    BootArch, BootMemoryMap, BootPlatform, FramebufferInfo, KernelCmdline, LiveEnvironmentConfig,
    MemoryRegion, MemoryType, SigmaBootInfo, SigmaBootManager, SigmaBootloaderEngine,
    SIGMA_BOOT_MAGIC,
};

pub use boot_snapshot::{
    BootSnapshotConfig, BootSnapshotEngine, BootSnapshotItem, BootSnapshotStatus,
};
pub use firmware::{
    efi_attr, CpuMicrocodePatchEngine, EfiVariable, EfiVariableStore, EsrtEntry, EsrtFirmwareType,
    FirmwareCapsuleUpdateManager, IommuArchitecture, IommuFirmwareEngine, MicrocodeHeader,
    MicrocodeVendor, SmbiosFirmwareParser, SmbiosType0BiosInfo, SmbiosType1SystemInfo,
    SmbiosType2BaseboardInfo, SmbiosType3ChassisInfo, EFI_GLOBAL_VARIABLE_GUID,
    SECURITY_DATABASE_GUID,
};
pub use pci::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use plymouth::{GtkPlymouthBootsplashEngine, PlymouthMode, PlymouthTheme};
pub use post::{PostDiagnostics, PostStatus, PostTest, TestType};
pub use sigma_boot::{
    BootEntry, BootManager, BootStageDescriptor, BootTheme, HandoffProtocol,
    SovereignDistroBootStageHandoff, SovereignFastBootServicePipeline,
};
pub use uefi::{
    AcpiParser, BootError, GopFramebuffer, GopSplashCanvas, MicrokernelProfile,
    MultiKernelBootSelector, SecureBoot, SimpleSecureBoot, SimpleUEFIBootloader,
    SovereignBootWatchdog, UEFIBootloader, UsbHostController,
};

pub use sovereign_multi_distro_bootloader::{
    BootloaderKind, FreeBsdLoaderConfig, LimineBootEntry, LimineConfig, OpenBsdBootConfig,
    RefindMenuEntry, SovereignMultiDistroBootloaderEngine, SystemdBootBlsEntry, UniversalBootEntry,
};
