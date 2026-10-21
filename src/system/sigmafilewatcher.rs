use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventMask {
    Access,
    Modify,
    Create,
    Delete,
}

#[derive(Debug, Clone)]
pub struct FileEvent {
    pub path: String,
    pub mask: EventMask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchState {
    Active,
    Paused,
}

#[derive(Debug, Clone)]
pub struct Watch {
    pub id: u64,
    pub path: String,
    pub state: WatchState,
}

pub struct FileWatcher;
