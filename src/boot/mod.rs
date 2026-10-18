#![no_std]

//! Boot System (GRUB2/systemd-boot/refind Inspiration)
//! Advanced boot manager with themes, secure boot, and boot environments

extern crate alloc;

pub mod assembly_bootstrap;
pub mod boot_snapshot;
pub mod bootloader;
pub mod bridge_grid;
pub mod firmware;
pub mod firmware_bridge;
pub mod initramfs;
pub mod initramfs_builder;
pub mod kernel_ld_script;
pub mod mkinitcpio_engine;
pub mod multiboot2;
pub mod optimization;
pub mod pci;
pub mod plymouth;
pub mod post;
pub mod secure;
pub mod sigma_boot;
pub mod sigma_bootloader;
pub mod smp_apic_calibration;
pub mod sovereign_multi_distro_bootloader;
pub mod start_assembly_entry;
pub mod uefi;
pub mod uefi_gop_framebuffer;
pub mod verified;

pub use bootloader::*;
pub use initramfs::*;
pub use initramfs_builder::{InitramfsBuilder, InitramfsEntry, CPIO_NEWC_MAGIC};
pub use kernel_ld_script::{KernelLinkerScriptConfig, DEFAULT_KERNEL_LD};
pub use smp_apic_calibration::{ApCpuInfo, ApCpuState, ApicCalibrator};

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
pub use uefi_gop_framebuffer::*;

pub use assembly_bootstrap::{
    AssemblyBootstrapEngine, AssemblyBootstrapPrGenerator, AssemblyHandoffPayload,
    Gdt64DescriptorTable, LongModePageTableSetup, Multiboot2HeaderSpec,
    MULTIBOOT2_BOOTLOADER_MAGIC, MULTIBOOT2_HEADER_MAGIC,
};
pub use sovereign_multi_distro_bootloader::{
    BootloaderKind, FreeBsdLoaderConfig, LimineBootEntry, LimineConfig, OpenBsdBootConfig,
    RefindMenuEntry, SovereignMultiDistroBootloaderEngine, SystemdBootBlsEntry, UniversalBootEntry,
};

pub mod sovereign_assembly_bootstrap_pr_suite;
pub use sovereign_assembly_bootstrap_pr_suite::*;

pub mod sovereign_kernel_and_bootloader_advancements_v47_pr;
pub use sovereign_kernel_and_bootloader_advancements_v47_pr::*;

pub mod sovereign_kernel_and_boot_innovations_v52_pr;
pub use sovereign_kernel_and_boot_innovations_v52_pr::*;

pub mod sovereign_kernel_and_boot_innovations_v53_pr;
pub use sovereign_kernel_and_boot_innovations_v53_pr::*;

pub mod sovereign_kernel_and_boot_innovations_v61_pr;
pub use sovereign_kernel_and_boot_innovations_v61_pr::*;

pub mod bare_metal_independent_boot_system;
pub use bare_metal_independent_boot_system::*;
