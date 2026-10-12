// SigmaOS Hardware and Bus Drivers Subsystem Mod

pub mod compatibility;
pub mod sigmahal;
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
