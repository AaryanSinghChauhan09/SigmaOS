// SigmaOS Hardware and Bus Drivers Subsystem Mod

pub mod compatibility;
pub mod sigmaautomount;
pub mod sigmahal;
pub mod sigmabluetooth;
pub mod sigmaprint;
pub mod tech_powerup_hardware_monitors;
pub mod win32;

pub use compatibility::{
    AcpiPowerState, CompatibilityError, CompatibilityReport, CompatibilityResult, DeviceID,
    DeviceType, FreeBsdCamStorageEngine, FreeBsdNetmapHighSpeedPacketEngine,
    LinuxNvmeOverFabricsEngine, LinuxThunderboltDisplayPortTunnelEngine,
    LinuxVirtioGpu3dVirglEngine, NetBsdNpfHardwareOffloadEngine, OpenBsdAmdGpuKmsEngine,
    OpenBsdUvideoWebcamEngine, SimpleAcpiManager, SimpleCompatibilityMatrix, SimpleDevice,
    SimpleDiagnostics, SupportStatus,
};

pub use tech_powerup_hardware_monitors::{
    HardwareBustersPsuRailMonitorEngine, PcWorldBatteryHealthControllerEngine,
    SovereignTechPowerUpHardwareMonitorsSuite, TechPowerUpGpuProfilerEngine,
};
pub use sigmahal::{
    SigmaHAL, HardwareDevice as HalDevice, DeviceType as HalDeviceType, DeviceStatus as HalDeviceStatus,
    init_sigmahal, get_sigmahal, add_hardware_device, update_device_status,
};
pub use sigmaautomount::{
    SigmaAutoMount, MountPoint, MountOption, FsType, MountStatus,
    init_sigmaautomount, get_sigmaautomount, add_mount_point, mount_device, unmount_device,
};
pub use sigmabluetooth::{
    SigmaBluetooth, BluetoothDevice, BluetoothDeviceType, BluetoothDeviceStatus, AdapterState,
    init_sigmabluetooth, get_sigmabluetooth, pair_bluetooth_device, connect_bluetooth_device,
};
pub use sigmaprint::{
    SigmaPrint, Printer, PrintJob, PrinterStatus, PrintJobStatus,
    init_sigmaprint, get_sigmaprint, add_print_job,
};
