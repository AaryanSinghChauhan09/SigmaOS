// SigmaOS Hardware and Bus Drivers Subsystem Mod

pub mod compatibility;

pub use compatibility::{
    CompatibilityMatrix, Device, DeviceID, DeviceType, DiagnosticResult, DriverManager,
    HardwareDiagnostics, SimpleCompatibilityMatrix, SimpleDevice, SimpleDriverManager,
    SimpleHardwareDiagnostics, SupportStatus,
};
