# SigmaOS Roadmap: Real-Time Scheduling Class
Add SCHED_RT support for hard real-time audio and robotics workloads.
## Goals
- EDF (Earliest Deadline First) scheduling
- Priority inversion protection via priority inheritance
## Key Milestones
- [ ] RT task flag in capability token
- [ ] EDF runqueue (static heap data structure)
- [ ] Priority inheritance mutex protocol