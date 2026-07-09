# SigmaOS Roadmap: Streaming Analytics with Windows
Process IPC event streams with tumbling and sliding windows.
## Goals
- Tumbling window aggregation (count, sum, min/max)
- Out-of-order event handling
## Key Milestones
- [ ] Window buffer in ring-buffer form
- [ ] Watermark-based late event handling
- [ ] Window result materialisation to sigma_db