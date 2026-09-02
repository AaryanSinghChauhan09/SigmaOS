# BSD Security Hardening & Isolation Guide

SigmaOS integrates security and containment mechanisms from BSD distributions:

*   **OpenBSD**: Syscall process restriction (`pledge`), file path masking (`unveil`), W^X memory execution policies, and Retguard per-function return address cookies.
*   **FreeBSD**: Jails virtualization with nested child jail hierarchies, RACCT/RCTL resource controls, and Capsicum capability delegation (`CapsicumCapability`).
*   **DragonFly BSD**: HAMMER2 PFS multi-version B-tree filesystem and variant symlinks (`varsyms`) path resolution (`DragonFlyVarsymsPfsResolver`).
*   **HardenedBSD**: PaX MPROTECT W^X protection and SegvGuard brute force crash mitigation.
