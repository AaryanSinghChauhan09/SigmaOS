// SigmaOS Drivers Module
// Core working drivers
pub mod ch340_usb;
pub mod e1000_nic;
pub mod even_more_devices;
pub mod gpu;
pub mod input;
pub mod intel_hda;
pub mod kernel_releases;
pub mod legacy_keyboard;
pub mod legacy_serial;
pub mod more_devices;
pub mod network;
pub mod nvme_storage;
pub mod peripheral;
pub mod storage;
pub mod usb_hid;
pub mod vesa;
pub mod linux_bsd_drivers;

// Working exports
pub use ch340_usb::Ch340Driver;
pub use e1000_nic::{E1000Driver, RxDescriptor, TxDescriptor};
pub use gpu::{GpuCommand, GpuCommandBuffer, GpuDriver, GpuError, GpuPipeline, GpuShader, ShaderStage};
pub use input::{InputDriver, InputEvent, InputType};
pub use intel_hda::{Bdle, IntelHdaDriver};
pub use legacy_keyboard::LegacyKeyboard;
pub use legacy_serial::LegacySerialPort;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use nvme_storage::{NvmeCmd, NvmeCqe, NvmeDriver};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralDeviceTrait, PeripheralDeviceInfo, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
pub use linux_bsd_drivers::{
    EvdevEvent, EvdevEventType, EvdevInputDevice, MultiTouchSlot,
    DrmAtomicKmsState, DrmConnectorType, DrmDisplayMode, FreeBsdDrmConnector,
    DriverCapability, OpenBsdDriverPledge,
    NetBsdRumpDriverHost,
    LinuxUrb, LinuxUrbQueue, UrbTransferType,
};
pub use more_devices::{
    FloppyDiskDriver, SoundBlaster16Driver, GameportJoystickDriver, IdeControllerDriver,
    ParallelPrinterDriver, CgaGraphicsDriver, PcieGen5NvmeDriver, Thunderbolt4Controller,
    Wifi7Adapter, IntelXeGpuDriver, CxlMemoryDriver, AppleSiliconUnifiedMemoryBus,
};
