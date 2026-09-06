// SigmaOS Drivers Module
pub mod boot_init;
pub mod dde;
pub mod distro_device_expansion;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;
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
pub mod peripheral;
pub mod storage;
pub mod touch_jingos;
pub mod printing;
pub mod usb_hid;
pub mod sovereign_driver_lifecycle;
pub mod vesa;

pub use printing::{CupsIppPrintSpooler, LpdSpooler, PpdDriverMatcher, PrintJob, PrintJobState};

pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_audio_ac97::LegacyAudioAc97;
pub use legacy_keyboard::LegacyKeyboard;
pub use legacy_parallel_printer::LegacyParallelPrinter;
pub use linux_bsd_drivers::{
    DrmAtomicKmsState, DrmConnectorType, DrmDisplayMode, EvdevEvent, EvdevInputDevice, EvdevEventType,
    FreeBsdDrmConnector, MultiTouchSlot, OpenBsdDriverPledge, DriverCapability,
    NetBsdRumpDriverHost, UrbTransferType, LinuxUrb, LinuxUrbQueue,
};
pub use modern_audio_intel_hda::ModernAudioIntelHda;
pub use modern_usb::ModernUsbController;
pub use modern_usb_printer::ModernUsbPrinterDriver;
pub use modern_wifi::ModernWifiDriver;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use linux_bsd_drivers::*;
pub use sovereign_driver_lifecycle::{
    ClusterAwarePeripheralManager, CommunityDriverRegistry, CrossOsDriverShim,
    DeclarativeDriverProfile, DeclarativeHardwareResolver, DriverShard, DriverShardManager,
    FirmwareType, IoBusType, ProgrammableIoStack, SandboxedHardwareModule,
    SignedDriverPackage, SovereignDriverLifecycleState, SovereignDriverManager,
    SovereignModularDeviceSupportEngine, TargetOsOrigin, UniversalFirmwareBridge,
};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};

pub use distro_device_expansion::{
    CanBusSocketDriver, Cxl3MemoryExpanderDriver, FloppyDiskControllerDriver,
    IntelI2cSmbusControllerDriver, IntelIgbNicDriver, IntelIwfWifiDriver, IntelXeArcGpuDriver,
    Mpt3SasControllerDriver, RadeonKmsGpuDriver, RaspberryPiGpioMailboxDriver,
    RealtekAlcAudioDriver, RealtekRtl8169Driver, SoundBlaster16IsaDriver, SynapticsTouchpadDriver,
    ThreeCom3c59xEthernetDriver, VirtioScsiControllerDriver, WacomGraphicsTabletDriver,
};

pub mod sovereign_hardware_roadmap;
pub use sovereign_hardware_roadmap::{
    SigmaDriverShard, ForeignDriverOrigin, CrossOsDriverAdapter, DeclarativeDriverProfileConfig,
    SigmaHotplugOrchestrator, DriverSandboxDomain, SigmaSandboxedHardwareModule,
    SigmaFirmwareBridge, SigmaFirmwareFreeDriver, SecurePeripheralIsolationGuard,
    SigmaDriverLayeringSystem, ClusterDeviceResource, SigmaDeviceClusterPool,
    SigmaProgrammableIoStack, TargetCpuArch, CrossArchDriverPortability,
    DriverSovereigntyPolicy, SigmaHardwarePolicyEngine, SigmaCryptographicBootChain,
    SigmaHardwareSovereigntyRoadmapEngine,
};
