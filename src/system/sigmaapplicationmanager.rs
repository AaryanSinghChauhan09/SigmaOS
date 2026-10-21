use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    Active,
    Inactive,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub state: AppState,
}

pub struct SigmaApplicationManager;
