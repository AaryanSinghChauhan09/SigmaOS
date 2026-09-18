# 🛡️ Security, Sandboxing & Hardening Guide

SigmaOS implements a multi-layered security architecture that synthesizes paradigms from **OpenBSD** (`pledge`/`unveil`), **Linux Landlock v5**, **FreeBSD Capsicum**, and **Zorin Exec Guard**.

---

## 🔒 Multi-Layer Sandboxing Architecture

```
   ┌────────────────────────────────────────────────────────────────────────┐
   │                     MULTI-LAYER SANDBOX GUARD                          │
   ├────────────────────────────────────────────────────────────────────────┤
   │  Layer 1: OpenBSD `pledge()` Syscall Restriction                       │
   │  Layer 2: OpenBSD `unveil()` Filesystem Path Masking                   │
   │  Layer 3: Linux Landlock v5 Path Access Enforcement                    │
   │  Layer 4: FreeBSD Capsicum Descriptor Delegation                       │
   │  Layer 5: Zorin Exec Guard Default-Deny Model                          │
   └────────────────────────────────────────────────────────────────────────┘
```

---

## 🛡️ OpenBSD `pledge()` & `unveil()` Primitives

SigmaOS applications use zero-cost syscall and filesystem restrictions natively in Rust (`src/security/`):

### 1. `pledge()` Syscall Filtering:
Restricts execution to specific categories of system calls (e.g., `stdio`, `rpath`, `wpath`, `cpath`, `inet`, `dns`).
```rust
// Restrict process to standard I/O and network sockets
sigma_security::pledge("stdio inet dns").expect("Pledge failed");
```

### 2. `unveil()` Filesystem Path Masking:
Restricts visibility of the filesystem to explicitly declared directory subtrees:
```rust
// Mask filesystem except read access to /etc and read-write access to /tmp
sigma_security::unveil("/etc", "r").unwrap();
sigma_security::unveil("/tmp", "rw").unwrap();
sigma_security::unveil_lock().unwrap(); // Lock unveiling permanently for process
```

---

## ⚡ Zorin Exec Guard Capability Permission Model

Zorin Exec Guard enforces a **default-deny permission model** for untrusted binary execution:

1. **Interception**: Intercepts `execve` calls on untrusted binaries, foreign scripts, or downloaded packages.
2. **Interactive Capability Dialog**: Prompts user via Zenith GUI / terminal overlay:
   - *Option A*: Run isolated in chroot container (`pledge("stdio")`).
   - *Option B*: Grant temporary capability permission.
   - *Option C*: Recommend native `.sigpkg` or verified WebApp alternative.

---

## 🔐 Hardware Enclaves & Post-Quantum Cryptography

### 1. Confidential Enclave Isolation (AMD SEV-SNP & Intel TDX):
SigmaOS supports confidential virtual machine and process enclaves. Enclave memory pages are encrypted in hardware, preventing hypervisor introspection or DMA snooping. Memory is zeroed on process termination using volatile writes.

### 2. Post-Quantum Cryptography (PQC):
- **Dilithium-5**: Used for digital signatures on `.sigpkg` package manifests, livepatch trampolines, and kernel modules.
- **Kyber-1024**: Used for key encapsulation mechanisms (KEM) in WireGuard VPN tunnels and IPC channels.

---

## 🧪 Validating Security Hardening

To run security audit and sandbox validation tests:
```bash
./run_sigma_tests.sh
```
The test suite validates pledge/unveil enforcement, capability token revocation, and memory zeroization.
