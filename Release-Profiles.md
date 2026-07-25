# SigmaOS Release Profiles

SigmaOS ships 8 purpose-built profiles compiled from a single shared codebase.
Each activates different kernel features via CMake feature flags.

## Profile Comparison

| Profile | Branch | Best For | GUI | RT | Min RAM |
|---------|--------|---------|-----|----|---------|
| **Standalone** | `release/standalone` | Developer laptops | ✅ Zenith | — | 512 MB |
| **Browser** | `release/browser` | Consumer / thin clients | ✅ Chromium | — | 256 MB |
| **Microkernel** | `release/microkernel` | Servers, research | ❌ | — | 8 MB |
| **Mobile** | `release/mobile` | ARM64 tablets, RPi | ✅ Touch | — | 256 MB |
| **RTOS** | `release/rtos` | Industrial, robotics | ❌ | ✅ EDF | 4 MB |
| **Dual-Boot** | `release/dual-boot` | Keep Windows/Linux | ✅ Zenith | — | 512 MB |
| **Cloud** | `release/cloud` | AWS/Azure VMs | ❌ | — | 128 MB |
| **Distributed** | `release/distributed` | Multi-node clusters | ❌ | — | 256 MB |

---

## Standalone Profile

**Target:** Developer workstations, power users.

**Key features:**
- Full Zenith desktop environment (C++ compositor)
- All profession tools pre-installed
- sigma-ai local LLM daemon
- Indian IME (Inscript + phonetic)
- sigma-pkg GUI package manager
- Auto-tiling window manager

**Build:**
```bash
make PROFILE=standalone iso
```

---

## Browser Profile

**Target:** Consumer devices, thin clients, Chromebooks.

**Key features:**
- `navigator.sigmaos.*` API for PWAs
- Zero-install packages (stream from repo)
- Boots to Chromium in < 3 seconds
- No traditional desktop stack

**Build:**
```bash
make PROFILE=browser iso
```

---

## Microkernel Profile

**Target:** Servers, hypervisors, research, security-critical nodes.

**Key features:**
- < 512 KB kernel image
- < 8 MB RAM footprint
- sigma-bus IPC
- Capability token passing
- No userland except sigma-sh
- Formal verification target (Coq/Frama-C)

**Build:**
```bash
make PROFILE=microkernel iso
```

---

## Mobile Profile

**Target:** Raspberry Pi 4/5, JioBook, ARM64 tablets.

**Key features:**
- ARM64 GIC interrupt controller
- P/C-state aware scheduler
- Touch-optimised Zenith UI
- NEON-accelerated Kyber
- sigma-ultra USSD text mode (Pi Zero)
- RISC-V support (planned)

**Build:**
```bash
make PROFILE=mobile ARCH=arm64 iso
```

---

## RTOS Profile

**Target:** Industrial control, robotics, CNC machines.

**Key features:**
- EDF (earliest-deadline-first) scheduler
- Priority inheritance protocol
- Bounded IRQ latency < 10 µs
- ROS 2 DDS middleware port
- sigma-twin real IoT sensor path
- PREEMPT_RT-style full preemption

**Build:**
```bash
make PROFILE=rtos iso
```

---

## Dual-Boot Profile

**Target:** Users who want to keep Windows or Linux.

**Key features:**
- EFI boot entry registration
- Partition layout auto-detector
- Windows NTFS read-only mount
- GRUB chainload fallback
- Non-destructive installer

**Build:**
```bash
make PROFILE=dualboot iso
```

---

## Cloud Profile

**Target:** AWS, Azure, GCP, BharatCloud government VMs.

**Key features:**
- Immutable root filesystem
- A/B partition atomic updates
- Container-first, no GUI overhead
- sigma-pod (cgroup + namespace)
- sigma-fleet MDM agent
- OpenTelemetry export

**Build:**
```bash
make PROFILE=cloud iso
```

---

## Distributed Profile

**Target:** Multi-node sovereign computing clusters.

**Key features:**
- ZeroNet mesh networking
- CRDT offline-first sync
- SovereignCloudFS
- sigma-mesh-compute scheduler
- Container orchestration
- DLT blockchain-lite

**Build:**
```bash
make PROFILE=distributed iso
```

---

*See also: [Branch-Guide](Branch-Guide) · [Development-Roadmap](Development-Roadmap) · [Feature-Roadmap](Feature-Roadmap)*
