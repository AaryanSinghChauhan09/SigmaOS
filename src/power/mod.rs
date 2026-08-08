// SigmaOS Power Management Module
pub mod battery;
pub mod governor;
pub mod management;
pub mod stack;

pub use battery::{
    Battery as BatteryDevice, BatteryError, BatteryInfo, BatteryState as BatteryStatus,
    SimpleBattery as SimpleBatteryDevice,
};
pub use governor::{
    AspmLevel, CpuFreqCore, CpuGovernor, EnergyAwareThreadBalancer, TlpPowerManager,
};
pub use stack::{
    Power as PowerManagement, PowerCapability, PowerError, PowerEvent, PowerStack,
    PowerProfile as PowerState, SimplePowerManager, SimplePowerStack,
};
pub use management::{
    PowerProfile as PowerProfileTrait, SimplePowerProfile, CPUGovernor as CPUGovernorTrait,
    SimpleCPUGovernor, ThermalManager, SimpleThermalManager, PowerManager,
    SimplePowerManager as ManagementPowerManager, BatteryManager as ManagementBatteryManager,
    BatteryStatus as ManagementBatteryStatus, SimpleBatteryManager as ManagementBatteryManagerImpl,
};
