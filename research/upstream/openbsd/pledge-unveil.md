# Upstream Study: OpenBSD pledge() and unveil()

1. **Upstream Problem**: Restricting system call surface area (`pledge`) and filesystem path access (`unveil`) in userspace.
2. **Data Structure & Control Flow**: `pledge` bitmask array; `unveil` radix path restriction tree on process descriptor.
3. **Security & Failure Assumptions**: One-way drop of privileges; path restrictions inherited by child processes.
4. **Direct Adoption**: Dual pledge promise filter and unveil path tree capability enforcement.
5. **Rust/SigmaOS Adaptation**: Safe Rust `ProcessCapabilityGuard` enforcing bitmask syscall filtering and path lookup.
6. **Exit Test Criteria**: Pledged process calling revoked `execve` or reading un-unveiled `/etc/shadow` is terminated.
7. **Licensing & Attribution**: ISC / BSD License.
