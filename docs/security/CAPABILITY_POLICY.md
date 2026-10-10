# Kernel Capability Policy

## Capability Model Rules

1. Every process is initialized with a bounded set of capabilities.
2. Privileged operations (e.g., `CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_SYS_PTRACE`) require explicit Capability Tokens.
3. Tokens are non-forgeable inside Ring 0.
4. Syscall dispatch checks token validity prior to executing kernel logic.
5. Violations trigger security audit logs and return `EPERM`.
