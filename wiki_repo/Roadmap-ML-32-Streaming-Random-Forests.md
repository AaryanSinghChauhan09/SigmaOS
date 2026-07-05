# SigmaOS Roadmap: Streaming Random Forests
Update decision tree structures on incoming data streams without rebuilding from scratch.
## Goals
- Implement Hoeffding Adaptive Trees (HAT) for streaming regression and classification.
- Support drift detection hooks per node split.
## Key Milestones
- [ ] Hoeffding tree node update algorithm
- [ ] Split point evaluator with online histograms
- [ ] Memory limit checker for tree nodes