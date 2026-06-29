# SigmaOS — Silicon Sovereignty

> A zero-dependency, browser-first operating system that boots straight to Chromium in under 3 seconds, runs the browser as the OS shell, and gives web apps direct access to raw Unix primitives.

---

## What is SigmaOS?

Most operating systems bolt a browser on top of a traditional desktop. SigmaOS flips this: **the browser IS the desktop**. The entire window manager, workspace system, and application launcher live inside Chromium. Native system calls — `spawn`, `pipe`, `mmap`, `/dev` files — are exposed to web apps through a capability-gated native messaging bridge.

Under the hood, a custom freestanding microkernel handles scheduling, memory management, and process isolation with no glibc dependency. The same kernel binary runs on x86_64, ARM64, and RISC-V.

```
Traditional OS:                    SigmaOS:
  Kernel                             Kernel (freestanding)
    └── Desktop environment            └── Go daemons (sigmad-*)
          └── Browser (one app)              └── Chromium (the shell)
                └── Web apps                        └── PWAs (the apps)
```

---

## Why SigmaOS Exists — The Problem It Solves

| Problem with Today's OSes | SigmaOS Answer | 
| --- | --- | 
| Web apps are second-class citizens — no access to real system primitives | `navigator.sigmaos.*` exposes `spawn`, `pipe`, `mmap`, `/dev` to any PWA | 
| Browsers are slow, memory-heavy, add 10+ sec to boot | SigmaOS boots *to* Chromium in under 3 sec — no desktop stack loading first | 
| Every app can see everything — no per-app filesystem restriction | `sigma_pledge` + `sigma_unveil` lock each process to exactly what it declared | 
| Crypto is bolt-on, hard to use correctly | Post-quantum (Kyber-1024 + Dilithium3) baked into TLS, package signing, and attestation | 
| Same OS image for laptops, servers, IoT — bloated everywhere | 8 purpose-built profiles from one shared kernel codebase | 

---

## Quick Architecture Sketch

```
┌────────────────────────────────────────────────────────────────┐
│  USER: PWAs, Zenith Desktop, Extensions, AI Kits               │
├────────────────────────────────────────────────────────────────┤
│  BROWSER: Custom Chromium + navigator.sigmaos.* APIs           │
├────────────────────────────────────────────────────────────────┤
│  DAEMONS: Go services (process, clipboard, window, AI, health)  │
├────────────────────────────────────────────────────────────────┤
│  KERNEL: Freestanding microkernel — no glibc, no hosted stdlib  │
│    ├── Scheduler (MLFQ + SCHED_SOVEREIGN real-time class)       │
│    ├── Memory (4-level paging, ASLR 42-bit, W^X enforcement)    │
│    ├── Security (pledge/unveil, AVC, namespace isolation)        │
│    ├── Network (TLS 1.3 + Kyber, DNS/DoH, DHCP, WPA3/SAE)      │
│    └── Filesystem (VFS layer, OSTree atomic updates)            │
└────────────────────────────────────────────────────────────────┘
```

Full diagram → [Architecture Overview](Architecture-Overview)

---

## Deployment Profiles

SigmaOS ships as **8 purpose-built profiles** compiled from a single shared codebase. Each activates different kernel features and daemon sets via CMake feature flags.

| Profile | Branch | Best For | What's Different | 
| --- | --- | --- | --- | 
| **Standalone** | `release/standalone` | Developer laptops | Full Zenith DE, sigma IDE, one-command installer | 
| **Browser** | `release/browser` | Consumer / thin clients | `navigator.sigmaos.*` API, zero-install packages | 
| **Microkernel** | `release/microkernel` | Servers, research, hypervisors | No GUI overhead, minimal ring-0 binary | 
| **Mobile** | `release/mobile` | ARM64 / RISC-V tablets | Adaptive P/C-state scheduling, touch UI | 
| **RTOS** | `release/rtos` | Industrial control, robotics | `SCHED_SOVEREIGN` hard real-time EDF class | 
| **Dual-Boot** | `release/dual-boot` | Users keeping Windows/Linux | Multiboot2, GRUB chain-load, NTFS read driver | 
| **Cloud** | `release/cloud` | AWS / Azure / GCP VMs | Immutable root, A/B partition rollback, no GUI | 
| **Distributed** | `release/distributed` | Multi-node clusters | ZeroNet mesh, CRDT sync, container orchestration | 

→ [Branch Guide](Branch-Guide) for detailed per-profile feature lists

---

## Key Design Decisions (and why)

### Why a custom kernel?
Linux is 30 million lines. SigmaOS needs a kernel small enough to audit completely, boot in < 3 seconds, and ship on resource-constrained embedded targets. The freestanding binary (`-nostdlib -ffreestanding`) has zero glibc symbols.

### Why Go daemons?
Go gives safe memory management, built-in goroutine concurrency, and easy Unix socket servers. Every daemon exposes an HTTP API on a Unix socket — `curl --unix-socket /run/sigma/healthd.sock /health` works from any shell.

### Why the browser as shell?
Web technologies compose better than native widgets. The entire desktop is hot-reloadable. Any developer who knows HTML/CSS/JS can build a SigmaOS app without learning a native toolkit.

### Why post-quantum crypto now?
NIST standardised Kyber-1024 (FIPS 203) and Dilithium3 (FIPS 204) in 2024. Harvest-now-decrypt-later attacks are real — data encrypted today with classical crypto can be broken retroactively once quantum computers scale. We bake PQC in from day one.

---

## Feature Matrix (by profile)

| Feature | main | standalone | browser | rtos | cloud | distributed | 
| --- | --- | --- | --- | --- | --- | --- | 
| MLFQ Scheduler | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | 
| SCHED_SOVEREIGN (RT EDF) | ~ | ~ | — | ✓ | — | — | 
| 4-level paging + ASLR | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | 
| W^X enforcement | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | 
| pledge / unveil | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | 
| Namespace isolation (bwrap) | ✓ | ✓ | ✓ | — | ✓ | ✓ | 
| Zenith Desktop | ~ | ✓ | ✓ | — | ~ | — | 
| navigator.sigmaos API | ✓ | ✓ | ✓ | — | ~ | ~ | 
| TLS 1.3 + Kyber-1024 | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | 
| DNS-over-HTTPS + DNSSEC | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | 
| WPA3/SAE WiFi | ✓ | ✓ | ✓ | ✓ | — | — | 
| Immutable root (A/B) | — | — | — | — | ✓ | ✓ | 
| OSTree atomic updates | — | ✓ | ✓ | — | ✓ | ✓ | 
| Container orchestration | ~ | ✓ | ✓ | — | ✓ | ✓ | 
| Distributed VFS / CRDT | — | — | — | — | ~ | ✓ | 
| AI scheduler (TinyLlama) | ✓ | ✓ | ✓ | — | ✓ | ✓ | 

`✓` = present · `~` = partial/optional · `—` = not applicable for this profile

---

## 60-Second Quick Start

```bash
# 1. Clone
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# 2. Install build dependencies (Ubuntu 22.04)
sudo apt install -y build-essential nasm cmake qemu-system-x86 golang-go

# 3. Build + boot in QEMU
make clean && make all -j$(nproc)
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -serial stdio

# 4. Check for implementation stubs
make check-stubs
```

Full guide → [Building from Source](Building-from-Source)

---

## Engineering Roadmap

```
Phase 1 — Boot & HAL ............ ✓ COMPLETE
  ✓ Freestanding x86_64 kernel (no glibc)
  ✓ PID 1 signalfd event loop (infinite — fixed 5-iteration bug)
  ✓ IDT with 32 ISR stubs + hardware IRQ vectors
  ✓ Multi-arch HAL stubs (ARM64, RISC-V)

Phase 2 — Security (complete)
  ✓ sigma_pledge (per-process syscall restriction)
  ✓ sigma_unveil (per-process filesystem restriction)
  ✓ Namespace isolation — real unshare/pivot_root/seccomp
  ✓ ASLR 42-bit per-region + W^X enforcement
  ✓ AVC (O(1) MAC policy cache, SELinux-inspired)
  ✓ Zero-trust SPIFFE workload identities
  ✓ CryptFS — AES-256-GCM + TPM2 key unsealing (Issue #44 fixed)

Phase 3 — Network (complete)
  ✓ TLS 1.3 + X25519/Kyber-1024 hybrid key exchange
  ✓ DNS resolver — UDP/TCP/DoH + DNSSEC + LRU cache
  ✓ DHCP client — full RFC 2131/2132 state machine
  ✓ WPA3/SAE — dragonfly key exchange (P-256)
  ✓ Stateful firewall + NAT + conntrack

Phase 4 — System Services (in progress)
  ✓ sigma-healthd (CoreOS-inspired structured health)
  ✓ sigma-watchdog (hardware WDT + daemon liveness)
  ✓ sigma-metrics (Prometheus-compatible /metrics)
  ✓ sigma-telemetry (opt-in, PII-scrubbed)
  ✓ sigma-cloudsync (E2E encrypted, Argon2id)
  □ Ext4 JBD2 ordered journaling (planned)
  □ NVMe / e1000 production drivers (planned)

Phase 5 — Desktop & Tooling (planned)
  □ Zenith native C++ compositor (replacing JS prototype)
  □ Sigma Shell full POSIX scripting
  □ Graphical installer (Calamares equivalent)
  □ Signed .spkg registry with BLAKE2b + Dilithium3
```

---

## Rounds of Improvements

SigmaOS is developed in iterative improvement rounds, each inspired by a real production OS:

| Round | Theme | Key Addition | 
| --- | --- | --- | 
| 1 | Bug fixes | PID 1 loop, buffer overflows, CI tests | 
| 2 | OpenBSD / Gentoo | pledge, unveil, USE flags, staged rollout | 
| 3 | OSTree / Talos | atomic updates, namespace isolation, gRPC API | 
| 4–5 | HardenedBSD / SELinux | ASLR+W^X, AVC, DTrace, cgroup v2 | 
| 6–7 | seL4 / Genode | MCS scheduler, capability space, Dilithium fix | 
| 8 | dm-verity / snapd | verified boot, package assertions, SemanticFS | 
| 9 | Comprehensive | sigma-bus IPC, audio, session, driver framework | 
| 10–12 | SMP / eBPF / drivers | LAPIC, ACPI, eBPF VM, AHCI, LVM, India apps | 
| 13–14 | Compositor / roadmap | Wayland compositor, CryptFS fix, roadmap SPA | 
| 15–17 | Protocols | Full TLS 1.3, DNS, DHCP, firewall, shell, OOM | 

Full history → [Improvements Overview](Improvements-Overview)

---

## Wiki Navigation

| Section | Pages | 
| --- | --- | 
| **Getting Started** | [Building from Source](Building-from-Source) · [Branch Guide](Branch-Guide) · [FAQ](FAQ) | 
| **Architecture** | [Architecture Overview](Architecture-Overview) · [Kernel](Kernel) · [HAL](HAL) · [Networking](Networking) | 
| **Security** | [Security Model](Security-Model) · [Post-Quantum Crypto](Post-Quantum-Security) | 
| **Development** | [Developer Guide](Developer-Guide) · [Utilities Roadmap](Utilities-Roadmap) · [Contributor Roadmap](Contributor-Roadmap) | 
| **API** | [navigator.sigmaos API](API-Reference) · [Syscall Dispatcher](Syscall-Dispatcher) · [App Manifest](App-Manifest) | 
| **Profiles** | [Release Profiles](Release-Profiles) · [Zenith Desktop](Zenith-Desktop) · [Branch Guide](Branch-Guide) | 
| **Operations** | [System Daemons](System-Daemons) · [Improvements Overview](Improvements-Overview) · [Feature Roadmap](Feature-Roadmap) |
