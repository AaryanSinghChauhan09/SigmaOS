# Security Model

SigmaOS is designed with security as a first-class constraint, not an afterthought. This page documents the full trust model across all layers — from kernel memory hardening to per-process sandboxing to daemon management.

---

## 1. Capability System (navigator.sigmaos)

Every SigmaOS app must declare capabilities in `manifest.json` before any platform API call is permitted. See [App Manifest Format](App-Manifest) for the full capability reference.

---

## 2. sigma_pledge — Per-Process Syscall Restriction

**Inspired by: OpenBSD `kern_pledge.c`**

After a process calls `sigma_pledge(promises)`, any syscall outside the declared promise set triggers SIGABRT immediately and is logged to the audit ring. The restriction is irreversible — a process cannot expand its own promise set.

```c
// Promise bits
#define SIGMA_PROMISE_STDIO   (1ULL << 0)  // read/write/close/fstat
#define SIGMA_PROMISE_RPATH   (1ULL << 1)  // open/stat existing paths
#define SIGMA_PROMISE_NET     (1ULL << 4)  // socket/connect/send/recv
#define SIGMA_PROMISE_EXEC    (1ULL << 6)  // execve

// Pledge after startup — narrows permissions permanently
sigma_pledge(SIGMA_PROMISE_STDIO | SIGMA_PROMISE_RPATH);

// Any attempt to call socket() after this → SIGABRT + audit log entry
```

Source: `kernel/security/jail/sigma_pledge.cpp`
Tests: `tests/kernel/pledge/test_pledge_sigabrt.cpp` (real SIGABRT test, not a stub)

---

## 3. sigma_unveil — Per-Process Filesystem Restriction

**Inspired by: OpenBSD `kern_unveil.c`**

After calling `sigma_unveil_lock()`, any VFS operation on a path NOT in the unveil table returns `-ENOENT` — the path appears to not exist at all. Stronger than chroot because it applies per-path, not per-root.

```c
sigma_unveil(&ctx, "/sigma/data/app",  SIGMA_UV_READ | SIGMA_UV_WRITE);
sigma_unveil(&ctx, "/sigma/lib",       SIGMA_UV_READ | SIGMA_UV_EXEC);
sigma_unveil_lock(&ctx);
// /etc/shadow → ENOENT. /home → ENOENT. /proc/kcore → ENOENT.
```

Source: `kernel/security/mac/sigma_unveil.cpp`

---

## 4. Linux Namespace Isolation (sigma_jail)

**Inspired by: Bubblewrap (bwrap)**

Replaces the old 7-line printf stub. `sigma_jail_enter()` calls `unshare(2)` to create real Linux namespaces before `execve()`:

| Namespace | Flag | Effect | 
| --- | --- | --- | 
| PID | `CLONE_NEWPID` | Process cannot see other system processes | 
| Network | `CLONE_NEWNET` | Private network stack (loopback only by default) | 
| Mount | `CLONE_NEWNS` | Private filesystem view | 
| IPC | `CLONE_NEWIPC` | Private System V IPC and POSIX message queues | 
| UTS | `CLONE_NEWUTS` | Private hostname | 
| User | `CLONE_NEWUSER` | UID mapped to unprivileged host UID (65534 = nobody) | 

After namespace entry: drops all capabilities (`PR_SET_NO_NEW_PRIVS`), applies seccomp allowlist, then `pivot_root()` into the jail filesystem.

Source: `kernel/security/jail/sigma_namespace.cpp`

---

## 5. Secure Path Joining

**Inspired by: OCI runc `filepath-securejoin`**

`sigma_secure_join(root, unsafe_path, out, len)` resolves any path relative to a jail root and rejects symlink traversals that would escape it. Used by `sigma_unveil_check()`, `sigma_pivot_root()`, and `sigma_cgroup_create()`.

```c
char safe[256];
// Returns -1 (escape attempt) — never writes to out
sigma_secure_join("/sigma/jail", "../../etc/shadow", safe, sizeof(safe));

// Returns 0 — resolves to /sigma/jail/data/config.toml
sigma_secure_join("/sigma/jail", "data/config.toml", safe, sizeof(safe));
```

Source: `kernel/security/jail/sigma_securepath.cpp`

---

## 6. ASLR + W^X Enforcement

**Inspired by: HardenedBSD `kern_aslr.c`**

Every `exec()` generates a fresh random layout with 42-bit entropy per region:

```
Stack base:  random offset from 0x7FFFFFFFFFFF0000
Heap base:   random offset from 0x0000700000000000
mmap base:   random offset from 0x0000600000000000
vDSO base:   random offset from 0x00007FFF00000000
```

W^X enforcement: any `mmap()` with both `PROT_WRITE` and `PROT_EXEC` is denied with `-EPERM` and logged to the trace ring.

```bash
# Runtime tunables
sigma-sysctl security.aslr.enabled=1
sigma-sysctl security.aslr.entropy_bits   # read-only: 42
sigma-sysctl security.aslr.wx_enforcement=1
```

Source: `kernel/mm/sigma_aslr.cpp`

---

## 7. Access Vector Cache (AVC)

**Inspired by: SELinux `avc.c`**

MAC policy decisions are cached in a 512-slot hash table keyed on `(src_label, dst_label, operation)`. Cache hit returns in nanoseconds without re-evaluating policy. Cache is flushed on policy reload.

```
First call (src=browser, dst=kernel, op=NET): evaluates policy → cache miss → store
Every subsequent call with same triple: array lookup → nanosecond latency
```

Source: `kernel/security/mac/sigma_avc.cpp`

---

## 8. Zero-Trust Workload Identity

**Inspired by: SPIFFE/SPIRE**

Each process is assigned a SPIFFE URI: `spiffe://sigma.os/workload/<exec_path>`. Every capability check verifies identity, revocation status, and policy. Revocation is checked on **every** request, not just at authentication time.

```
[1719400234.512] ALLOW  pid=1042 spiffe=spiffe://sigma.os/workload/ffmpeg cap=fs:/tmp
[1719400234.514] REVOKE pid=1042 reason=manual_revocation
[1719400234.515] DENY   pid=1042  cap=fs:/tmp  reason=revoked
```

Source: `kernel/security/sigma_zerotrust.cpp` (revocation bug fixed in Round 1)

---

## 9. Cgroup v2 Resource Limits

**Inspired by: OCI runc `libcontainer/cgroups/fs2`**

Every daemon and workload runs inside a cgroup with explicit CPU, memory, PID, and I/O limits. If any limit write fails, the process is not started.

```c
sigma_cgroup_resources_t r = SIGMA_CGROUP_UNTRUSTED;  // 512MB, 2CPU, 128 PIDs
r.mem_limit_bytes = 256 * 1024 * 1024;
sigma_cgroup_create("my-workload", &r);
sigma_cgroup_enter("my-workload", child_pid);
```

Source: `userland/pkg/sigma_cgroup.cpp`

---

## 10. OCI Bundle Format

**Inspired by: OCI Runtime Specification**

All SigmaOS workloads use a standard OCI bundle (`config.json` + `rootfs/`). The `sigmaExtensions` block adds SigmaOS-specific pledge promises and trust labels alongside the standard OCI spec.

```json
"sigmaExtensions": {
  "trustLabel": "untrusted",
  "pledgePromises": "stdio rpath net dns",
  "unveilPaths": ["/sigma/data/zenith:rw", "/sigma/lib:rx"]
}
```

Example: `workloads/zenith-browser/config.json`

---

## 11. Cryptographic Attestation

**Zenith desktop only**

Kyber-1024 (KEM) generates a session keypair bound to the hardware fingerprint. Dilithium3 is used for package signatures (`sigma-manifest.toml`). Note: Kyber is for key exchange only — never for signatures.

---

## Threat Model Summary

| Threat | Mitigation | Source | 
| --- | --- | --- | 
| Malicious PWA calling system APIs | Extension capability gate | `background.js` | 
| Process escaping its sandbox | Namespace isolation + seccomp | `sigma_namespace.cpp` | 
| Symlink jail escape | `sigma_secure_join()` | `sigma_securepath.cpp` | 
| Buffer overflow in policy engine | `snprintf`/`sigma_strlcpy` everywhere | Round 1 fixes | 
| W^X bypass / ROP spraying | ASLR + W^X enforcement | `sigma_aslr.cpp` | 
| MAC policy O(n) bottleneck | Access Vector Cache O(1) | `sigma_avc.cpp` | 
| Revoked process continuing | ZeroTrust check on every call | `sigma_zerotrust.cpp` | 
| Fork bomb / memory exhaustion | cgroup v2 pids.max + memory.max | `sigma_cgroup.cpp` | 
| Stub crypto (zero-key encryption) | healthd surfaces at runtime | `sigmad/healthd` | 
| XSS in web shell | `textContent` DOM insertion | `web-shell/index.html` | 

---

*See also: [Architecture Overview](Architecture-Overview) · [API Reference](API-Reference) · [Contributor Roadmap](Contributor-Roadmap)*
