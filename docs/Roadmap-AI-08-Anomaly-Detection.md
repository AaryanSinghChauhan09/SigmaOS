# SigmaOS Roadmap: Real-Time System Anomaly Detection
Detect abnormal process behaviour using unsupervised ML on kernel telemetry.
## Goals
- Online Isolation Forest trained on CPU/mem/IPC patterns
- Alert Security Center Daemon on anomaly score > threshold
## Key Milestones
- [ ] Feature extraction from sigma_monitoring.rs
- [ ] Isolation Forest C implementation (no_std compatible)
- [ ] Threshold auto-calibration on boot