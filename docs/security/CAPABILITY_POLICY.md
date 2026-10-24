# SigmaOS Capability Policy

## Overview
SigmaOS replaces traditional UID 0 / root privilege with capability tokens. Every process holds a capability set specifying allowed system operations.

## Kernel Capability Policy Rules

1. **Explicit Token Checks**: Privileged syscalls must verify required capabilities via `CapabilityPolicy::has_capability(pid, cap)`.
2. **Least Privilege**: Tasks inherit a restricted subset of capabilities unless explicitly granted by process launchers.
3. **Capability Drop**: Unprivileged services must drop non-essential capabilities after initialization.
4. **Audit Logging**: Any capability denial must log a structured audit record containing `pid`, `requested_capability`, and `denial_reason`.
