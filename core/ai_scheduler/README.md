# AI Scheduling Engine

Predictive resource allocator that uses an on-device ML model to anticipate
workload bursts and pre-warm CPU/memory resources.

## Model Architecture
- Lightweight LSTM trained on historical shard behaviour
- Runs entirely in a sandboxed inference shard (no GPU required at boot)
- Inference latency < 50 µs on baseline x86_64

## Integration Points
- Feeds scheduling hints to `scheduling/hybrid/`
- Monitors memory pressure and pre-triggers compaction
- Signals VPN shard of expected burst traffic

## Roadmap
- [ ] Feature extraction pipeline (CPU/mem/io counters)
- [ ] Model training harness (offline, on reference hardware)
- [ ] Kernel hook for hint injection
