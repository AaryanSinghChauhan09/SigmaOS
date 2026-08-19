// SigmaOS Drivers Module
// Core working drivers
pub mod gpu;
pub mod input;
pub mod legacy_keyboard;
pub mod legacy_serial;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod vesa;
pub mod more_devices;
pub mod usb_hid;
pub mod nvme_storage;
pub mod ch340_usb;
pub mod e1000_nic;
pub mod intel_hda;

// Temporarily disabled problematic modules
// pub mod even_more_devices;
// pub mod kernel_releases;
// pub mod legacy_floppy;
// pub mod modern_usb;
// pub mod more_devices;
// pub mod boot_init;
// pub mod dde;
// pub mod flipper_gpio_sensor;
// pub mod virtio;
// pub mod intel_e1000;
// pub mod legacy_audio_ac97;
// pub mod modern_audio_intel_hda;
// pub mod modern_nvme;
// pub mod modern_usb_printer;
// pub mod modern_wifi;
// pub mod touch_jingos;
// pub mod unified_dma;

// Working exports
pub use gpu::{GpuCommand, GpuDriver, GpuError, GpuCommandBuffer, GpuPipeline, GpuShader, ShaderStage};
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_keyboard::LegacyKeyboard;
pub use legacy_serial::LegacySerialPort;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralDeviceInfo, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
pub use usb_hid::{UsbHidDriver, HidError, HidKeyboardEvent, HidReportType};
pub use nvme_storage::{NvmeDriver, NvmeCmd, NvmeCqe};
pub use ch340_usb::Ch340Driver;
pub use e1000_nic::{E1000Driver, RxDescriptor, TxDescriptor};
pub use intel_hda::{IntelHdaDriver, Bdle};
pub use more_devices::{
    FloppyDiskDriver, SoundBlaster16Driver, GameportJoystickDriver, IdeControllerDriver,
    ParallelPrinterDriver, CgaGraphicsDriver, PcieGen5NvmeDriver, Thunderbolt4Controller,
    Wifi7Adapter, IntelXeGpuDriver, CxlMemoryDriver, AppleSiliconUnifiedMemoryBus,
};
