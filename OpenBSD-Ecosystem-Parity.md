# OpenBSD Ecosystem Parity & Proactive Security Hardening

SigmaOS incorporates the renowned security discipline and proactive mitigations of **OpenBSD**, replacing legacy unsafe C paradigms with Rust's compile-time memory guarantees and uncompromising kernel defenses.

---

## 🔒 Proactive Defense Primitives from OpenBSD

### 1. Pledge(2) System Call Promise Restriction
- **Syscall Surface Minimization**: Applications explicitly drop privileges by declaring promises (`"stdio"`, `"rpath"`, `"wpath"`, `"cpath"`, `"inet"`, `"dns"`, `"proc"`, `"exec"`).
- **Instant Process Termination**: If a pledged process attempts an undeclared syscall, the kernel immediately terminates the process with `SIGABRT` and logs an audit record.
- **SigmaOS Implementation**: `sigmaos::security::pledge::PledgeManager`.

### 2. Unveil(2) Filesystem Path Isolation
- **Path Whitelisting**: Processes restrict their filesystem view to explicitly unveiled paths with permissions (`r`, `w`, `x`, `c`).
- **Complete Path Concealment**: Non-unveiled directories return `ENOENT`, rendering unauthorized filesystem traversal impossible.
- **SigmaOS Implementation**: `sigmaos::security::unveil::UnveilManager`.

### 3. W^X (Write XOR Execute) & Memory Protections
- **Strict Page Permissions**: Pages may be writable or executable, but never both simultaneously.
- **Kernel Address Randomized Link (KARL)**: Relinking kernel binary objects on every boot into unique randomized layouts.
- **Guard Pages & Splay-Tree Malloc**: Guard pages around all heap allocations and randomized slot distributions.

### 4. Packet Filter (PF) & Cryptographic CSPRNG
- **Stateful Rule Engine**: Microsecond packet filtering, NAT, and ALTQ traffic shaping.
- **`arc4random` ChaCha20 Entropy Engine**: Fast, secure random number generation with automated kernel entropy re-seeding.

```rust
pub struct OpenBsdPledgePromises {
    pub stdio: bool,
    pub rpath: bool,
    pub wpath: bool,
    pub cpath: bool,
    pub inet: bool,
    pub dns: bool,
    pub proc: bool,
    pub exec: bool,
}

pub struct OpenBsdUnveilEntry {
    pub path: String,
    pub permissions: String, // e.g. "rwxc"
}
```

---

## 🛡️ Security Surface Comparison
| Mitigation | Standard Linux | OpenBSD | SigmaOS Sovereign Implementation |
| :--- | :--- | :--- | :--- |
| **Syscall Restriction** | Seccomp-BPF (Complex) | `pledge()` (Simple) | **`pledge()` + Rust Trait Typestates** |
| **Path Concealment** | Landlock (Varies) | `unveil()` (Built-in) | **`unveil()` with Zero-Allocation Lookups** |
| **Memory Safety** | Manual ASAN | W^X + Guard Pages | **Rust Lifetime Tracking + W^X + Guard Pages** |
