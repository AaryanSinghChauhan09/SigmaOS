// SigmaOS Drivers Module
pub mod driver_manager;
pub mod acpi;
pub mod advanced_types;
pub mod ahci_sata;
pub mod ata_bus_controller;
pub mod audio_intel_hda;
pub mod block_io;
pub mod boot_init;
pub mod dde;
pub mod distro_device_expansion;
pub mod drm_kms;
pub mod ethernet;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;
pub mod framebuffer;
pub mod gpu;
pub mod input;
pub mod legacy_audio_ac97;
pub mod legacy_keyboard;
pub mod legacy_parallel_printer;
pub mod linux_bsd_drivers;
pub mod modern_audio_intel_hda;
pub mod modern_nvme;
pub mod modern_usb;
pub mod modern_usb_printer;
pub mod modern_wifi;
pub mod network;
pub mod nvme_driver;
pub mod pci_bus;
pub mod peripheral;
pub mod printing;
pub mod rtc;
pub mod rtc_cmos;
pub mod serial;
pub mod sovereign_comprehensive_drivers;
pub mod sovereign_driver_lifecycle;
pub mod sovereign_hardware_expansion;
pub mod sovereign_usb_xhci;
pub mod storage;
pub mod touch_jingos;
pub mod usb_audio;
pub mod usb_hid;
pub mod usb_mass_storage;
pub mod usb_stack;
pub mod usb_video;
pub mod vesa;
pub mod wifi_80211;

pub use gpu::{
    GpuCommand, GpuCommandBuffer, GpuDriver, GpuError, GpuPipeline, GpuResetState, GpuShader,
    ShaderStage,
};
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_audio_ac97::LegacyAudioAc97;
pub use legacy_keyboard::LegacyKeyboard;
pub use legacy_parallel_printer::LegacyParallelPrinter;
pub use linux_bsd_drivers::*;
pub use linux_bsd_drivers::{
    AmdgpuDrmDriver, AmdgpuIpBlockType, AppleSiliconDartIommu, BroadcomBcmWifiDriver,
    BsdWgNetgraphHardwareDriver, DriverCapability, DrmAtomicKmsState, DrmConnectorType,
    DrmDisplayMode, EvdevEvent, EvdevEventType, EvdevInputDevice, FreeBsdDrmConnector,
    IntelIgcEthernetDriver, IntelXeDrmDriver, LinuxIioImuSensorDriver, LinuxUrb, LinuxUrbQueue,
    LsiMegaRaidHbaDriver, MultiTouchSlot, NetBsdRumpDriverHost, NvidiaNouveauGpuDriver,
    OpenBsdDriverPledge, QualcommAdrenoMaliGpuDriver, RaidLevel, RealtekR8169EthernetDriver,
    RpiBcmSocDriver, SdhciEmmcStorageDriver, SensorReadings, SovereignDeviceManager,
    SovereignWirelessCardDriver, ThunderboltSecurityLevel, ThunderboltUsb4Driver, Uac2AudioDriver,
    UrbTransferType, UvcCameraDriver, VideoPixelFormat, VirtioGpu3dDriver, VirtioSoundDriver,
    WacomPrecisionTouchpadDriver,
};
pub use modern_audio_intel_hda::ModernAudioIntelHda;
pub use modern_audio_intel_hda::*;
pub use modern_nvme::ModernNvmeDriver;
pub use modern_nvme::*;
pub use modern_usb::ModernUsbController;
pub use modern_usb_printer::ModernUsbPrinterDriver;
pub use modern_wifi::ModernWifiDriver;
pub use modern_wifi::*;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use sovereign_driver_lifecycle::{
    ClusterAwarePeripheralManager, CommunityDriverRegistry, CrossOsDriverShim,
    DeclarativeDriverProfile, DeclarativeHardwareResolver, DriverShard, DriverShardManager,
    FirmwareType, IoBusType, ProgrammableIoStack, SandboxedHardwareModule, SignedDriverPackage,
    SovereignDriverLifecycleState, SovereignDriverManager, SovereignModularDeviceSupportEngine,
    TargetOsOrigin, UniversalFirmwareBridge,
};
pub use sovereign_hardware_expansion::{
    ExpandedHardwareClass, HardwareDriverState, SovereignHardwareDriverExpansionEngine,
};
pub use sovereign_usb_xhci::{
    SovereignXhciTrb, SovereignXhciTrbType, SovereignXhciUsb3Driver, UsbDeviceSlotContext,
    UsbEndpointSpeed, XHCI_MAX_PORTS, XHCI_MAX_SLOTS, XHCI_TRB_RING_SIZE,
};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use touch_jingos::TouchJingosDriver;
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};

pub use driver_manager::{
    Driver, DriverManager, DriverStatistics, DriverStatus, DriverType,
};
pub use ethernet::{
    E1000Device, EtherType, EthernetError, EthernetFrame, EthernetHeader, MacAddr, NetDevStats,
    NetDevice,
};
pub use nvme_driver::{
    NvmeAdminOpcode, NvmeCQEntry, NvmeController, NvmeError, NvmeIOOpcode, NvmeNamespace,
    NvmeQueuePair, NvmeSQEntry,
};
pub use usb_stack::{
    UsbDevice, UsbDeviceDescriptor, UsbDeviceState, UsbDirection, UsbEnumerator, UsbError,
    UsbHostController, UsbSetupPacket, UsbSpeed, UsbTransferType,
};
pub use wifi_80211::{
    BssInfo, ChannelWidth, ConnectParams, Dot11Header, FrameType, ScanRequest, StationInfo,
    WifiBand, WifiCapabilities, WifiChannel, WifiDriver, WifiError, WifiMode, WifiSecurity,
    WifiStandard,
};

// Re-export advanced driver types
pub use advanced_types::*;
