// SigmaOS Drivers Module
pub mod ancient_devices;
pub mod gpu;
pub mod input;
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
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod usb_hid;
pub mod vesa;

pub use ancient_devices::{
    create_cga_graphics, create_floppy_disk, create_parallel_printer, create_sound_blaster_16,
    UdfAncientDevice,
};
pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
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
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};

// Test and backward compatibility aliases
pub type Bluetooth5_4Adapter = Bluetooth5_4_Adapter;
pub type MainlineReleaseDriver = MainlineGpuDriver;
pub type StableReleaseDriver = Stable6_22_SensorDriver;
pub type LongtermReleaseDriver = Longterm6_18_StorageDriver;
pub type PrepatchRcDriver1 = Prepatch6_23_Rc1_AiDriver;
pub type PrepatchRcDriver2 = Longterm6_12_NetworkDriver;
pub type PrepatchRcDriver3 = Longterm6_6_AudioDriver;
pub type PrepatchRcDriver4 = Longterm6_1_InputDriver;
pub type PrepatchRcDriver5 = Longterm5_15_SerialDriver;
pub type PrepatchRcDriver6 = Longterm5_10_TpmDriver;
