pub mod engine;
pub mod message;

pub use engine::{IpcEngine, IpcEndpoint};
pub use message::{Message, MessageType, Payload};

/// SigmaIPC: Native Inter-Process Communication replacing DBus.
/// Implements a broker-less peer-to-peer capability-verified messaging bus.
pub struct SigmaIpc {
    engine: IpcEngine,
}

impl Default for SigmaIpc {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaIpc {
    pub fn new() -> Self {
        Self {
            engine: IpcEngine::new(),
        }
    }
}
