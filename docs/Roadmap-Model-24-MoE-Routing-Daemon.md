# SigmaOS Roadmap: Mixture-of-Experts routing Daemon
Provide a system-wide daemon that routes AI queries to specialized expert models.
## Goals
- Gate routing based on input prompt classification.
- Support hot-swapping expert weights dynamically.
## Key Milestones
- [ ] MoE router daemon (moed.rs)
- [ ] Dynamic weight loader
- [ ] Task priority router queue