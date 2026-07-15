// SigmaOS Orchestration Module
pub mod cross_device;

pub use cross_device::{CrossDeviceOrchestrator, ConnectedDevice, SmartHomeDevice, AutomationRule, DeviceType, ConnectionStatus, DeviceCapability, AutomationTrigger, CrossDeviceAction, OrchestrationError};
