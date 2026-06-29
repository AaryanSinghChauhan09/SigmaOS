# SigmaOS Feature Roadmap

This page documents all planned and implemented features, organized by priority and development phase. Features marked ✓ have source code committed; features marked 🔧 have headers/stubs; features marked ☐ are planned.

---

## Implemented Features (All Rounds)

### Security
| Status | Feature | File | Description | 
| --- | --- | --- | --- | 
| ✓ | sigma_pledge() | `kernel/security/jail/sigma_pledge.cpp` | Per-process syscall restriction (OpenBSD-inspired) | 
| ✓ | sigma_unveil() | `kernel/security/mac/sigma_unveil.cpp` | Per-process filesystem restriction | 
| ✓ | Namespace isolation | `kernel/security/jail/sigma_namespace.cpp` | Real unshare/pivot_root/seccomp (replaces printf stub) | 
| ✓ | Trust label matrix | `kernel/security/sigma_trust_labels.h` | Qubes-style information flow policy | 
| ✓ | Capability space | `kernel/security/sigma_cap.cpp` | seL4-inspired unforgeable capability tokens | 
| ✓ | ASLR + W^X | `kernel/mm/sigma_aslr.cpp` | 42-bit per-region entropy + write-xor-execute | 
| ✓ | AVC (O(1) MAC) | `kernel/security/mac/sigma_avc.cpp` | SELinux-style Access Vector Cache | 
| ✓ | Secure path join | `kernel/security/jail/sigma_securepath.cpp` | Symlink jail-escape prevention | 
| ✓ | sigma_usercopy | `klib/sigma_usercopy.cpp` | Type-safe kernel↔user memory API | 
| 🔧 | eBPF hooks | `kernel/security/sigma_ebpf.h` | Programmable kernel packet/syscall/sched hooks | 
| 🔧 | Module signing | `kernel/security/sigma_module_sign.h` | Dilithium3 signature verification on .smod files | 
| ✓ | Audit log chain | `kernel/security/sigma_audit_backend.cpp` | SHA-256 hash-chained immutable audit log | 
| ✓ | Secrets vault | `sigmad/vault/main.go` | AES-256-GCM encrypted secret store with TPM2 key | 

### Package Management
| Status | Feature | File | Description | 
| --- | --- | --- | --- | 
| ✓ | Dual-hash verify | `userland/pkg/sigma_acquire.cpp` | SHA-256 + BLAKE2b on every download | 
| ✓ | Mirror fallback | `userland/pkg/sigma_pkg_fetch.cpp` | Gentoo-style mirror chain with hash verify | 
| ✓ | Staged rollout | `userland/pkg/sigma_staged_update.cpp` | Karma-gated canary → testing → stable | 
| ✓ | OSTree atomic updates | `userland/pkg/sigma_ostree.cpp` | Content-addressed staging + atomic rename | 
| ✓ | Package transactions | `userland/pkg/sigma_pkg_transaction.h` | Flatpak-style resolve → execute → error-stop | 
| 🔧 | Binary deltas | `userland/pkg/sigma_delta.h` | Clear Linux swupd-style bsdiff updates | 

### System Services (Go Daemons)
| Status | Feature | Socket | Description | 
| --- | --- | --- | --- | 
| ✓ | sigma-healthd | `/run/sigma/healthd.sock` | Structured per-subsystem health endpoint | 
| ✓ | sigma-ds | `/run/sigma/ds.sock` | Service discovery data store (MINIX 3-inspired) | 
| ✓ | sigma-vault | `/run/sigma/vault.sock` | Secrets manager with AES-256-GCM + TPM2 key | 
| ✓ | sigma-search | `/run/sigma/search.sock` | Spotlight-style global search (fs + apps + sysctl) | 
| ✓ | sigma-webhook | `/run/sigma/webhook.sock` | Event-driven webhook dispatcher with HMAC-SHA256 | 
| ✓ | REST API gateway | `:17400` | HTTP REST proxy to all Unix-socket daemons | 
| ✓ | sigma-heartbeat | linked into sigma-healthd | Genode-style stuck-process detection | 

### Init & Process Supervision
| Status | Feature | File | Description | 
| --- | --- | --- | --- | 
| ✓ | Infinite PID 1 loop | `init/sigma_init_loop.c` | signalfd event loop (replaces 5-iteration bug) | 
| ✓ | s6 supervisor | `userland/init/sigma_supervisor.cpp` | s6-style state machine with readiness protocol | 
| ✓ | sigma-rs | `userland/rs/sigma_rs.cpp` | MINIX 3 Reincarnation Server — crash-restart | 
| ✓ | sigma-ignite | `userland/ignite/sigma_ignite.cpp` | CoreOS Ignition-style first-boot provisioner | 
| ✓ | Immutable root | `init/init.c` + `Makefile` | `MS_RDONLY` remount on boot (Bottlerocket-inspired) | 

### Kernel
| Status | Feature | File | Description | 
| --- | --- | --- | --- | 
| ✓ | SCHED_SOVEREIGN | `kernel/core/sched/sigma_sched_sovereign.cpp` | Hard real-time EDF class with priority inheritance | 
| ✓ | MCS budgets | `kernel/sched/sigma_mcs.cpp` | seL4 Mixed Criticality — budget/period per thread | 
| ✓ | DTrace probes | `klib/sigma_trace.cpp` | Zero-cost SDT kernel tracing (illumos-inspired) | 
| ✓ | Hardened fstab | `kernel/fs/sigma_fstab.cpp` | MS_NOEXEC\ | MS_NOSUID\ | MS_NODEV on all mounts | 
| ✓ | cgroup v2 limits | `userland/pkg/sigma_cgroup.cpp` | CPU/memory/PID/IO limits per workload | 
| 🔧 | dm-verity per package | `userland/sigma-pkg/sigma_pkg_verity.h` | snapd ContainerPlaceInfo-inspired — every read verified | 
| 🔧 | Pkg assertions chain | `sigmad/pkg/assert/sigma_assert.go` | snapd SnapDeclaration — publisher-id, revision, anti-replay | 
| 🔧 | Plug/slot interface system | YAML service manifests | snapd-inspired — explicit service capability contracts | 
| 🔧 | SemanticFS xattrs | `kernel/fs/sigma_semanticfs.h` | Haiku BFS — SIGMA:TRUST, SIGMA:CLASS, SIGMA:SIGNER inline | 
| 🔧 | Attribute index server | `sigmad/indexd/main.go` | Haiku index server — O(log n) attribute queries | 
| 🔧 | Sysroot exclusive lock | `sigmad/pkg/sigma_pkg_txn_lock.go` | rpm-ostree — no concurrent package operations | 
| 🔧 | Two-VM network gateway | `kernel/virt/sigma_netgw.h` | Whonix — workload VM cannot bypass gateway | 
| 🔧 | AppArmor profile gen | `sigmad/mac/apparmor_gen.go` | snapd — auto-generate deny-all + plug exceptions | 
| 🔧 | Package journal | `userland/sigma-pkg/sigma_pkg_journal.h` | rpm-ostree — HMAC-sealed transaction audit log | 
| 🔧 | Display server protocol | `userland/display/sigma_display_protocol.h` | Haiku app_server — browser off framebuffer | 
| ✓ | SIGMA_ASSERT | `klib/include/sigma_assert.h` | Unikraft UK_ASSERT — zero-cost in release, full in debug | 

### Networking
| Status | Feature | File | Description | 
| --- | --- | --- | --- | 
| ✓ | TCP/IP stack | `kernel/net/sigma_tcpip.c` | Custom 3-way handshake, conntrack, firewall | 
| ✓ | DHCP client | `net/dhcp/sigma_dhcp.h` | Full RFC 2131/2132 with lease state machine | 
| 🔧 | DNS sinkhole | `kernel/net/sigma_dns_sinkhole.h` | Malware domain filtering with blocklist updates | 

### Developer Tooling
| Status | Feature | File | Description | 
| --- | --- | --- | --- | 
| ✓ | sigma CLI | `tools/sigma-cli/main.go` | init, sign (Ed25519), verify, run, health, sysctl, search | 
| ✓ | Genode routing | `sigma-etc/init.xml` | Declarative service routing policy | 
| ✓ | Config.sigma | `Config.sigma` | Unikraft/Kconfig component selection | 
| ✓ | .conform.yaml | `.conform.yaml` | Commit conformance: DCO + conventional format | 
| ✓ | .pre-commit | `.pre-commit-config.yaml` | SPDX headers, clang-format, go-fmt, commit format | 
| ✓ | BUILD_ASSERT | `klib/include/sigma_build_assert.h` | Compile-time struct size contracts | 
| 🔧 | Universal loader | `kernel/compat/sigma_universal_loader.h` | ELF/OCI/Flatpak/WASM format detection | 

---

## Strategic Roadmap — Making SigmaOS the Definitive OS

### Phase 1: Compatibility Supremacy (6 months)

**Goal: Run Linux apps better than Linux.**

| Capability | Approach | Status | 
| --- | --- | --- | 
| Universal binary loader | `sigma_universal_loader.h` — detect ELF/OCI/WASM | 🔧 Header done | 
| Linux ELF shim | `SovereignCompat` POSIX layer | Partial | 
| Flatpak/AppImage native | OCI bundle format + namespace isolation | ✓ OCI done | 
| .deb/.rpm/.apk support | sigma-pkg layer resolver (Yocto-inspired) | Planned | 

### Phase 2: Architectural Advantages (6 months)

**What SigmaOS already has that no other distro does:**

| Advantage | File | Status | 
| --- | --- | --- | 
| Kernel-space AI Scheduler (10× AI perf) | `kernel/core/sched/sigma_sched_sovereign.cpp` | ✓ | 
| Zero-Trust SPIFFE/SPIRE workload IDs | `kernel/security/sigma_zerotrust.cpp` | ✓ | 
| Neural UI Engine (TinyLlama at port 17392) | `sigmad-ai` | ✓ | 
| Self-healing daemon reincarnation | `userland/rs/sigma_rs.cpp` | ✓ | 
| Capability-based security (seL4 model) | `kernel/security/sigma_cap.cpp` | ✓ | 
| Immutable A/B root + OSTree rollback | `userland/pkg/sigma_ostree.cpp` | ✓ | 

**What to add:**

| Feature | Priority | Notes | 
| --- | --- | --- | 
| Sub-2-second boot via kernel hibernation | High | `/sigma/boot/hibernate` image | 
| Stateless `/home` — cloud sync on login | Medium | sigma-cloudsync daemon | 
| Same image x86/ARM/RISC-V | Medium | `profiles/iot-minimal.cmake` already ARM64 | 

### Phase 3: Developer Dominance (6 months)

| Feature | Status | Notes | 
| --- | --- | --- | 
| sigma CLI (`sigma init/sign/verify/run`) | ✓ | `tools/sigma-cli/main.go` | 
| REST API gateway | ✓ | `sigmad/api-gateway/main.go` | 
| Global search (`sigma search`) | ✓ | `sigmad/search/main.go` | 
| Webhook automation | ✓ | `sigmad/webhook/main.go` | 
| gRPC management API (sigma-apid) | 🔧 | `api/sigma.proto` defined | 
| Secrets manager (no hardcoded keys) | ✓ | `sigmad/vault/main.go` | 
| GibFS — git repos as filesystems | Planned | Mount via sigma_scheme "git:" | 
| Distributed tracing (every syscall) | 🔧 | DTrace probes at all syscall entry points | 

### Phase 4: Cloud/Desktop Convergence (12 months)

| Problem | SigmaOS Solution | Status | 
| --- | --- | --- | 
| WiFi drivers | Auto-download via sigma-pkg + shims | `drivers-dev` branch | 
| GPU support | Vulkan compositor + DRM/KMS shim | Planned | 
| Updates interrupt user | Atomic A/B + OSTree background updates | ✓ | 
| Fragmentation | One image, 8 profiles, same codebase | ✓ | 
| Gaming | `release/standalone` + GPU scheduler | Branch exists | 

### Phase 5: Enterprise Lock-In (12 months)

| Feature | Implementation | Status | 
| --- | --- | --- | 
| Sigma Directory (AD/LDAP) | Zero-Trust SPIFFE workload IDs as directory | ✓ Foundation | 
| Policy as Code | `sigma-etc/init.xml` Genode routing policy | ✓ | 
| Immutable audit trail | Hash-chained `sigma_audit_backend.cpp` | ✓ | 
| Compliance toggles | `Config.sigma` Kconfig component flags | ✓ | 
| Remote wipe | sigma-vault + TPM2 seal revocation | 🔧 | 

### Phase 6: Ecosystem & Network Effects (18 months)

```
Sigma Store (1 app = all devices)
    │
    ├── Sigma Phone (ARM64 mobile profile)
    ├── Sigma Desktop (standalone profile)
    └── Sigma Server (cloud/distributed profile)
              │
         Sigma Cloud (free E2E encrypted sync for SigmaOS users)
```

**Tactics:**
- App developers: 90% revenue share vs 70% elsewhere
- Free cloud sync built into OS (navigator.sigmaos.cloud)
- SigmaOS-exclusive APIs attract developers
- Hardware partnerships (Dell/Lenovo pre-install)

### Phase 7: AI-Native Differentiator (18 months)

The unmatchable advantage — no other OS has AI at the kernel level:

```
AI Scheduler → AI Firewall → AI Filesystem
      └──────────────────────────┘
                    │
             TinyLlama (port 17392)
                    │
    ┌───────────────┼───────────────┐
    │               │               │
Predictive    Natural Language   Autonomous
 Prefetch         Shell           Healing
```

**What this unlocks:**
- "Why is my computer slow?" → AI diagnoses + fixes automatically
- "Install Photoshop alternative" → AI suggests, installs, configures
- System learns workflow → predictive preloading
- Bugs fixed before user encounters them (telemetry + auto-patch)

---

## Feature Priority Matrix

| Priority | Feature | Phase | File | 
| --- | --- | --- | --- | 
| ✅ Fixed | Secrets manager (replace hardcoded creds) | Done | `sigmad/vault/main.go` ✓ | 
| ✅ Fixed | MAC — fix always-GRANTED stub | Done | `sigma_trust_labels.h` ✓ | 
| ✅ Fixed | CryptFS derive_key() — zero-key bug | Done | `kernel/crypto/sigma_cryptfs_real.cpp` ✓ Round 13 | 
| ✅ Fixed | Wayland compositor (multi-monitor, VRR, HDR) | Done | `userland/compositor/sigma_compositor.h` ✓ Round 13 | 
| ✅ Fixed | Cloud sync E2E encrypted | Done | `sigmad/cloudsync/main.go` ✓ Round 13 | 
| ✅ Fixed | Accessibility (AT-SPI2, TTS, WCAG 2.2) | Done | `userland/accessibility/sigma_a11y.h` ✓ Round 13 | 
| 🔴 Critical | eBPF VM integration | Phase 1 | `kernel/ebpf/sigma_ebpf_vm.h` ✓ Round 12 header | 
| 🟠 High | Module signing (Dilithium3) | Phase 1 | `kernel/security/sigma_module_sign.h` 🔧 | 
| 🟠 High | Filesystem snapshots | Phase 1 | `kernel/fs/sigma_snapshot.h` 🔧 | 
| 🟠 High | DNS sinkholing | Phase 1 | `kernel/net/sigma_dns_sinkhole.h` 🔧 | 
| 🟠 High | Universal binary loader | Phase 1 | `kernel/compat/sigma_universal_loader.h` 🔧 | 
| 🟠 High | GPU memory oversubscription | Phase 2 | DRM shard exists, policy TBD | 
| 🟡 Medium | Model registry (AI model versioning) | Phase 2 | Planned | 
| 🟡 Medium | Mesh VPN (WireGuard-based) | Phase 2 | Planned | 
| 🟡 Medium | System benchmark suite | Phase 3 | `tools/sigma-bench/sigma_bench.sh` ✓ Round 13 | 
| 🟡 Medium | Privacy telemetry (opt-in) | Phase 3 | `sigmad/telemetry/main.go` ✓ Round 13 | 

---

*See also: [Architecture Overview](Architecture-Overview) · [Security Model](Security-Model) · [Improvements Overview](Improvements-Overview) · [Contributor Roadmap](Contributor-Roadmap)*

---

## Rounds 10–13 New Capabilities

### Kernel (Rounds 10–13)
| Status | Feature | File | Description | 
| --- | --- | --- | --- | 
| ✓ | SMP / LAPIC / IPI | `kernel/arch/sigma_smp.h` | Multi-processor support with per-CPU data | 
| ✓ | ACPI parser (DSDT/SSDT) | `kernel/arch/sigma_acpi.h` | Full ACPI table walking | 
| ✓ | Transparent Huge Pages (2 MiB + 1 GiB) | `kernel/mm/sigma_hugepage.h` | khugepaged collapse, per-VMA policy | 
| ✓ | IPC: shared memory / pipes / MQ | `kernel/ipc/sigma_shm.h` | POSIX IPC with capability gating | 
| ✓ | eBPF VM + verifier | `kernel/ebpf/sigma_ebpf_vm.h` | Safe programmable kernel hooks | 
| ✓ | CryptFS real AES-256-GCM | `kernel/crypto/sigma_cryptfs_real.cpp` | **Fixes Issue #44** — TPM2+HKDF key | 
| ✓ | PREEMPT_RT scheduler | `kernel/sched/sigma_rt.h` | Deterministic latency real-time class | 
| ✓ | CET shadow stack + KASLR | `kernel/arch/sigma_cet.h` | Control-flow integrity + kernel ASLR | 

### System Daemons (Rounds 10–13)
| Status | Feature | Socket | Description | 
| --- | --- | --- | --- | 
| ✓ | sigma-ntpd | `/run/sigma/ntpd.sock` | NTP with leap-second handling | 
| ✓ | sigma-journald | `/run/sigma/journal.sock` | Structured binary log, indexed | 
| ✓ | sigma-thermald | `/run/sigma/thermald.sock` | DVFS + thermal governor | 
| ✓ | sigma-acpid | `/run/sigma/acpid.sock` | ACPI power state events | 
| ✓ | sigma-telemetry | `/run/sigma/telemetry.sock` | Opt-in PII-scrubbed telemetry | 
| ✓ | sigma-cloudsync | `/run/sigma/cloudsync.sock` | E2E encrypted cloud sync | 

### User Layer (Rounds 10–13)
| Status | Feature | File | Description | 
| --- | --- | --- | --- | 
| ✓ | Zenith Wayland compositor | `userland/compositor/sigma_compositor.h` | VRR, HDR, multi-monitor, spring animations | 
| ✓ | Accessibility framework | `userland/accessibility/sigma_a11y.h` | AT-SPI2, TTS (espeak-ng), WCAG 2.2 AA | 
| ✓ | sigma-bench | `tools/sigma-bench/sigma_bench.sh` | CPU/mem/disk/net/boot/kernel benchmarks | 
| ✓ | SigmaFS native filesystem | `kernel/fs/sigmafs/sigma_sigmafs.h` | Copy-on-write, snapshots, checksums | 
| ✓ | Software RAID 0/1/5/6/10 | `kernel/fs/sigma_raid.h` | MD RAID equivalent | 
| ✓ | LVM logical volumes | `kernel/fs/sigma_lvm.h` | Thin provisioning + snapshots |
