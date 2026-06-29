# SigmaOS — Final Excellence Roadmap
## Feedback Loops · Boot Experience · IPC Quality · Data Management
## Error Handling · Accessibility Deep-Dive · Performance Profiling
## Security Hardening Checklist · Master Implementation Schedule

Tenth and synthesising roadmap document. Covers remaining dimensions
and consolidates all work into a single executable schedule.

---

## 1. Feedback Loops & Continuous Improvement

### FL1 — User Feedback System

```bash
# Built into every tool — one command to report:
sigma-feedback report \
  --tool sigma-ca \
  --severity minor \
  --msg "GST compute gives wrong IGST for e-commerce"
# → Creates GitHub Issue automatically via sigma-bus → sigmad/feedback
# → User gets issue number back immediately
# → Issue tagged: sigma-ca, gstn, computation, minor

# Rate an interaction after use:
sigma-ca gst file --period 2026-06
# ✓ GSTR-3B filed. ARN: AA2706250007XXX
# Was this helpful? [y/n/s(kip)] y
# Thanks! Rating saved locally, synced on next update-check.
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-feedback CLI | `userland/tools/sigma_feedback_cli.cpp` | `tools-dev` | `sigma-feedback report/list/sync` |
| GitHub Issue auto-create | `sigmad/feedback/main.go` | `tools-dev` | POST to GitHub API on `sigma-feedback report` |
| In-tool rating prompt | All CLI tools | `release/standalone` | Optional y/n after mutating commands |
| Feedback aggregation | `sigmad/feedback/main.go` | `tools-dev` | Collect ratings locally, sync on `sigma_automation.sh update` |
| Feedback privacy policy | `wiki_repo/Feedback-Privacy.md` | `docs-update` | No PII collected; only tool + rating + optional message |

### FL2 — Telemetry-Free Quality Metrics

SigmaOS collects zero telemetry. Quality metrics come from CI only.

```bash
# Weekly automated quality report (no data leaves device):
sigma_automation.sh quality-report
# Generates: .sigma/reports/quality-2026-06-28.md
# Contents:
#   - Boot time trend (last 30 days, from CI)
#   - Unit test pass rate (from CI logs)
#   - Open critical issues (from CURRENT_PROBLEMS_MANIFEST.md)
#   - Stub count trend (from make check-stubs)
#   - Module size budget status
#   - Wiki freshness (last update per page)
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `cmd_quality_report()` | `scripts/sigma_automation.sh` | `tools-dev` | Aggregate CI metrics into local Markdown report |
| Trend tracking (JSON history) | `scripts/sigma_automation.sh` | `tools-dev` | Append metrics to `.sigma/metrics/history.json` |
| Regression alert | `scripts/sigma_automation.sh` | `tools-dev` | If any metric regresses >10% week-over-week, warn |
| Weekly CI trigger | `.github/workflows/sigma_ci.yml` | all | `@weekly` schedule generates quality report |

### FL3 — Benchmark Regression Dashboard

```
sigmaos.dev/perf  (auto-updated on every merge to main):

Boot Time (NVMe SSD)
  Target: < 2s   Current: unknown (kernel not bootable yet)
  ────────────────────────────────────── target
  |                                      |
  Now ──────────────────────── BUILDING ──▶

Context Switch p99
  Target: < 50ns  Historical best: N/A
  [placeholder until kernel boots]

Kyber-1024 ops/sec (AVX-512)
  Target: 5.8M   Current: 0 (PRNG placeholder)
  Phase 0 ────────────────────────────▶ Phase 4
           PRNG          real NTT     AVX-512
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `bench_results.json` in repo | `.github/workflows/sigma_ci.yml` | `performance-optimized` | Store benchmark JSON as git notes |
| D3.js trend charts | `gh-pages` site | `gh-pages` | Plot bench_results.json over time |
| Regression gate | `.github/workflows/sigma_ci.yml` | `performance-optimized` | Fail if any metric degrades >10% vs last week |
| Badge in README | `README.md` | all | `![boot-time](badge-url)` shields.io badge |

---

## 2. Boot Experience Roadmap

### BE1 — Boot Sequence Quality

**Target: user sees a usable screen in < 2 seconds from power-on.**

```
t=0ms    sigma-boot.efi loads (UEFI firmware hands off)
t=80ms   ML-DSA-87 kernel signature verified
t=120ms  Kernel decompressed and mapped
t=180ms  APIC + MM + scheduler initialised
t=280ms  sigma-dna hardware profile built (CPUID/PCI scan)
t=350ms  Level 0 shards loaded in parallel
t=500ms  Level 1 drivers loaded (GPU, NIC, storage)
t=700ms  VFS mounted, sigma-bus ready
t=900ms  Level 2 services started (net, crypto, auth)
t=1200ms DRM/KMS scanout active (pixels visible)
t=1400ms Zenith compositor first frame rendered
t=1600ms sigma-input daemon ready (keyboard/pointer live)
t=1800ms sigma-dm login screen shown
t=2000ms User starts typing — system fully interactive
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Parallel shard ignition | `kernel/core/sigma_shard_loader.cpp` | `kernel-exp` | Load independent shards concurrently using CPU cores |
| sigma-dna CPUID at t=280ms | `kernel/core/sigma_dna.cpp` | `kernel-exp` | Non-blocking: probe PCI in background while shards load |
| Lazy driver init (load on first use) | `hal/SovereignHAL.cpp` | `drivers-dev` | Don't init USB audio until first audio request |
| Boot time CI metric | `.github/workflows/sigma_ci.yml` | all | `time qemu-boot.sh` — fail if > 2s threshold |
| Boot time flamegraph | `scripts/sigma_automation.sh` | `performance-optimized` | `sigma-perf record --boot` produces boot flamegraph |
| Splash screen at t=1200ms | `drivers/display/sigma_vesa.cpp` | `release/standalone` | Show Σ logo on first pixel from GPU |
| Silent boot option | `Config.sigma` | `release/standalone` | `boot.silent = true` skips splash for kiosks |
| Boot time display on first run | `userland/installer/sigma_oobe.cpp` | `release/standalone` | OOBE shows: "Booted in 1.8s" |

### BE2 — Boot Resilience Depth

```
Scenario 1: Clean boot (99%+ of cases)
  sigma-boot.efi → kernel → shards → desktop

Scenario 2: Boot counter exceeded (3 failed boots)
  sigma-boot.efi reads fail_count from EFI variable
  fail_count >= 3 → boot B-slot (last known good)
  Show: "Previous boot failed. Running from safe slot."

Scenario 3: B-slot also fails
  sigma-boot.efi shows text: recovery menu
  Options: [1] sigma-recovery shell
           [2] Reinstall from USB
           [3] Factory reset (wipes data)
           [4] Export diagnostics to USB

Scenario 4: Firmware / hardware fault
  All slots fail → sigma-boot.efi shows hardware info
  Print CPUID, APIC state, memory map
  Save to EFI partition as fault_report.txt
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Fail counter EFI variable | `sigma-boot/sigma_ab_slot.c` | `kernel-exp` | Increment on every boot; reset on `sigma-boot commit` |
| B-slot auto-select on 3 fails | `sigma-boot/sigma_ab_slot.c` | `kernel-exp` | `fail_count >= 3` → set `SigmaBootSlot=B` |
| Text recovery menu in sigma-boot | `sigma-boot/sigma_boot.c` | `kernel-exp` | UEFI text mode before kernel loads |
| Hardware fault report to EFI fs | `sigma-boot/sigma_boot.c` | `kernel-exp` | Write `EFI/sigma/fault_report.txt` on all-fail |
| Recovery CI scenario | `tests/chaos/test_3boot_fail.sh` | all | 3 forced failed boots → verify B-slot + recovery |

---

## 3. IPC Quality (sigma-bus)

### IQ1 — sigma-bus Performance

```
sigma-bus message flow:
  Publisher → kernel message queue → Subscriber notification → delivery

Target latencies:
  Same-shard IPC:     < 100 ns  (shared ring buffer, no copy)
  Cross-shard IPC:    < 500 ns  (kernel-mediated, one copy)
  Cross-pod IPC:      < 2 µs    (sigma-pod network namespace boundary)
  Remote IPC (fleet): < 10 ms   (sigma-fleet TCP channel)
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Lock-free MPSC ring buffer | `kernel/ipc/sigma_bus_ring.cpp` | `kernel-exp` | Michael-Scott queue per topic, no mutex |
| Zero-copy large payload | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | Payloads > 4KB: share physical page, pass handle |
| Capability check on hot path | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | AVC cache lookup: (publisher, topic) → allowed/denied |
| Message schema validation | `kernel/ipc/sigma_bus.cpp` | `kernel-exp` | Validate payload size + magic number; reject malformed |
| sigma-bus benchmark CI | `tests/perf/bench_bus.cpp` | `performance-optimized` | 1M messages, measure p50/p99 latency |
| Dead-letter queue | `userland/daemons/sigma_queue.cpp` | `release/cloud` | Undeliverable → retry 3× with backoff, then DLQ |

### IQ2 — sigma-bus Observability

```bash
sigma-bus monitor                    # live IPC message trace
sigma-bus monitor --topic "sigma.gst.*"  # filter by topic glob
sigma-bus stats                      # throughput + latency histogram
sigma-bus trace sigma-ca             # trace all messages to/from sigma-ca
sigma-bus list                       # registered services + caps
sigma-bus capabilities sigma-ca      # list sigma-ca capability tokens
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `/proc/sigma/bus/` stats | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | Per-topic message counts + latency percentiles |
| `sigma-bus monitor` live trace | `userland/tools/sigma_bus_cli.cpp` | `tools-dev` | Read from procfs, print with timestamp |
| Topic glob filter | `userland/tools/sigma_bus_cli.cpp` | `tools-dev` | `sigma.*` matches all sigma namespace topics |
| sigma-bus audit integration | `kernel/ipc/sigma_bus.cpp` | all | Every IPC logged to sigma-audit with DID |

---

## 4. Data Management Roadmap

### DM1 — Profession Data Architecture

```
Data ownership model:
  Each profession app owns its data shard:
  /sigma/data/
    sigma-ca/
      clients.db          ← SQLite, per-client ledger
      gst_returns/        ← JSON filings archive (7 years)
      irns/               ← e-Invoice IRN cache
    sigma-health/
      patients.db         ← FHIR R4 bundles (encrypted)
      prescriptions/      ← NMC e-Rx archive
    sigma-agri/
      farm_plots.db       ← Land + crop records
      msp_cache.db        ← Offline MSP table
    sigma-accounts/
      ledger.db           ← Double-entry SQLite
      vouchers/           ← DID-signed voucher archive
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Per-app data directory setup | `userland/daemons/sigma_appd.cpp` | `release/standalone` | Create `/sigma/data/<app>/` on first app launch |
| Encryption at rest | `kernel/security/sigma_cryptofs.cpp` | `kernel-exp` | All `/sigma/data/` encrypted with Argon2id-derived key |
| Per-app data backup | `scripts/sigma_automation.sh` | `tools-dev` | `sigma_automation.sh backup` includes all app data |
| Data export per app | All profession apps | `release/standalone` | `sigma-ca export --format json > ca_data.json` |
| Data import on migration | `userland/installer/sigma_migrate_*.cpp` | `release/dual-boot` | Import Tally/Excel/EHR data at install time |
| Retention policy | `Config.sigma` | `release/standalone` | `data.retention_years = 7` for GST (mandatory) |
| DPDP Act compliance | All profession apps | `release/standalone` | User can delete any data; consent logged |

### DM2 — Offline-First Data Sync

```
Priority: data always accessible without internet.

Data tier model:
  Tier 0 — Always local (never syncs):
    DID keys, sigma-policy files, Config.sigma

  Tier 1 — Local first, sync when online:
    Profession app data (GST returns, health records)
    Offline India data (MSP table, HSN codes, ICD-10)

  Tier 2 — Cache + TTL:
    eNAM live prices (TTL: 1 hour)
    GSTN API responses (TTL: 24 hours)
    IMD weather (TTL: 6 hours)

  Tier 3 — Online only (graceful degradation):
    ABDM FHIR live push
    GST filing (requires internet)
    UPI payment (requires internet)
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| CRDT-based sync (Tier 1) | `net/sigma_offline_sync.cpp` | `release/distributed` | Last-write-wins for profession data across devices |
| TTL cache for API responses | `userland/indiastack/sigma_indiastack_cache.cpp` | `tools-dev` | SQLite cache with expiry column |
| Graceful degradation UX | All profession apps | `release/standalone` | Show "Last updated: 2h ago" when offline |
| Offline indicator in Zenith | `zenith_desktop/taskbar/sigma_systray.cpp` | `release/standalone` | Wi-Fi icon greyed + "Offline mode" badge |
| Conflict resolution UI | `zenith_desktop/ui/sigma_dialog.cpp` | `release/standalone` | Show diff when CRDT conflict detected |

---

## 5. Error Handling Philosophy

### EH1 — Error Handling Contract

```cpp
// The one rule: EVERY function returns sigma_err_t.
// No exceptions. No silent failures. No assert() in production.

// Pattern 1: propagate with SIGMA_TRY
sigma_err_t sigma_ca_file_gstr1(sigma_gst_return_data_t* d) {
    SIGMA_TRY(sigma_gstn_client_connect());     // propagate immediately
    SIGMA_TRY(sigma_gstn_client_auth(d->gstin));
    SIGMA_TRY(sigma_gstn_api_file_gstr1(d));
    SIGMA_TRY(sigma_audit_log("gstr1_filed", d->gstin));
    return SIGMA_OK;
}

// Pattern 2: handle locally with meaningful context
sigma_err_t sigma_pkg_install(const char* name) {
    sigma_err_t e = sigma_pkg_download(name);
    if (e == SIGMA_ERR_NETWORK) {
        sigma_log_warn("sigma-pkg: network unavailable, trying cache");
        return sigma_pkg_install_from_cache(name);
    }
    if (e != SIGMA_OK) return e;
    // ...
}

// Pattern 3: user-facing error — always include next step
void sigma_cli_report_error(sigma_err_t e, const char* context) {
    const char* msg   = sigma_err_to_string(e);
    const char* fix   = sigma_err_to_fix(e);      // NEW: fix suggestion
    const char* docs  = sigma_err_to_docs_url(e); // NEW: docs link
    sigma_printf("✗ %s: %s\n  Fix: %s\n  Docs: %s\n", context, msg, fix, docs);
}
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `sigma_err_to_fix()` lookup | `include/sigma_error_codes.h` | `tools-dev` | Map each error → human fix suggestion |
| `sigma_err_to_docs_url()` | `include/sigma_error_codes.h` | `tools-dev` | Map each error → wiki URL |
| `SIGMA_TRY` macro | `include/sigma_error_codes.h` | `tools-dev` | Already designed — implement + adopt everywhere |
| No `assert()` in production | `.github/workflows/sigma_ci.yml` | all | CI: grep for `assert(` in kernel/ → fail |
| Error code coverage test | `tests/unit/test_error_codes.cpp` | `tools-dev` | Every `sigma_err_t` value has string + fix |

### EH2 — Error Recovery Patterns

| Pattern | Where used | Implementation |
|---------|-----------|----------------|
| Retry with backoff | India Stack API calls | 1s/2s/4s/8s, max 3 retries |
| Circuit breaker | GSTN/ABDM client | Open after 3 fails; half-open after 60s |
| Fallback to cache | All API calls | Return cached response with age warning |
| Graceful degradation | sigma-ai, sigma-bhashini | "AI unavailable; showing cached response" |
| Crash-restart loop prevention | SDF drivers | sigma-heal: max 3 restarts per 5 minutes |
| User notification on persistent error | All daemons | Toast: "GSTN API unreachable since 2h" |

---

## 6. Accessibility Deep-Dive

### AC1 — Full Accessibility Stack

```
AT-SPI2 accessibility tree (sigma-zenith)
  │
  ▼ sigma-a11y daemon
  │   Walk widget tree every 100ms
  │   Detect focus changes
  │   Build speech queue
  │
  ▼ sigma-bhashini TTS (offline)
  │   Text → phonemes → audio
  │   22 Indian languages
  │   < 200ms first-word latency
  │
  ▼ sigma-audio PCM output
  │   HDA device at 44100 Hz
  │
  ▼ User hears spoken UI
```

| Feature | File | Branch | Target |
|---------|------|--------|--------|
| AT-SPI2 widget tree walker | `userland/a11y/sigma_a11y.cpp` | `release/standalone` | Every widget: name + role + state |
| Focus change announcement | `userland/a11y/sigma_a11y.cpp` | `release/standalone` | Announce on every Tab/click |
| sigma-bhashini → HDA pipeline | `userland/a11y/sigma_a11y.cpp` | `release/standalone` | TTS PCM → sigma-audio write |
| Keyboard-only navigation | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Full UI operable without pointer |
| Skip navigation links | `zenith_desktop/ui/` | `release/standalone` | Alt+M → main content, Alt+N → nav |
| High-contrast theme | `zenith_desktop/themes/zdl-high-contrast.sigma-theme` | `release/standalone` | Minimum 7:1 contrast ratio (AAA) |
| Large text mode (200%) | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Scale all UI elements 2× |
| Reduce motion mode | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Zero animations when enabled |
| Braille display (BRLTTY) | `userland/a11y/sigma_braille.cpp` | `release/standalone` | USB Braille device support |
| Switch access scanning | `userland/a11y/sigma_switch.cpp` | `release/standalone` | Single-switch dwell-time scanning |
| WCAG 2.2 AA CI gate | `.github/workflows/sigma_ci.yml` | `prepare-sigmaos-launch` | aXe automated scan on Zenith UI |

### AC2 — India-Specific Accessibility

| Feature | Detail | Branch |
|---------|--------|--------|
| Hindi screen reader | sigma-bhashini TTS for all UI text | `release/standalone` |
| Tamil / Telugu / Bengali TTS | sigma-bhashini 22-language offline | `release/standalone` |
| Voice navigation | Speak command → sigma-bhashini ASR → action | `release/standalone` |
| Low-literacy mode | Icon-first UI, minimal text | `release/mobile` |
| USSD accessibility | Text-only USSD menus for feature phones | `release/mobile` |
| 2G-compatible UI | All UI functional at 2G latency | `release/mobile` |

---

## 7. Security Hardening Checklist

### SH1 — Complete Security Hardening Checklist

Every item must be verifiable via `sigma-sec status`:

```
KERNEL MEMORY SAFETY
[ ] KASLR: kernel base randomised at every boot
[ ] W^X: no page simultaneously writable and executable
[ ] CET shadow stack: ROP mitigated on Intel CET CPUs
[ ] Stack canaries: -fstack-protector-strong on all kernel code
[ ] ASLR: heap/stack/mmap randomised for all processes
[ ] NX bit: data pages marked non-executable
[ ] CFI: Control-Flow Integrity on kernel binaries
[ ] Heap guard pages: 4KB guard after every slab allocation

CRYPTOGRAPHY
[ ] Argon2id CryptFS: Issue #44 resolved
[ ] ML-KEM-1024: real NTT, not PRNG placeholder
[ ] ML-DSA-87: real NTT, not PRNG placeholder
[ ] Constant-time comparison: sigma_ct_memcmp everywhere
[ ] No hardcoded keys or passwords: CI scan passes
[ ] All keys zero-wiped after use: sigma_secure_memzero

BOOT CHAIN
[ ] sigma-boot.efi: no GRUB dependency
[ ] ML-DSA-87 kernel verification at boot
[ ] TPM2 PCR measurements recorded
[ ] CryptFS key sealed to TPM2 PCR 0+7
[ ] A/B rollback: max 3 failed boots before auto-rollback
[ ] Immutable root filesystem: dm-verity on /

SUPPLY CHAIN
[ ] All .spkg packages: ML-DSA-87 signed
[ ] dm-verity on every package install
[ ] SBOM generated with every release
[ ] Reproducible builds: two builds = identical SHA256
[ ] No untrusted upstream mirrors

RUNTIME ISOLATION
[ ] sigma-mac: every syscall checked against policy
[ ] Capability tokens: every shard declares required caps
[ ] sigma-pod namespaces: PID/NET/MNT/IPC/UTS/USER
[ ] cgroup v2: CPU/memory/IO limits enforced
[ ] Zero capability escalation paths: verified by CI

AUDIT
[ ] sigma-audit: every security event logged
[ ] ML-DSA-87 per log entry: tamper-evident
[ ] Hash chain: detect any modification
[ ] WORM register backup for critical events
[ ] sigma-ids: behavioural anomaly detection live
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `sigma-sec status` full report | `userland/tools/sigma_sec_cli.cpp` | `tools-dev` | Check all 30+ items, print ✓/✗ per item |
| Security posture score | `userland/tools/sigma_sec_cli.cpp` | `tools-dev` | Score 0–100; published in quality report |
| Pre-release security gate | `scripts/sigma_quality_check.sh` | `prepare-sigmaos-launch` | Block release if any critical item fails |

---

## 8. Tools Ecosystem — Complete Inventory

### TL1 — All sigma-* Tools (target state)

```
Core system tools:
  sigma-cli         ← modular command hub [✅ partial]
  sigma-sh          ← login shell [✅ parser, ❌ TTY]
  sigma-pkg         ← package manager [⚠️ no repo server]
  sigma-drv         ← driver management [❌ needs kernel]
  sigma-shard       ← shard management [❌ needs shard loader]
  sigma-boot        ← boot management [❌ needs sigma-boot.efi]
  sigma-config      ← Config.sigma editor [❌ build]
  sigma-profile     ← ~/.sigma_profile editor [⚠️ partial]

System monitoring:
  sigma-observatory ← live system dashboard [❌ needs procfs]
  sigma-monitor     ← process monitor [⚠️ partial]
  sigma-doctor      ← health diagnostics [❌ build]
  sigma-perf        ← performance profiler [❌ needs PMU]
  sigma-audit       ← audit log viewer [⚠️ partial]
  sigma-mem         ← memory diagnostics [❌ needs kernel]
  sigma-sched       ← scheduler diagnostics [❌ needs kernel]

Networking:
  sigma-net         ← network manager [❌ needs TCP stack]
  sigma-firewall    ← firewall rules [❌ build]
  sigma-vpn         ← VPN client [❌ build]
  sigma-dns         ← DNS resolver [❌ build]

Security:
  sigma-sec         ← security posture [⚠️ partial]
  sigma-pqc         ← PQC operations [⚠️ PRNG not real]
  sigma-trust       ← DID/attestation [⚠️ partial]
  sigma-ids         ← intrusion detection [⚠️ partial]

Developer tools:
  sigma-gdb         ← debugger [❌ needs ptrace]
  sigma-strace      ← syscall tracer [❌ needs ptrace]
  sigma-perf        ← profiler [❌ needs PMU]
  sigma-memcheck    ← memory analyser [❌ build]
  sigma-contrib     ← contributor scaffold [❌ build]

Container/Fleet:
  sigma-pod         ← container CLI [⚠️ stubs]
  sigma-fleet       ← device management [❌ build]
  sigma-kube        ← cluster manager [❌ build]
  sigma-cron        ← scheduled tasks [❌ build]
  sigma-hook        ← event automation [❌ build]

Compatibility:
  sigma-wine        ← Windows EXE runner [⚠️ PE loader done]
  sigma-compat      ← Linux ELF runner [⚠️ partial]
  sigma-update      ← OTA update manager [❌ build]
  sigma-recovery    ← recovery wizard [⚠️ text shell only]

India Stack:
  sigma-gst         ← GST tools [⚠️ header only]
  sigma-abdm        ← health identity [❌ no API client]
  sigma-upi         ← payments [❌ no API client]
  sigma-digilocker  ← document store [❌ no API client]
  sigma-feedback    ← user feedback [❌ build]
```

| Task | File | Branch | Priority |
|------|------|--------|---------|
| sigma-doctor | `userland/tools/sigma_doctor_cli.cpp` | `tools-dev` | 🟠 |
| sigma-config | `userland/tools/sigma_config_cli.cpp` | `tools-dev` | 🟠 |
| sigma-feedback | `userland/tools/sigma_feedback_cli.cpp` | `tools-dev` | 🟡 |
| sigma-update | `userland/daemons/sigma_updated.cpp` | all | 🔴 |
| sigma-cron | `userland/daemons/sigma_cron.cpp` | `tools-dev` | 🟠 |
| sigma-hook | `userland/daemons/sigma_hook.cpp` | `tools-dev` | 🟠 |
| sigma-recovery | `userland/tools/sigma_recover_cli.cpp` | all | 🔴 |

---

## 9. Master Implementation Schedule

### MIS1 — Complete Task Order (All Branches)

The full ordered list of what to build, in strict dependency order:

#### Tier 0 — Boot (Blocks Everything)
```
Week 1-2:  sigma_sched.cpp bodies (round-robin first)
Week 1-2:  sigma_mm.cpp bodies (buddy + slab + VMM)
Week 2-3:  sigma_irq.cpp (APIC + PIC)
Week 2-3:  sigma_timer.cpp (HPET/APIC timer)
Week 3-4:  sigma_syscall_dispatch.cpp (30 syscalls)
Week 4-5:  sigma-boot.efi UEFI loader
Week 5-6:  VESA/GOP framebuffer driver
Week 6:    make iso → bootable SigmaOS.iso
Week 6:    QEMU CI real boot test (not echo)
Week 7:    sigma_argon2id.cpp (fix Issue #44)
```

#### Tier 1 — Connect (Unblocks Packages & Apps)
```
Week 8-9:  e1000 DMA TX/RX rings (real hardware)
Week 9-10: TCP state machine (RFC 793)
Week 10:   UDP socket layer
Week 10-11:DHCP client
Week 11:   DNS resolver
Week 11-12:sigma-repo-server (Go HTTPS)
Week 12:   sigma-pkg install (end-to-end)
Week 12-13:VFS open/read/write bodies
Week 13:   sigma-sh TTY read connected
Week 13-14:musl-libc bundle for sigma-compat
```

#### Tier 2 — Visible (Unblocks Desktop & Profession Apps)
```
Week 14-15:VirtIO-GPU DRM/KMS
Week 15-16:Zenith compositor composite_window()
Week 16-17:Input event loop (keyboard + pointer)
Week 17-18:App launcher (Super key)
Week 18-19:sigma-bhashini ASR offline
Week 19-20:Indian IME (Inscript Devanagari)
Week 20-21:DID login screen (sigma-dm)
Week 21-22:sigma-ai llama.cpp daemon
Week 22-23:sigma-ca GST compute (GSTN sandbox)
Week 23-24:sigma-accounts double-entry + GSTR-1
```

#### Tier 3 — Indian (Unblocks Production Use)
```
Month 6-7: ABDM FHIR R4 client
Month 6-7: GST IRN + e-Way Bill live API
Month 7-8: UPI pay + collect
Month 7-8: MGNREGS attendance API
Month 8-9: sigma-legal eCourts live
Month 8-9: sigma-pod kernel namespace enforcement
Month 9:   ML-KEM real NTT (replaces PRNG)
Month 9-10:sigma-fleet 100 devices
Month 10:  WCAG 2.2 AA automated pass
Month 10-11:22-language UI strings
```

#### Tier 4 — Trusted (Enterprise & Security Grade)
```
Month 12: sigma-boot.efi + TPM2 PCR sealing
Month 12: ML-DSA FIPS 204 final bindings
Month 13: sigma-mac enforced on every syscall
Month 13: sigma-ids live with anomaly detection
Month 14: Reproducible build verified in CI
Month 14: Physical hardware CI (RPi4 + ThinkPad)
Month 15: sigma-wine W2 (Python CLI for Windows)
Month 15: OTA A/B update working end-to-end
Month 16: sigma-pqc-native (no liboqs dependency)
Month 18: Zero critical bugs (CURRENT_PROBLEMS 🔴 = 0)
```

### MIS2 — Quality Gates Per Milestone

| Milestone | Gate | Verification |
|-----------|------|-------------|
| v0.1 First Boot | QEMU reaches shell | CI: `assert boot_prompt` |
| v0.2 First Network | `ping 10.0.2.2` works | CI: `assert ping success` |
| v0.3 First Package | `sigma-pkg install vim` | CI: vim binary exists |
| v0.4 First Desktop | Zenith renders frame | CI: screenshot diff |
| v0.5 First Profession | sigma-ca computes GST | CI: GSTR-1 JSON valid |
| v15.1 Launch | All Q0 gates pass | `sigma_quality_check.sh` |
| v16.0 Apex | Boot < 2s, PQC real | CI benchmark gates |
| v17.0 Sovereign | Enterprise pilot ready | Fleet 100 devices |
| v18.0 Singularity | 0 memory-safety CVEs | 12-month clean streak |

### MIS3 — Contributor Onboarding Path

**For a new contributor starting today:**

```bash
# Day 1: Get building
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS
./scripts/setup.sh                   # install build deps
make PROFILE=microkernel             # first build (~5 min)
make test                            # unit tests pass

# Day 2: Understand the codebase
sigma-cli --help                     # explore CLI surface
cat CONTRIBUTING.md                  # contribution guide
cat CURRENT_PROBLEMS_MANIFEST.md     # find something to fix

# Day 3: Pick a good-first-issue
# Suggested first contributions:
# 1. Write unit tests for sigma_nanolib.h functions
# 2. Add --json flag to sigma-cli profile list
# 3. Fix sigma_agri PMFBY API stub (call real pmfby.gov.in)
# 4. Write Hindi translation strings (userland/locales/hi.sigma-l10n)
# 5. Fix BSP tree rebuild on remove_window (known TODO)

# Week 1: First PR
sigma-contrib check                  # validate your changes
git commit -m "feat(agri): PMFBY enrollment API integration"
# PR is auto-labelled, auto-assigned to reviewer
```

---

## 10. Summary — Complete Roadmap Collection

All 10 roadmap documents with page counts:

| # | Document | Key topics | Lines |
|---|----------|-----------|-------|
| 1 | [Quality-Stability-Performance-Roadmap](Quality-Stability-Performance-Roadmap) | Stability S1-S4, Performance P1-P6, Quality Q1-Q5, UX U1-U6 | ~1,000 |
| 2 | [Stability-Performance-Extended](Stability-Performance-Extended) | Energy, Reliability, Observability, Release Engineering, Network QA | ~900 |
| 3 | [Compatibility-Automation-Personalisation-Roadmap](Compatibility-Automation-Personalisation-Roadmap) | Linux/Win32/POSIX compat, Automation, Customisation, Personalisation | ~700 |
| 4 | [Advanced-Quality-Roadmap](Advanced-Quality-Roadmap) | PQC production, Network stack, Enterprise, AI/ML, i18n, Rural | ~700 |
| 5 | [Systems-Excellence-Roadmap](Systems-Excellence-Roadmap) | Gaming, IoT, Dev tools, Packages, Updates, Multi-platform | ~700 |
| 6 | [Engineering-Principles-Roadmap](Engineering-Principles-Roadmap) | SOLID/OOP, Design patterns, CLI architecture, Refactoring | ~700 |
| 7 | [Modularisation-Architecture-Roadmap](Modularisation-Architecture-Roadmap) | Shard system, Build modularity, Plugin API, Automation depth | ~700 |
| 8 | [Sovereignty-UserDefined-Roadmap](Sovereignty-UserDefined-Roadmap) | Foreign dep reduction, User extensions, India-first, DID | ~700 |
| 9 | [Continuous-Improvement-Roadmap](Continuous-Improvement-Roadmap) | Versioning, Code review, Testing, Docs, ZDL, sigma-nanolib | ~800 |
| 10 | [Final-Excellence-Roadmap](Final-Excellence-Roadmap) | Feedback, Boot experience, IPC quality, Data mgmt, Error handling, Accessibility, Security checklist, Tools inventory, Master schedule | ~800 |

**Total: 10 documents, ~7,700 lines of actionable engineering roadmap.**

---

*See also: [Continuous Improvement Roadmap](Continuous-Improvement-Roadmap) · [Sovereignty UserDefined Roadmap](Sovereignty-UserDefined-Roadmap) · [Development Roadmap](Development-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap) · [Gap Analysis](Gap-Analysis)*
