# SigmaOS — Advanced Quality Roadmap

## Security Hardening · Networking Sovereignty · Enterprise · AI/ML Quality

## Internationalisation · Education Stack · Community & Ecosystem

Continues from [Quality-Stability-Performance-Roadmap](Quality-Stability-Performance-Roadmap),
[Stability-Performance-Extended](Stability-Performance-Extended), and
[Compatibility-Automation-Personalisation-Roadmap](Compatibility-Automation-Personalisation-Roadmap).

---

## 1. Security Hardening Depth

### SH1 — Post-Quantum Cryptography (Production Grade)

**Current state:** Kyber + Dilithium use PRNG placeholders — not cryptographically
secure. Headers define correct API, bodies simulate via splitmix64.

#### Immediate fixes (blocks every security claim)

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Integrate liboqs Kyber-1024 NTT | `crypto/SovereignKyber.cpp` | `performance-optimized` | Replace `kyber_gen_matrix` with `pqcrystals_kyber1024_ref_keypair` |
| Integrate liboqs Dilithium-5 | `crypto/SovereignDilithium5.cpp` | `performance-optimized` | Replace LFSR XOF with `pqcrystals_dilithium5_ref_keypair` |
| SHAKE-256 / SHA-3 (Keccak) | `crypto/sigma_sha3.cpp` | `performance-optimized` | Keccak-f\[1600\] state machine, no external dep |
| Fix CryptFS `derive_key()` (Issue #44) | `crypto/SovereignCryptFS.cpp` | `kernel-exp` | Real Argon2id: time=3, memory=65536, threads=4 |
| Constant-time byte comparison | `crypto/sigma_ct_compare.cpp` | `performance-optimized` | `sigma_ct_memcmp` using CMOV — prevents timing oracle |
| ML-KEM FIPS 203 final bindings | `crypto/sigma_mlkem.cpp` | `performance-optimized` | Replace draft Kyber with NIST FIPS 203 final API |
| ML-DSA FIPS 204 final bindings | `crypto/sigma_mldsa.cpp` | `performance-optimized` | Replace draft Dilithium with NIST FIPS 204 final API |
| SLH-DSA FIPS 205 (hash-based sig) | `crypto/sigma_slhdsa.cpp` | `performance-optimized` | XMSS-like; no lattice assumptions — for code signing |
| PQC key rotation on demand | `userland/tools/sigma_pqc_cli.cpp` | `tools-dev` | `sigma-pqc rotate` generates new keypair, re-signs config |
| Hybrid TLS handshake (X25519 + ML-KEM) | `net/tls/sigma_tls.cpp` | `drivers-dev` | RFC draft hybrid KEM for TLS 1.3 |

#### PQC performance benchmarks (CI gates)

| Algorithm | FIPS std | Target (AVX-512) | Target (NEON) | CI test |
|-----------|---------|-----------------|---------------|---------|
| ML-KEM-1024 KeyGen | FIPS 203 | ≥ 10 M ops/sec | ≥ 3 M ops/sec | `tests/perf/bench_pqc.cpp` |
| ML-KEM-1024 Encap | FIPS 203 | ≥ 8 M ops/sec | ≥ 2.5 M ops/sec | `tests/perf/bench_pqc.cpp` |
| ML-DSA-87 Sign | FIPS 204 | ≥ 2 M sig/sec | ≥ 600 K sig/sec | `tests/perf/bench_pqc.cpp` |
| ML-DSA-87 Verify | FIPS 204 | ≥ 4 M ver/sec | ≥ 1.2 M ver/sec | `tests/perf/bench_pqc.cpp` |

### SH2 — Mandatory Access Control (sigma-mac)

**Current:** `kernel/security/sigma_mac.cpp` header complete. No enforcement at syscall boundary.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| MAC label assignment at `execve` | `kernel/security/sigma_mac.cpp` | `kernel-exp` | Read `.sigma-policy` label for executable path |
| MAC check at every syscall | `kernel/security/sigma_mac.cpp` | `kernel-exp` | Before `open/read/write/exec` — check subject→object label |
| AVC cache (< 1 µs per check) | `kernel/security/sigma_mac_avc.cpp` | `kernel-exp` | Hash table: (subject, object, perm) → allow/deny |
| Policy compiler | `userland/tools/sigma_mac_compile.cpp` | `tools-dev` | Compile `.sigma-policy` TOML → binary rule set |
| AI policy suggester | `userland/ai/sigma_mac_suggest.cpp` | `release/standalone` | `sigma-sec mac suggest --app firefox` → suggested policy |
| Policy hot-reload | `kernel/security/sigma_mac.cpp` | all | Reload without reboot via `sigma-sec mac reload` |
| Violation logging | `kernel/security/sigma_mac.cpp` | all | Every denied syscall → sigma-audit entry |
| Policy test suite | `tests/security/test_mac.sh` | all | 20 known allow/deny patterns verified |

### SH3 — Kernel Memory Safety

| Task | File | Branch | Detail |
|------|------|--------|--------|
| KASLR (kernel address randomisation) | `kernel/core/sigma_start.cpp` | `kernel-exp` | RDRAND-seeded base offset at boot |
| W^X enforcement (no RWX pages) | `kernel/mm/sigma_vmm.cpp` | `kernel-exp` | No page simultaneously writable and executable |
| CET shadow stack (Intel) | `arch/x86_64/sigma_cet.asm` | `kernel-exp` | Write `MSR_IA32_S_CET`; shadow stack for ROP mitigation |
| Stack canaries on all kernel code | `Makefile` | `kernel-exp` | `-fstack-protector-strong` on all kernel `.cpp` / `.c` |
| ASLR for all user processes | `kernel/core/sigma_mm.cpp` | `kernel-exp` | Randomise heap + stack + mmap base per process |
| Heap guard pages | `klib/sigma_slab_debug.cpp` | `kernel-exp` | 4 KB guard page after every slab allocation |
| Kernel CFI (Control-Flow Integrity) | `Makefile` | `kernel-exp` | `-fsanitize=cfi` on kernel builds |
| NX bit on data pages | `kernel/mm/sigma_vmm.cpp` | `kernel-exp` | Set `NX` bit in all non-executable page table entries |

### SH4 — Secure Supply Chain

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Dilithium3-sign every `.spkg` | `scripts/sign_release.sh` | `prepare-sigmaos-launch` | CI gate: unsigned package cannot install |
| dm-verity on every package extract | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Hash tree check before extracting to VFS |
| SBOM (CycloneDX) generation | `scripts/gen_sbom.sh` | `prepare-sigmaos-launch` | Every release includes machine-readable SBOM |
| Reproducible builds (bit-for-bit) | `.github/workflows/sigma_ci.yml` | all | Two builds → identical SHA256 |
| Dependency pinning in recipes | `sigma_pkg_registry/recipes/` | `tools-dev` | All dependencies use exact version, not `>=` |
| Source provenance tracking | `sigma_pkg_registry/` | `tools-dev` | Every package links to signed source commit |
| Typosquatting detection | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Warn if package name is 1-edit-distance from known package |

### SH5 — Secure Boot & Attestation

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-boot.efi (UEFI loader) | `sigma-boot/sigma_boot.c` | `kernel-exp` | EDK2 UEFI application — Phase 0 critical |
| Kernel ML-DSA-87 signature verify | `sigma-boot/sigma_secboot.c` | `kernel-exp` | Verify kernel ELF before loading via sigma-boot |
| TPM2 PCR measurement chain | `security/SovereignTPM.cpp` | `kernel-exp` | PCR 0 (firmware) + PCR 4 (bootloader) + PCR 8 (kernel) |
| TPM2 seal CryptFS key to PCR | `security/SovereignTPM.cpp` | `kernel-exp` | `TPM2_CC_Seal` — key only unseals if PCRs match boot state |
| Remote attestation (sigma-trustd) | `security/SovereignAttestation.cpp` | `release/cloud` | Enterprise: verify device state without physical access |
| `sigma-boot status` — full chain view | `userland/tools/sigma_boot_cli.cpp` | `kernel-exp` | Display every PCR value + signature verification status |
| Secure boot CI gate | `.github/workflows/sigma_ci.yml` | `prepare-sigmaos-launch` | Fail if any boot chain signature unverified |

---

## 2. Networking Sovereignty Roadmap

### NS1 — Network Stack Completeness

**Current:** Zero-copy packet queue + TCP header parse. No state machine.

```
Completeness progress:
  Layer 2 (Ethernet): ✅ e1000 TX/RX wired
  Layer 3 (IPv4/ICMP): ✅ ICMP echo, ⚠️ IP fragmentation stub
  Layer 3 (IPv6): ❌ missing
  Layer 4 (TCP): ❌ no state machine
  Layer 4 (UDP): ❌ missing
  Layer 4 (QUIC): ❌ missing
  Layer 5 (TLS 1.3): ❌ missing
  Layer 7 (DNS/DHCP): ❌ missing
```

| Priority | Task | File | Branch |
|----------|------|------|--------|
| 🔴 | TCP state machine (RFC 793) | `net/tcp/sigma_tcp.cpp` | `drivers-dev` |
| 🔴 | UDP socket layer | `kernel/net/sigma_net_socket.cpp` | `drivers-dev` |
| 🔴 | DNS resolver (stub + recursive) | `net/dns/sigma_dns.cpp` | `drivers-dev` |
| 🔴 | DHCP client | `net/dhcp/sigma_dhcp_client.cpp` | `drivers-dev` |
| 🟠 | IPv6 (SLAAC + DHCPv6 + ICMPv6) | `kernel/net/sigma_net_ipv6.cpp` | `drivers-dev` |
| 🟠 | TLS 1.3 (X25519 + ML-KEM hybrid) | `net/tls/sigma_tls.cpp` | `drivers-dev` |
| 🟠 | ARP cache with 30 s timeout | `kernel/net/sigma_net_arp.cpp` | `drivers-dev` |
| 🟡 | QUIC transport (HTTP/3) | `net/quic/sigma_quic.cpp` | `drivers-dev` |
| 🟡 | VPN (WireGuard-style) | `net/vpn/sigma_vpn.cpp` | `drivers-dev` |

### NS2 — Sigma-native Network Services

| Service | File | Branch | Detail |
|---------|------|--------|--------|
| sigma-netd (network manager daemon) | `userland/daemons/sigma_netd.cpp` | `drivers-dev` | Manage interfaces, DHCP leases, DNS cache |
| sigma-firewall (stateful L4 filter) | `net/firewall/sigma_firewall.cpp` | `drivers-dev` | Connection tracking, iptables-compatible rules |
| sigma-dns-cache (local resolver) | `net/dns/sigma_dns_cache.cpp` | `drivers-dev` | Cache DNS responses, DoH upstream option |
| sigma-proxy (HTTP/SOCKS5 proxy) | `net/proxy/sigma_proxy.cpp` | `release/standalone` | Local SOCKS5 for app traffic isolation |
| sigma-vpn (WireGuard-compatible) | `net/vpn/sigma_vpn.cpp` | `release/standalone` | PQC-encrypted tunnel |
| sigma-zero-trust-net | `net/zero_trust/sigma_ztnet.cpp` | `release/cloud` | Per-pod network policy, no implicit trust |
| BharatNet PoP daemon | `userland/daemons/sigma_bharatnet.cpp` | `release/mobile` | TRAI PM WANI compliance, QoS HTB, NAT |
| sigma-commnet mesh | `net/mesh/sigma_mesh_net.cpp` | `release/distributed` | 802.11s mesh networking for village PoPs |

### NS3 — Network Quality & Testing

```bash

# Network quality benchmarks (CI):

sigma-net bench throughput --iface eth0      # iperf3 equivalent

sigma-net bench latency --host 8.8.8.8       # RTT histogram

sigma-net bench dns --resolver 1.1.1.1       # DNS query latency

sigma-net bench tls --url https://abdm.gov.in # TLS handshake time

```

| Task | File | Branch | Target |
|------|------|--------|--------|
| TCP throughput CI | `tests/net/bench_net.sh` | `drivers-dev` | ≥ 900 Mbps (1 GbE) inside QEMU |
| TCP latency CI | `tests/net/bench_latency.sh` | `drivers-dev` | p99 RTT < 1 ms on loopback |
| TLS handshake CI | `tests/net/bench_tls.sh` | `drivers-dev` | < 50 ms full handshake (PQC-TLS) |
| DHCP lease CI | `tests/net/test_dhcp.sh` | `drivers-dev` | DISCOVER→ACK < 2 s |
| IPv6 SLAAC CI | `tests/net/test_ipv6.sh` | `drivers-dev` | RA received, address configured < 5 s |
| Wi-Fi reconnect CI | `tests/net/test_wifi_roam.sh` | `drivers-dev` | Re-associate + DHCP < 10 s |
| BharatNet latency | `tests/net/bench_bharatnet.sh` | `release/mobile` | < 100 ms RTT on rural 4G fallback |

---

## 3. Enterprise Features Roadmap

### EF1 — Identity & Access Management

| Feature | File | Branch | Detail |
|---------|------|--------|--------|
| sigma-trustd DID server | `security/SovereignDID.cpp` | `release/cloud` | Resolve DIDs, issue/revoke credentials |
| Group policy via `.sigma-policy` | `userland/tools/sigma_sec_cli.cpp` | `release/cloud` | Signed TOML policies pushed by sigma-fleet |
| LDAP/AD bridge (read-only) | `userland/auth/sigma_ldap.cpp` | `release/cloud` | Government orgs can use existing AD for bootstrap |
| MFA: TOTP / FIDO2 | `userland/auth/sigma_mfa.cpp` | `release/standalone` | Time-based OTP + hardware key (YubiKey) |
| Role-based access control | `kernel/security/sigma_mac.cpp` | `release/cloud` | User roles map to MAC policy sets |
| Passwordless login everywhere | `userland/display/sigma_dm.cpp` | `release/standalone` | DID QR → session; no username/password ever |
| Certificate pinning for APIs | `net/tls/sigma_tls.cpp` | `release/standalone` | Pin GSTN, ABDM, UPI CA certificates |
| Session timeout + auto-lock | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Lock screen after 10 min idle |

### EF2 — sigma-fleet (Enterprise Device Management)

**Target:** Manage 10,000+ SigmaOS devices from a single console.

```bash

# Fleet management commands:

sigma-fleet register <server> <token>    # enroll device

sigma-fleet status                       # heartbeat + health

sigma-fleet policy get                   # fetch + apply .sigma-policy

sigma-fleet update pull                  # download OS update

sigma-fleet update apply                 # apply A/B update

sigma-fleet inventory                    # hardware fingerprint

sigma-fleet audit push                   # send audit log to server

sigma-fleet lock [--wipe]                # remote lock / wipe

sigma-fleet app deploy sigma-ca --all    # deploy profession app to fleet

sigma-fleet config set <key> <val>       # push Config.sigma key

sigma-fleet report compliance            # STQC / MeitY compliance report

```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-fleet agent daemon | `userland/daemons/sigma_fleet_agent.cpp` | `release/cloud` | Heartbeat every 60 s, poll for new policy/update |
| Fleet server (Go) | `sigmad/fleet/main.go` | `release/cloud` | REST API: device registry, policy push, audit collect |
| sigma-fleet CLI | `userland/tools/sigma_fleet_cli.cpp` | `release/cloud` | All commands above |
| Remote kpatch via fleet | `userland/daemons/sigma_fleet_agent.cpp` | `release/cloud` | Fleet server pushes kpatch → agent applies live |
| Compliance report generator | `userland/tools/sigma_fleet_cli.cpp` | `release/cloud` | Export MeitY/STQC compliance checklist as PDF |
| Fleet dashboard (sigma-observatory) | `userland/tools/sigma_observatory.cpp` | `release/cloud` | Map of all managed devices + health |

### EF3 — sigma-siem (Security Information & Event Management)

```bash
sigma-siem status                 # SIEM pipeline health

sigma-siem rules list             # detection rules

sigma-siem rules add <sigma_rule> # add Sigma detection rule

sigma-siem alerts list            # recent alerts

sigma-siem export <file>          # export in Splunk/ELK/CEF format

sigma-siem report cert-in         # CERT-In 6-hour disclosure format

```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-ids event collection | `kernel/security/sigma_ids.cpp` | `release/cloud` | sigma-ids → sigma-bus → sigma-siem pipeline |
| Sigma detection rules parser | `userland/tools/sigma_siem_cli.cpp` | `release/cloud` | Parse `.yml` Sigma rules → sigma-ids patterns |
| CERT-In JSON export | `userland/tools/sigma_siem_cli.cpp` | `release/cloud` | 6-hour mandatory disclosure format |
| OpenTelemetry export | `userland/sigma_otel_export.cpp` | `release/cloud` | Forward to Splunk/Grafana/Elastic |

### EF4 — BharatOS Pilot Readiness

**Target:** 1,000 NIC government machines (Phase 7, Month 30).

| Task | File | Branch | Detail |
|------|------|--------|--------|
| NIC single-sign-on integration | `userland/auth/sigma_nic_sso.cpp` | `release/cloud` | NIC SSO (eID) → sigma-trustd DID |
| GeM marketplace integration | `userland/apps/sigma-gov/sigma_gov.cpp` | `release/standalone` | sigma-gov gem order via GeM API |
| PFMS payment integration | `userland/apps/sigma-gov/sigma_gov.cpp` | `release/standalone` | Public Financial Management System |
| Government PKI (NIC CA) | `security/SovereignDID.cpp` | `release/cloud` | Accept NIC-issued certificates alongside Dilithium3 |
| `sigma-fleet deploy --profile bharatos` | `userland/daemons/sigma_fleet_agent.cpp` | `release/cloud` | Specialised government profile |
| MeitY empanelment checklist | `wiki_repo/MeitY-Compliance.md` | `docs-update` | Auto-generated from sigma-quality-check output |

---

## 4. AI/ML Integration Quality

### AI1 — sigma-ai Daemon Quality

**Current:** `userland/ai/` directory exists. No LLM backend. sigma-heal/sigma-lex reference sigma-ai but it's a stub.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| llama.cpp integration | `userland/ai/sigma_ai_llama.cpp` | `release/standalone` | `llama_backend_init()` + `llama_load_model_from_file()` |
| sigma-ai daemon IPC | `userland/ai/sigma_ai_daemon.cpp` | `release/standalone` | sigma-bus: `sigma_ai_ask(prompt, &response)` |
| Model quality test (Hindi WER) | `tests/ai/test_sarvam1_hindi.sh` | `release/standalone` | Sarvam-1: WER < 15% on Hindi test set |
| sigma-heal crash analysis | `userland/ai/sigma_heal_ai.cpp` | `release/standalone` | Feed kernel panic log → get diagnosis + fix suggestion |
| sigma-lex Gazette parser | `userland/ai/sigma_lex_ai.cpp` | `release/standalone` | GST rate change detected within 24 h of Gazette publication |
| Inference latency CI | `tests/ai/bench_inference.sh` | `release/standalone` | First-token latency < 500 ms on 4 GB RAM device |
| Context window management | `userland/ai/sigma_ai_llama.cpp` | `release/standalone` | 4096-token context, auto-evict oldest when full |
| Streaming output to sigma-sh | `userland/ai/sigma_ai_daemon.cpp` | `release/standalone` | Token-by-token streaming via sigma-bus |

### AI2 — Federated Learning Quality

| Task | File | Branch | Detail |
|------|------|--------|--------|
| FL coordinator server (Go) | `sigmad/fl/main.go` | `release/distributed` | `fl.sigmaos.dev` — manage rounds, aggregate gradients |
| Differential privacy (ε-DP) | `userland/ai/sigma_fedlearn.cpp` | `release/distributed` | Add calibrated Gaussian noise before gradient upload |
| sigma-tax-anomaly FL network | `userland/apps/sigma-ca/sigma_ca_fedlearn.cpp` | `release/standalone` | 100 CAs training GST error detector |
| sigma-crop-disease FL network | `userland/apps/sigma-agri/sigma_agri_fedlearn.cpp` | `release/standalone` | 1,000 farmers training crop disease model |
| Privacy audit | `tests/ai/test_fl_privacy.sh` | `release/distributed` | Verify no raw data leaves device |
| FL round convergence CI | `tests/ai/test_fl_convergence.sh` | `release/distributed` | Model converges in < 20 rounds |

### AI3 — sigma-bhashini Quality

| Test | File | Branch | Target |
|------|------|--------|--------|
| Hindi ASR WER | `tests/ai/test_bhashini_asr.sh` | `release/standalone` | < 15% Word Error Rate |
| Tamil ASR WER | `tests/ai/test_bhashini_asr.sh` | `release/standalone` | < 20% WER |
| Hindi TTS MOS score | `tests/ai/test_bhashini_tts.sh` | `release/standalone` | Mean Opinion Score ≥ 3.5/5 |
| ASR latency | `tests/ai/bench_asr.sh` | `release/standalone` | < 300 ms end-to-end on 4 GB RAM |
| TTS latency | `tests/ai/bench_tts.sh` | `release/standalone` | < 200 ms for 10-word sentence |
| 22-language offline bundle | `sigma_pkg_registry/recipes/sigma-bhashini-all.recipe` | `release/standalone` | All 22 installed via `sigma-pkg install sigma-bhashini-all` |

---

## 5. Internationalisation (i18n) Quality

### I18N1 — 22-Language UI Support

**Current:** `sigma_locale.h` exists. Translation strings not written. `sigma_l10n.cpp` missing.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Message catalogue format | `userland/locales/sigma_l10n.cpp` | `tools-dev` | `.po` / `.sigma-l10n` TOML catalogue |
| Hindi (hi) translation | `userland/locales/hi.sigma-l10n` | `release/standalone` | 200+ system messages |
| Tamil (ta) translation | `userland/locales/ta.sigma-l10n` | `release/standalone` | 200+ system messages |
| Telugu (te) translation | `userland/locales/te.sigma-l10n` | `release/standalone` | 200+ system messages |
| Bengali (bn) translation | `userland/locales/bn.sigma-l10n` | `release/standalone` | 200+ system messages |
| Marathi (mr) translation | `userland/locales/mr.sigma-l10n` | `release/standalone` | 200+ system messages |
| Auto-detect locale at boot | `userland/ignite/sigma_ignite.cpp` | `release/standalone` | Read `SIGMA_LANG` env or profile preferred_lang |
| Locale-aware date/time/number | `userland/locales/sigma_l10n.cpp` | `release/standalone` | ₹ currency, dd/mm/yyyy date, 12h/24h |
| RTL support (Urdu, Sindhi) | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | Bidirectional text rendering via HarfBuzz |
| Translation CI | `tests/i18n/test_translations.sh` | `release/standalone` | Verify 0 untranslated strings in shipped languages |
| Community translation platform | `wiki_repo/Translation-Guide.md` | `docs-update` | Guide for community translators |

### I18N2 — Input Method Quality

| Test | File | Branch | Target |
|------|------|--------|--------|
| Inscript-Devanagari layout | `tests/ime/test_inscript.cpp` | `release/standalone` | 47-key layout: every key produces correct Unicode |
| Phonetic-Hindi round-trip | `tests/ime/test_phonetic.cpp` | `release/standalone` | 1,000 words: "namaste" → "नमस्ते", 0 errors |
| Tamil 99 keyboard | `tests/ime/test_tamil99.cpp` | `release/standalone` | Tamil 99 standard layout compliance |
| Conjunct consonant rendering | `tests/ui/test_conjuncts.sh` | `release/standalone` | ट्र, क्ष, ज्ञ render correctly |
| IME switch latency | `tests/ime/bench_ime.sh` | `release/standalone` | < 50 ms to switch between languages |
| Voice-to-text accuracy | `tests/ai/test_bhashini_ime.sh` | `release/standalone` | sigma-bhashini ASR → IME: WER < 20% |

### I18N3 — Date, Time & Legal Calendar

```bash

# India-specific temporal features:

sigma-cal show                     # Indian national calendar (Saka)

sigma-cal holidays 2026            # all central + state public holidays

sigma-cal gst-due 2026-07          # GST filing deadlines for July 2026

sigma-cal court-vacation --state MH  # Maharashtra court vacation calendar

sigma-cal itr-due 2025-26          # ITR filing deadline for AY2025-26

sigma-cal pmfby-window --state PB   # PMFBY enrollment window Punjab

```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Saka national calendar | `userland/locales/sigma_cal.cpp` | `release/standalone` | Indian national calendar alongside Gregorian |
| Public holiday database | `userland/locales/sigma_cal.cpp` | `release/standalone` | Central + all 28 states + 8 UTs offline SQLite |
| GST compliance calendar | `userland/apps/sigma-ca/sigma_ca.cpp` | `release/standalone` | GSTR-1/3B/9 deadlines per GSTIN registration type |
| Court calendar integration | `userland/apps/sigma-legal/sigma_legal.cpp` | `release/standalone` | eCourts vacation + working day calculator |

---

## 6. Education & Rural Stack Quality

### EDU1 — sigma-edu Quality

**Current:** `userland/apps/sigma-edu/sigma_edu.cpp` — partial implementation.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| UDISE+ school data API | `sigma_edu.cpp` | `release/standalone` | Pull school-level stats from UDISE+ |
| DIKSHA content integration | `sigma_edu.cpp` | `release/standalone` | Offline download of DIKSHA e-textbooks |
| NISHTHA teacher training | `sigma_edu.cpp` | `release/standalone` | Link NISHTHA course completion to service book |
| NIPUN Bharat literacy tracker | `sigma_edu.cpp` | `release/standalone` | Grade 1-3 foundational literacy progress |
| Offline exam mode | `sigma_edu.cpp` | `release/mobile` | Conduct exams without internet |
| Parent notification via sigma-bhashini | `sigma_edu.cpp` | `release/standalone` | Voice SMS in parent's language |
| PM POSHAN meal attendance | `sigma_edu.cpp` | `release/standalone` | Daily meal count → NIC PM POSHAN API |
| sigma-gamelearn integration | `sigma_edu.cpp` | `release/standalone` | Gamified learning via sigma-gamelearn |

### RU1 — sigma-gram Rural Stack Quality

| Task | File | Branch | Detail |
|------|------|--------|--------|
| MGNREGS job card API | `userland/apps/sigma-gram/sigma_gram.cpp` | `release/mobile` | NREGASoft `POST /attendance` |
| PM Gram Sadak Yojana | `sigma_gram.cpp` | `release/mobile` | PMGSY road project status + complaint |
| Jal Jeevan Mission dashboard | `sigma_gram.cpp` | `release/mobile` | JJM sensor data → water supply hours |
| e-Shram unorganised worker | `sigma_gram.cpp` | `release/mobile` | e-Shram registration + portability |
| DBT subsidy tracker | `sigma_gram.cpp` | `release/mobile` | PM-KISAN, PMJAY, PMAY status in one view |
| MGNREGS payment < 24 hours | `sigma_gram.cpp` | `release/mobile` | Attendance → payment in < 24 h (vs current 7–30 days) |
| Offline-first: 100% without internet | `sigma_gram.cpp` | `release/mobile` | All forms usable offline, sync on next connect |
| Aadhaar eKYC for shared device | `sigma_gram.cpp` | `release/mobile` | OTP-based identity on panchayat terminal |

### RU2 — sigma-ultra (feature phone) Quality

| Test | File | Branch | Target |
|------|------|--------|--------|
| USSD menu boot time | `tests/mobile/bench_ultra.sh` | `release/mobile` | < 2 s from power-on to menu |
| 2G data efficiency | `tests/mobile/test_2g_compression.sh` | `release/mobile` | LZ4-compressed API responses < 1 KB |
| UPI USSD (`*99#`) | `tests/mobile/test_upi_ussd.sh` | `release/mobile` | Pay flow completes in < 10 USSD messages |
| Offline MSP lookup | `tests/mobile/test_msp_offline.sh` | `release/mobile` | Works with no network (embedded table) |
| Battery life (Pi Zero 2W) | `tests/perf/bench_power.sh` | `release/mobile` | > 8 hours on 5,000 mAh bank |
| Hindi USSD messages | `tests/i18n/test_ussd_hindi.sh` | `release/mobile` | All menus available in Hindi |

---

## 7. Community & Ecosystem Quality

### CE1 — Contribution Quality Standards

| Standard | Tool | Branch | Detail |
|----------|------|--------|--------|
| Commit message convention | `.conform.yaml` | all | Conventional commits: `feat/fix/docs/perf/security` |
| Max PR size: 500 lines | `.github/PULL_REQUEST_TEMPLATE.md` | all | Large PRs must be split; CI warns if > 500 lines |
| Every PR has test evidence | `.github/PULL_REQUEST_TEMPLATE.md` | all | Mandatory checkbox: "Tests added / updated" |
| Every PR updates docs | `.github/PULL_REQUEST_TEMPLATE.md` | all | Mandatory checkbox: "Docs updated" |
| Review turnaround < 48 h | Community policy | all | Maintainers commit to 48-hour first review |
| Good-first-issues labelled | GitHub issues | all | Minimum 20 labelled at all times from Phase G list |
| Architecture Decision Records | `docs/adr/` | `docs-update` | ADR for: scheduler choice, PQC algorithms, SDF design |

### CE2 — Developer Ecosystem

```bash

# Developer quick-start (target: productive in 30 minutes):

git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS
./scripts/setup.sh                  # install all deps

make PROFILE=microkernel            # first build (~5 minutes)

make test                           # unit tests pass

sigma-drv list                      # see loaded drivers

sigma-agri msp --crop wheat --year 2026  # test profession app

```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `setup.sh` verified on Ubuntu 22.04 | `scripts/setup.sh` | all | CI matrix includes fresh Ubuntu install |
| `setup.sh` verified on macOS (cross-compile) | `scripts/setup.sh` | all | macOS + cross-compiler for ARM64 |
| Dev container verified | `.devcontainer/devcontainer.json` | all | `gh cs create` → working build environment |
| IDE integration (clangd) | `.clangd` | all | `compile_commands.json` generated by cmake |
| Example apps directory | `docs/examples/` | `docs-update` | hello_sigma.cpp, hello_india.cpp, sigma_profession_app.cpp |
| Bug bounty program live | `wiki_repo/BUG_BOUNTY.md` | `prepare-sigmaos-launch` | Paid rewards for CVE reports |
| Hacktoberfest participation | `.github/` labels | all | October: `hacktoberfest` label on 30+ issues |
| sigma-contrib CLI tool | `userland/tools/sigma_contrib_cli.cpp` | `tools-dev` | `sigma-contrib new-driver`, `sigma-contrib new-app` scaffolding |

### CE3 — Package Ecosystem Quality

| Target | Metric | Branch | Detail |
|--------|--------|--------|--------|
| Bootstrap packages (50) | All install cleanly | `tools-dev` | bash, coreutils, curl, git, Python, GCC, Go, vim, nano, htop |
| Profession apps (55) | All installable | `release/standalone` | `sigma-pkg install sigma-ca` works |
| Package signature verification | 100% | all | Every install checks Dilithium3 sig |
| Mirror latency | < 200 ms | `tools-dev` | NIC CDN + India mirror auto-selected |
| Package index freshness | < 24 h | `tools-dev` | Index updated daily via scheduled CI |
| Delta updates | Supported | `tools-dev` | `sigma_delta.h` → implement binary delta |

---

## Comprehensive Quality Dashboard

### sigmaos.dev/quality (target page)

```
Boot CI:        ████████████░░  92% pass rate (target: 99%)
Unit Tests:     █████████████░  95% pass rate (target: 100%)
Fuzzing:        ████░░░░░░░░░░  30% coverage  (target: 80%)
PQC Bench:      ░░░░░░░░░░░░░░  0 M ops/sec   (target: 5.8M — real NTT missing)
Boot Time:      ░░░░░░░░░░░░░░  unknown        (target: < 2 s)
Open 🔴 Issues: 14             (target: 0 before v16.0)
WCAG 2.2:       ░░░░░░░░░░░░░░  not tested     (target: AA)
Hindi ASR WER:  ░░░░░░░░░░░░░░  not measured   (target: < 15%)
Fleet devices:  0              (target: 1,000 in BharatOS pilot)
```

| Dashboard metric | Source | Branch | Update frequency |
|-----------------|--------|--------|-----------------|
| Boot CI pass rate | GitHub Actions | all | Per commit |
| Unit test coverage | gcov/llvm-cov | `kernel-exp` | Per commit |
| Fuzz coverage | AFL++ coverage | `performance-optimized` | Nightly |
| PQC ops/sec | `bench_pqc.cpp` | `performance-optimized` | Weekly |
| Boot time trend | `bench_boot.sh` | all | Per merge to main |
| Open critical issues | GitHub API | all | Hourly |
| WCAG score | aXe CI | `release/standalone` | Per release |
| Hindi ASR WER | `test_bhashini.sh` | `release/standalone` | Weekly |

---

## Summary: All Roadmap Documents

| Document | Topics | Lines |
|----------|--------|-------|
| [Quality-Stability-Performance-Roadmap](Quality-Stability-Performance-Roadmap) | Stability S1-S4, Performance P1-P6, Quality Q1-Q5, UX U1-U6, Security SE1-SE2, Accessibility A1-A2, DX D1-D4, per-branch gates | ~1,000 |
| [Stability-Performance-Extended](Stability-Performance-Extended) | Energy E1-E2, Reliability R1-R3, Observability O1-O2, Release RE1-RE2, Community C1-C2, Network NR1-NR2, India QA IQ1-IQ3, Testing TI1-TI3, Hardware HC1-HC2, Formal verification LE1-LE3, Rust migration, KPIs v15.1→v18 | ~900 |
| [Compatibility-Automation-Personalisation-Roadmap](Compatibility-Automation-Personalisation-Roadmap) | Linux compat L1-L3, Win32 W1-W5, POSIX, File formats, Package formats, Automation A1-A5, Config.sigma, Profiles, Themes, sigma-cron, sigma-hook, ~/.sigma_profile, DID personalisation, Rural personalisation | ~700 |
| [Advanced-Quality-Roadmap](Advanced-Quality-Roadmap) | PQC production SH1-SH5, MAC enforcement, Memory safety, Supply chain, Network stack NS1-NS3, Enterprise EF1-EF4, AI/ML quality AI1-AI3, i18n I18N1-I18N3, Education EDU1, Rural RU1-RU2, Community CE1-CE3, Quality dashboard | ~700 |

---

*See also: [Quality Stability Performance Roadmap](Quality-Stability-Performance-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [India Profession Tools Roadmap](India-Profession-Tools-Roadmap) · [CLI Commands Roadmap](CLI-Commands-Roadmap) · [Development Roadmap](Development-Roadmap)*
