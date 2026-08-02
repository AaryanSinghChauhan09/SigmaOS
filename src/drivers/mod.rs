#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Drivers Module
pub mod gpu;
pub mod input;
pub mod legacy_floppy;
pub mod legacy_keyboard;
pub mod legacy_serial;
pub mod modern_usb;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod usb_hid;
pub mod vesa;
pub mod boot_init;
pub mod dde;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;

// Exposing additional audio, printer, wifi drivers
pub mod legacy_audio_ac97;
pub mod modern_audio_intel_hda;
pub mod legacy_parallel_printer;
pub mod modern_usb_printer;
pub mod modern_wifi;

pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_floppy::LegacyFloppyDisk;
pub use legacy_keyboard::LegacyKeyboard;
pub use legacy_serial::LegacySerialPort;
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
