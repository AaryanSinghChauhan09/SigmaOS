# SigmaOS Roadmap: Transfer Learning Fine-Tuning
Fine-tune small language models on user-specific data without cloud upload.
## Goals
- LoRA adaptor training on local text corpus
- 4-bit quantised base model + F16 LoRA weights
## Key Milestones
- [ ] LoRA weight injection into GGUF loader
- [ ] Gradient accumulation on CPU (batch=1)
- [ ] Checkpoint saving to SovereignFS