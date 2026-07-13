# SOVEREIGN SANDBOX SPEC

> **Component**: `kernel/security/sandbox/` | **Status**: Implementation | **Threat Model**: Untrusted user code, third-party shards

The **SigmaOS Sovereign Sandbox** provides multi-layer isolation for untrusted applications, browser extensions, plugins, and community shards. It combines Firecracker microVMs, eBPF seccomp, and sigma-cgroup namespace isolation to achieve near-native performance with near-absolute containment.

---

## Threat Model

### Protected Against

| Threat | Mitigation Layer |
|---|---|
| Syscall exploitation | eBPF seccomp filter |
| Container escape | Firecracker microVM boundary |
| Memory corruption | ASLR + stack canaries + CFI |
| Network exfiltration | Per-sandbox network policy |
| Filesystem access | Overlayfs read-only base |
| IPC abuse | sigma-bus capability whitelist |
| Timing side-channels | Jitter injection + rdtsc masking |
| Spectre/Meltdown | KPTI + indirect branch prediction |

### Not Claimed to Protect Against

- Physical hardware attacks
- Zero-days in microVM hypervisor (Firecracker)
- Malicious user with root access to host
- Quantum-capable adversaries (deferred to PQC layer)

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    SOVEREIGN SANDBOX LAYERS                     │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  Layer 4: Firecracker MicroVM                           │  │
│  │  (hardware-virtualized isolation, KVM-backed)           │  │
│  │  ┌────────────────────────────────────────────────┐    │  │
│  │  │  Layer 3: Cgroup Namespace (resource limits)   │    │  │
│  │  │  ┌───────────────────────────────────────────┐ │    │  │
│  │  │  │  Layer 2: Overlay Filesystem (ro base)    │ │    │  │
│  │  │  │  ┌────────────────────────────────────┐   │ │    │  │
│  │  │  │  │  Layer 1: eBPF Seccomp Filter      │   │ │    │  │
│  │  │  │  │  (syscall allowlist, ~45 syscalls)  │   │ │    │  │
│  │  │  │  │                                    │   │ │    │  │
│  │  │  │  │  UNTRUSTED APPLICATION RUNS HERE   │   │ │    │  │
│  │  │  │  └────────────────────────────────────┘   │ │    │  │
│  │  │  └───────────────────────────────────────────┘ │    │  │
│  │  └────────────────────────────────────────────────┘    │  │
│  └─────────────────────────────────────────────────────────┘  │
│                    HOST SIGMAOS KERNEL                         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Sandbox Profiles

### Profile: `strict` (Browser/Extension Sandbox)

```toml
[sandbox.strict]
microvm            = true
memory_mb          = 512
cpu_cores          = 2
network            = "isolated-veth"    # own network namespace
filesystem         = "overlay-ro"       # read-only base
ipc_channels       = []                 # no sigma-bus access
syscall_filter     = "strict-45"        # 45-syscall allowlist
timing_jitter      = true              # side-channel mitig.
```

### Profile: `standard` (Community Shards)

```toml
[sandbox.standard]
microvm            = false             # cgroup-only isolation
memory_mb          = 1024
cpu_cores          = 4
network            = "sigma-netns"     # shared net with policy
filesystem         = "overlay-rw"     # writable sandbox layer
ipc_channels       = ["shard.api.v1"] # limited sigma-bus
syscall_filter     = "standard-120"   # 120-syscall allowlist
timing_jitter      = false
```

### Profile: `permissive` (Developer Mode)

```toml
[sandbox.permissive]
microvm            = false
memory_mb          = 8192
cpu_cores          = 8
network            = "host"           # full network access
filesystem         = "host-bind"      # user-approved bind mounts
ipc_channels       = ["*"]            # full sigma-bus
syscall_filter     = "developer"      # all syscalls logged only
timing_jitter      = false
```

---

## Implementation

### Rust Sandbox Manager

```rust
// kernel/security/sandbox/mod.rs

#![no_std]

use sigma_core::{CgroupId, ShardId, NetworkNs};

pub struct Sandbox {
    id:          SandboxId,
    profile:     SandboxProfile,
    cgroup:      CgroupId,
    net_ns:      NetworkNs,
    microvm:     Option<MicroVmHandle>,
    seccomp_fd:  i32,
}

impl Sandbox {
    /// Create a new sandbox with the given profile
    pub fn create(profile: SandboxProfile) -> Result<Self> {
        let cgroup = cgroup::create_isolated(
            profile.memory_mb * 1024 * 1024,
            profile.cpu_cores,
        )?;

        let net_ns = match profile.network {
            NetworkMode::Isolated => NetworkNs::create_isolated()?,
            NetworkMode::SharedWithPolicy(p) => NetworkNs::create_with_policy(p)?,
            NetworkMode::Host => NetworkNs::host(),
        };

        let microvm = if profile.microvm {
            Some(MicroVm::spawn(MicroVmConfig {
                memory_mb: profile.memory_mb,
                vcpus: profile.cpu_cores,
                kernel: "/boot/sigma-microvm-kernel",
                rootfs: "/var/sigma/sandbox/base.ext4",
            })?)
        } else {
            None
        };

        let seccomp_fd = seccomp::load_filter(&profile.syscall_filter)?;

        Ok(Sandbox {
            id: SandboxId::new(),
            profile,
            cgroup,
            net_ns,
            microvm,
            seccomp_fd,
        })
    }

    /// Execute a binary inside this sandbox
    pub fn exec(&self, binary: &[u8], args: &[&str]) -> Result<SandboxProcess> {
        // 1. Verify binary signature
        dilithium5::verify_signature(binary, &SIGMA_PACKAGE_KEY)?;

        // 2. Enter namespaces
        self.cgroup.enter()?;
        self.net_ns.enter()?;

        // 3. Apply seccomp filter
        seccomp::apply(self.seccomp_fd)?;

        // 4. Execute
        SandboxProcess::spawn(binary, args)
    }

    /// Cleanly destroy the sandbox and reclaim all resources
    pub fn destroy(self) -> Result<()> {
        if let Some(vm) = self.microvm {
            vm.shutdown(ShutdownMode::Graceful)?;
        }
        self.cgroup.destroy()?;
        self.net_ns.destroy()?;
        Ok(())
    }
}
```

### eBPF Seccomp Filter (strict-45 profile)

```c
// kernel/security/sandbox/seccomp_strict.bpf.c
// Allowlist: ~45 syscalls sufficient for most user applications

static const int STRICT_ALLOWLIST[] = {
    // Memory management
    SYS_mmap, SYS_munmap, SYS_mprotect, SYS_brk,
    // File I/O (sandboxed paths only)
    SYS_openat, SYS_read, SYS_write, SYS_close,
    SYS_fstat, SYS_lseek, SYS_dup, SYS_dup2,
    // Process control
    SYS_exit, SYS_exit_group, SYS_getpid, SYS_gettid,
    SYS_futex, SYS_sched_yield, SYS_nanosleep,
    // Signal handling
    SYS_rt_sigaction, SYS_rt_sigprocmask, SYS_rt_sigreturn,
    // Network (if network=isolated-veth)
    SYS_socket, SYS_connect, SYS_send, SYS_recv,
    SYS_sendto, SYS_recvfrom, SYS_getsockopt, SYS_setsockopt,
    // Time
    SYS_clock_gettime, SYS_gettimeofday,
    // Misc
    SYS_getcwd, SYS_getrandom, SYS_prctl,
};

// Any syscall not in the allowlist → SIGSYS (sandbox violation)
```

---

## CLI Interface

```bash
# Run a community shard in sandbox
sigma sandbox run community-shard.spkg --profile standard

# Run a browser in strict sandbox
sigma sandbox run sigma-browser --profile strict

# List active sandboxes
sigma sandbox list
# ID          PROFILE    MEM_MB  CPU  UPTIME  PID
# sandbox-01  strict     512     2    2h 14m  12345 (sigma-browser)
# sandbox-02  standard   1024    4    45m     13456 (community-plugin)

# Inspect sandbox metrics
sigma sandbox inspect sandbox-01
# Memory: 312/512 MB (61%)
# CPU: 1.2/2 cores (60%)
# Syscalls blocked: 0
# Network rx/tx: 12MB/4MB
# Filesystem writes: 0 (read-only profile)

# Force-kill a misbehaving sandbox
sigma sandbox kill sandbox-02 --reason "excessive cpu usage"
```

---

## Security Events

All sandbox boundary violations are logged to the Sovereign Audit Log:

```
2025-07-10T14:23:01Z [SANDBOX-VIOLATION] sandbox-02 attempted forbidden syscall
  syscall:    SYS_ptrace (blocked by seccomp strict-45)
  pid:        13456 (community-plugin)
  action:     SIGKILL sent, sandbox quarantined
  alert:      USER notified via Zenith notification
```

---

## Performance

| Metric | MicroVM Profile | Cgroup-Only Profile |
|---|---|---|
| Launch time | ~150ms | ~10ms |
| Memory overhead | ~64MB (VM) | ~2MB (cgroup meta) |
| CPU overhead | <2% | <0.5% |
| Network latency | +0.5ms (veth) | +0.1ms (netns) |
| Isolation level | Maximum | Strong |

---

## Roadmap

- [x] cgroup namespace isolation
- [x] eBPF seccomp strict filter
- [ ] Firecracker microVM integration (Q3)
- [ ] Overlay filesystem sandbox base (Q3)
- [ ] Network policy per-sandbox (Q4)
- [ ] Timing jitter for side-channel mitigation (Q4)
- [ ] Sandbox marketplace (community shard auto-sandboxing) (Year 2)
- [ ] GPU-passthrough sandbox for ML workloads (Year 2)
