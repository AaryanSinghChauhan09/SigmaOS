// SigmaOS Drivers Module
pub mod gpu;
pub mod input;
pub mod legacy_keyboard;
pub mod modern_usb;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod usb_hid;
pub mod vesa;

// Exposing additional audio, printer, wifi drivers
pub mod legacy_audio_ac97;
pub mod modern_audio_intel_hda;
pub mod legacy_parallel_printer;
pub mod modern_usb_printer;
pub mod modern_wifi;

pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};

pub use legacy_audio_ac97::LegacyAudioAc97;
pub use modern_audio_intel_hda::ModernAudioIntelHda;
pub use legacy_parallel_printer::LegacyParallelPrinter;
pub use modern_usb_printer::ModernUsbPrinterDriver;
pub use modern_wifi::ModernWifiDriver;
