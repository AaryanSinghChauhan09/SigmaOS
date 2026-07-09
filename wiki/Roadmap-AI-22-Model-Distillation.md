# SigmaOS Roadmap: On-Device Model Distillation
Compress large teacher models into tiny student models optimised for SigmaOS hardware.
## Goals
- Knowledge Distillation (KD) training loop on local GPU
- Student model 10x smaller than teacher with <5% accuracy loss
## Key Milestones
- [ ] Teacher logit extraction pipeline
- [ ] KL-divergence loss student training loop
- [ ] Automatic student GGUF export