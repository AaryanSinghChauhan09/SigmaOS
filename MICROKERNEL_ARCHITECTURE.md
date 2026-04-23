# Microkernel Architecture Exploration

## Vision
SigmaOS adopts a microkernel-inspired design to maximize modularity, security, and resilience. 

## Structure
By separating the core kernel primitives (scheduling, IPC, base memory management) from high-level services (drivers, file systems, networking), SigmaOS ensures that a crash in a driver does not compromise the entire system.

- **Kernel Core:** Lean, fast, minimal attack surface.
- **User-Space Services:** Everything else runs as isolated, unprivileged tasks communicating via zero-copy IPC.
