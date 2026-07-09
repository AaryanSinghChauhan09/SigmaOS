# SigmaOS Roadmap: Lightweight Neural Network Inference
Run small MLP networks (2-3 layers) for on-device classification tasks.
## Goals
- Dense layer forward pass in zero-alloc Rust
- ReLU, Sigmoid, Softmax activations
- INT8 quantisation support
## Key Milestones
- [ ] Dense layer matrix multiply (reuse sigma_math.rs)
- [ ] Activation function enum
- [ ] ONNX-lite model loader