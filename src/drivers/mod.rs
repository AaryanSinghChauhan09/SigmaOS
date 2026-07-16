// SigmaOS Drivers Module
pub mod gpu;
pub mod input;
pub mod network;
pub mod storage;
pub mod usb_hid;
pub mod vesa;
pub mod peripheral;
pub mod legacy_keyboard;
pub mod modern_usb;
pub mod kernel_releases;

pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
pub use peripheral::{PeripheralDevice, PeripheralManager, DeviceGeneration, PowerState};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;
pub use kernel_releases::{
    LinuxReleaseDriver, KernelReleaseInfo,
    MainlineGpuDriver, Longterm6_18_StorageDriver, Longterm6_12_NetworkDriver,
    Longterm6_6_AudioDriver, Longterm6_1_InputDriver, Longterm5_15_SerialDriver,
    Longterm5_10_TpmDriver, Stable6_22_SensorDriver, Prepatch6_23_Rc1_AiDriver,
};
