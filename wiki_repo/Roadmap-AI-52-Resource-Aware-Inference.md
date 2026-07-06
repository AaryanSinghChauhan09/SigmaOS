# SigmaOS Roadmap: Resource-Aware Dynamic LLM Inference
Scale down model context windows and speculative decoding runs depending on thermal thresholds.
## Goals
- Dynamically prune KV-caches and adjust batch sizes when battery level drops or temperature spikes.
- Integrate with sigma_monitoring.rs telemetry streams.
## Key Milestones
- [ ] Multi-tier fallback configuration in sigma.toml
- [ ] Dynamic KV-cache allocation manager
- [ ] Dynamic batching scheduling thread