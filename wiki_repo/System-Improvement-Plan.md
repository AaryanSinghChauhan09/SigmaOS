# SigmaOS "Zenith" System Improvement Plan

Official engineering roadmap for performance scaling, memory optimization, architectural expansion,
and cryptographic optimization across the SigmaOS Zenith v15.0/15.1 microkernel lattice.

---

## 1. Executive Summary

To maintain undisputed superiority over monolithic operating systems, SigmaOS must continuously
optimize its low-level algorithms. This plan outlines targeted performance optimizations, zero-copy
abstractions, and post-quantum cryptographic speedups to scale the 600-shard mesh to maximum
throughput.

---

## 2. Kernel Performance Enhancements (O(1) Slab Compaction)

### Lock-Free Free-List Compaction
- Employs atomic compare-and-swap (CAS) loops to defragment active slabs in constant O(1) time
- Eliminates pause sweeps entirely
- Implementation: `klib/sigma_slab_lockfree.cpp`

### Core-Local Cache Affinity
- Dynamically maps core-local memory partitions to specific hardware threads
- Prevents NUMA cross-talk and bus saturation
- Implementation: `kernel/mm/sigma_numa_affinity.h`

### Microsecond Context Switching
- Streamlines Ring-0 to Ring-3 transition vectors
- Target: < 12 clock cycles for syscall dispatcher latency
- Implementation: `arch/x86_64/syscall_entry.asm`

```
Current Linux context switch:  500–2000 ns
PREEMPT_RT Linux:               80–200 ns
SigmaOS target:                  < 50 ns   (custom asm SYSCALL entry)
```

---

## 3. Storage Acceleration Layer (Zero-Copy Buffer Cache)

### Unified Buffer Cache (UBC)
- Integrates filesystem and virtual memory caches
- Enables direct DMA transfers from block controllers to user space without intermediate copies
- Implementation: `kernel/fs/sigma_ubc.h`

### Relativistic Journaling
- Circular log-structured ring buffers
- Transforms multiple directory writes into sequential disk sweeps
- Reduces write amplification on flash storage
- Implementation: `kernel/fs/sigmafs/sigma_journal.h`

### Pre-emptive Read-Ahead
- Analyzes sequential block access histories to fetch subsequent sectors before IO dispatch
- Adaptive: learns per-file access patterns via sigma-ai inference
- Implementation: `kernel/fs/sigma_readahead.cpp`

---

## 4. GPU Rendering Optimizations (Vulkan Ring Buffering)

### Triple-Buffered Compositor
- Pre-allocates Vulkan command queues to submit display updates concurrently
- No CPU render-lock waits
- Frame pipeline:
```
App render → sigma-display protocol → Vulkan command buffer (triple) → DRM/KMS → display
Target latency: 1 frame (8.3ms @ 120Hz)
```

### Vectorized Matrix Scaling
- Replaces standard loops with SIMD-vectorized floating-point math
- Desktop scaling updates rendered instantly
- AVX-512 on x86; NEON on ARM

### Zero-Alloc UI Styling
- Bypasses dynamic heap requests inside Sovereign Window Manager
- Static memory buffers cache window textures and styles
- Zero allocations on the hot render path

---

## 5. Post-Quantum Cryptographic Speedups (PQC Engine)

### Vectorized Kyber Operations

```
CRYSTALS-Kyber-1024 NTT performance:
  Reference C:      ~2,400 cycles/poly-mul
  AVX-512:            ~180 cycles/poly-mul   (13.3× speedup)
  ARM NEON:           ~420 cycles/poly-mul    (5.7× speedup)

Target throughput (KEM operations/sec):
  Reference C:          ~450,000 ops/sec
  SigmaOS AVX-512:    ~5,800,000 ops/sec
  SigmaOS NEON:       ~2,100,000 ops/sec
```

### Dilithium-5 Attestation Pipeline
- Asynchronous public key audits execute in the background
- System boots while cryptography checks run concurrently — no blocking
- Implementation: `crypto/SovereignDilithium5.cpp`

### Secure Shard Ring Buffers
- Pre-allocated circular rings for PQC key exchanges
- Removes heap allocation overhead in networking tools
- Zero-copy key material via DMA-BUF sharing

---

## 6. Unified API Expansion Roadmap

| Phase | Target Subsystem | Improvement Feature | Expected Benefit | 
| --- | --- | --- | --- | 
| Phase I | SovereignBoot | Async concurrent shard ignition | Boot time < 400 ms | 
| Phase II | SovereignVideo | SIMD-accelerated non-linear edits | 4× faster HEVC transcode | 
| Phase III | SovereignCloudFS | Encrypted multi-node block syncing | Zero-overhead distributed replication | 
| Phase IV | S-ERA / S-CCF | High-performance batch auditing | Real-time analysis for corporate registers | 

---

## 7. Quality Assurance & Fuzzing Strategies

### Lattice Fuzzing Pools
- Continuous input fuzzing across all 256 syscall vectors
- Detects edge-case boundaries before production
- Integration: AFL++ + libFuzzer hybrid

### Deterministic Regression Sweeps
- Strict structural validations after every branch merge
- Prevents regression drift
- CI gate: `make check-regressions` must pass on every PR

### PQC Cryptographic Verification
- Verifies Dilithium signatures across all active userland binaries
- Integrated into sigma-pkg install pipeline
- Every package verified before exec permission granted

---

## 8. Performance Benchmark Targets

| Metric | Ubuntu 24.04 | Fedora 41 | SigmaOS Target | 
| --- | --- | --- | --- | 
| Boot time (NVMe SSD) | 43 s | 9 s | **< 2 s** | 
| Idle RAM (desktop) | 847 MB | 900 MB | **< 150 MB** | 
| Context switch latency | ~1,000 ns | ~300 ns | **< 50 ns** | 
| Kyber-1024 ops/sec | N/A | N/A | **5.8 M ops/sec** | 
| Kernel CVE patch | Reboot | Reboot | **No reboot (kpatch)** | 
| App launch (cold) | 1.5 s | 1.2 s | **< 0.5 s** | 

---

## 9. Competitive Improvement Matrix (v15.0/15.1)

| Dimension | Competitor | Competitor USP | SigmaOS Status | SigmaOS Plan | 
| --- | --- | --- | --- | --- | 
| **Declarative Consistency** | NixOS | Immutable reproducible builds, transaction rollback | `SovereignRegistry` stubs | **SovereignRegistry + TimeMachine**: CRYSTALS-Dilithium signed JSON boot configs with journal-level rollback across the 600-shard lattice | 
| **Mathematical Throughput** | Clear Linux | Aggressively vectorized math, auto-tuned CFS | Shard-aware runqueues | **SIMD-Vectorized PQC Engines**: AVX-512 + NEON for Kyber/Dilithium acceleration | 
| **Forensic Integrity** | CAINE / Tails | Zero-trace RAM scrubbing, write-blocking | Ring-3 driver model + basic secure boot | **SovereignForensics**: Hardware-assisted page scrubbing on namespace termination; WORM audit registers | 
| **System Recovery** | RescueZilla | One-click GUI disk cloning, Btrfs restores | CLI `sigma_fsck` + checkers | **`sigma-recover`**: Encrypted local backup restores; partition verification at boot stage | 
| **Immutable Orchestration** | Fedora CoreOS | Container-native, ignition provisioning | Shard-level boundaries | **SovereignCluster + ASI**: Asynchronous Shard Ignition; write-once images; no hypervisor overhead | 
| **Desktop UX** | SteamOS / Solus | Custom compositor, gamepad integration | Zenith stubs + vanilla CSS | **SovereignThemeEngine + Vulkan**: Direct Vulkan triple-buffer compositor; zero-copy GPU composition | 

---

## 10. Three-Axis Improvement Plan

### Axis 1 — Algorithms & System Performance

**NUMA-Aware CFS Scheduling**
- Allocates execution threads to nearest physical CPU memory node
- Reduces cross-socket bus contention on multi-NUMA systems
- Path: `kernel/sched/sigma_numa.cpp` reads ACPI SRAT at boot

**Lock-Free Concurrency Primitives**
- CAS loops inside task scheduling queues
- Eliminates spinlock pauses under high-contention workloads
- Path: `klib/sigma_lockfree.h` — Michael-Scott queue + Treiber stack

**Microsecond Ring Transitions**
- Custom-optimized Assembly entry points for `SYSCALL` / `SYSRET`
- Target: < 12 clock cycles for context switch overhead
- Path: `arch/x86_64/syscall_entry.asm`

### Axis 2 — Code, Programs & System Customization

**Zero-Dependency Core**
- Compiles without GNU `libc` headers
- Custom: `sigma_memcpy`, `sigma_strlen`, slab allocator, no `malloc`/`free` in kernel paths
- Path: `klib/include/sigma_nanolib.h`

**Declarative Configuration Manager**
- Boots by parsing Dilithium-signed `Config.sigma` registry
- Configures: network adapters, memory segments, GPU shards, service topology
- Parser: `userland/ignite/sigma_ignite.cpp`

**Profile-Based Hot-Swap**
```bash
sigma-svc profile switch --to forensic --attest dilithium3
sigma-svc profile switch --to gaming
sigma-svc profile switch --to developer
sigma-svc profile switch --to container-host
# Each profile activates: MAC policy, service set, kernel parameters, cgroup slices
```

### Axis 3 — User Experience & Desktop GUI

**SovereignThemeEngine**
```
Traditional compositor path:
  App → X11/Wayland → compositor (wlroots) → DRM/KMS → display
  Latency: 3-8 frame delays, multiple buffer copies

SigmaOS Zenith compositor path:
  App → sigma-display protocol → Vulkan triple-buffer → DRM/KMS → display
  Latency: 1 frame max, zero-copy via DMA-BUF
```

**High-Contrast Screen Reader**
- AT-SPI2 accessibility tree with hardware audio output via sigma-audio
- Indian language TTS via sigma-bhashini (offline, 22 languages)
- No round-trip through speech-dispatcher
- WCAG 2.2 AA compliant

**Declarative UI Engine**
- UI configs defined as lightweight JSON schemas
- Users customize dashboard without touching C++ source
- Hot-reload: changes apply within 200 ms
- Path: `userland/gui/sigma_ui_engine.h`

---

*See also: [Competitive Gap Matrix](Competitive-Gap-Matrix) · [Zenith System Improvement Plan](Zenith-System-Improvement-Plan) · [Development Roadmap](Development-Roadmap) · [Gap Analysis](Gap-Analysis)*

---

## 11. 🔧 Technical Improvements

### Kernel Optimization

Efficient scheduling, memory management, and I/O handling are the foundation. A lean kernel reduces overhead and improves responsiveness across all SigmaOS profiles.

**Scheduling**

| Technique | Description | Implementation | 
| ----------- | ------------- | ---------------- | 
| MLFQ + MCS scheduler | Multi-Level Feedback Queue with MCS spinlocks for zero-contention runqueues | `kernel/core/sigma_sched.cpp` | 
| NUMA-aware placement | Threads allocated to the nearest physical CPU memory node; reads ACPI SRAT at boot | `kernel/sched/sigma_numa.cpp` | 
| Lock-free CAS queues | Compare-and-swap loops eliminate spinlock pauses under high-contention workloads | `klib/sigma_lockfree.h` | 
| Real-time policy | SCHED_DEADLINE + EDF for latency-sensitive workloads (audio, sensor, AI inference) | `kernel/sched/sigma_rt.cpp` | 
| cgroup v2 enforcement | Per-pod CPU/memory/IO quotas enforced in kernel path, not just CLI | `kernel/core/process/sigma_cgroup.c` | 

**Memory Management**

| Technique | Description | Implementation | 
| ----------- | ------------- | ---------------- | 
| Buddy allocator + slab | O(log n) physical page allocation with per-CPU slab caches | `kernel/core/sigma_mm.cpp` | 
| O(1) slab compaction | CAS-based lock-free free-list defragmentation without pause sweeps | `klib/sigma_slab_lockfree.cpp` | 
| Huge page support | 2 MB and 1 GB transparent huge pages for large working-set workloads | `kernel/mm/sigma_thp.cpp` | 
| Memory pressure events | Proactive reclaim notifies userland before OOM — no surprise kills | `kernel/mm/sigma_pressure.cpp` | 
| Zero-copy DMA paths | Block controller → userspace transfers without intermediate kernel buffers | `kernel/fs/sigma_ubc.h` | 

**I/O Handling**

| Technique | Description | Implementation | 
| ----------- | ------------- | ---------------- | 
| io_uring equivalent | Submission/completion ring buffers for async I/O with zero syscall overhead per op | `kernel/io/sigma_uring.cpp` | 
| Pre-emptive read-ahead | Learns sequential access patterns via sigma-ai; prefetches before user IO dispatch | `kernel/fs/sigma_readahead.cpp` | 
| NVMe multiqueue | Per-CPU submission queues to NVMe controller for lock-free storage I/O | `drivers/storage/sigma_nvme.cpp` | 
| Relativistic journaling | Log-structured circular ring journal; sequential writes on flash | `kernel/fs/sigmafs/sigma_journal.h` | 

---

### Hardware Abstraction

Expanding HAL coverage makes SigmaOS run on the widest range of silicon — from ₹3,000 Raspberry Pi boards to enterprise NUMA servers.

**SDF (Sovereign Driver Framework)**

The SDF is the core architectural differentiator: drivers run in Ring-3 userspace with capability-gated DMA access. A crashing driver cannot panic the kernel.

```
Traditional Linux driver:   driver crash → kernel panic → data loss
SigmaOS SDF driver:         driver crash → sigma-heal restarts it → zero data loss
```

| Driver Category | Priority | Target Hardware | 
| ---------------- | ---------- | ----------------- | 
| GPU / DRM/KMS | Critical | Intel i915, AMD amdgpu, VirtIO-GPU | 
| Wi-Fi 802.11ax | Critical | Intel iwlwifi, MediaTek mt7921, Realtek rtl8xxxu | 
| Bluetooth 5.3 | High | USB HCI, QCA, Intel AX series | 
| NVMe / AHCI | Done ✓ | All PCIe NVMe and SATA controllers | 
| USB xHCI | Done ✓ | USB 3.2 host controller | 
| ARM64 BSP | High | Raspberry Pi 4/5 (BCM2711/2712), JioBook, VisionFive 2 | 
| RISC-V | Medium | StarFive VisionFive 2, Milk-V Pioneer | 
| Neural accelerators | Future | Qualcomm Hexagon, Apple Neural Engine, Hailo-8 | 

**Hardware Profiling (sigma-dna)**

`sigma-dna` reads CPUID, DMI, ACPI, and PCI topology at boot to build a hardware profile. This powers:
- Automatic profile selection (embedded / desktop / server / gaming)
- Silicon-aware scheduler tuning (Atom vs. Core, Zen 3 vs. Zen 4)
- PGO (Profile-Guided Optimization) target selection at package build time

---

### Security Enhancements

Modern security is not a feature bolted on — it is the default execution environment.

**Sandboxing**

Every process in SigmaOS runs inside a capability sandbox from its first syscall. There is no "unsandboxed" execution mode in production profiles.

```
Process launch sequence:
  sigma-init spawns process
    → sigma-mac assigns MAC label from .sigma-policy
      → capability set derived from label
        → cgroup v2 slice enforced
          → seccomp filter applied
            → process executes in restricted namespace
```

| Mechanism | Description | Implementation | 
| ----------- | ------------- | ---------------- | 
| sigma-mac | Mandatory Access Control — policy-driven, AI-suggested rules | `kernel/security/sigma_mac.cpp` | 
| Capability sandbox | POSIX capabilities + custom sigma-caps for SDF/DMA access | `kernel/security/sigma_caps.h` | 
| Seccomp-BPF equivalent | Per-process syscall allowlist loaded at exec time | `kernel/security/sigma_seccomp.cpp` | 
| Namespace isolation | PID, mount, network, IPC, UTS per container — kernel-enforced | `kernel/core/process/sigma_namespace.cpp` | 

**Secure Boot Chain**

```
sigma-boot.efi (ML-DSA signed)
    └── Kernel ELF (ML-DSA signed + dm-verity)
        └── initramfs (hash-verified)
            └── root filesystem (dm-verity read-only)
                └── sigma-trustd verifies TPM2 PCR measurements
                    └── session key unsealed from TPM2
                        └── CryptFS decrypts user partition (Argon2id)
```

| Component | Status | Notes | 
| ----------- | -------- | ------- | 
| sigma-boot.efi UEFI loader | `[ ]` | Blocked by Phase 0 kernel work | 
| TPM2 PCR measurement chain | `[~]` | Header complete; needs EFI binary | 
| dm-verity root FS | `[~]` | Framework exists; build integration pending | 
| Argon2id CryptFS | `[ ]` | Issue #44 — currently returns zero bytes | 
| ML-DSA (FIPS 204) package signing | `[~]` | Dilithium header present; NIST final bindings missing | 

**Memory Protection**

| Feature | Description | Implementation | 
| --------- | ------------- | ---------------- | 
| KASLR | Kernel address randomization at every boot | `arch/x86_64/sigma_kaslr.cpp` | 
| Stack canaries | Compile-time `-fstack-protector-strong` on all kernel code | `Makefile` | 
| W^X enforcement | No page is simultaneously writable and executable | `kernel/mm/sigma_wxprotect.cpp` | 
| Shadow stack (CET) | Intel CET shadow stack support for ROP mitigation | `arch/x86_64/sigma_cet.asm` | 
| ASLR for userland | Full address space randomization for all processes | `kernel/core/sigma_mm.cpp` | 

**Post-Quantum by Default**

All new APIs, package signatures, and network connections use NIST PQC final standards. Legacy RSA/ECDSA accepted only in compatibility mode.

| Algorithm | NIST Standard | Use | 
| ----------- | --------------- | ----- | 
| ML-KEM-1024 | FIPS 203 | Key encapsulation (TLS, disk encryption) | 
| ML-DSA-87 | FIPS 204 | Package signing, boot chain attestation | 
| SLH-DSA-SHAKE-256 | FIPS 205 | Code signing (hash-based, no lattice assumptions) | 
| Hybrid X25519+ML-KEM | RFC draft | TLS 1.3 key exchange during transition period | 

---

### Modular Design

Loose coupling ensures any subsystem can be updated, replaced, or hot-patched without destabilizing the rest of the system.

**Shard Architecture**

SigmaOS is structured as a 600-shard lattice. Each shard is an independently loadable, versioned, capability-bounded unit.

```
Shard properties:
  ├── Versioned ABI contract (semver)
  ├── Dilithium-signed manifest
  ├── Capability declarations (what hardware/syscalls it needs)
  ├── Recovery handler (what sigma-heal does when it crashes)
  └── Dependency graph (loaded/unloaded in topological order)
```

**Live Patching (kpatch)**

The `sigma-kpatch` subsystem applies security patches to the running kernel without reboot:

```bash
sigma-pkg install sigma-kpatch-CVE-2026-XXXX
# → patch downloaded, Dilithium3-verified
# → sigma-kpatch applies function-level binary patch to live kernel
# → /proc/sigma/kpatch shows active patches
# No reboot. No downtime
```

**Hot-Swap Profiles**

```bash
sigma-svc profile switch --to forensic    # activates WORM audit, write-block mounts
sigma-svc profile switch --to gaming      # disables audit, enables Vulkan perf mode
sigma-svc profile switch --to developer   # enables debug symbols, relaxed MAC
sigma-svc profile switch --to container-host  # max cgroup enforcement, no GUI
```

Each switch reconfigures: MAC policy, service set, cgroup slices, kernel parameters, network policy — without rebooting.

---

## 12. 🌐 Ecosystem & Compatibility

### Driver Support

SigmaOS needs a broad driver matrix to be viable on real hardware. Current coverage and targets:

| Platform | Coverage | Gap | Target Phase | 
| ---------- | ---------- | ----- | ------------- | 
| x86_64 (Intel/AMD) | Good — NIC, NVMe, USB | GPU, Wi-Fi, Bluetooth | Phase 1–2 | 
| ARM64 (Pi 4/5) | Stubs only | Full BSP, GPU, wireless | Phase 5 | 
| RISC-V | Stubs only | Full BSP | Phase 5 | 
| Embedded (ARM Cortex-M) | Not started | sigma-ultra target | Phase 5 | 

**Linux driver compatibility bridge** (`sigma-linux-compat`) allows existing Linux kernel modules to run inside SDF sandboxes as a migration path while native SDF drivers are developed.

### Application Layer

SigmaOS exposes three API surfaces to application developers:

```
1. sigma-syscall ABI      — direct syscall interface (C/C++/Rust)
2. sigma-sdk              — high-level C++ SDK with profession-app bindings
3. sigma-web API          — browser-accessible JS API (24 Web API drivers)
```

**sigma-sdk features:**
- Zero-dependency: links against `klib/sigma_nanolib.h`, not GNU libc
- India Stack bindings: ABDM, GST, UPI, DigiLocker built into SDK
- PQC-first: all network calls use ML-KEM by default
- Profession contexts: `sigma_sdk_ca`, `sigma_sdk_doctor`, `sigma_sdk_farmer` pre-configure the right API set

**ABI Stability Policy**

Once a syscall or SDK function is marked `SIGMA_STABLE`, it must not change. This is enforced by CI:
```bash
make check-abi    # fails if SIGMA_STABLE symbol changes signature
```

**Developer Tooling Roadmap**

| Tool | Description | Status | 
| ------ | ------------- | -------- | 
| `sigma-gdb` | Source-level debugger for SigmaOS binaries | `[ ]` | 
| `sigma-perf` | Hardware PMU-based profiler (cycles, cache misses, branch mispredictions) | `[ ]` | 
| `sigma-strace` | Syscall tracer with PQC audit log output | `[~]` | 
| `sigma-sdk` CLI | Scaffold, build, sign, and publish `.spkg` | `[~]` | 
| Doxygen API docs | Auto-generated from all `.h` files | `[~]` | 
| `sigma-observatory` | Native Prometheus+Grafana equivalent in Zenith | `[ ]` | 

### Virtualization & Containerization

SigmaOS supports both containerization (via `sigma-pod`) and Type-1 hypervisor semantics (via `SovereignContainer`) without depending on Docker, containerd, or QEMU userspace.

**Container stack:**
```
sigma-pod run-native demo.spkg
  → sigma-pod-cli sends IPC to kernel orchestrator
    → kernel creates namespaces (PID, NET, MNT, IPC, UTS, USER)
      → cgroup v2 slice enforced (CPU/mem/IO limits)
        → dm-verity verifies .spkg image
          → process starts inside isolated environment
```

No Docker daemon. No containerd. No OCI registry dependency by default.

**Hypervisor (SovereignContainer):**

| Feature | Description | Status | 
| --------- | ------------- | -------- | 
| KVM acceleration | Hardware-assisted virtualization via KVM ioctl interface | `[~]` | 
| VirtIO device model | VirtIO-net, VirtIO-blk, VirtIO-GPU for guest VMs | `[~]` | 
| Live migration | VM state snapshot + transfer to another SigmaOS host | `[ ]` | 
| Nested virtualization | SigmaOS as a guest inside itself or other hypervisors | `[ ]` | 

---

## 13. 🖥️ User Experience

### Intuitive CLI

SigmaOS ships a coherent CLI surface — not a collection of independent tools.

```bash
# Everything flows through sigma-cli
sigma-cli profile show                    # current active profile
sigma-cli profile switch --to developer   # hot-swap profile
sigma-cli pkg install vim                 # install package
sigma-cli pkg update                      # update all packages
sigma-cli pod run demo.spkg               # run containerized app
sigma-cli health check                    # sigma-heal status report
sigma-cli backup now                      # trigger backup
sigma-cli net status                      # network + routing table
sigma-cli boot rollback                   # revert to last known-good boot
```

**sigma-sh (Sovereign Shell):**
- History with Dilithium-signed audit log (tamper-evident command history)
- Tab completion for all sigma-cli subcommands
- Inline India Stack shortcuts: `gst <amount>`, `upi <vpa> <amount>`
- Profile-aware prompt: shows active profile, PQC status, audit mode

### Documentation

Documentation is a first-class subsystem, not an afterthought:

| Resource | Location | Status | 
| ---------- | ---------- | -------- | 
| Wiki (300+ pages) | `wiki_repo/*.md` → GitHub Wiki | `[x]` | 
| Doxygen API reference | `Doxyfile` → `docs/api/html/` | `[~]` | 
| Man pages (sigma-cli tools) | `docs/man/` | `[~]` | 
| Getting started guide | `wiki_repo/Getting-Started.md` | `[x]` | 
| Kernel developer handbook | `wiki_repo/Kernel-Developer-Handbook.md` | `[x]` | 
| Stability playbook | `wiki_repo/Stability-Playbook.md` | `[x]` | 
| India Stack guide | `wiki_repo/Advanced-India-Features.md` | `[x]` | 
| Architecture whitepaper | `wiki_repo/ARCHITECTURE_WHITEPAPER.md` | `[x]` | 

**Doc-per-PR policy:** every subsystem PR is blocked by CI (`sigma_ci.yml`) until docs are updated. No undocumented changes merge to `main`.

### Community Tools

| Tool | Description | Status | 
| ------ | ------------- | -------- | 
| `sigma-monitor` | Real-time system stats (CPU, memory, I/O, network, PQC audit events) | `[~]` | 
| `sigma-secure` | Security posture dashboard — shows open CVEs, unsigned binaries, policy violations | `[~]` | 
| `sigma-observatory` | Prometheus-compatible metrics + Grafana-style Zenith dashboard | `[ ]` | 
| `sigma-pkg` | Package manager with dependency resolution + Dilithium3 verification | `[~]` | 
| `sigma-automation.sh` | Backup / update / recovery / wiki-sync automation engine | `[x]` | 
| Bug bounty program | `wiki_repo/BUG_BOUNTY.md` — paid rewards for valid CVE reports | `[x]` | 

---

## 14. 🚀 Future-Oriented Features

### AI/ML Integration

sigma-ai is a first-class OS service, not an optional add-on:

```
sigma-ai architecture:
  sigma-ai daemon (llama.cpp backend)
    ├── sigma-heal: crash analysis + hotfix suggestions
    ├── sigma-lex: Gazette of India parser + compliance updates
    ├── sigma-bhashini: offline ASR/TTS for 22 Indian languages
    ├── sigma-twin: digital twin simulation (IoT sensor data)
    └── sigma-fedlearn: federated learning coordinator
```

**On-device inference targets:**

| Model | Size | RAM needed | Use | 
| ------- | ------ | ----------- | ----- | 
| Sarvam-1 (Q4_K_M) | 4.1 GB | 4 GB | Hindi-English general assistant | 
| OpenHathi (Q4) | 3.8 GB | 4 GB | Hindi domain specialist | 
| Krutrim (Q5) | 7.2 GB | 8 GB | Multi-language India assistant | 
| Custom GST model | 120 MB | 256 MB | Sigma-accounts tax advice | 

**Hardware acceleration path:**
- AVX-512 (Intel/AMD) — 8–12× faster than scalar inference
- ARM SVE2 (Cortex-X4, Neoverse) — 6–9× speedup
- Qualcomm Hexagon DSP — target for sigma-ultra on JioPhone
- NPU/Neural accelerators — sigma-dna detects and routes inference to accelerator if present

**AI-enhanced kernel features:**
- Predictive scheduler: sigma-ai learns per-app CPU usage patterns, pre-warms scheduling state
- Adaptive read-ahead: per-file access prediction from LLM inference rather than heuristics
- sigma-ids anomaly detection: ML-based intrusion detection beyond signature matching

### Scalability

SigmaOS scales from sigma-ultra (16 MB RAM) to enterprise multi-node clusters:

```
sigma-ultra (16 MB):
  ├── USSD text mode
  ├── 5 core India Stack apps
  └── NavIC GPS, offline payments

sigma-standalone (512 MB):
  ├── Full CLI + Zenith desktop
  ├── All profession apps
  └── Local LLM (Sarvam-1)

sigma-server (8 GB+):
  ├── SovereignCluster orchestration
  ├── sigma-fleet device management
  └── Federated learning coordinator

sigma-cluster (N nodes):
  ├── SovereignCloudFS distributed block storage
  ├── sigma-mesh-compute national grid
  └── sigma-blockchain-lite DLT for govt records
```

**Horizontal scaling primitives:**

| Primitive | Description | Implementation | 
| ----------- | ------------- | ---------------- | 
| SovereignCloudFS | Encrypted multi-node block sync with zero-overhead replication | `net/sigma_cloudf.cpp` | 
| sigma-mesh-compute | National distributed computing grid over BharatNet | `net/sigma_mesh.cpp` | 
| Asynchronous Shard Ignition (ASI) | Parallel boot of independent shards — scales to 600 concurrent | `kernel/core/boot/sigma_boot.c` | 
| sigma-fleet | Manage 10,000+ devices from single console | `userland/tools/sigma_fleet.cpp` | 

### Energy Efficiency

Critical for sigma-ultra devices and rural/edge deployments running on solar or intermittent power:

**Power management stack:**

| Feature | Description | Implementation | 
| --------- | ------------- | ---------------- | 
| ACPI P/C-states | Full CPU frequency scaling and core parking | `kernel/power/sigma_power_manager.cpp` | 
| sigma-perf governor | Silicon-aware frequency policy — not just ondemand/powersave | `kernel/power/sigma_perf_governor.cpp` | 
| Wakeup source accounting | Every wakeup attributed to a process/driver — no mystery power drain | `kernel/power/sigma_wakeup.cpp` | 
| Suspend-to-RAM (S3) | Full system suspend with TPM2 state preservation | `kernel/power/sigma_suspend.cpp` | 
| Runtime PM | Per-device runtime power management — idle devices cut power automatically | `hal/sigma_rpm.cpp` | 
| Display DPMS | Aggressive display power management via DRM/KMS connector properties | `drivers/graphics/sigma_kms.cpp` | 

**Energy efficiency targets:**

| Scenario | Reference (Linux) | SigmaOS Target | 
| ---------- | ------------------ | ---------------- | 
| Idle (desktop, screen off) | ~4.5 W (laptop) | **< 2.5 W** | 
| sigma-ultra idle | ~0.8 W (Pi Zero) | **< 0.4 W** | 
| Video playback (1080p H.265) | ~8 W | **< 5 W** (HW decode) | 
| sigma-ai inference (7B Q4) | ~15 W | **< 10 W** (NPU routing) | 
| Server idle (2-socket EPYC) | ~120 W | **< 80 W** (NUMA-aware parking) | 

**Thermal management:**
- `sigma-thermal`: reads ACPI thermal zones + hardware temperature sensors
- Dynamic throttling before hitting thermal limits — avoids hard shutdowns
- Per-core thermal state shared with sigma-ai scheduler for predictive throttle avoidance

---

## 15. Implementation Priority Matrix (All Categories)

| Category | Feature | Blocks Boot | Priority | Phase | 
| ---------- | --------- | ------------- | ---------- | ------- | 
| **Kernel** | Scheduler (MLFQ) | Yes | 🔴 Critical | Phase 0 | 
| **Kernel** | Memory manager (buddy+slab) | Yes | 🔴 Critical | Phase 0 | 
| **Kernel** | Syscall dispatch (30 calls) | Yes | 🔴 Critical | Phase 0 | 
| **Kernel** | IRQ/APIC controller | Yes | 🔴 Critical | Phase 0 | 
| **Hardware** | VESA/GOP framebuffer | Yes (desktop) | 🔴 Critical | Phase 0 | 
| **Hardware** | GPU DRM/KMS | Yes (desktop) | 🟠 High | Phase 2 | 
| **Hardware** | Wi-Fi drivers | No | 🟠 High | Phase 1 | 
| **Hardware** | ARM64 BSP | No | 🟠 High | Phase 5 | 
| **Security** | Argon2id CryptFS (fix #44) | No | 🔴 Critical | Phase 1 | 
| **Security** | sigma-boot.efi + TPM2 | No | 🟠 High | Phase 4 | 
| **Security** | ML-DSA (FIPS 204) final | No | 🟠 High | Phase 4 | 
| **Modular** | sigma-kpatch live patching | No | 🟡 Medium | Phase 4 | 
| **Ecosystem** | sigma-repo-server | No | 🔴 Critical | Phase 1 | 
| **Ecosystem** | sigma-sdk + ABI policy | No | 🟠 High | Phase 2 | 
| **Ecosystem** | KVM/VirtIO hypervisor | No | 🟡 Medium | Phase 3 | 
| **UX** | Zenith compositor event loop | No | 🟠 High | Phase A | 
| **UX** | sigma-sh history + completion | No | 🟡 Medium | Phase B | 
| **UX** | sigma-observatory dashboard | No | 🟢 Low | Phase C | 
| **AI/ML** | sigma-ai llama.cpp backend | No | 🟠 High | Phase 2 | 
| **AI/ML** | sigma-heal AI analysis | No | 🟠 High | Phase 6 | 
| **Scalability** | SovereignCloudFS | No | 🟡 Medium | Phase 3 | 
| **Scalability** | sigma-fleet (10K devices) | No | 🟡 Medium | Phase 7 | 
| **Energy** | ACPI P/C-states | No | 🟠 High | Phase 1 | 
| **Energy** | sigma-ultra < 0.4 W idle | No | 🟡 Medium | Phase 5 | 

---

*See also: [Competitive Gap Matrix](Competitive-Gap-Matrix) · [Zenith System Improvement Plan](Zenith-System-Improvement-Plan) · [Development Roadmap](Development-Roadmap) · [Gap Analysis](Gap-Analysis) · [Security Model](Security-Model) · [Driver Framework](Driver-Development)*
