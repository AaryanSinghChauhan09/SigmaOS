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

<<<<<<< HEAD
// Expose rich hidden Linux/BSD-inspired drivers
pub mod ch340_usb;
pub mod e1000_nic;
pub mod intel_hda;
pub mod legacy_floppy;
pub mod legacy_serial;
pub mod modern_wifi;
pub mod modern_nvme;
pub mod legacy_parallel_printer;
pub mod touch_jingos;
pub mod modern_audio_intel_hda;
pub mod legacy_audio_ac97;
pub mod modern_usb_printer;
pub mod nvme_storage;
pub mod soc;
pub mod kernel_io_suite;

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
||||||| 23ef22a4a
pub use gpu::{GpuCommand, GpuDriver, GpuError};
=======
pub mod ch340_usb;
pub mod e1000_nic;
pub mod intel_hda;
pub mod nvme_storage;

pub use gpu::{
    GpuCommand, GpuCommandBuffer, GpuDriver, GpuError, GpuPipeline, GpuShader, ShaderStage,
};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub use input::{InputDriver, InputEvent, InputType};
pub use legacy_keyboard::LegacyKeyboard;
pub use modern_usb::ModernUsbController;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};

// Re-exports of hidden drivers
pub use ch340_usb::{Ch340Driver, CH340_VENDOR_ID, CH340_PRODUCT_ID};
pub use e1000_nic::{E1000Driver, RxDescriptor, TxDescriptor};
pub use intel_hda::{IntelHdaDriver, Bdle};
pub use legacy_floppy::LegacyFloppyDisk;
pub use legacy_serial::LegacySerialPort;
pub use modern_wifi::ModernWifiDriver;
pub use modern_nvme::{ModernNvmeDriver, NvmeCmd as ModernNvmeCmd, NvmeSubmissionQueue, NvmeCompletionQueue, SmartTelemetry, AhciCommandHeader, AhciPort};
pub use legacy_parallel_printer::LegacyParallelPrinter;
pub use touch_jingos::TouchJingosDriver;
pub use modern_audio_intel_hda::ModernAudioIntelHda;
pub use legacy_audio_ac97::LegacyAudioAc97;
pub use modern_usb_printer::ModernUsbPrinterDriver;
pub use nvme_storage::{NvmeDriver, NvmeCmd as NvmeStorageCmd, NvmeCqe};
pub use soc::{PinController, ClockController, GenericPin, GenericClock, SocPinController, SocClockController, UnifiedSocController, PinDirection, PinPull, PinError, ClockError};
pub use kernel_io_suite::{BluetoothHciDriver, BluetoothMode, AclPacket, ScoPacket, L2capChannel, L2capState, BluetoothError, PrinterCupsDriver, PrinterProtocol, PrinterBackend, PrintJob, PrintFormat, JobStatus, PrinterError, GpuAccelerationDriver, CommandBuffer, GpuCommand as SuiteGpuCommand, PrimitiveType, CommandStatus, FlipRequest, DisplayMode, PixelFormat, GpuError as SuiteGpuError, AlsaSoundDriver, RingBuffer, SampleFormat, AlsaError, WifiFullStackDriver, WifiState, ScanResult, SecurityType, BssInfo, WpaToken, WpaTokenType, QosMapping, WifiError, MultiTouchDriver, TouchProtocol, TouchContact, GestureState, GestureType, TouchError, VesaFramebufferDriver, Cursor, VesaFramebufferError, UsbHidFullDriver, HidInputReport, HidOutputReport, HidFullError, AncientDeviceLayer, Uart8250, IsaBus, IsaDevice, Ne2000Ethernet, MfmDiskInterface, AdLibSynth, EgaCgaAdapter, VideoMode, AncientError, HidTokenType, PrinterFormat};
