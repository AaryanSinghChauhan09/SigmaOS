# SigmaOS Roadmap: GNN VFS Access Predictor
Predict next file access paths using Graph Neural Networks on directory trees.
## Goals
- Model directories as graphs and apply message passing to predict traversal.
- Pre-cache file blocks into memory ahead of time.
## Key Milestones
- [ ] Graph representation of directory node states
- [ ] Message passing GNN layer
- [ ] Prefetch buffer interface hook