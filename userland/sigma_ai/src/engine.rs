use crate::backend::ModelBackend;

/// Manages the lifecycle of AI models and routes inference requests.
pub struct InferenceEngine {
    backend: Option<Box<dyn ModelBackend>>,
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl InferenceEngine {
    pub fn new() -> Self {
        Self { backend: None }
    }

    pub fn load_model(&mut self, backend: Box<dyn ModelBackend>) -> Result<(), String> {
        self.backend = Some(backend);
        Ok(())
    }

    pub fn predict(&self, input: &str) -> Result<String, String> {
        if let Some(backend) = &self.backend {
            backend.predict(input)
        } else {
            Err("No model loaded".to_string())
        }
    }
}
