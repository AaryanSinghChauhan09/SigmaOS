// SigmaOS Drivers Module
pub mod ancient_devices;
pub mod boot_init;
pub mod dde;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;
pub mod gpu;
pub mod input;
pub mod legacy_keyboard;
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
||||||| 43be3a7e8
pub mod peripheral;
pub mod legacy_keyboard;
pub mod modern_usb;
||||||| 43be3a7e8
pub mod peripheral;
pub mod legacy_keyboard;
pub mod modern_usb;
||||||| 0ddf2eac7
pub mod boot_init;
pub mod dde;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;
||||||| 165ded71c
pub mod boot_init;
pub mod dde;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;

pub use ancient_devices::{
    create_cga_graphics, create_floppy_disk, create_parallel_printer, create_sound_blaster_16,
    UdfAncientDevice,
};
pub use boot_init::{AcpiTableParser, UefiGopDriver, XhciHostController};
pub use dde::{
    BusType, DeviceError, DeviceId, DriverType, GenericDriver, HardwareBroker, LinuxDdeShim,
    UdfInterpreter, UnifiedPeripheral, WasmDriverVm, WindowsNdisWrapper,
};
pub use even_more_devices::{
    AdLibSynthDriver, Bluetooth5_4_Adapter, Ne2000NetworkDriver, NvlinkBusDriver, PciIdeBridge,
    PcieGen6Bridge, Ps2MouseDriver, Sata3Controller, SerialMouseDriver, Ufs4StorageDriver,
    Usb4HostController, VgaTextModeDriver,
};
pub use flipper_gpio_sensor::FlipperGpioSensor;
pub use gpu::{GpuCommand, GpuDriver, GpuError};
||||||| 43be3a7e8
pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub mod ch340_usb;
pub mod e1000_nic;
pub mod intel_hda;
pub mod nvme_storage;

pub use gpu::{
    GpuCommand, GpuCommandBuffer, GpuDriver, GpuError, GpuPipeline, GpuShader, ShaderStage,
};
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
||||||| 43be3a7e8
pub use peripheral::{PeripheralDevice, PeripheralManager, DeviceGeneration, PowerState};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;

pub use ch340_usb::Ch340Driver;
pub use e1000_nic::{E1000Driver, RxDescriptor, TxDescriptor};
pub use intel_hda::{Bdle, IntelHdaDriver};
pub use nvme_storage::{NvmeCmd, NvmeCqe, NvmeDriver};
||||||| 43be3a7e8
pub use peripheral::{PeripheralDevice, PeripheralManager, DeviceGeneration, PowerState};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;
