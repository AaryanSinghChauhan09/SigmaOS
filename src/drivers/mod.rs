// SigmaOS Drivers Module
pub mod ancient_devices;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;
pub mod gpu;
pub mod input;
pub mod kernel_releases;
pub mod legacy_audio_ac97;
pub mod legacy_floppy;
pub mod legacy_keyboard;
pub mod legacy_parallel_printer;
pub mod legacy_serial;
pub mod modern_audio_intel_hda;
pub mod modern_nvme;
pub mod modern_usb;
pub mod modern_usb_printer;
pub mod modern_wifi;
pub mod more_devices;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod touch_jingos;
pub mod usb_hid;
pub mod vesa;

pub use ancient_devices::{
    create_cga_graphics, create_floppy_disk, create_parallel_printer, create_sound_blaster_16,
    UdfAncientDevice,
};
pub use even_more_devices::{
    AdLibSynthDriver, Bluetooth5_4_Adapter, Ne2000NetworkDriver, NvlinkBusDriver, PciIdeBridge,
    PcieGen6Bridge, Ps2MouseDriver, Sata3Controller, SerialMouseDriver, Ufs4StorageDriver,
    Usb4HostController, VgaTextModeDriver,
};
pub use flipper_gpio_sensor::FlipperGpioSensor;
pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use kernel_releases::{
    KernelReleaseInfo, LinuxReleaseDriver, Longterm5_10_TpmDriver, Longterm5_15_SerialDriver,
    Longterm6_12_NetworkDriver, Longterm6_18_StorageDriver, Longterm6_1_InputDriver,
    Longterm6_6_AudioDriver, MainlineGpuDriver, Prepatch6_23_Rc1_AiDriver, Stable6_22_SensorDriver,
    LongtermReleaseDriver, MainlineReleaseDriver, PrepatchRcDriver1, PrepatchRcDriver2, 
    PrepatchRcDriver3, PrepatchRcDriver4, PrepatchRcDriver5, PrepatchRcDriver6, StableReleaseDriver,
};
pub use legacy_audio_ac97::LegacyAudioAc97;
pub use legacy_floppy::LegacyFloppyDriver;
pub use legacy_keyboard::LegacyKeyboard;
pub use legacy_parallel_printer::LegacyParallelPrinter;
pub use legacy_serial::LegacySerialPort;
pub use modern_audio_intel_hda::ModernAudioIntelHda;
pub use modern_nvme::ModernNvmeDriver;
pub use modern_usb::ModernUsbController;
pub use modern_usb_printer::ModernUsbPrinterDriver;
pub use modern_wifi::ModernWifiDriver;
pub use more_devices::{
    AppleSiliconUnifiedMemoryBus, CgaGraphicsDriver, CxlMemoryDriver, FloppyDiskDriver,
    GameportJoystickDriver, IdeControllerDriver, IntelXeGpuDriver, ParallelPrinterDriver,
    PcieGen5NvmeDriver, SoundBlaster16Driver, Thunderbolt4Controller, Wifi7Adapter,
};
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use touch_jingos::TouchJingosDriver;
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
