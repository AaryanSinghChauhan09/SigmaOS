# SigmaOS — Production Readiness & Go-Live Roadmap
## Phase Gates · India Stack Go-Live · Hardware Reference Design
## Community Growth · Research Agenda · Full Gantt Schedule

Eleventh roadmap document. Covers production readiness criteria,
go-live milestones, hardware partnerships, and the complete
month-by-month development schedule through v18.0 Singularity.

---

## 1. Production Readiness Framework

### PR1 — Definition of "Production Ready"

SigmaOS is production ready when a CA in Maharashtra can:

```
1. Boot SigmaOS on a JioBook laptop (< 10 seconds)
2. Log in via ABHA DID (no username/password)
3. Open sigma-ca dashboard (< 2 seconds)
4. Enter client GSTIN and generate GSTR-1 JSON (< 5 seconds)
5. File GSTR-1 on GSTN portal (< 30 seconds, TLS 1.3 PQC-hybrid)
6. Generate e-Invoice IRN + QR code (< 5 seconds)
7. Share IRN-signed invoice to client via DID
8. Close laptop lid → suspend-to-RAM (S3)
9. Reopen → resume in < 3 seconds
10. Entire workflow: no internet required except step 5 and 7
```

Every step must pass automated CI before v15.1 ships.

### PR2 — Production Readiness Gates by Category

#### Infrastructure gates

| Gate | Measurement | Target | CI check | 
| ------ | ------------ | -------- | --------- | 
| Boot time (JioBook ARM64) | `bench_boot.sh` | < 10 s | Required | 
| Boot success rate | CI pass rate | ≥ 99.5% | Required | 
| Package install success | sigma-pkg CI | ≥ 99.9% | Required | 
| Reproducible build | SHA256 diff | Identical | Required | 
| ISO size | `ls -lh SigmaOS.iso` | < 500 MB | Required | 
| Kernel size (microkernel) | `size vmlinuz` | < 512 KB | Required | 

#### Security gates

| Gate | Tool | Target | CI check | 
| ------ | ------ | -------- | --------- | 
| Zero critical CVEs | sigma-sec status | 0 | Required | 
| PQC: real NTT (not PRNG) | bench_pqc.cpp | ≥ 1M ops/sec | Required | 
| CryptFS Argon2id working | `sigma-boot status` | Not fake | Required | 
| Signed ISO | ML-DSA-87 sig file | Verifiable | Required | 
| SBOM present | cyclonedx JSON | Valid | Required | 
| No hardcoded secrets | CI grep scan | 0 hits | Required | 

#### Quality gates

| Gate | Tool | Target | CI check | 
| ------ | ------ | -------- | --------- | 
| Unit test pass rate | GTest/ctest | 100% | Required | 
| Open 🔴 items | CURRENT_PROBLEMS | 0 | Required | 
| Stub count | `make check-stubs` | < 50 | Required | 
| Broken wiki links | `sigma-docs check` | 0 | Required | 
| WCAG 2.2 AA | aXe CI scan | Pass | Required | 
| sigma-doctor healthy | `sigma-doctor --json` | All ✓ | Required | 

#### India Stack gates

| Gate | Test | API | CI check | 
| ------ | ------ | ----- | --------- | 
| GSTN GSTR-1 sandbox | `test_gstn_sandbox.sh` | GSTN IRP | Weekly | 
| ABDM ABHA create | `test_abdm_sandbox.sh` | ABDM NHA | Weekly | 
| sigma-agri MSP lookup | `test_msp_values.cpp` | Offline | Required | 
| sigma-legal BNS map | `test_bns_map.sh` | Offline | Required | 
| Hindi UI render | `test_font_hindi.sh` | N/A | Required | 
| IME phonetic accuracy | `test_phonetic.cpp` | Offline | Required | 

---

## 2. India Stack Go-Live Milestones

### IL1 — GST Stack Go-Live

**Current:** sigma-ca has full API design. GSTN client is a stub.

```
Phase 1: GSTN Sandbox (Month 6-9)
  sigma-ca gst compute --gstin 27ABCDE1234F1Z5 --period 2026-06
  → Correct CGST/SGST/IGST amounts computed offline ✓ (already real)

  sigma-ca gst file --gstin ... --period 2026-06
  → Files on GSTN IRP sandbox → gets test ARN ← TARGET

  sigma-einvoice generate --invoice invoice.json
  → Gets real test IRN from NIC IRP sandbox ← TARGET

Phase 2: GSTN Production (Month 9-12)
  Real CA files GSTR-3B for a real client
  Real CA generates IRN for a ₹5 lakh B2B invoice
  Test: 10 real CAs, 1 month filing cycle, 0 errors

Phase 3: Scale (Month 12+)
  100 CAs filing via sigma-ca
  GSTN rate limit handling (API: 100 req/min per GSTIN)
  Bulk filing mode for large practices
```

| Task | File | Branch | Blocked by | 
| ------ | ------ | -------- | ------------ | 
| GSTN IRP OAuth2 client | `userland/indiastack/sigma_gstn_client.cpp` | `release/standalone` | TLS 1.3 (sigma-tls) + TCP stack | 
| GSTR-1 JSON schema v1.4 | `userland/apps/sigma-ca/sigma_ca.cpp` | `release/standalone` | GSTN schema download | 
| IRN generation endpoint | `userland/apps/sigma-ca/sigma_ca.cpp` | `release/standalone` | GSTN sandbox credentials | 
| e-Way Bill API | `userland/apps/sigma-ca/sigma_ca.cpp` | `release/standalone` | GSTN IRP client | 
| GSTN sandbox CI weekly | `.github/workflows/sigma_ci.yml` | `release/standalone` | GSTN test GSTIN credentials | 
| Rate limit + retry handler | `userland/indiastack/sigma_gstn_client.cpp` | `release/standalone` | Circuit breaker pattern | 

### IL2 — ABDM Health Stack Go-Live

```
Phase 1: ABDM Sandbox (Month 9-12)
  sigma-health patient create --name "Ramesh Kumar"
  → Creates ABHA health ID on ABDM sandbox ← TARGET

  sigma-health prescribe --patient P001
  → Generates NMC-compliant e-Rx, signs with doctor DID ← TARGET

  sigma-health lab result --patient P001 --test CBC
  → Pushes FHIR DiagnosticReport to ABDM ← TARGET

Phase 2: ABDM Production (Month 12-18)
  Real MBBS doctor creates ABHA for 10 patients
  Real e-prescription signed with NMC registration DID
  Real PMJAY claim submitted via NHCX

Phase 3: Scale (Month 18+)
  100 doctors using sigma-health
  Hospital integration via FHIR bulk export
  Telemedicine: sigma-health + sigma-display video call
```

| Task | File | Branch | Blocked by | 
| ------ | ------ | -------- | ------------ | 
| ABDM OAuth2 M1 (ABHA create) | `userland/indiastack/sigma_abdm_client.cpp` | `release/standalone` | TLS client | 
| FHIR R4 bundle POST | `userland/apps/sigma-health/sigma_health.cpp` | `release/standalone` | ABDM client | 
| NHCX claim API | `userland/apps/sigma-health/sigma_health.cpp` | `release/standalone` | ABDM claim schema | 
| Drug interaction SQLite DB | `userland/apps/sigma-health/sigma_health.cpp` | `release/standalone` | WHO AEDS data bundle | 
| ICD-10 offline lookup | `userland/apps/sigma-health/sigma_health.cpp` | `release/standalone` | ICD-10 SQLite (12K codes) | 
| NMC e-Rx format + DID sign | `userland/apps/sigma-health/sigma_health.cpp` | `release/standalone` | sigma-trust DID | 

### IL3 — Agriculture & Rural Go-Live

```
Phase 1: Offline stack (Month 3-6) — already partially real
  sigma-agri msp --crop wheat       ✓ (real MSP table)
  sigma-agri insurance premium      ✓ (PMFBY formula)
  sigma-agri weather --district Ludhiana  ← IMD API

Phase 2: Online integration (Month 6-9)
  sigma-agri enam prices --mandi Azadpur  ← eNAM live
  sigma-agri pmkisan status              ← PM-Kisan API
  sigma-agri land records --khatauni 12345 ← DILRMP

Phase 3: Rural stack (Month 9-18)
  sigma-gram mgnregs attendance --date today ← NREGASoft
  sigma-gram jjm status --village VG01       ← JJM API
  sigma-ultra pay --vpa farmer@upi ₹2000    ← UPI USSD
  MGNREGS payment cycle: attendance → payment in < 24h
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| IMD weather API | `userland/apps/sigma-agri/sigma_agri.cpp` | `release/standalone` | `api.weather.imd.gov.in` REST | 
| eNAM commodity prices | `userland/apps/sigma-agri/sigma_agri.cpp` | `release/standalone` | enam.gov.in daily price API | 
| DILRMP land records | `userland/apps/sigma-agri/sigma_agri.cpp` | `release/standalone` | State land portal APIs | 
| NREGASoft attendance | `userland/apps/sigma-gram/sigma_gram.cpp` | `release/mobile` | MoRD NREGASoft API | 
| sigma-ultra UPI USSD | `userland/sigma_ultra.cpp` | `release/mobile` | NPCI `*99#` USSD protocol | 

---

## 3. Hardware Reference Design

### HD1 — SigmaOS Reference Hardware Tiers

```
Tier 1 — SigmaBook (target: ₹15,000)
  CPU:  MediaTek Dimensity 6020 (ARM Cortex-A55, 4-core)
  RAM:  4 GB LPDDR4X
  Storage: 64 GB eMMC 5.1
  Display: 11.6" 1366×768 IPS
  Network: mt7921 WiFi 6 + 4G LTE
  I/O: USB-C (power + data), USB-A, HDMI, 3.5mm
  Target: Students, rural professionals, panchayat workers

Tier 2 — SigmaPro (target: ₹35,000)
  CPU:  Intel Core i5-1235U (10-core, Intel UHD)
  RAM:  8 GB DDR5
  Storage: 256 GB NVMe SSD
  Display: 14" 1920×1080 IPS, 60Hz
  Network: Intel AX211 WiFi 6E
  I/O: Thunderbolt 4, USB-A ×2, HDMI 2.0
  Target: CAs, doctors, lawyers, engineers

Tier 3 — SigmaServer (target: ₹80,000)
  CPU:  AMD EPYC 7003 (8-core)
  RAM:  32 GB ECC DDR4
  Storage: 1 TB NVMe + 4 TB HDD
  Network: Intel X550 10GbE
  Target: BharatOS government servers, sigma-fleet management

Tier 4 — SigmaBox (target: ₹2,000)
  CPU:  Raspberry Pi Zero 2W (Cortex-A53, 4-core)
  RAM:  512 MB LPDDR2
  Storage: 32 GB microSD
  Network: 802.11b/g/n, Bluetooth 4.2
  Target: Village kiosks, panchayat terminals, IoT nodes
```

### HD2 — Hardware Certification Programme

```bash
# Every device gets a sigma-compatibility score:
sigma-hardware-cert run --device JioBook-2026
# Tests: boot, network, display, storage, audio, power
# Output: SigmaOS Certified Level 2 (8/9 tests pass)
# Badge: sigmaos.dev/certified/jiobook-2026
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Hardware cert test suite | `tests/hardware/sigma_cert.sh` | `prepare-sigmaos-launch` | 9-point checklist script | 
| Compatibility database | `wiki_repo/Hardware-Compatibility.md` | `docs-update` | Device × feature × status table | 
| sigma-dna hardware profiles | `kernel/core/sigma_dna.cpp` | `kernel-exp` | Pre-built profiles for certified devices | 
| OEM partnership doc | `wiki_repo/OEM-Partnership.md` | `docs-update` | How Lava/Micromax can ship SigmaOS | 
| ARM64 reference build | `release/mobile` branch | `release/mobile` | Official JioBook SigmaOS image | 

### HD3 — Hardware Abstraction Quality

Every driver must pass the SDF quality bar:

```cpp
// Every SDF driver must implement:
class SovereignDriverBase {
public:
    virtual sigma_err_t probe(SigmaDeviceInfo* dev) = 0;  // detect hardware
    virtual sigma_err_t init() = 0;                         // allocate resources
    virtual sigma_err_t shutdown() = 0;                     // release resources
    virtual sigma_err_t health_check() = 0;                 // watchdog ping
    virtual sigma_err_t recover(const char* reason) = 0;    // after crash
    virtual const SigmaShardManifest* manifest() = 0;       // shard metadata
    virtual ~SovereignDriverBase() = default;
};

// Driver CI requirements:
// 1. probe() returns within 100ms
// 2. init() allocates only declared capabilities
// 3. health_check() returns within 10ms
// 4. Crash test: kill driver process → restart < 500ms
// 5. No memory leak over 1000 init/shutdown cycles
```

| Priority driver | Target device | File | Branch | 
| ----------------- | -------------- | ------ | -------- | 
| GPU DRM/KMS i915 | Intel UHD (SigmaPro) | `drivers/graphics/sigma_i915.cpp` | `drivers-dev` | 
| GPU DRM/KMS amdgpu | AMD Radeon (SigmaPro) | `drivers/graphics/sigma_amdgpu.cpp` | `drivers-dev` | 
| Wi-Fi mt7921 | JioBook (SigmaBook) | `drivers/net/sigma_mt7921.cpp` | `drivers-dev` | 
| Wi-Fi iwlwifi AX211 | Intel (SigmaPro) | `drivers/net/sigma_iwlwifi.cpp` | `drivers-dev` | 
| HDA audio | All tiers | `drivers/audio/sigma_hda.cpp` | `drivers-dev` | 
| BCM2711 BSP | SigmaBox (RPi Zero 2W) | `arch/arm64/sigma_bcm2711.cpp` | `release/mobile` | 

---

## 4. Community Growth Roadmap

### CG1 — Open Source Community Strategy

```
Month 1-3:  Foundation
  - 20+ "good-first-issue" labels on Phase G items
  - CONTRIBUTING.md complete (5-minute setup)
  - Dev container verified on GitHub Codespaces
  - Architecture video walkthrough published

Month 3-6:  First contributors
  - 10 external PRs merged (unit tests, docs, translations)
  - sigma-contrib scaffold tool live
  - ADR-001 through ADR-005 published
  - Bug bounty programme active

Month 6-12: Community maturity
  - Hacktoberfest: 30+ issues labelled
  - 5 community-contributed profession app recipes
  - Community translation: Hindi + Tamil docs
  - First community-contributed SDF driver (BT or webcam)

Month 12+:  Ecosystem
  - sigma-pkg community recipes: 50+ packages
  - sigma-theme community gallery: 20+ themes
  - sigma-script marketplace: 100+ automation scripts
  - Regional meetups: Delhi, Mumbai, Bangalore, Chennai
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Good-first-issues (20+) | GitHub Issues | all | Tag from Phase G list + sigma-nanolib functions | 
| CONTRIBUTING.md complete | `CONTRIBUTING.md` | `docs-update` | 5-minute setup guide verified | 
| Architecture video | `docs/videos/` | `docs-update` | 10-min walkthrough: kernel → SDF → Zenith → profession apps | 
| Bug bounty live | `wiki_repo/BUG_BOUNTY.md` | `prepare-sigmaos-launch` | CVE bounties funded | 
| Community theme gallery | `sigma_pkg_registry/themes/` | `prepare-sigmaos-launch` | Accept community `.sigma-theme` submissions | 
| sigma-script marketplace | `sigma_pkg_registry/scripts/` | `tools-dev` | Community automation scripts via sigma-pkg | 

### CG2 — India Developer Ecosystem

```
Target: India becomes the primary contributor base for SigmaOS.

Key activities:
  IIT/IISc collaboration: formal verification (Phase 9)
  NIC partnership: BharatOS pilot (Phase 7)
  NASSCOM engagement: startup grants, developer programme
  MeitY TIDE 2.0: accelerator programme for India OS startups
  College hackathons: sigma-dev challenge at IITs + NITs
  India Open Source Fund: India-specific FOSS funding
```

| Task | Detail | Timeline | 
| ------ | -------- | --------- | 
| IIT collaboration MOU | Formal verification research agreement | Phase 9 | 
| NIC technical liaison | BharatOS pilot technical support | Phase 7 | 
| NASSCOM member | Access to developer community + events | Phase 3 | 
| MeitY TIDE 2.0 application | ₹30 lakh grant for India OS development | Phase 2-3 | 
| College sigma-dev challenge | Annual hackathon: build a profession app | Annual from Phase 2 | 
| sigma-edu course | "Build a SDF Driver" open online course | Phase 3 | 

---

## 5. Research Agenda (Phase 9, Month 36-60)

### RA1 — Formal Verification Programme

```
Target: Publish formally-verified IPC + scheduler proofs at top venue.

Verification targets (priority order):
  1. sigma-bus IPC: prove no deadlock, bounded message delay
     Tool: Frama-C WP plugin (deductive verification)
     Invariant: every published message delivered in < 100ms

  2. MLFQ scheduler: prove no starvation
     Tool: Frama-C + ACSL annotations
     Invariant: every task gets CPU in bounded time regardless of priority

  3. Buddy allocator: prove no double-free, no use-after-free
     Tool: Frama-C Value plugin
     Already has magic header detection — verify formally

  4. sigma-trustd DID chain: prove no impersonation
     Tool: ProVerif (cryptographic protocol verifier)
     Model: Dolev-Yao attacker, verify DID ownership proof

  5. sigma-tls handshake: prove forward secrecy
     Tool: Tamarin prover
     Model: ML-KEM + X25519 hybrid KEM security proof
```

| Task | Tool | Branch | Timeline | 
| ------ | ------ | -------- | --------- | 
| sigma-bus deadlock proof | Frama-C WP | `release/microkernel` | Phase 9, Month 36 | 
| MLFQ no-starvation proof | Frama-C ACSL | `kernel-exp` | Phase 9, Month 42 | 
| Allocator correctness | Frama-C Value | `kernel-exp` | Phase 9, Month 40 | 
| DID chain ProVerif | ProVerif | `release/standalone` | Phase 9, Month 48 | 
| TLS Tamarin proof | Tamarin | `drivers-dev` | Phase 9, Month 54 | 
| USENIX/IEEE S&P paper | LaTeX | Phase 9 | Phase 9, Month 60 | 

### RA2 — Rust Migration Research

```
Goal: Zero memory-safety CVEs in kernel for 12 consecutive months.

Migration priority:
  Phase 9.1 (Month 36): sigma-net TCP stack → Rust
    - Highest attack surface
    - Clean interface boundary (sigma-bus IPC)
    - Rust async networking (tokio-like, no-std)

  Phase 9.2 (Month 42): sigma-fs VFS layer → Rust
    - Complex state machine (mount, inode, dentry)
    - Rust ownership model eliminates use-after-free

  Phase 9.3 (Month 48): SDF driver framework → Rust
    - Ring-3 drivers: Rust + bindgen for hardware registers
    - All new drivers must use Rust bindings from this point

  Phase 9.4 (Month 54): sigma-tls → Rust
    - Leverage rustls or write minimal sovereign alternative
    - Zero unsafe blocks except at hardware boundary

  Target: By Month 60, 0 memory-safety CVEs for 12 months
```

### RA3 — sigma-telco (5G/O-RAN Research)

```
Target: SigmaOS as the OS for BSNL private 5G cores.

Research tasks:
  O-RAN 7.2x split: implement on SigmaOS ARM64
  CU/DU functional split: sigma-pod containers per function
  ORAN Alliance compliance: O-RAN Software Community port
  TEC evaluation: TRAI/TEC certification for telecom use
  Deployment: pilot BSNL 5G private network node
```

---

## 6. Full Month-by-Month Gantt Schedule

### G1 — Phase 0 (Months 1-3): Make It Boot

| Month | Key milestones | Branch | Output | 
| ------- | --------------- | -------- | -------- | 
| 1 | Round-robin scheduler + buddy allocator | `kernel-exp` | `sigma_sched.cpp`, `sigma_mm.cpp` | 
| 1 | x86-64 page table walker + APIC | `kernel-exp` | `sigma_vmm.cpp`, `sigma_irq.cpp` | 
| 2 | 30-syscall dispatch table | `kernel-exp` | `sigma_syscall_dispatch.cpp` | 
| 2 | VESA/GOP framebuffer | `drivers-dev` | `sigma_vesa.cpp` | 
| 2 | sigma-boot.efi UEFI loader | `kernel-exp` | `sigma-boot.efi` | 
| 3 | `make iso` → bootable ISO | `kernel-exp` | `SigmaOS-0.1.0.iso` | 
| 3 | Real QEMU CI (not echo stubs) | all | CI passes | 
| 3 | Argon2id CryptFS (fix #44) | `kernel-exp` | `sigma_argon2id.cpp` | 

### G2 — Phase 1 (Months 3-6): Make It Connect

| Month | Key milestones | Branch | Output | 
| ------- | --------------- | -------- | -------- | 
| 3-4 | e1000 DMA TX/RX rings | `drivers-dev` | Real NIC driver | 
| 4 | TCP state machine RFC 793 | `drivers-dev` | `sigma_net_tcp.cpp` | 
| 4 | UDP + DNS + DHCP | `drivers-dev` | Basic networking | 
| 4-5 | sigma-repo-server (Go) | `tools-dev` | `packages.sigmaos.dev` live | 
| 5 | sigma-pkg install end-to-end | `tools-dev` | `sigma-pkg install vim` works | 
| 5 | VFS open/read/write bodies | `fs-dev` | Tmpfs working | 
| 5-6 | sigma-sh TTY connected | `tools-dev` | Interactive shell | 
| 6 | Wi-Fi mt7921/iwlwifi | `drivers-dev` | JioBook connects | 

### G3 — Phase 2 (Months 6-9): Make It Visible

| Month | Key milestones | Branch | Output | 
| ------- | --------------- | -------- | -------- | 
| 6-7 | VirtIO-GPU + DRM/KMS | `drivers-dev` | Zenith renders | 
| 7 | Compositor composite_window() | `release/standalone` | Windows blend | 
| 7-8 | Input event loop | `release/standalone` | Keyboard/mouse work | 
| 8 | App launcher + taskbar | `release/standalone` | Desktop usable | 
| 8-9 | Indian IME (Inscript) | `release/standalone` | Hindi input | 
| 8-9 | sigma-bhashini offline ASR | `release/standalone` | Voice input | 
| 9 | DID login screen | `release/standalone` | No password login | 
| 9 | sigma-ai llama.cpp daemon | `release/standalone` | Local LLM running | 

### G4 — Phase 3 (Months 9-14): Make It Indian

| Month | Key milestones | Branch | Output | 
| ------- | --------------- | -------- | -------- | 
| 9-10 | sigma-ca GSTN sandbox | `release/standalone` | GSTR-1 filed | 
| 10-11 | sigma-health ABDM sandbox | `release/standalone` | ABHA created | 
| 10-11 | sigma-accounts IRN | `release/standalone` | e-Invoice generated | 
| 11-12 | UPI pay + collect | `release/standalone` | Payment works | 
| 11-12 | sigma-gram MGNREGS API | `release/mobile` | Attendance submitted | 
| 12-13 | sigma-legal eCourts | `release/standalone` | Case status fetched | 
| 13-14 | sigma-pod kernel enforcement | `release/cloud` | Cgroup limits real | 
| 14 | v15.1 public release | `prepare-sigmaos-launch` | First public ISO | 

### G5 — Phase 4 (Months 12-18): Make It Trusted

| Month | Key milestones | Branch | Output | 
| ------- | --------------- | -------- | -------- | 
| 12-13 | sigma-boot.efi + TPM2 PCR | `kernel-exp` | Secure boot chain | 
| 13 | ML-DSA FIPS 204 final | `performance-optimized` | Real NTT | 
| 13-14 | sigma-mac enforced | `kernel-exp` | Every syscall checked | 
| 14-15 | sigma-pqc-native (no liboqs) | `performance-optimized` | Sovereign PQC | 
| 15-16 | sigma-tls (no OpenSSL) | `drivers-dev` | Sovereign TLS | 
| 16-17 | Physical hardware CI | `prepare-sigmaos-launch` | RPi4 + ThinkPad CI | 
| 17 | sigma-wine W2 (Python CLI) | `tools-dev` | Win32 compat | 
| 18 | v16.0 Apex release | all | Production-grade | 

### G6 — Phase 5-9 (Months 15-60): Make It Universal

| Month | Milestone | Branch | 
| ------- | ----------- | -------- | 
| 21 | ARM64 Raspberry Pi 4/5 boots | `release/mobile` | 
| 21 | sigma-ultra on Pi Zero | `release/mobile` | 
| 24 | sigma-ai + federated learning live | `release/standalone` | 
| 24 | sigma-telco O-RAN research begins | Phase 9 | 
| 30 | BharatOS 1,000 NIC machines | `release/cloud` | 
| 30 | v17.0 Sovereign release | all | 
| 36 | Formal verification proofs begin | `release/microkernel` | 
| 36 | Rust sigma-net migration | `kernel-exp` | 
| 42 | sigma-RuralStack 1,000 villages | `release/mobile` | 
| 42 | v18.0 Singularity release | all | 
| 54 | Rust sigma-tls migration | `drivers-dev` | 
| 60 | Zero memory-safety CVEs (12 months) | all | 
| 60 | Formal verification papers published | Research | 

---

## 7. Key Performance Indicators (Final Targets)

### KPI1 — Technical KPIs

| KPI | v15.1 | v16.0 | v17.0 | v18.0 | 
| ----- | ------- | ------- | ------- | ------- | 
| Boot time (NVMe) | Unknown | < 2 s | < 1.5 s | < 1 s | 
| Boot time (ARM64 RPi4) | N/A | < 10 s | < 8 s | < 5 s | 
| Context switch p99 | Unknown | < 100 ns | < 50 ns | < 50 ns | 
| Kyber-1024 (AVX-512) | PRNG | ≥ 1M | ≥ 5.8M | ≥ 5.8M | 
| Idle RAM (desktop) | Unknown | < 200 MB | < 150 MB | < 100 MB | 
| Kernel CVE (12 months) | N/A | N/A | 0 critical | 0 all | 
| Memory-safety CVEs | N/A | N/A | N/A | 0 | 

### KPI2 — India Impact KPIs

| KPI | v15.1 | v16.0 | v17.0 | v18.0 | 
| ----- | ------- | ------- | ------- | ------- | 
| Profession apps working | 3 | 10 | 30 | 55 | 
| CAs filing GST via sigma-ca | 0 | 10 | 1,000 | 10,000 | 
| Doctors using sigma-health | 0 | 10 | 1,000 | 10,000 | 
| Farmers using sigma-agri | 0 | 100 | 10,000 | 100,000 | 
| Panchayats on sigma-gram | 0 | 10 | 1,000 | 25,000 | 
| Managed devices (fleet) | 0 | 0 | 1,000 | 10,000 | 
| Village sigma-RuralStack | 0 | 0 | 100 | 1,000 | 
| Languages fully supported | 0 | 3 | 10 | 22 | 

---

## 8. The North Star — Why SigmaOS Exists

```
By 2030, SigmaOS is:

The only OS that is simultaneously:

  ┌─ SOVEREIGN ────────────────────────────────────────┐
  │ No foreign cloud dependency                         │
  │ No foreign identity provider                        │
  │ No foreign crypto library                           │
  │ No GRUB. No GNU libc. No OpenSSL.                  │
  │ Every line of code auditable by Indian engineers    │
  └─────────────────────────────────────────────────────┘

  ┌─ COMPLIANT ─────────────────────────────────────────┐
  │ GST filing: sigma-ca (for 8M+ CAs)                 │
  │ ABDM prescriptions: sigma-health (for 1M+ doctors) │
  │ MGNREGS attendance: sigma-gram (for 250K panchayats)│
  │ eNAM trading: sigma-agri (for 150M+ farmers)       │
  │ Every Indian regulation covered by sigma-lex       │
  └─────────────────────────────────────────────────────┘

  ┌─ ACCESSIBLE ────────────────────────────────────────┐
  │ Runs on ₹2,000 hardware (SigmaBox)                 │
  │ Works on 2G (sigma-ultra USSD mode)                │
  │ 22 Indian languages out of the box                 │
  │ Voice-first (sigma-bhashini ASR/TTS)               │
  │ Offline-first (India Stack cached locally)         │
  └─────────────────────────────────────────────────────┘

  ┌─ TRUSTED ───────────────────────────────────────────┐
  │ Post-quantum by default (ML-KEM + ML-DSA)           │
  │ Zero telemetry (no phone-home)                      │
  │ DID identity (no Google/Microsoft login)            │
  │ Formally verified IPC + scheduler (by 2030)         │
  │ 0 memory-safety CVEs for 12+ months                │
  └─────────────────────────────────────────────────────┘

Built in India. For India. By India.
```

---

## 9. All Roadmap Documents — Master Index

| # | Document | Lines | Key focus | 
| --- | ---------- | ------- | ----------- | 
| 1 | [Quality-Stability-Performance-Roadmap](Quality-Stability-Performance-Roadmap) | ~1,000 | S/P/Q/UX/Security/A11y/DX | 
| 2 | [Stability-Performance-Extended](Stability-Performance-Extended) | ~900 | Energy/Reliability/Observability/Network | 
| 3 | [Compatibility-Automation-Personalisation-Roadmap](Compatibility-Automation-Personalisation-Roadmap) | ~700 | Linux/Win32/POSIX, Automation, Custom | 
| 4 | [Advanced-Quality-Roadmap](Advanced-Quality-Roadmap) | ~700 | PQC/TLS/Enterprise/AI/i18n/Rural | 
| 5 | [Systems-Excellence-Roadmap](Systems-Excellence-Roadmap) | ~700 | Gaming/IoT/DevTools/Packages/Sprint | 
| 6 | [Engineering-Principles-Roadmap](Engineering-Principles-Roadmap) | ~700 | SOLID/OOP/CLI arch/Optimisation | 
| 7 | [Modularisation-Architecture-Roadmap](Modularisation-Architecture-Roadmap) | ~700 | Shards/Caps/Plugins/Feature flags | 
| 8 | [Sovereignty-UserDefined-Roadmap](Sovereignty-UserDefined-Roadmap) | ~700 | sigma-nanolib/sigma-tls/sigma-boot.efi | 
| 9 | [Continuous-Improvement-Roadmap](Continuous-Improvement-Roadmap) | ~800 | Versioning/Review/Tests/Docs/ZDL | 
| 10 | [Final-Excellence-Roadmap](Final-Excellence-Roadmap) | ~800 | Feedback/Boot/IPC/Data/Error/Tools | 
| 11 | [Production-Readiness-Roadmap](Production-Readiness-Roadmap) | ~900 | Go-live gates/India Stack/Gantt/KPIs | 

**Grand total: 11 documents, ~8,600 lines of engineering roadmap.**

*See also: [Development Roadmap](Development-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap) · [Gap Analysis](Gap-Analysis) · [India Profession Coverage](India-Profession-Coverage)*
