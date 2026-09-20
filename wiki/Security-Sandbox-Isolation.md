# Security Sandbox & Isolation

SigmaOS implements a comprehensive security sandboxing model combining multiple approaches from Linux, BSD, and other secure operating systems. The sandboxing is designed to provide defense-in-depth security while maintaining usability.

## Security Models

### Linux Landlock v5
Landlock provides fine-grained filesystem access control:
- File hierarchy restrictions (read-only, read-write, execute)
- Process tree inheritance of restrictions
- Unprivileged sandbox creation without root access
- Integration with existing Linux applications

### FreeBSD Capsicum
Capsicum offers capability-based security:
- File descriptor rights delegation
- Capability mode for process confinement
- Rights limiting per file descriptor
- Global capability namespace hiding

### OpenBSD Pledge/Unveil
OpenBSD-style process restrictions:
- Pledge: Limit system calls a process can make
- Unveil: Restrict filesystem paths a process can access
- Instant process termination on violation
- Simple, declarative security model

## Sentinel Exec Guard

The Sentinel Exec Guard (Zorin) is SigmaOS's default-deny capability permission model:

### Features
- Default-deny policy: all capabilities denied by default
- Capability tokens for resource access
- Fine-grained permission checking per syscall
- Automatic capability revocation on process termination

### Usage
```bash
# Grant capabilities to a process
sentinel grant --read /home/user/Documents
sentinel grant --network --socket /tmp/app.sock

# Check process capabilities
sentinel inspect <pid>

# Revoke capabilities
sentinel revoke --all <pid>
```

### Capability Categories
- **File System**: Read, write, execute, create, delete
- **Network**: Socket creation, binding, connection
- **Process**: Fork, exec, signal, ptrace
- **Device**: Device access, raw I/O, ioctl
- **IPC**: Shared memory, semaphores, message queues

## Sandboxing Components

### Application Sandboxing
Applications are sandboxed based on their security requirements:
- Browser: Network, graphics, file read (user data only)
- Terminal: Shell access, file system, device access
- Text Editor: File read/write, limited network
- Media Player: Graphics, audio, file read (media only)

### Sandbox Profiles
Pre-defined sandbox profiles for common application types:
- `browser`: Web browsing with restricted file access
- `terminal`: Full shell access with device I/O
- `editor`: Text editing with file system access
- `media`: Media playback with graphics/audio
- `network`: Network applications with minimal file access

### Custom Sandboxes
Users can create custom sandbox profiles:
```toml
[profile my-app]
allow-read = ["/home/user/data", "/etc/config"]
allow-write = ["/home/user/data"]
allow-network = true
allow-graphics = true
deny-syscalls = ["ptrace", "kexec"]
```

## Hardware Enclaves

SigmaOS supports hardware enclaves for additional security:

### Intel SGX
- Secure enclave for sensitive data processing
- Memory encryption for enclave contents
- Attestation for enclave identity verification
- Remote attestation for cloud deployments

### AMD SEV
- Encrypted virtual machine memory
- Secure nested virtualization
- Guest VM attestation
- Memory integrity protection

### Post-Quantum Cryptography
- Dilithium-5 for signatures
- Kyber-1024 for key exchange
- Integration with cryptographic primitives
- Future-proof security against quantum attacks

## Implementation

### Sentinel Integration
The security sandbox is integrated throughout SigmaOS:
- Package manager: Packages declare required capabilities
- Desktop environment: Applications launched with appropriate sandbox
- Service manager: Services run with minimal required permissions
- System daemons: Background processes with restricted capabilities

### Cross-Platform Compatibility
Security sandboxing works across all supported platforms:
- Linux: Landlock v5 with seccomp filters
- FreeBSD: Capsicum with jail support
- OpenBSD: pledge/unveil with malloc conf
- Solaris/Illumos: Zones with Trusted Extensions

### Performance Impact
Sandboxing is designed for minimal performance overhead:
- Fast path for common operations
- Lazy capability checking where possible
- Cache frequently used capability decisions
- Inline capability verification in hot paths

## Troubleshooting

### Application Won't Start
If an application fails to start due to sandboxing:
1. Check denied capabilities: `sentinel inspect <pid>`
2. Adjust sandbox profile: `sentinel profile edit <profile>`
3. Grant additional capabilities: `sentinel grant --<capability> <path>`
4. Use relaxed sandbox for testing: `sentinel --relaxed <app>`

### Performance Issues
If sandboxing causes performance problems:
1. Check capability cache: `sentinel stats --cache`
2. Profile capability checks: `sentinel profile --timing <app>`
3. Reduce capability granularity where appropriate
4. Use fast-path capabilities for common operations

### Security Violations
If security violations occur:
1. Check violation logs: `journalctl -u sentinel -f`
2. Review capability grants: `sentinel audit --recent`
3. Revoke overly permissive capabilities
4. Report security issues for investigation

---

**[Security](Category-Security)** | **[Landlock](Landlock-Sandboxing)** | **[Capsicum](Capsicum-Integration)** | **[Pledge/Unveil](Pledge-Unveil-Enforcement)**
