# Upstream Study: FreeBSD Jails Container Isolation

1. **Upstream Problem**: Lightweight OS-level virtualization with isolated filesystem roots, IP addresses, and IPC.
2. **Data Structure & Control Flow**: `prison` struct, `jail_set()` system call, VNET virtual network stack isolation.
3. **Security & Failure Assumptions**: Strict default-deny jail capabilities; host kernel boundary protection.
4. **Direct Adoption**: Root directory isolation, process tree restriction, and VNET network stack isolation.
5. **Rust/SigmaOS Adaptation**: `SigmaSandboxContainer` merging FreeBSD Jails with Linux cgroups v2 semantics.
6. **Exit Test Criteria**: Jailed process attempting raw socket creation or parent filesystem access is blocked.
7. **Licensing & Attribution**: BSD 2-Clause License.
