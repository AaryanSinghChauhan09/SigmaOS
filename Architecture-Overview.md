# Architecture Overview

SigmaOS is structured as four clearly separated layers. Each layer communicates only with the one directly below it, enforcing strict isolation boundaries.

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
├─────────────────────────────────────────────────────────────┤
│                    OS BASE LAYER                            │
│  Buildroot Linux · systemd · bubblewrap · seccomp           │
│  Sovereign Microkernel · Physical Hardware                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Layer 1: OS Base

The foundation is a **minimal Linux image built with Buildroot**. There is no traditional desktop environment, no display manager, no package manager visible to the user. The only process that matters at this level is systemd, which starts Chromium directly.

Key components:
- **Buildroot**: Produces a stripped Linux root filesystem (~50 MB). Only what's needed to run Chromium and the Go daemons is included.
- **systemd**: PID 1. Starts Chromium in kiosk mode and all `sigmad-*` daemons on boot.
- **bubblewrap (bwrap)**: Userland sandboxing tool. Every process spawned by a web app is wrapped in isolated Linux namespaces (PID, network, mount, user).
- **seccomp**: Syscall filtering layer. Limits the syscalls available inside each bwrap container to only what the app's capability manifest declares.
- **Sovereign Microkernel** (bare-metal mode): Custom x86_64 freestanding kernel with MLFQ scheduler, VMM with 4-level paging, VFS, TCP/IP stack, and shard manager.

---

## Layer 2: System Daemons

Go-language daemons that bridge the kernel/OS to the browser. Each daemon handles one domain and communicates with the Chrome extension via Chrome's native messaging protocol over a Unix socket.

| Daemon | Port / Socket | Responsibility |
|---|---|---|
| `sigmad-process` | `/run/sigma/process.sock` | Spawn processes, stream stdout/stderr via SSE, manage PTYs |
| `sigmad-clipboard` | `/run/sigma/clipboard.sock` | Shared clipboard across all web apps |
| `sigmad-bluetooth` | `/run/sigma/bt.sock` | BlueZ wrapper, device scan/pair/connect |
| `sigmad-workspace` | `/run/sigma/ws.sock` | Virtual workspace state (create, switch, close) |
| `sigmad-window` | `/run/sigma/window.sock` | Native frameless window lifecycle |
| `sigmad-hotplug` | `/run/sigma/hotplug.sock` | USB/storage device events and safe eject |
| `sigmad-ai` | `localhost:17392` | TinyLlama inference endpoint (`/v1/complete`, `/v1/predict`) |
| `sigmad-fleet` | `localhost:17400` | Enterprise telemetry streaming via SSE |

All daemons are **capability-gated**: the Chrome extension checks the app's declared capabilities before forwarding any request. An app without `caps: ["process:spawn"]` cannot call `sigmad-process`.

---

## Layer 3: Browser (Custom Chromium Fork)

The SigmaOS Chromium fork adds:

- **`navigator.sigmaos` API surface** — the full platform API available to web apps and PWAs.
- **SigmaOS Extension** — a privileged Chrome extension acting as the capability gatekeeper. It intercepts `navigator.sigmaos.*` calls, checks permissions, and forwards approved requests to the correct daemon via native messaging.
- **Multi-profile manager** — each user workspace gets an isolated Chromium profile (cookies, storage, extensions).
- **Tab suspension** — idle tabs are frozen to reduce memory pressure, similar to The Great Suspender but built in.
- **Native WebKit Windows** — `navigator.sigmaos.window.create()` spawns a frameless `BrowserWindow`-style floating widget that renders a URL outside the normal tab chrome.

---

## Layer 4: User Layer

Everything the user sees and interacts with is a web app.

- **SigmaOS Shell**: The React/Svelte desktop UI. Renders the workspace switcher, taskbar, notification center, and app launcher. Runs as the default Chromium start page.
- **PWAs**: Any progressive web app can be installed and run. SigmaOS adds platform APIs to PWAs that aren't available in a normal browser.
- **Zenith Desktop**: The experimental flagship desktop. Features silicon attestation (Kyber-1024), a persistent shard matrix for storage, dynamic theme adjustments via the Neural UI Engine, and automated responses to system events.

---

## Privilege Ring Separation

On bare-metal, SigmaOS enforces CPU privilege ring separation:

```
       ┌─────────────────────────────────┐
       │         RING 3: USERLAND        │
       │  shell · apps · web applications│
       └──────────────┬──────────────────┘
                      │  SYSCALL (int 0x80 / syscall)
       ┌──────────────▼──────────────────┐
       │         RING 0: KERNEL          │
       │  MLFQ Scheduler · VMM · VFS     │
       │  TCP/IP Stack · Device Drivers  │
       └─────────────────────────────────┘
```

Userland code cannot directly access hardware I/O ports or kernel memory. Any attempt triggers a General Protection Fault (Ring 3 → I/O) or Page Fault (Ring 3 → kernel memory), which the registered IDT handler catches and terminates the offending process.

---

## Data Flow: Web App Makes a System Call

```
PWA calls navigator.sigmaos.process.spawn("ffmpeg", [...args])
         │
         ▼
SigmaOS Extension (background.js)
  checks capability: "process:spawn" ∈ app manifest?
  YES → forward to native messaging host
  NO  → reject with PermissionDeniedError
         │
         ▼
Native Messaging Host → Unix socket → sigmad-process (Go)
         │
         ▼
sigmad-process calls bwrap with seccomp profile from capability list
         │
         ▼
ffmpeg runs inside isolated namespace
stdout/stderr streamed back via SSE → PWA receives output in real time
```

---

*See also: [Security Model](Security-Model) · [API Reference](API-Reference) · [Kernel](Kernel)*
