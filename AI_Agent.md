# AI Agent Roadmap (SigmaAI)

## Local-First Philosophy
SigmaOS embeds AI capabilities natively via a local, offline-first runtime. 

## Quantized Models
We focus on INT4/INT8 quantized ONNX/HuggingFace formats capable of running efficiently on CPUs/NPUs without relying on cloud APIs.

## NL -> CLI Runtime (v0.1)
- Enables Natural Language system administration ("Find all auth errors").
- **Safety Guardrails:** Includes a Dry-Run enforcement layer. The AI stages the command but cannot execute it without explicit human elevation.

## Model Marketplace
A decentralized but cryptographically signed repository for model weights ensures supply chain integrity for AI components.
