# SigmaOS Roadmap: Binary and Ternary Embedding Models
Support highly-quantized binary (1-bit) and ternary (2-bit) text embeddings.
## Goals
- Reduce document vector storage sizes by up to 90% in sigma_db.
- Leverage bitwise operations for distance comparisons.
## Key Milestones
- [ ] Embedding binarizer module
- [ ] Hamming distance optimizer (POPCNT instruction)
- [ ] Comparison benchmarks vs FP32 embeddings