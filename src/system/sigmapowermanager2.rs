use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
}

#[derive(Debug, Clone)]
pub struct PowerPlan {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerState {
    Active,
    Saving,
}

pub struct SigmaPowerManager2;
