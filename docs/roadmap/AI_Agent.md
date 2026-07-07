# AI Agent Roadmap (SigmaAI)

## 1. Local-First Runtime & Quantization
SigmaOS embeds an offline-first machine learning runtime (`sigma_ai`), eliminating cloud dependancies and privacy leaks.
- **Inference Engine**: Highly optimized, quantized ONNX/HuggingFace runtime targeting local CPUs/NPUs.
- **Quantization strategy**: Focus on INT4 and INT8 quantized formats, running large language models smoothly within a minimal memory footprint (e.g. Phi-3, Gemma-2B).

## 2. NL -> CLI Safety & Guardrails
- **Dry-run enforcement**: Translates natural language system requests ("Show network interfaces and active sockets") into staged commands.
- **Safety checks**: Staged CLI commands are validated against security rules. Destructive commands are blocked or require explicit TPM verification.

## 3. Signed Model Marketplace
- Users cannot download untrusted weights.
- Models must be signed by the SigmaOS authority and verified before loading.
- Provenance logs record all AI-generated suggestions to ensure administrative accountability.

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: Basic ONNX parser and model runtime loading stubs.
- **Phase 2 (3–6m)**: CLI suggestion interface with Dry-Run safety buffers.
- **Phase 3 (6–9m)**: Model verification and cryptographic signature validation routines.
- **Phase 4 (9–12m)**: Advanced NPU hardware acceleration optimization.

## 5. Contributor Guidelines
- Implement local fallback paths for all AI features.
- Ensure all models pass the strict integrity signature checks before integration.
