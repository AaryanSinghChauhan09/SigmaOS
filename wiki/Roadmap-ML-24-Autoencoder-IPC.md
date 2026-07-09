# SigmaOS Roadmap: Autoencoder-Based IPC Profiling
Analyse kernel IPC messages using autoencoders to identify architectural bottlenecks.
## Goals
- Train autoencoders on normal IPC message payload sizes and destinations.
- Tag messages with anomalous delays or structural properties.
## Key Milestones
- [ ] IPC message trace capture module
- [ ] Autoencoder training loop
- [ ] Anomaly alerting engine