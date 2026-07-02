# Architecture Overview

SigmaOS is structured as four clearly separated layers. Each layer communicates only with the one directly below it.

```
┌─────────────────────────────────────────────────────────────┐
│                     USER LAYER                              │
│  SigmaOS Shell (React/Svelte) · PWAs · Browser Extensions  │
│  Workspaces · AI Kits · Resource Manager · Zenith Desktop   │
├─────────────────────────────────────────────────────────────┤
│                    BROWSER LAYER                            │
│  Custom Chromium Fork · SigmaOS Extension · Multi-profile  │
│  Native Messaging Host · Tab Suspension · SigmaOS APIs      │
├─────────────────────────────────────────────────────────────┤
│                    SYSTEM LAYER (Go Daemons)                │
│  sigmad-process · sigmad-clipboard · sigmad-bluetooth       │
│  sigmad-workspace · sigmad-window · sigmad-ai (TinyLlama)   │
│  sigma-apid (gRPC) · sigma-healthd · sigma-sysctl           │
├─────────────────────────────────────────────────────────────┤
│                    OS BASE LAYER                            │
│  Buildroot Linux · systemd · bubblewrap · seccomp           │
│  Sovereign Microkernel · Physical Hardware                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Layer 1: OS Base

The foundation is a **minimal Linux image built with Buildroot** (~50 MB). systemd is PID 1 on the Buildroot layer; on bare-metal, SigmaOS's own freestanding `sigma_init` is PID 1.

Key components:
- **Buildroot**: Stripped root filesystem — only what's needed to run Chromium and the Go daemons.
- **bubblewrap (bwrap)**: Real namespace isolation. PID, network, mount, IPC, UTS, and user namespaces. `sigma_namespace.cpp` replaced the old 7-line printf stub.
- **seccomp**: Syscall allowlist generated from the app's capability manifest and pledge promises.
- **Sovereign Microkernel**: Custom x86_64 freestanding kernel with MLFQ + SCHED_SOVEREIGN scheduler, VMM+ASLR, VFS, TCP/IP, DTrace probes, AVC, pledge/unveil.

---

## Layer 2: System Daemons

Go-language daemons bridge the kernel/OS to the browser via Chrome's native messaging protocol over Unix sockets.

| Daemon | Socket | Responsibility |
|---|---|---|
| `sigmad-process` | `/run/sigma/process.sock` | Spawn processes, PTY over WebSocket, cgroup enforcement |
| `sigmad-clipboard` | `/run/sigma/clipboard.sock` | Cross-app clipboard (Round 2) |
| `sigmad-bluetooth` | `/run/sigma/bt.sock` | BlueZ wrapper |
| `sigmad-workspace` | `/run/sigma/ws.sock` | Virtual workspace state |
| `sigmad-window` | `/run/sigma/window.sock` | Native frameless window lifecycle |
| `sigmad-hotplug` | `/run/sigma/hotplug.sock` | USB/storage events, safe eject |
| `sigmad-ai` | `localhost:17392` | TinyLlama inference (`/v1/complete`, `/v1/predict`) |
| `sigmad-fleet` | `localhost:17400` | Enterprise telemetry (SSE) |
| `sigma-apid` | `/run/sigma/apid.sock` | **gRPC management API** (Round 3 — Talos-inspired) |
| `sigma-healthd` | `/run/sigma/healthd.sock` | **Structured health endpoint** (Round 4 — CoreOS-inspired) |

**sigma-apid** (defined in `api/sigma.proto`) gives the `sigmactl` CLI full remote management: list services, install packages, rollback deployments, read/write sysctl, view pledge/unveil status, stream audit events. All calls are mTLS-authenticated and audit-logged.

**sigma-healthd** surfaces stub subsystems at runtime:
```
✓ zero-trust   ok      0 violations in 24h
✗ cryptfs      FAILED  derive_key() is a stub — filesystem NOT encrypted
✓ net-firewall ok      142 active flows
```

---

## Layer 3: Browser (Custom Chromium Fork)

- **`navigator.sigmaos` API** — full platform API for PWAs
- **SigmaOS Extension** — capability gate; permission state held in memory (not `chrome.storage`, which was the Bug #4 hang)
- **Multi-profile manager** — isolated Chromium profile per workspace
- **Tab suspension** — idle tabs frozen to reduce memory pressure
- **Native WebKit Windows** — `navigator.sigmaos.window.create()` for frameless floating widgets

---

## Layer 4: User Layer

Everything the user sees is a web app. The Zenith Desktop is the flagship: silicon attestation (Kyber-1024), shard matrix storage, Neural UI Engine, reactive system events, ASLR-backed process isolation for every spawned workload.

---

## Privilege Ring Separation

```
┌─────────────────────────────────┐
│       RING 3: USERLAND          │
│  shell · apps · web applications│
│  pledge/unveil enforced here    │
└──────────────┬──────────────────┘
               │  SYSCALL (int 0x80 / syscall)
               │  → sigma_pledge_check() on every entry
┌──────────────▼──────────────────┐
│       RING 0: KERNEL            │
│  MLFQ + SCHED_SOVEREIGN · VMM   │
│  ASLR+W^X · AVC · pledge/unveil │
│  TCP/IP · sigma_trace probes    │
└─────────────────────────────────┘
```

---

## OCI Workload Format

All SigmaOS workloads use the OCI Runtime Specification bundle format (`config.json` + `rootfs/`). The `sigmaExtensions` block adds pledge promises and unveil paths:

```json
"sigmaExtensions": {
  "trustLabel": "untrusted",
  "pledgePromises": "stdio rpath net dns",
  "unveilPaths": ["/sigma/data/zenith:rw", "/sigma/lib:rx"],
  "cgroupName": "zenith-browser"
}
```

See `workloads/zenith-browser/config.json` for a complete example.

---

## Data Flow: Web App Spawns a Process

```
PWA: navigator.sigmaos.process.spawn("ffmpeg", [...])
  │
  ▼ SigmaOS Extension: checkPermission("process:spawn")
  │   ↳ DENIED → PermissionDeniedError
  │   ↳ ALLOWED → native messaging host
  │
  ▼ sigmad-process (Go daemon)
    creates OCI bundle + config.json
    sigma_cgroup_create("ffmpeg-job", SIGMA_CGROUP_UNTRUSTED)
    sigma_jail_enter() → unshare(CLONE_NEWPID|CLONE_NEWNET|CLONE_NEWNS|...)
    sigma_pledge(SIGMA_PROMISE_STDIO | SIGMA_PROMISE_RPATH)
    sigma_unveil("/tmp", UV_READ|UV_WRITE); sigma_unveil_lock()
    execve("ffmpeg", args)
  │
  ▼ ffmpeg runs in isolated cgroup + namespace
    stdout/stderr → SSE → PWA
```

---

*See also: [Security Model](Security-Model) · [Kernel Architecture](Kernel) · [API Reference](API-Reference)*
