# Architecture Overview

SigmaOS is structured as four strictly separated layers. Each layer **only communicates with the one directly below it** — no layer can reach across to skip a level. This makes the attack surface well-defined and the system auditable.

---

## The Four-Layer Model

```
╔════════════════════════════════════════════════════════════════╗
║  LAYER 4 — USER                                                ║
║                                                                ║
║  SigmaOS Shell (React/Svelte UI)                               ║
║  Progressive Web Apps (PWAs)                                   ║
║  Zenith Desktop Environment                                    ║
║  AI Kits, Workspace Manager, Resource Monitor                  ║
╠═══════════════════════════╦════════════════════════════════════╣
║  LAYER 3 — BROWSER        ║  What talks to it:                 ║
║                           ║    ↑ Web apps via JS APIs          ║
║  Custom Chromium fork     ║    ↓ Native messaging daemons      ║
║  navigator.sigmaos.* APIs ║                                    ║
║  Capability gate          ║  How it's isolated:                ║
║  Multi-profile manager    ║    Chrome sandbox (renderer proc)  ║
╠═══════════════════════════╬════════════════════════════════════╣
║  LAYER 2 — DAEMONS        ║  What talks to it:                 ║
║                           ║    ↑ Chromium native messaging     ║
║  Go language services     ║    ↓ Kernel syscalls               ║
║  Unix socket HTTP APIs    ║                                    ║
║  IPC via sigma-bus        ║  How it's isolated:                ║
║  sigma-healthd, sigma-apid║    pledge + unveil + cgroup v2     ║
╠═══════════════════════════╬════════════════════════════════════╣
║  LAYER 1 — KERNEL         ║  What talks to it:                 ║
║                           ║    ↑ Syscalls from ring 3          ║
║  Freestanding microkernel ║    ↓ Physical hardware             ║
║  No glibc, no hosted libs ║                                    ║
║  MLFQ + SCHED_SOVEREIGN   ║  How it's protected:               ║
║  4-level paging + ASLR    ║    W^X enforcement, KASLR, CET     ║
║  pledge/unveil/AVC        ║    shadow stacks (Intel CET)       ║
╚═══════════════════════════╩════════════════════════════════════╝
```

---

## Layer 1 — The Kernel

The SigmaOS kernel is a **freestanding x86_64 binary** compiled with `-nostdlib -ffreestanding`. It contains zero glibc symbols. All runtime support comes from `klib/` (the kernel's own stdlib).

### Why freestanding?

A kernel linked against glibc inherits all of glibc's assumptions about the runtime environment (heap via `sbrk`, threads via `pthread`, signals via libc wrappers). A freestanding kernel owns all these mechanisms itself — it can implement them correctly for a bare-metal environment instead of fighting libc's assumptions.

### Kernel subsystems

```
kernel/
├── core/
│   ├── sched/          MLFQ + SCHED_SOVEREIGN real-time EDF class
│   ├── mm/             4-level paging, PMM bitmap, ASLR, W^X, THP
│   └── sigma_amnesic   RAM-only mode (Tails-inspired, no disk writes)
├── security/
│   ├── jail/           sigma_pledge, sigma_unveil, namespace isolation
│   ├── mac/            Access Vector Cache (SELinux-inspired O(1) MAC)
│   └── sigma_secboot   Secure Boot + TPM2 PCR seal/unseal
├── net/
│   └── sigma_firewall  Stateful packet filter + NAT + conntrack
├── fs/
│   ├── sigma_vfs       VFS layer — filesystem-agnostic read/write routing
│   ├── sigmafs/        Native CoW filesystem (Btrfs-inspired)
│   └── sigma_fstab     Hardened mount flags (MS_NOEXEC | MS_NOSUID | MS_NODEV)
├── drivers/
│   └── core/           Unified probe/remove/suspend driver model
└── kpatch/             Live kernel patching (kpatch-inspired)
```

### The Scheduler

Two scheduling classes coexist:

**MLFQ (default)** — 4 priority levels. New tasks start at level 0. CPU-bound tasks drop to lower levels. Interactive tasks stay high. A periodic boost every 50 ms prevents starvation. Suitable for desktop and server workloads.

**SCHED_SOVEREIGN (real-time, `release/rtos`)** — Earliest Deadline First (EDF) within the RT queue. Priority inheritance via `SovereignMutex` prevents unbounded priority inversion. Deadline miss detection logs to the audit ring. Tasks with priority ≥ 80 are automatically promoted to this class.

```
Priority queue (simplified):
  Level 0 [high]:  interactive tasks — keyboard input, UI events
  Level 1:         normal foreground tasks
  Level 2:         CPU-bound background jobs
  Level 3 [low]:   idle tasks, cleanup

Every 50 ms: all tasks boosted back to Level 0 (anti-starvation)
```

### Memory Layout

```
0xFFFFFFFF80000000+    Kernel higher half (mapped at boot)
0x00007FFF00000000     vDSO (random ASLR offset, 42-bit entropy)
0x0000700000000000     mmap base (random ASLR offset)
0x0000600000000000     heap base (random ASLR offset)
0x0000000000400000     ELF load base
0x0000000000000000     NULL guard page
```

ASLR entropy: 42 bits per region on x86_64. Every `exec()` picks fresh random offsets for stack, heap, mmap, and vDSO independently.

---

## Layer 2 — System Daemons

Go-language services bridge the kernel to the browser. Each daemon:
- Listens on a Unix socket (`/run/sigma/*.sock`)
- Exposes an HTTP API (no external network exposure)
- Runs under pledge + unveil constraints
- Reports health to sigma-healthd

### Core daemons

| Daemon | Socket | What it does | 
| --- | --- | --- | 
| `sigma-busd` | `/run/sigma/busd.sock` | IPC message bus (D-Bus replacement, capability-gated) | 
| `sigma-healthd` | `/run/sigma/healthd.sock` | Structured per-subsystem health endpoint | 
| `sigma-apid` | `/run/sigma/apid.sock` | gRPC management API (Talos-inspired) | 
| `sigma-watchdog` | `/run/sigma/watchdog.sock` | WDT petting + daemon liveness monitoring | 
| `sigma-metrics` | `/run/sigma/metrics.sock` | Prometheus-compatible `/metrics` exporter | 
| `sigma-netd` | `/run/sigma/netd.sock` | Network namespace management | 
| `sigma-power` | `/run/sigma/power.sock` | Battery, lid close/open, suspend/hibernate | 
| `sigma-telemetry` | `/run/sigma/telemetry.sock` | Opt-in PII-scrubbed anonymous telemetry | 
| `sigma-cloudsync` | `/run/sigma/cloudsync.sock` | E2E encrypted cloud file sync | 
| `sigmad-ai` | `localhost:17392` | TinyLlama inference (`/v1/complete`, `/v1/predict`) | 

### How daemons communicate with the kernel

```
Web app
  │ navigator.sigmaos.process.spawn("ffmpeg")
  ▼
Chrome native messaging → /run/sigma/process.sock
  │ HTTP POST /process/spawn { cmd: "ffmpeg", args: [...] }
  ▼
sigmad-process (Go)
  │ fork() + execve()
  │ sigma_pledge(STDIO | RPATH)        ← kernel enforces from here
  │ sigma_unveil("/tmp", RW)
  │ sigma_cgroup_enter("ffmpeg-job")
  ▼
ffmpeg runs in isolated cgroup + namespace
stdout → SSE → Web app
```

---

## Layer 3 — Browser Layer

The custom Chromium fork adds the `navigator.sigmaos` namespace directly to the JavaScript engine. This means web apps call `navigator.sigmaos.process.spawn()` the same way they call `navigator.geolocation.getCurrentPosition()` — it's a first-class browser API, not a polyfill.

### Capability enforcement

Every `navigator.sigmaos.*` call passes through the SigmaOS Extension before reaching the native messaging host. The extension holds a per-app capability set loaded from `manifest.json`. A call without the declared capability gets `PermissionDeniedError` — no IPC at all.

```javascript
// App manifest declared: "capabilities": ["process:spawn"]
// This succeeds:
const proc = await navigator.sigmaos.process.spawn("python3", ["-c", "print('hi')"]);

// This throws PermissionDeniedError (capability not declared):
const f = await navigator.sigmaos.window.create({ url: "https://example.com" });
```

### Native WebKit Windows

`navigator.sigmaos.window.create()` creates a **frameless floating widget** backed by a real native window (not a browser popup). Used by the Zenith Desktop panels, notifications, and picture-in-picture video.

---

## Layer 4 — User Layer

Everything the user sees is a web app. The **Zenith Desktop** is the flagship, demonstrating what's possible when the browser has full OS access:

- Silicon attestation (Kyber-1024 hardware binding)
- Persistent shard matrix (distributed storage)
- Neural UI Engine (TinyLlama powers natural-language system commands)
- Reactive system events (battery, network, display changes trigger UI updates instantly)

---

## Data Flow: Package Installation

To see all four layers working together, here's what happens when a user runs `sigma-pkg install vim`:

```
1. User: navigator.sigmaos.pkg.ensure("vim")
        [Layer 4 — web app calls sigmaOS API]

2. Extension: checkCapability("pkg:install")
        [Layer 3 — capability gate]

3. sigmad-apid: POST /pkg/install { name: "vim" }
        [Layer 2 — daemon validates, resolves deps]
   → sigma_acquire: downloads SHA-256+BLAKE2b verified package
   → sigma_ostree:  stages new deployment in /sysroot-pending/
   → atomic rename: /sysroot ↔ /sysroot-pending  (or reboot for OS pkgs)

4. sigma_cgroup: new vim process gets resource limits
   sigma_pledge:  vim pledged to "stdio rpath wpath"
   sigma_unveil:  vim unveiled /home/user/.vimrc:rw, /usr/share/vim:rx
        [Layer 1 — kernel enforces all constraints]
```

---

## OCI Workload Format

Every SigmaOS workload is an OCI Runtime Specification bundle. The `sigmaExtensions` block adds pledge/unveil/trust information:

```json
{
  "ociVersion": "1.0.2",
  "root": { "path": "rootfs", "readonly": false },
  "process": { "args": ["ffmpeg", "-i", "input.mp4", "output.webm"] },
  "sigmaExtensions": {
    "trustLabel": "untrusted",
    "pledgePromises": "stdio rpath wpath net dns",
    "unveilPaths": [
      "/home/user/videos:rw",
      "/sigma/lib/codec:rx"
    ],
    "cgroupName": "ffmpeg-transcode",
    "cgroupProfile": "SIGMA_CGROUP_UNTRUSTED"
  }
}
```

The pledge + unveil configuration in `sigmaExtensions` is enforced by the kernel — the daemon cannot override it even if compromised.

---

## Privilege Ring Separation

```
  RING 3 (userland)
  ─────────────────────────────────────────
  shell · apps · web applications
  pledge/unveil enforced at syscall boundary
  ASLR, W^X, shadow stacks

      │ syscall (int 0x80 / SYSCALL instruction)
      │ → sigma_pledge_check() on EVERY kernel entry
      ▼

  RING 0 (kernel)
  ─────────────────────────────────────────
  MLFQ + SCHED_SOVEREIGN scheduler
  4-level page tables + ASLR
  AVC (Access Vector Cache)
  pledge/unveil enforcement tables
  DTrace SDT probes (zero-cost when disabled)
  TCP/IP + firewall + conntrack
```

Every syscall entry point calls `sigma_pledge_check()`. If the calling process has pledged away the syscall class, it receives SIGABRT immediately — no partial execution.

---

## What Makes SigmaOS Different from Linux

| Aspect | Linux | SigmaOS | 
| --- | --- | --- | 
| Shell environment | Bash + X11/Wayland desktop | Chromium IS the shell | 
| Per-process syscall restriction | seccomp (requires BPF expertise) | `sigma_pledge()` — one call, declarative | 
| Per-process filesystem restriction | chroot (coarse) | `sigma_unveil()` — per-path, per-permission | 
| Package management | dpkg/rpm/pacman | OSTree atomic updates + staged rollout | 
| MAC policy | SELinux/AppArmor (complex policy files) | AVC + trust label matrix (O(1) decisions) | 
| Crypto | bolt-on (openssl CLI) | TLS 1.3 + Kyber-1024 + Dilithium3 built-in | 
| Build system | Autotools/Make/Meson mix | CMake + Kconfig + USE flags | 
| Kernel size | 30 million lines | Custom microkernel, minimal surface area | 

---

*See also: [Kernel Architecture](Kernel) · [Security Model](Security-Model) · [Networking](Networking) · [Building from Source](Building-from-Source)*
