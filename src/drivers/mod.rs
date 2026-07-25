// SigmaOS Drivers Module
pub mod ancient_devices;
pub mod gpu;
pub mod input;
pub mod legacy_audio_ac97;
pub mod legacy_floppy;
pub mod legacy_keyboard;
pub mod legacy_parallel_printer;
pub mod legacy_serial;
pub mod modern_audio_intel_hda;
pub mod modern_nvme;
pub mod modern_usb;
pub mod modern_usb_printer;
pub mod modern_wifi;
pub mod network;
pub mod peripheral;
pub mod storage;
pub mod usb_hid;
pub mod vesa;

pub use ancient_devices::{
    create_cga_graphics, create_floppy_disk, create_parallel_printer, create_sound_blaster_16,
    UdfAncientDevice,
};
pub use gpu::{GpuCommand, GpuDriver, GpuError};
pub use input::{InputDriver, InputEvent, InputType};
pub use kernel_io_suite::{
    AclPacket, AdLibSynth, AdLibSynthDriver as KernelAdLibSynth, AlsaError, AlsaSoundDriver,
    AncientDeviceLayer, AncientError, BluetoothError, BluetoothHciDriver, BluetoothMode, BssInfo,
    CommandBuffer, CommandStatus, Cursor, DisplayMode, EgaCgaAdapter, FlipRequest, GestureState,
    GestureType, GpuAccelerationDriver, GpuCommand as KernelGpuCommand, GpuError, HidFullError,
    HidInputReport, HidOutputReport, HidTokenType, IsaBus, IsaDevice, JobStatus, L2capChannel,
    L2capState, MfmDiskInterface, MultiTouchDriver, Ne2000Ethernet, PixelFormat, PrimitiveType,
    PrintJob, PrinterBackend, PrinterCupsDriver, PrinterError, PrinterFormat, PrinterProtocol,
    QosMapping, RingBuffer, SampleFormat, ScanResult, ScoPacket, SecurityType, TouchContact,
    TouchError, TouchProtocol, Uart8250, UsbHidFullDriver, VesaFramebufferDriver,
    VesaFramebufferError, VideoMode, WifiError, WifiFullStackDriver, WifiState, WpaToken,
    WpaTokenType,
};
pub use kernel_releases::{
    KernelReleaseInfo, Linux5_15ReleaseDriver, Linux6_12ReleaseDriver, Linux6_1ReleaseDriver,
    Linux6_6ReleaseDriver, LinuxReleaseDriver, LongtermReleaseDriver, MainlineReleaseDriver,
    PrepatchReleaseDriver, RcReleaseDriver, StableReleaseDriver,
};
pub use legacy_keyboard::LegacyKeyboard;
pub use legacy_parallel_printer::LegacyParallelPrinter;
pub use legacy_serial::LegacySerialPort;
pub use modern_audio_intel_hda::ModernAudioIntelHda;
pub use modern_nvme::ModernNvmeDriver;
pub use modern_usb::ModernUsbController;
pub use modern_usb_printer::ModernUsbPrinterDriver;
pub use modern_wifi::ModernWifiDriver;
pub use network::{NetworkCommand, NetworkDriver, NetworkError, NetworkType};
pub use peripheral::{DeviceGeneration, PeripheralDevice, PeripheralManager, PowerState};
pub use storage::{StorageCommand, StorageDriver, StorageError, StorageType};
pub use usb_hid::{HidError, HidKeyboardEvent, HidReportType, UsbHidDriver};
pub use vesa::{VesaDriver, VesaError, VesaModeInfo};
