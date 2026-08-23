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
pub mod more_devices;
pub mod ch340_usb;
pub mod e1000_nic;
pub mod intel_hda;
pub mod nvme_storage;
pub mod usb_hid;

// Exposing additional audio, printer, wifi drivers
pub mod legacy_parallel_printer;

pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{
    DeviceGeneration, PeripheralDevice, PeripheralDevice as PeripheralDeviceTrait,
    PeripheralDeviceInfo, PeripheralManager, PowerState,
};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
pub use more_devices::{
    FloppyDiskDriver, SoundBlaster16Driver, GameportJoystickDriver, IdeControllerDriver,
    ParallelPrinterDriver, CgaGraphicsDriver, PcieGen5NvmeDriver, Thunderbolt4Controller,
    Wifi7Adapter, IntelXeGpuDriver, CxlMemoryDriver, AppleSiliconUnifiedMemoryBus,
};
pub use ch340_usb::Ch340Driver;
pub use e1000_nic::{E1000Driver, RxDescriptor, TxDescriptor};
pub use intel_hda::{IntelHdaDriver, Bdle};
pub use nvme_storage::{NvmeDriver, NvmeCmd, NvmeCqe};
pub use usb_hid::{UsbHidDriver, HidError, HidKeyboardEvent, HidReportType};
