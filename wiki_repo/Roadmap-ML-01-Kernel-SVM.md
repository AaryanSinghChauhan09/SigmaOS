# SigmaOS Roadmap: Kernel SVM Classifier
Implement a Support Vector Machine classifier natively in sigma_data.rs.
## Goals
- SMO (Sequential Minimal Optimisation) solver in zero-alloc Rust
- RBF and polynomial kernel support
## Key Milestones
- [ ] SMO solver implementation
- [ ] Kernel function enum dispatch
- [ ] Integration with sigma_bench.rs benchmarks