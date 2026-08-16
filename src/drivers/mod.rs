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

pub mod ch340_usb;
pub mod e1000_nic;
pub mod intel_hda;
pub mod nvme_storage;
pub mod modern_nvme;
pub mod modern_wifi;
pub mod modern_audio_intel_hda;
pub mod bluetooth_hci;

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

pub use ch340_usb::Ch340Driver;
pub use e1000_nic::{E1000Driver, RxDescriptor, TxDescriptor};
pub use intel_hda::{Bdle, IntelHdaDriver};
pub use nvme_storage::{NvmeCmd, NvmeCqe, NvmeDriver};
pub use modern_nvme::{AhciStorageDriver, ModernNvmeDriver};
pub use modern_wifi::{ModernWifiDriver, Wpa3HandshakeState, ScannedAccessPoint};
pub use modern_audio_intel_hda::{ModernAudioIntelHda, HdaCodecVerb};
pub use bluetooth_hci::{BluetoothHciDriver, HciPacketType, DiscoveredBluetoothDevice};
