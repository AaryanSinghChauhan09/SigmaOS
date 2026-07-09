# SigmaOS Roadmap: Embedded Data Pipeline Engine
Build a zero-allocation streaming data pipeline for real-time telemetry.
## Goals
- Source â†’ Transform â†’ Sink pipeline DSL
- Support CSV, JSON, and binary telemetry formats
## Key Milestones
- [ ] Pipeline node enum in sigma_logic.rs
- [ ] CSV parser (no_std, zero-alloc)
- [ ] Sink adapters: sigma_db, file, network