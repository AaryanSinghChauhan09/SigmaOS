# AI Agent Development Instructions for Security & Memory Protection Subsystems (`src/security/`)

This directory implements memory exploit mitigations, binary protection standards, ASLR/KASLR address space randomization, stack canaries, non-executable memory stacks (NX/DEP), control flow integrity (CFI), OpenBSD KARL (Kernel Address Randomized Link), input bounds validation, and process privilege containment for SigmaOS.

## Subsystem Architecture & Directives

### 1. Memory Exploit Mitigations & Binary Protection
- Enforce Non-Executable Stacks and Pages (NX / DEP) across all userland processes and dynamic kernel allocations.
- Enforce Stack Canary Guards (-fstack-protector-strong parity) on function prologue/epilogue frames to catch stack frame buffer overwrites before return address unwinding.
- KASLR & KARL: Randomize kernel base addresses and re-link kernel symbol locations on every boot to neutralize fixed-address ROP/JOP chain exploits.

### 2. Input Bounds Validation & Buffer Management
- All string and byte slice operations must enforce explicit length checks (len() <= MAX_ALLOWED_LEN) before copy operations.
- Strictly prohibit unbounded memory copies (memcpy without explicit boundary bounds). Use safe Rust slice copies (copy_from_slice) or safe wrapper APIs.
- Sanitize all external IPC payloads and device driver ioctl input buffers.

### 3. Syscall Filtering & Sandboxing
- Restrict process capabilities using eBPF seccomp syscall filters and OpenBSD pledge/unveil path restrictions to mitigate post-exploitation privilege escalation.

### 4. Verification & Audit
- Validate code changes with cargo check --lib and run relevant security unit tests before submitting.

## Key Security Principles (from OOPS/SOLID/Design Patterns)

### OOPS Principles Applied:
- **Encapsulation**: Security controls wrapped in modules with clear public APIs
- **Abstraction**: Security traits hide implementation details
- **Inheritance**: Security policies extend base traits
- **Polymorphism**: Multiple security backends (SELinux, AppArmor, seccomp)

### SOLID Principles:
- **Single Responsibility**: Each security module handles one concern
- **Open/Closed**: Security modules open for extension, closed for modification
- **Liskov Substitution**: Security policies interchangeable
- **Interface Segregation**: Fine-grained security interfaces
- **Dependency Inversion**: Depend on abstractions, not concrete security implementations

### Design Patterns:
- **Decorator**: Layer security on existing operations
- **Proxy**: Control access to resources
- **Strategy**: Swap security policies at runtime
- **Observer**: Monitor security events

## Memory Protection Guidelines

### Buffer Overflow Prevention
- Always check bounds before copying
- Use safe Rust slice operations
- Implement stack canaries in kernel code

### Pointer Safety
- Validate all pointer dereferences
- Use Option<T> for nullable pointers
- Implement zeroing on drop for sensitive data

### Concurrency Safety
- Use atomic operations for shared state
- Implement proper locking for sensitive regions
- Avoid race conditions in security-critical code
