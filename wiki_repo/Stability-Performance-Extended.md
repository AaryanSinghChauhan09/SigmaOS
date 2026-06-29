# SigmaOS — Extended Quality & Excellence Roadmap

Continuation of [Quality-Stability-Performance-Roadmap](Quality-Stability-Performance-Roadmap).
Covers: network reliability, India-specific quality, testing infrastructure,
benchmarking framework, hardware compatibility, long-term engineering excellence.

---

## Network Reliability & Quality

### NR1 — TCP/IP Stack Correctness

**Current state:** Zero-copy packet queue + header parse exists (`net/sigma_tcp_ip.cpp`).
No state machine. No connection tracking. No retransmit.

| Task | File | Branch | Target | 
| ------ | ------ | -------- | -------- | 
| RFC 793 TCP state machine | `net/tcp/sigma_tcp.cpp` | `drivers-dev` | SYN→SYN-ACK→ACK, FIN→FIN-ACK, RST paths | 
| TCP retransmit timer | `net/tcp/sigma_tcp.cpp` | `drivers-dev` | Exponential backoff, 3 retransmits then RST | 
| TCP Nagle + delayed ACK | `net/tcp/sigma_tcp.cpp` | `drivers-dev` | Nagle on by default, disable with TCP_NODELAY | 
| SACK (Selective ACK) | `net/tcp/sigma_tcp.cpp` | `drivers-dev` | RFC 2018 — recover from multiple losses | 
| TCP congestion control (CUBIC) | `net/tcp/sigma_tcp_cc.cpp` | `drivers-dev` | RFC 8312 — internet-grade throughput | 
| UDP socket layer | `kernel/net/sigma_net_socket.cpp` | `drivers-dev` | DNS, NTP, DHCP, UPI over UDP | 
| IPv6 + SLAAC + DHCPv6 | `kernel/net/sigma_net_ipv6.cpp` | `drivers-dev` | Full dual-stack operation | 
| ARP cache + timeout | `kernel/net/sigma_net_arp.cpp` | `drivers-dev` | Replace current stub with real ARP table | 
| ICMP rate limiting | `kernel/net/sigma_net.c` | `drivers-dev` | Prevent ICMP flood amplification | 
| DNS resolver | `net/dns/sigma_dns.cpp` | `drivers-dev` | `/etc/sigma/resolv.conf` + DoH option | 
| DHCP client | `net/dhcp/sigma_dhcp_client.cpp` | `drivers-dev` | DISCOVER→OFFER→REQUEST→ACK | 
| TLS 1.3 (hybrid X25519 + ML-KEM) | `net/tls/sigma_tls.cpp` | `drivers-dev` | All India Stack API calls use PQC-TLS | 

### NR2 — Network Stability Tests

| Test | File | Branch | What it verifies | 
| ------ | ------ | -------- | ------------------ | 
| TCP SYN flood | `tests/net/test_tcp_synflood.sh` | `drivers-dev` | SYN cookies, no OOM under flood | 
| TCP out-of-order segments | `tests/net/test_tcp_ooo.cpp` | `drivers-dev` | Correct reassembly | 
| Packet loss recovery | `tests/net/test_tcp_loss.cpp` | `drivers-dev` | SACK + retransmit restores data | 
| MTU discovery | `tests/net/test_mtu.sh` | `drivers-dev` | Path MTU discovery, no fragment issues | 
| Long-duration TCP (24h) | `tests/net/test_tcp_soak.sh` | `drivers-dev` | No memory leak, no connection drop | 
| Wi-Fi disconnect/reconnect | `tests/net/test_wifi_roam.sh` | `drivers-dev` | Re-associate, re-DHCP, resume transfers | 
| BharatNet latency | `tests/net/bench_bharatnet.sh` | `release/mobile` | < 100 ms RTT on BharatNet PoP | 

---

## India-Specific Quality Standards

### IQ1 — Regulatory Compliance Quality

Every profession app must meet these standards before shipping:

| Standard | App scope | Test | Branch | 
| ---------- | ----------- | ------ | -------- | 
| GSTN API sandbox pass | sigma-ca, sigma-accounts | `tests/india/test_gstn_sandbox.sh` | `release/standalone` | 
| ABDM FHIR R4 compliance | sigma-health | `tests/india/test_abdm_fhir.sh` | `release/standalone` | 
| NMC prescription format | sigma-health | `tests/india/test_nmc_prescription.sh` | `release/standalone` | 
| BNS/BNSS 2023 section map accuracy | sigma-legal, sigma-police | `tests/india/test_bns_map.sh` | `release/standalone` | 
| MSP data accuracy (FY2025-26) | sigma-agri | `tests/india/test_msp_values.cpp` | `release/standalone` | 
| PMFBY premium formula | sigma-agri | `tests/india/test_pmfby_calc.cpp` | `release/standalone` | 
| GST HSN/SAC code accuracy | sigma-accounts, sigma-ca | `tests/india/test_hsn.sh` | `release/standalone` | 
| EPFO/ESIC ECR format | sigma-hrms | `tests/india/test_ecr_format.sh` | `release/standalone` | 
| DigiLocker API mock | all apps | `tests/india/test_digilocker_mock.sh` | `release/standalone` | 

### IQ2 — Indian Language Quality

| Test | File | Branch | Pass criteria | 
| ------ | ------ | -------- | --------------- | 
| Devanagari render (no tofu) | `tests/ui/test_font_hindi.sh` | `release/standalone` | All Unicode Devanagari block renders | 
| Tamil render | `tests/ui/test_font_tamil.sh` | `release/standalone` | All Tamil Unicode block renders | 
| Telugu / Bengali / Marathi | `tests/ui/test_font_multi.sh` | `release/standalone` | 5 major languages render without fallback | 
| sigma-bhashini ASR WER < 15% | `tests/ai/test_bhashini_asr.sh` | `release/standalone` | Hindi ASR Word Error Rate < 15% | 
| Phonetic IME accuracy | `tests/ime/test_phonetic.cpp` | `release/standalone` | "namaste" → "नमस्ते" correct | 
| Inscript keyboard layout | `tests/ime/test_inscript.cpp` | `release/standalone` | 47-key Inscript layout maps correctly | 
| CLI output in Hindi | `tests/cli/test_hindi_output.sh` | `release/standalone` | `SIGMA_LANG=hi sigma-agri msp` prints Hindi | 

### IQ3 — India Stack Integration Tests

```bash
# All run against government sandboxes (not production):
./tests/india/test_gstn_sandbox.sh       # GSTR compute + file
./tests/india/test_abdm_sandbox.sh       # ABHA create + FHIR push
./tests/india/test_upi_sandbox.sh        # UPI collect + confirm
./tests/india/test_digilocker_mock.sh    # Fetch/verify document
./tests/india/test_aadhaar_offline.sh    # Offline eKYC XML parse
./tests/india/test_navic_parse.sh        # NavIC NMEA sentence parse
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| GSTN sandbox CI | `tests/india/test_gstn_sandbox.sh` | `release/standalone` | Weekly scheduled CI job | 
| ABDM sandbox CI | `tests/india/test_abdm_sandbox.sh` | `release/standalone` | Weekly scheduled CI job | 
| Offline India data freshness | `tests/india/test_data_freshness.sh` | `release/standalone` | MSP/HSN/ICD-10 data not older than 1 year | 

---

## Testing Infrastructure Roadmap

### TI1 — Test Harness Completeness

**Current:** Tests in `tests/unit/`, `tests/kernel/`, `tests/fuzz/`, `tests/integration/`,
`tests/net/`, `tests/openqa/`, `tests/posix/`, `tests/regression/`. Some are real, some empty.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `tests/Makefile` run all suites | `tests/Makefile` | all | `make -C tests all` runs every suite | 
| Test result XML (JUnit format) | `tests/Makefile` | all | GitHub Actions can parse JUnit XML | 
| Coverage report (gcov/llvm-cov) | `.github/workflows/sigma_ci.yml` | `kernel-exp` | Coverage HTML published to gh-pages | 
| Property-based testing (QuickCheck-style) | `tests/unit/sigma_proptest.cpp` | `kernel-exp` | Random inputs, verify invariants | 
| `tests/chaos/` infrastructure | `tests/chaos/run_all.sh` | all | Orchestrate chaos tests safely | 
| Physical hardware CI farm | `tests/hardware/` | `prepare-sigmaos-launch` | Raspberry Pi 4, x86 laptop, NVMe drive | 
| CI timing budgets | `.github/workflows/sigma_ci.yml` | all | Fail if PR CI exceeds 15 min | 

### TI2 — OpenQA Integration (existing `tests/openqa/`)

**Current:** `tests/openqa/sigma_scenarios.py` exists — QEMU boot is `echo` only.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Wire `boot_default` to real QEMU | `tests/openqa/sigma_scenarios.py` | all | Remove echo stub, real `qemu-system-x86_64` | 
| `zerotrust_revoke` scenario | `tests/openqa/sigma_scenarios.py` | all | Revoke DID mid-session, verify access denied | 
| `pkg_verity_tamper` scenario | `tests/openqa/sigma_scenarios.py` | all | Corrupt .spkg, verify dm-verity detects | 
| `kpatch_unsigned_reject` scenario | `tests/openqa/sigma_scenarios.py` | all | Attempt unsigned kpatch, verify rejected | 
| `profession_sigma_ca` scenario | `tests/openqa/sigma_scenarios.py` | `release/standalone` | sigma-ca GST compute end-to-end | 
| `wifi_connect_dhcp` scenario | `tests/openqa/sigma_scenarios.py` | `drivers-dev` | Connect Wi-Fi, get DHCP IP, ping gateway | 
| Screenshot comparison | `tests/openqa/sigma_scenarios.py` | `release/standalone` | Verify Zenith renders correctly (pixel diff) | 

### TI3 — Continuous Benchmarking

```bash
# Run on every merge to main — results stored in git notes:
./tests/perf/bench_boot.sh          # boot time
./tests/perf/bench_sched.cpp        # context switch latency
./tests/perf/bench_pqc.cpp          # Kyber/Dilithium ops/sec
./tests/perf/bench_net.sh           # network throughput
./tests/perf/bench_io.sh            # disk IOPS
./tests/perf/bench_sigma_ca.sh      # GST compute throughput
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Benchmark result storage | `.github/workflows/sigma_ci.yml` | `performance-optimized` | Store JSON in git notes, plot trend | 
| Regression alert | `.github/workflows/sigma_ci.yml` | `performance-optimized` | Alert if any metric degrades > 10% vs last week | 
| Dashboard at sigmaos.dev/perf | `gh-pages` | `gh-pages` | D3.js time-series for all 6 metrics | 

---

## Hardware Compatibility Matrix

### HC1 — Verified Hardware (target for v16.0 Apex)

| Category | Device | Status | Branch | 
| ---------- | -------- | -------- | -------- | 
| **x86-64 Desktop** | Generic Intel i5/i7 (UHD 630) | 🎯 Target | `drivers-dev` | 
| **x86-64 Laptop** | ThinkPad X1 Carbon (11th gen) | 🎯 Target | `drivers-dev` | 
| **x86-64 Budget** | Any with e1000 NIC + VESA | ⚠️ Partial | `kernel-exp` | 
| **QEMU/KVM** | `qemu-system-x86_64` | ✅ Supported | all | 
| **VirtualBox** | x86-64 guest | 🎯 Target | `kernel-exp` | 
| **ARM64** | Raspberry Pi 4 (BCM2711) | 🎯 v16.0 target | `release/mobile` | 
| **ARM64** | Raspberry Pi 5 (BCM2712) | 🎯 v17.0 target | `release/mobile` | 
| **ARM64 tiny** | Raspberry Pi Zero 2W | 🎯 sigma-ultra target | `release/mobile` | 
| **RISC-V** | StarFive VisionFive 2 | 🔮 v17.0 target | `release/mobile` | 
| **India laptop** | JioBook (ARM, mt7921 WiFi) | 🎯 Critical target | `release/mobile` | 

### HC2 — Hardware Compatibility Test Matrix

For each verified device, CI must pass:

```
✅ Boot to shell prompt
✅ Network connectivity (ping gateway)
✅ Storage read/write (512 MB file roundtrip)
✅ Display output (Zenith renders at native resolution)
✅ Keyboard input (sigma-sh processes typed commands)
✅ sigma-pkg install (one package from repo)
✅ sigma-agri msp --crop wheat (offline profession tool)
✅ Suspend/resume (S3) without data loss
✅ 60-minute soak (no crash, no memory leak)
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Hardware CI matrix definition | `.github/workflows/sigma_qemu.yml` | all | Add matrix per target device | 
| `tests/hardware/verify_device.sh` | `tests/hardware/verify_device.sh` | all | Run all 9 checks above on real device | 
| Compatibility report wiki | `wiki_repo/Hardware-Compatibility.md` | `docs-update` | Device × feature × status table | 

---

## Long-Term Engineering Excellence

### LE1 — Formal Verification (Phase 9, Month 36–60)

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| sigma-bus IPC Frama-C proofs | `kernel/ipc/sigma_bus.cpp` + Frama-C | `release/microkernel` | Prove no deadlock, bounded message delay | 
| Scheduler no-starvation proof | `kernel/sched/sigma_mcs.cpp` | `release/microkernel` | Frama-C WP: every task gets CPU in bounded time | 
| sigma-trustd certificate chain | `security/SovereignDID.cpp` | `release/standalone` | Alloy model of DID trust chain | 
| Memory allocator correctness | `kernel/memory/sigma_allocator.cpp` | `kernel-exp` | Frama-C: no double-free, no use-after-free | 
| IIT/IISc collaboration | Research MOU | Phase 9 | Publish proofs at USENIX/IEEE S&P | 

### LE2 — Rust Migration (Phase 9, Month 36–60)

**Goal:** Zero memory-safety CVEs for 12 consecutive months.

| Component | Rust file | Branch | Priority | 
| ----------- | ----------- | -------- | ---------- | 
| sigma-net (TCP/IP stack) | `net/rust/sigma_net.rs` | `kernel-exp` | Highest — most attack surface | 
| sigma-fs (VFS layer) | `fs/rust/sigma_vfs.rs` | `fs-dev` | High — file corruption risk | 
| SDF driver framework | `hal/rust/sigma_sdf.rs` | `drivers-dev` | High — ring-3 driver safety | 
| sigma-tls | `net/rust/sigma_tls.rs` | `drivers-dev` | High — crypto correctness | 
| sigma-pkg package manager | `userland/rust/sigma_pkg.rs` | `tools-dev` | Medium — supply chain | 
| sigma-cli | `userland/rust/sigma_cli.rs` | `tools-dev` | Low — minimal attack surface | 

**Milestones:**
```
Month 36: sigma-net in Rust — no unsafe blocks except FFI boundary
Month 42: sigma-fs in Rust
Month 48: SDF framework in Rust
Month 54: sigma-tls + sigma-pkg in Rust
Month 60: 0 memory-safety CVEs for 12 months in Rust components
```

### LE3 — Zero-Trust Security (permanent target)

| Principle | Implementation | Status | 
| ----------- | --------------- | -------- | 
| Every process sandboxed from first syscall | sigma-mac + seccomp | `[~]` partial | 
| No implicit trust between components | sigma-bus capability tokens | `[~]` partial | 
| Every package Dilithium3-signed | sigma-pkg verify | `[~]` partial | 
| Every network connection PQC-TLS | sigma-tls with ML-KEM | `[ ]` missing | 
| Every audit event DID-signed | sigma-audit | `[x]` done | 
| No root/administrator account | DID + capability model | `[~]` partial | 
| Immutable root filesystem | dm-verity + read-only mount | `[~]` partial | 
| Attestable boot chain | sigma-boot.efi + TPM2 | `[ ]` missing | 

---

## Quality KPIs — Measurable Targets by Version

### v15.1 (Current Release) — Minimum Viable Quality

| KPI | Target | Measurement | 
| ----- | -------- | ------------- | 
| Boot success rate (QEMU CI) | ≥ 95% | GitHub Actions pass rate | 
| Unit test pass rate | 100% | `ctest --output-on-failure` | 
| Static analysis warnings | 0 errors, < 50 warnings | `clang-tidy` output | 
| Kernel stub count | < 100 | `make check-stubs` | 
| CURRENT_PROBLEMS open 🔴 items | ≤ 5 | `CURRENT_PROBLEMS_MANIFEST.md` | 
| Wiki pages with broken links | 0 | `sigma-docs check` | 

### v16.0 Apex (6 months) — Production Ready

| KPI | Target | Measurement | 
| ----- | -------- | ------------- | 
| Boot time (NVMe) | < 2 s | CI timer | 
| Context switch p99 | < 100 ns | `tests/perf/bench_sched` | 
| QEMU test pass rate | 100% | CI | 
| Profession app test pass | 100% | `tests/india/` | 
| Fuzz time (weekly) | 10 min / target | AFL++ nightly | 
| Open critical bugs | 0 | GitHub Issues | 
| sigma-pkg install success rate | ≥ 99.9% | Package install CI | 
| WCAG 2.2 AA automated pass | 100% | aXe CI scan | 

### v17.0 Sovereign (18 months) — Enterprise Grade

| KPI | Target | Measurement | 
| ----- | -------- | ------------- | 
| Boot time (NVMe) | < 1.5 s | CI timer | 
| Context switch p99 | < 50 ns | `tests/perf/bench_sched` | 
| Kyber-1024 ops/sec | ≥ 5.8 M | `tests/perf/bench_pqc` | 
| MTBF kernel | > 10,000 hours | Soak test logs | 
| Real hardware CI | RPi4 + x86 laptop | Physical CI farm | 
| sigma-fleet managed devices | 1,000 | BharatOS pilot | 
| Rust components | sigma-net + sigma-fs | Code audit | 

### v18.0 Singularity (36 months) — World-Class

| KPI | Target | Measurement | 
| ----- | -------- | ------------- | 
| Boot time | < 1 s | CI timer | 
| Context switch p99 | < 50 ns | `tests/perf/bench_sched` | 
| Memory-safety CVEs (12 months) | 0 | CVE tracker | 
| Formal verification | IPC + scheduler proved | Published paper | 
| WCAG 2.2 AA manual audit | Pass | External audit | 
| Indian language support | 22 languages | Language coverage CI | 

---

## CURRENT_PROBLEMS_MANIFEST.md — Phase Q Updates

The following quality issues must be added to the manifest and tracked:

```markdown
## Phase Q (Quality & Stability) — Open

| ID | Area | Priority | File | Status | 
| ---- | ------ | ---------- | ------ | -------- | 
| Q-01 | QEMU CI tests use echo stubs, not real QEMU | 🔴 | sigma_qemu.yml | Open | 
| Q-02 | CryptFS derive_key() returns zeros (Issue #44) | 🔴 | SovereignCryptFS.cpp | Open | 
| Q-03 | Kyber/Dilithium: PRNG not real NTT | 🔴 | SovereignKyber.cpp | Open | 
| Q-04 | Syscall stubs return 0 silently instead of ENOSYS | 🔴 | sigma_syscalls.cpp | Open | 
| Q-05 | No kernel panic handler (no register dump) | 🔴 | sigma_panic.cpp | Open | 
| Q-06 | VMM page tables are stubs (virtual=physical) | 🔴 | sigma_vmm.cpp | Open | 
| Q-07 | Shell REPL reads empty line (no TTY) | 🟠 | sigma_shell.cpp | Open | 
| Q-08 | No compositor alpha blend (composite_window empty) | 🟠 | sigma_compositor.cpp | Open | 
| Q-09 | NIC/NVMe DMA is simulated (no real ring) | 🟠 | SovereignNIC/NVMe | Open | 
| Q-10 | Attestation always returns true | 🟠 | SovereignAttestation.cpp | Open | 
| Q-11 | No OOBE / first-boot wizard | 🟡 | sigma_oobe.cpp | Open | 
| Q-12 | App launcher not implemented | 🟡 | sigma_launcher.cpp | Open | 
| Q-13 | Braille display not supported | 🟡 | sigma_braille.cpp | Open | 
| Q-14 | 22-language UI strings not written | 🟡 | sigma_l10n.cpp | Open | 
```

---

## scripts/sigma_quality_check.sh (new script)

```bash
#!/usr/bin/env bash
# SigmaOS quality gate script — run before any PR merge to main
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "[sigma-quality] Running all quality gates..."

FAIL=0

# 1. Stub count
STUBS=$(grep -r "return 0; // stub\ | TODO\ | FIXME\ | return -1; // NYI" \
  "$ROOT/kernel" "$ROOT/net" "$ROOT/drivers" --include="*.cpp" --include="*.c" \
  2>/dev/null | wc -l)
echo "[sigma-quality] Open stubs: $STUBS"
[ "$STUBS" -lt 200 ] | | { echo "FAIL: too many stubs ($STUBS)"; FAIL=1; }

# 2. SPDX headers
MISSING=$(find "$ROOT/kernel" "$ROOT/userland" "$ROOT/crypto" \
  -name "*.cpp" -o -name "*.h" -o -name "*.c" 2>/dev/null | \
  xargs grep -rL "SPDX-License-Identifier" 2>/dev/null | wc -l)
echo "[sigma-quality] Files missing SPDX: $MISSING"
[ "$MISSING" -eq 0 ] | | { echo "WARN: $MISSING files missing SPDX"; }

# 3. Hardcoded credentials
if grep -rn 'password\s*=\s*"[^"]\{4,\}"\ | secret\s*=\s*"[^"]\{4,\}"' \
  "$ROOT/kernel" "$ROOT/userland" --include="*.cpp" --include="*.h" \
  2>/dev/null | grep -v "test\ | example\ | stub"; then
  echo "FAIL: hardcoded credentials found"; FAIL=1
fi

# 4. CURRENT_PROBLEMS open critical items
CRITICAL=$(grep -c "🔴" "$ROOT/CURRENT_PROBLEMS_MANIFEST.md" 2>/dev/null | | echo 0)
echo "[sigma-quality] Open critical problems: $CRITICAL"
[ "$CRITICAL" -le 5 ] | | { echo "WARN: $CRITICAL critical problems open"; }

# 5. Wiki sync check
WIKI_BEHIND=$(git -C "$ROOT/wiki_repo" log --oneline HEAD..origin/main 2>/dev/null | wc -l)
[ "$WIKI_BEHIND" -eq 0 ] | | echo "WARN: wiki is $WIKI_BEHIND commits behind origin"

echo "[sigma-quality] Done. FAIL=$FAIL"
exit "$FAIL"
```

**New file:** `scripts/sigma_quality_check.sh`
**Used by:** `sigma-automation.sh quality-check` and CI pre-merge gate.

---

*See also: [Quality Stability Performance Roadmap](Quality-Stability-Performance-Roadmap) · [Gap Analysis](Gap-Analysis) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [India Profession Tools Roadmap](India-Profession-Tools-Roadmap) · [Development Roadmap](Development-Roadmap)*
