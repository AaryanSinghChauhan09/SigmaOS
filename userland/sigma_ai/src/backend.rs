/// Trait abstraction over various AI inference runtimes (e.g. ONNX, local LLM).
pub trait ModelBackend {
    fn predict(&self, input: &str) -> Result<String, String>;
}

/// A stub backend for testing Neural UI concepts.
pub struct StubBackend;

impl ModelBackend for StubBackend {
    fn predict(&self, input: &str) -> Result<String, String> {
        Ok(format!("Predicted UI layout for: {}", input))
    }
}
