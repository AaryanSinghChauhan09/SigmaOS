# SigmaOS Roadmap: AI Predictive File Prefetching
Pre-load files into page cache before the user requests them.
## Goals
- Markov chain model of file access sequences
- Prefetch top-K predicted files during idle cycles
## Key Milestones
- [ ] File access log in VFS layer
- [ ] Markov transition matrix (static array)
- [ ] Async prefetch kernel call