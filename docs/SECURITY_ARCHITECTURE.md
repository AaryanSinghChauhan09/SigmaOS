# SigmaOS Security Architecture

## Security Baseline

SigmaOS enforces defense-in-depth across kernel and userspace boundaries, inspired by Capsicum, Landlock, OpenBSD pledge/unveil, and POSIX capability models.

## Security Principles

1. **Least Privilege**: Components run with minimal required capabilities.
2. **Explicit Capability Checking**: Privileged operations require explicit capability validation.
3. **Memory Isolation**: Virtual address space isolation (W^X / DEP) between kernel and userspace.
4. **Audit and Logging**: Security-relevant denials and access attempts produce audit trails.
