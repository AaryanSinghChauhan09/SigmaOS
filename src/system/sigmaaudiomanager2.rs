use alloc::string::String;

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioDeviceState {
    Active,
    Idle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioDeviceType {
    Input,
    Output,
}

pub struct SigmaAudioManager2;
