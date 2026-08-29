# SigmaOS Security Model

SigmaOS is built with security as a foundational principle, eschewing legacy access control models for strict capability-based isolation and memory safety.

## Memory Safety via Rust
By implementing the kernel and user-space components entirely in Rust, SigmaOS inherently eliminates entire classes of vulnerabilities common in C/C++ operating systems:
- Buffer overflows
- Use-after-free
- Double-free
- Data races

`unsafe` blocks are strictly minimized and heavily audited.

## Sentinel Security Subsystem
Sentinel is SigmaOS's capability-based security enforcer. 
Instead of relying on user IDs and ambient authority (like root/sudo), processes only have access to resources explicitly granted to them via capabilities (tokens).

### Comparison to Legacy Systems
- **SELinux/AppArmor**: These rely on complex, global policy files to restrict ambient authority. Sentinel restricts at the micro-level—if a process doesn't hold the capability token for a file descriptor, it cannot access it, regardless of policy files.
- **pledge/unveil (OpenBSD)**: `pledge` restricts syscalls, while `unveil` restricts filesystem access. Sentinel provides a more granular and unified object-capability model.

## Path Traversal Protection
The VFS and Security Subsystem rigorously sanitize all filesystem paths. Recent enhancements include robust validation against relative `.` and `..` traversals across different delimiters (e.g., `/`, `\`, and `:` for URI/Windows-style paths).

## Secure Boot Process
SigmaOS supports standard UEFI Secure Boot. The bootloader and kernel images are signed, ensuring a cryptographically verified chain of trust from the firmware up to the init process.
