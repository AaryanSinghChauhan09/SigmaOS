// SigmaOS Drivers Module
pub mod even_more_devices;
pub mod gpu;
pub mod input;
pub mod kernel_releases;
pub mod legacy_keyboard;
pub mod legacy_serial;
pub mod legacy_floppy;
pub mod modern_usb;
pub mod more_devices;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod usb_hid;
pub mod vesa;
pub mod boot_init;
pub mod dde;
pub mod flipper_gpio_sensor;

// Ubiquitous virtualization & gigabit networking drivers (inspired by Linux & BSD)
pub mod virtio;
pub mod intel_e1000;

// Broad PC architecture compatibility drivers
pub mod legacy_audio_ac97;
pub mod modern_audio_intel_hda;
pub mod modern_nvme;
pub mod modern_usb_printer;
pub mod modern_wifi;
pub mod touch_jingos;

pub use even_more_devices::*;
pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use kernel_releases::*;
pub use legacy_keyboard::LegacyKeyboard;
pub use legacy_serial::LegacySerialPort;
pub use legacy_floppy::LegacyFloppyDisk;
pub use modern_usb::ModernUsbController;
pub use more_devices::*;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};

// Re-export the newly added virtual and physical standard device drivers
// pub use virtio::{VirtioBlkDriver, VirtioNetDriver, VirtioRngDriver, VirtQueue, VirtioDesc};
// pub use intel_e1000::{IntelE1000Driver, E1000RxDesc, E1000TxDesc};
// pub use legacy_audio_ac97::LegacyAudioAc97;
// pub use modern_audio_intel_hda::ModernAudioIntelHda;
// pub use modern_nvme::ModernNvmeDriver;
// pub use modern_usb_printer::ModernUsbPrinter;
// pub use modern_wifi::ModernWifiDriver;
// pub use touch_jingos::TouchJingOS;
pub mod pci;
pub mod virtio_blk;
pub mod virtio_net;
pub mod ahci;
pub mod framebuffer;
