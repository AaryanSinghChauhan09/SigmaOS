// Sovereign Application Manager

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub state: AppState,
}

#[derive(Debug, Default)]
pub struct SigmaApplicationManager {
    pub applications: Vec<Application>,
}

impl SigmaApplicationManager {
    pub fn new() -> Self {
        Self::default()
    }
}
