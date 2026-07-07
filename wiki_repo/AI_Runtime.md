# SigmaOS AI Runtime (SigmaAI)

## Overview
SigmaOS incorporates a localized, offline-first AI runtime (`SigmaAI`) utilizing quantized ONNX and Hugging Face architectures. The engine operates entirely locally, utilizing WebNN, DirectML, and Vulkan compute pipelines, ensuring data security and offline functionality for system queries, code autocompletion, and natural language translations.

## Architectural Flow
```
 [User Prompt / System Query]
              │
              ▼
   [Local Tokenizer (Rust)]
              │
              ▼
   [Quantized ONNX Engine (INT4/INT8)] ◄──► [Local Weights (Llama-3/Phi-3)]
              │
              ▼
   [Vulkan/WebNN Acceleration]
              │
              ▼
   [Secure Output Sandbox]
```

## System Properties
Models are stored under `/usr/share/sigma-ai/models/` and managed under the `sigma-ai` daemon.

Example settings (`sigma-ai.conf`):
```toml
[runtime]
engine = "onnxruntime"
device = "gpu"
quantization = "int4"
threads = 4

[models]
shell_agent = "phi-3-mini-q4.onnx"
translator = "gemma-2b-it-q4.onnx"
```

## Technical Implementation
The execution pipeline exposes high-speed inference endpoints without loading external Python interpreters.

```rust
// agents/sigma_ai_agent.rs
pub struct SigmaAIRuntime {
    pub session: onnxruntime::Session,
    pub tokenizer: Tokenizer,
}

impl SigmaAIRuntime {
    pub fn infer(&self, input: &str) -> Result<String, ModelError> {
        let tokens = self.tokenizer.encode(input)?;
        let inputs = vec![onnxruntime::Tensor::new(&tokens)];
        let outputs = self.session.run(inputs)?;
        let decoded = self.tokenizer.decode(&outputs[0])?;
        Ok(decoded)
    }
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Rust binding wrapper for ONNX runtime with INT4 quantization support.
- **Phase 2 (Months 3-6)**: Vulkan compute shaders for accelerated offline inference on Intel/AMD/NVIDIA cards.
- **Phase 3 (Months 6-9)**: NL-to-CLI translation engine integrated into the Zenith terminal emulator.
- **Phase 4 (Months 9-12)**: Federated learning pipeline for local fine-tuning without uploading user data.
