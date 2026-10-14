# SigmaOS Capability Policy Specification

## Capability Tokens

Capabilities in SigmaOS are fine-grained permission tokens assigned to processes:

| Capability Flag | Description | Operations Protected |
| :--- | :--- | :--- |
| `CAP_SYS_ADMIN` | Global administration | Mounting, loading kernel modules, system halt |
| `CAP_NET_ADMIN` | Network configuration | Socket binding < 1024, route changes, firewall rules |
| `CAP_DAC_OVERRIDE` | Bypass file permissions | Read/write access ignoring inode DAC permission bits |
| `CAP_SYS_PTRACE` | Process tracing | `ptrace`, reading `/proc/<pid>/mem` of other users |

---

## Evaluation Directives
- Every privileged syscall path MUST invoke `verify_capability(token, cap)` before executing the privileged action.
- Denied operations must generate an audit log record in system diagnostics.
