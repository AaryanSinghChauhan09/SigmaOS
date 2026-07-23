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

pub mod even_more_devices;
pub mod kernel_releases;
pub mod more_devices;

pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};

pub use more_devices::{
    AppleSiliconUnifiedMemoryBus, CgaGraphicsDriver, CxlMemoryDriver, FloppyDiskDriver,
    GameportJoystickDriver, IdeControllerDriver, IntelXeGpuDriver, ParallelPrinterDriver,
    PcieGen5NvmeDriver, SoundBlaster16Driver, Thunderbolt4Controller, Wifi7Adapter,
};

pub use even_more_devices::{
    AdLibSynthDriver, Bluetooth5_4_Adapter, Ne2000NetworkDriver, NvlinkBusDriver, PciIdeBridge,
    PcieGen6Bridge, Ps2MouseDriver, Sata3Controller, SerialMouseDriver, Ufs4StorageDriver,
    Usb4HostController, VgaTextModeDriver,
};

pub use kernel_releases::{
    Longterm5_10_TpmDriver, Longterm5_15_SerialDriver, Longterm6_12_NetworkDriver,
    Longterm6_18_StorageDriver, Longterm6_1_InputDriver, Longterm6_6_AudioDriver,
    MainlineGpuDriver, Prepatch6_23_Rc1_AiDriver, Stable6_22_SensorDriver,
};
