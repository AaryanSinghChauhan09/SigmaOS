// SigmaOS Drivers Module
pub mod ancient_devices;
pub mod boot_init;
pub mod dde;
pub mod even_more_devices;
pub mod flipper_gpio_sensor;
pub mod gpu;
pub mod input;
pub mod kernel_io_suite;
pub mod kernel_releases;
pub mod legacy_keyboard;
pub mod modern_usb;
pub mod more_devices;
pub mod network;
pub mod peripheral;
pub mod soc;
pub mod storage;
pub mod usb_hid;
pub mod vesa;

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
    AdLibSynthDriver, Bluetooth5_4_Adapter, Ne2000NetworkDriver,
    NvlinkBusDriver, PciIdeBridge, PcieGen6Bridge, Ps2MouseDriver, Sata3Controller,
    SerialMouseDriver, Ufs4StorageDriver, Usb4HostController, VgaTextModeDriver,
};
pub use flipper_gpio_sensor::FlipperGpioSensor;
pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use kernel_io_suite::{
    AclPacket, AdLibSynth, AlsaError, AlsaSoundDriver,
    AncientDeviceLayer, AncientError, BluetoothError, BluetoothHciDriver, BluetoothMode, BssInfo,
    CommandBuffer, CommandStatus, Cursor, DisplayMode, EgaCgaAdapter, FlipRequest, GestureState,
    GestureType, GpuAccelerationDriver, GpuCommand as KernelGpuCommand, GpuError as KernelGpuError, HidFullError,
    HidInputReport, HidOutputReport, IsaBus, IsaDevice, JobStatus, L2capChannel,
    L2capState, MfmDiskInterface, MultiTouchDriver, Ne2000Ethernet, PixelFormat, PrimitiveType,
    PrintJob, PrinterBackend, PrinterCupsDriver, PrinterError, PrinterProtocol,
    QosMapping, RingBuffer, SampleFormat, ScanResult, ScoPacket, SecurityType, TouchContact,
    TouchError, TouchProtocol, Uart8250, UsbHidFullDriver, VesaFramebufferDriver,
    VesaFramebufferError, VideoMode, WifiError, WifiFullStackDriver, WifiState, WpaToken,
    WpaTokenType,
};
pub use kernel_releases::{
    KernelReleaseInfo, LinuxReleaseDriver,
    Longterm5_10_TpmDriver, Longterm5_15_SerialDriver,
    Longterm6_12_NetworkDriver, Longterm6_18_StorageDriver, Longterm6_1_InputDriver,
    Longterm6_6_AudioDriver, MainlineGpuDriver, Prepatch6_23_Rc1_AiDriver, Stable6_22_SensorDriver,
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
pub use soc::{
    ClockController, ClockError, GenericClock, GenericPin, PinController, PinDirection, PinError,
    PinPull, SocClockController, SocPinController, UnifiedSocController,
};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};

// Test and backward compatibility aliases
pub type Bluetooth5_4Adapter = Bluetooth5_4_Adapter;
pub type MainlineReleaseDriver = MainlineGpuDriver;
pub type StableReleaseDriver = Stable6_22_SensorDriver;
pub type LongtermReleaseDriver = Longterm6_18_StorageDriver;
pub type PrepatchRcDriver1 = Prepatch6_23_Rc1_AiDriver;
pub type PrepatchRcDriver2 = Longterm6_12_NetworkDriver;
pub type PrepatchRcDriver3 = Longterm6_6_AudioDriver;
pub type PrepatchRcDriver4 = Longterm6_1_InputDriver;
pub type PrepatchRcDriver5 = Longterm5_15_SerialDriver;
pub type PrepatchRcDriver6 = Longterm5_10_TpmDriver;
