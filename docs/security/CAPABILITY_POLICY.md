# SigmaOS Capability Policy & Sandbox Governance

## Capability Token Matrix

| Capability Token | Scope | Allowed Actions | Restricted Actions |
| :--- | :--- | :--- | :--- |
| `CAP_SYS_ADMIN` | System Governance | Mount filesystems, configure network interfaces | Directly write kernel memory |
| `CAP_NET_RAW` | Networking | Open raw network sockets, bind privileged ports (<1024) | Modify routing table without admin |
| `CAP_SYS_PTRACE` | Debugging | Attach ptrace to same-UID processes | Ptrace setuid or root processes |
| `CAP_DAC_OVERRIDE` | VFS | Bypass standard POSIX permission bits | Access unveil-restricted paths |

---

## Sandbox Profiles

Every userspace daemon or desktop application must operate under one of the following profiles:
1. **Strict Daemon Sandbox**: No network, restricted VFS read-only paths (`/etc`, `/usr`), no process fork/exec.
2. **Network Service Sandbox**: Bound to specific socket descriptor, restricted read/write to `/var/service`.
3. **Desktop Application Sandbox**: Access restricted to user home directory and IPC socket to desktop compositor.
