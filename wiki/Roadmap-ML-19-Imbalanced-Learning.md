# SigmaOS Roadmap: Imbalanced Class Learning
Handle heavily imbalanced OS event classes (e.g. 1% anomalies vs 99% normal).
## Goals
- SMOTE over-sampling in zero-alloc Rust
- Cost-sensitive loss weighting
## Key Milestones
- [ ] k-NN-based SMOTE synthetic sample generator
- [ ] Weighted cross-entropy loss
- [ ] F1/PR-AUC evaluation vs accuracy