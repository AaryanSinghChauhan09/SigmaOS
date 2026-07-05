# SigmaOS Roadmap: Random Forest Classifier
Implement an ensemble tree classifier for tabular OS telemetry classification.
## Goals
- Static-array based decision tree nodes (no heap)
- OOB (out-of-bag) error estimation
## Key Milestones
- [ ] Decision tree node struct (no_std)
- [ ] Bootstrap sampling from sigma_db records
- [ ] Majority vote aggregation