# SigmaOS Roadmap: Gradient Boosting Engine (XGBoost-equivalent)
High-performance gradient boosted trees for tabular telemetry classification.
## Goals
- Histogram-based tree building (LightGBM-style)
- Parallel tree construction using IPC thread pool
## Key Milestones
- [ ] Histogram bin construction
- [ ] Gradient/hessian computation
- [ ] Tree ensemble serialisation to sigma_db