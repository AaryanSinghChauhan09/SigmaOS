# SigmaOS Capability Policy

## Enforcement Rules
1. **Syscall Gateway Check:** Syscall entry points (`Posix450SyscallDispatcher`) validate capability masks before executing privileged operations (e.g. `CAP_SYS_ADMIN` for `mount`, `CAP_NET_RAW` for raw sockets).
2. **Fail Closed Default:** Unhandled or ambiguous requests fail closed with `EPERM` / `EACCES`.
3. **Capability Inheritance:** Process cloning inherits capability masks unless explicitly restricted via `unshare()` or `setns()`.
