#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Hardware and Bus Drivers Subsystem Mod

pub mod compatibility;

pub use compatibility::{
    AcpiLoadBalancer, AcpiPowerState, CompatibilityCheck, CompatibilityError, CompatibilityReport,
    CompatibilityResult, DeviceID, DeviceType, HardwareDevice, HotplugEvent, HotplugManager,
    SimpleAcpiManager, SimpleCompatibilityMatrix, SimpleDevice, SimpleDiagnostics, SupportStatus,
};
