/// SigmaOS Kernel Subsystem Registry
/// Inspired by Linux initcall mechanism — provides ordered subsystem initialization
/// OOP-based: every kernel module implements the KernelSubsystem trait
pub mod registry;

pub use registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority, SubsystemRegistry,
    SubsystemState,
};
