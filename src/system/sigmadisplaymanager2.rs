use alloc::string::String;

#[derive(Debug, Clone)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct DisplayOutput {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayState {
    Connected,
    Disconnected,
}

pub struct SigmaDisplayManager2;
pub type SigmaDisplayManager = SigmaDisplayManager2;
