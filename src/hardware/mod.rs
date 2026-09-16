// SigmaOS Hardware and Bus Drivers Subsystem Mod

pub mod compatibility;

pub use compatibility::{
    AcpiPowerState, CompatibilityError, CompatibilityReport,
    CompatibilityResult, DeviceID, DeviceType, SupportStatus,
    SimpleAcpiManager, SimpleCompatibilityMatrix, SimpleDevice, SimpleDiagnostics,
};
