# SigmaOS Roadmap: Neural Network-Driven Thread Scheduling
Dynamically predict thread runtime resource requirements using a lightweight on-device MLP.
## Goals
- Replace static heuristic schedulers with a forward-pass MLP predicting time-quantum exhaustion.
- Core scheduling decision loop completed in under 500 nanoseconds.
## Key Milestones
- [ ] Thread features extraction (IPC frequency, context switches, cache misses)
- [ ] Fast zero-alloc MLP execution code
- [ ] Scheduler integration in kernel