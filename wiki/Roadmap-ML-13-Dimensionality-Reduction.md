# SigmaOS Roadmap: Advanced Dimensionality Reduction
PCA, t-SNE, and UMAP for telemetry visualisation and embedding compression.
## Goals
- PCA via eigendecomposition (sigma_math.rs extension)
- t-SNE for 2D embedding visualisation in Zenith
## Key Milestones
- [ ] Power iteration for top-K eigenvalues
- [ ] t-SNE gradient descent (Barnes-Hut tree)
- [ ] 2D scatter export to terminal renderer