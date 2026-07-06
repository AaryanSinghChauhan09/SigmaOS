# SigmaOS Roadmap: AI-Adaptive Process Scheduler
Use ML to predict CPU burst patterns and pre-warm cold caches.
## Goals
- Collect per-shard CPU/IO telemetry time-series
- Train lightweight LSTM predictor on-device
## Key Milestones
- [ ] Telemetry ring buffer in sigma_monitoring.rs
- [ ] LSTM inference (8-step lookahead)
- [ ] Scheduler hint API in kernel IPC