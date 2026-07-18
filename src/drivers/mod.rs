// SigmaOS Drivers Module
pub mod even_more_devices;
pub mod gpu;
pub mod input;
pub mod kernel_releases;
pub mod legacy_keyboard;
pub mod modern_usb;
pub mod more_devices;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod usb_hid;
pub mod vesa;

pub use even_more_devices::{
    AdLibSynthDriver, Bluetooth5_4Adapter, Ne2000NetworkDriver, NvlinkBusDriver, PciIdeBridge,
    PcieGen6Bridge, Ps2MouseDriver, Sata3Controller, SerialMouseDriver, Ufs4StorageDriver,
    Usb4HostController, VgaTextModeDriver,
};
pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use kernel_releases::{
    KernelReleaseInfo, LinuxReleaseDriver, LongtermReleaseDriver, MainlineReleaseDriver,
    PrepatchRcDriver1, PrepatchRcDriver2, PrepatchRcDriver3, PrepatchRcDriver4, PrepatchRcDriver5,
    PrepatchRcDriver6, StableReleaseDriver,
};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;
pub use more_devices::{
    AppleSiliconUnifiedMemoryBus, CgaGraphicsDriver, CxlMemoryDriver, FloppyDiskDriver,
    GameportJoystickDriver, IdeControllerDriver, IntelXeGpuDriver, ParallelPrinterDriver,
    PcieGen5NvmeDriver, SoundBlaster16Driver, Thunderbolt4Controller, Wifi7Adapter,
};
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
