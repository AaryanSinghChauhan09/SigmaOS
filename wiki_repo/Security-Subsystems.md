# Kernel Security Subsystems

SigmaOS features a hardened, defense-in-depth kernel architecture tailored for sovereign security and resilience. All security components run entirely in `no_std`, without dynamic memory.

## Mandatory Access Control (MAC)

Implemented in `sigma_mac.rs`. 
Similar to SELinux, it relies on static subject/object contexts and permission bitmasks (`READ`, `WRITE`, `EXEC`, `TRANS`). If a policy rule doesn't explicitly allow an action, it defaults to deny.

## Seccomp-BPF Syscall Filter

Implemented in `sigma_seccomp.rs`.
Provides a minimal BPF virtual machine directly in the kernel to filter syscalls by number and argument. Allows userland tools to construct tight security sandboxes and jail misbehaving software.

## Kernel Audit Trail

Implemented in `sigma_audit.rs`.
A secure ring buffer tracking system events like:
- Denied MAC accesses
- Login failures
- Suspicious syscalls
Userland tools can poll this ring buffer to stream events to remote log aggregators.

## Intrusion Detection System (IDS)

Implemented in `sigma_ids.rs`.
Tracks syscall anomalies on a per-PID basis. If a process repeatedly issues failed syscalls (e.g. `EPERM` on files) or calls sensitive hooks rapidly, its anomaly score increases. Once the score exceeds a configurable threshold, the kernel will terminate the process.
