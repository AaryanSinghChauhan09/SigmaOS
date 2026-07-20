#![allow(unused_imports, unused_variables, dead_code, unused_mut, clippy::all)]
// SigmaOS Drivers Module
pub mod ancient_devices;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;
pub mod gpu;
pub mod input;
pub mod kernel_releases;
pub mod legacy_keyboard;
pub mod more_devices;
pub mod modern_usb;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod usb_hid;
pub mod vesa;

pub use ancient_devices::{
    create_cga_graphics, create_floppy_disk, create_parallel_printer, create_sound_blaster_16,
    UdfAncientDevice,
};
pub use even_more_devices::{
    AdLibSynthDriver, Bluetooth5_4_Adapter, Bluetooth54Adapter, Ne2000NetworkDriver,
    NvlinkBusDriver, PciIdeBridge, PcieGen6Bridge, Ps2MouseDriver, Sata3Controller,
    SerialMouseDriver, Ufs4StorageDriver, Usb4HostController, VgaTextModeDriver,
};
pub use flipper_gpio_sensor::FlipperGpioSensor;
pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use kernel_releases::{
    KernelReleaseInfo, Linux5_15ReleaseDriver, Linux6_12ReleaseDriver, Linux6_1ReleaseDriver,
    Linux6_6ReleaseDriver, LinuxReleaseDriver, LongtermReleaseDriver, MainlineReleaseDriver,
    PrepatchReleaseDriver, RcReleaseDriver, StableReleaseDriver,
};
pub use legacy_keyboard::LegacyKeyboard;
pub use more_devices::{
    AppleSiliconUnifiedMemoryBus, CgaGraphicsDriver, CxlMemoryDriver, FloppyDiskDriver,
    GameportJoystickDriver, IdeControllerDriver, IntelXeGpuDriver, ParallelPrinterDriver,
    PcieGen5NvmeDriver, SoundBlaster16Driver, Thunderbolt4Controller, Wifi7Adapter,
};
pub use modern_usb::ModernUsbController;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
