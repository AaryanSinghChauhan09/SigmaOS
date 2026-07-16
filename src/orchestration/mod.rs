// SigmaOS Orchestration Module
pub mod cross_device;

pub use cross_device::{
    AutomationRule, AutomationTrigger, ConnectedDevice, ConnectionStatus, CrossDeviceAction,
    CrossDeviceOrchestrator, DeviceCapability, DeviceType, OrchestrationError, SmartHomeDevice,
};
