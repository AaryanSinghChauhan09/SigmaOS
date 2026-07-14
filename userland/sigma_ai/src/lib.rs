pub mod engine;
pub mod backend;

pub use engine::InferenceEngine;
pub use backend::ModelBackend;

/// SigmaAI: Native AI Inference Engine for SigmaOS
/// Provides on-device model execution and Neural UI backend services.
pub struct SigmaAi {
    pub engine: InferenceEngine,
}

impl Default for SigmaAi {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaAi {
    pub fn new() -> Self {
        Self {
            engine: InferenceEngine::new(),
        }
    }
}
