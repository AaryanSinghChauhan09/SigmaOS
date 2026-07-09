# SigmaOS Roadmap: Local LLM Inference Engine
Embed quantized large language models natively inside the OS without cloud dependency.
## Goals
- Integrate llama.cpp / whisper.cpp GGUF model loader into local_llm.rs
- Provide a standard OS-level AI context API for all apps
- Support Q4_K_M quantization for models up to 13B parameters on 8GB RAM
## Key Milestones
- [ ] GGUF model loader with mmap zero-copy IO
- [ ] CPU SIMD (AVX2/NEON) matrix multiply optimisation
- [ ] Per-shard context isolation via Capability Tokens
- [ ] Streaming token output to Zenith terminal