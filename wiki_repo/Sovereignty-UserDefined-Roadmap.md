# SigmaOS — Sovereignty & User-Defined Roadmap
## Reduce Foreign Dependency · User-Defined Extensions
## Sovereign Alternatives · Zero-Trust Supply Chain
## Per-User OS Configuration · User-Defined Apps & Scripts

---

## Executive Summary

SigmaOS has two sovereignty goals:

1. **Technical sovereignty** — no critical dependency on foreign software,
   foreign cloud, or foreign standards that could be denied or compromised.
2. **User sovereignty** — every user can define, extend, and own their OS
   behaviour without asking permission from any vendor.

---

## Part 1 — Reducing Foreign Dependencies

### FD1 — Dependency Audit (Current State)

Every foreign dependency is a potential supply chain attack, a sanctions risk,
and a sovereignty gap. This is the complete audit:

#### Critical dependencies (currently blocks boot or basic function)

| Dependency | Used for | Foreign risk | Sovereign replacement | Status | 
| ----------- | --------- | ------------- | ---------------------- | -------- | 
| GRUB | Bootloader (because sigma-boot.efi doesn't exist) | GNU/GRUB maintainers | `sigma-boot.efi` (UEFI PE binary) | ❌ not built | 
| QEMU | CI boot testing | Red Hat / various | Physical CI + sigma-vm | ⚠️ acceptable for dev | 
| GNU binutils (ld, as, nm) | Linking kernel | GNU Project | LLVM lld (already in toolchain) | ⚠️ migrate to lld | 
| GCC | Compiling kernel | GNU Project | clang/LLVM (already used) | ⚠️ finish migration | 
| CMake | Build system | Kitware | sigma-build (future) or keep CMake | 🟡 low risk | 

#### High-priority dependencies (needed for features)

| Dependency | Used for | Sovereign replacement | Target branch | Status | 
| ----------- | --------- | ---------------------- | -------------- | -------- | 
| liboqs | Real Kyber/Dilithium NTT | Sigma-native PQC NTT (pure C++, no external) | `performance-optimized` | ❌ NTT not written | 
| libargon2 | CryptFS key derivation (Issue #44) | Sigma-native Argon2id (pure C++) | `kernel-exp` | ❌ not implemented | 
| SQLite | Profession app data, registry | sigma-kv (sovereign key-value store) OR keep SQLite | `fs-dev` | ⚠️ SQLite is FOSS, low risk | 
| llama.cpp | sigma-ai LLM inference | sigma-infer (sovereign inference engine, long-term) | `release/standalone` | ❌ llama.cpp needed | 
| musl-libc | POSIX compat layer | sigma-nanolib already exists; expand it | `tools-dev` | ⚠️ musl is sovereign-friendly | 
| OpenSSL (legacy) | TLS 1.2 compat | sigma-tls (sovereign TLS) | `drivers-dev` | ❌ sigma-tls not built | 

#### Acceptable dependencies (FOSS, low risk, difficult to replace near-term)

| Dependency | Used for | Notes | 
| ----------- | --------- | ------- | 
| HarfBuzz | Text shaping (Devanagari, Tamil) | Excellent FOSS project, no cloud dependency | 
| FreeType2 | Font rendering | Rock-solid FOSS | 
| clang/LLVM | Compiler toolchain | Apache-licensed, community-governed | 
| Go | sigma-repo-server, sigma-fleet | Google origin, but Apache-licensed open standard | 
| Rust (planned) | sigma-net, sigma-fs migration | Mozilla origin, Apache/MIT dual-licensed | 

### FD2 — Sovereign Cryptography (sigma-pqc-native)

**Goal:** Replace liboqs with a sovereign NTT implementation that compiles
to pure C++17 with no external dependencies.

```
sigma-pqc-native architecture:
  crypto/ntt/
    sigma_ntt_generic.cpp     — pure C++ reference NTT (any platform)
    sigma_ntt_avx512.cpp      — x86-64 AVX-512 optimised NTT
    sigma_ntt_neon.cpp        — ARM NEON optimised NTT
  crypto/pqc/
    sigma_kyber1024.cpp       — ML-KEM-1024 (FIPS 203) using sigma-ntt
    sigma_dilithium5.cpp      — ML-DSA-87 (FIPS 204) using sigma-ntt
    sigma_slhdsa.cpp          — SLH-DSA (FIPS 205) hash-based
  crypto/primitives/
    sigma_shake256.cpp        — SHAKE-256 / SHA-3 (Keccak) — zero dep
    sigma_sha2.cpp            — SHA-2 family (256/384/512) — zero dep
    sigma_aes_gcm.cpp         — AES-256-GCM — zero dep
    sigma_argon2id.cpp        — Argon2id (fix Issue #44) — zero dep
    sigma_blake3.cpp          — BLAKE3 hash — zero dep
    sigma_chacha20poly.cpp    — ChaCha20-Poly1305 — zero dep
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Keccak-f\[1600\] state machine | `crypto/primitives/sigma_shake256.cpp` | `performance-optimized` | Pure C++, no external headers | 
| NTT butterfly (generic C++) | `crypto/ntt/sigma_ntt_generic.cpp` | `performance-optimized` | Cooley-Tukey NTT, modular arithmetic | 
| AVX-512 NTT butterfly | `crypto/ntt/sigma_ntt_avx512.cpp` | `performance-optimized` | 8-wide SIMD, 13× speedup | 
| ARM NEON NTT butterfly | `crypto/ntt/sigma_ntt_neon.cpp` | `release/mobile` | 4-wide NEON, 5.7× speedup | 
| ML-KEM-1024 on sigma-ntt | `crypto/pqc/sigma_kyber1024.cpp` | `performance-optimized` | Replace `SovereignKyber.cpp` PRNG with real NTT | 
| ML-DSA-87 on sigma-ntt | `crypto/pqc/sigma_dilithium5.cpp` | `performance-optimized` | Replace `SovereignDilithium5.cpp` PRNG with real NTT | 
| Argon2id (zero dep) | `crypto/primitives/sigma_argon2id.cpp` | `kernel-exp` | Fix Issue #44 — time=3, mem=65536, threads=4 | 
| SHA-2 family | `crypto/primitives/sigma_sha2.cpp` | `performance-optimized` | SHA-256, SHA-384, SHA-512 — pure C++ | 
| BLAKE3 | `crypto/primitives/sigma_blake3.cpp` | `performance-optimized` | For package checksums — faster than SHA-256 | 
| sigma-pqc-native CI benchmark | `tests/perf/bench_pqc_native.sh` | `performance-optimized` | vs liboqs reference: within 20% | 
| CI: build with no external crypto | `.github/workflows/sigma_ci.yml` | all | `-DSIGMA_NO_EXTERNAL_CRYPTO=ON` build target | 

### FD3 — Sovereign Bootloader (sigma-boot.efi)

**Goal:** Boot without GRUB. sigma-boot.efi is a native UEFI PE binary.

```
sigma-boot.efi architecture:
  sigma-boot/
    sigma_boot.c              — UEFI efi_main() entry point
    sigma_elf_loader.c        — Load kernel ELF from ESP
    sigma_gop.c               — UEFI GOP framebuffer
    sigma_mmap.c              — Build E820/multiboot2 memory map
    sigma_ab_slot.c           — A/B slot selection from EFI variable
    sigma_secboot.c           — ML-DSA-87 kernel signature verify
    sigma_dna_early.c         — Early CPUID hardware profile
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `efi_main()` UEFI entry | `sigma-boot/sigma_boot.c` | `kernel-exp` | EDK2-free: use only UEFI firmware services | 
| EFI file protocol read | `sigma-boot/sigma_elf_loader.c` | `kernel-exp` | `EFI_SIMPLE_FILE_SYSTEM_PROTOCOL` | 
| GOP framebuffer setup | `sigma-boot/sigma_gop.c` | `kernel-exp` | `EFI_GRAPHICS_OUTPUT_PROTOCOL` | 
| Build PE binary (clang) | `Makefile` | `kernel-exp` | `clang --target=x86_64-pc-win32-coff -o sigma-boot.efi` | 
| A/B slot EFI variable | `sigma-boot/sigma_ab_slot.c` | `kernel-exp` | Read/write `SigmaBootSlot` via `SetVariable` | 
| ML-DSA-87 kernel verify | `sigma-boot/sigma_secboot.c` | `kernel-exp` | Verify kernel `.sig` file using sigma-pqc-native | 
| GRUB fallback removal | `Makefile` | `prepare-sigmaos-launch` | `make iso` uses sigma-boot.efi, not GRUB | 

### FD4 — Sovereign Network Stack (sigma-tls)

**Goal:** TLS 1.3 without OpenSSL. All India Stack API calls use sigma-tls.

```
net/tls/
  sigma_tls.cpp              — TLS 1.3 record layer
  sigma_tls_handshake.cpp    — Hybrid X25519+ML-KEM-1024 key exchange
  sigma_tls_cipher.cpp       — AES-256-GCM, ChaCha20-Poly1305
  sigma_tls_cert.cpp         — X.509 + ML-DSA certificate chain
  sigma_tls_session.cpp      — Session resumption (0-RTT for India APIs)
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| TLS 1.3 record layer | `net/tls/sigma_tls.cpp` | `drivers-dev` | RFC 8446 ClientHello/ServerHello | 
| Hybrid KEM (X25519 + ML-KEM) | `net/tls/sigma_tls_handshake.cpp` | `drivers-dev` | Draft RFC hybrid KEM for TLS 1.3 | 
| AES-256-GCM (sovereign) | `crypto/primitives/sigma_aes_gcm.cpp` | `performance-optimized` | Pure C++ AES-NI intrinsics | 
| X.509 cert parser | `net/tls/sigma_tls_cert.cpp` | `drivers-dev` | Parse GSTN/ABDM CAs; pin their certs | 
| 0-RTT session resumption | `net/tls/sigma_tls_session.cpp` | `drivers-dev` | Reduce GSTN API latency from 500→100 ms | 
| GSTN cert pin | `net/tls/sigma_tls_cert.cpp` | `release/standalone` | Hard-pin GSTN NIC CA — detect MITM | 
| CI: sigma-tls vs GSTN sandbox | `tests/net/test_sigma_tls.sh` | `drivers-dev` | Full handshake + GSTR-1 fetch | 

### FD5 — Sovereign Kernel Linker (migrate from GNU ld)

**Current:** Uses GNU ld via gcc for linking.

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Migrate to LLVM lld | `CMakeLists.txt` | all | `-fuse-ld=lld` — already in many CI configs | 
| Custom linker script | `linker.ld` | `kernel-exp` | Fine-tune section placement for shard lattice | 
| Verify lld reproducibility | `.github/workflows/sigma_ci.yml` | all | lld output SHA256 == gcc ld output SHA256 | 
| Remove binutils from `setup.sh` | `scripts/setup.sh` | all | Only clang + lld required | 

### FD6 — Sovereign Build System (sigma-build, long-term)

**Long-term goal (Phase 9):** Replace CMake with a sovereign build system
that understands shard manifests natively.

```python
# sigma-build (Python DSL, long-term):
shard("sigma-net-tcp",
    version = "1.0.0",
    sources = ["net/tcp/sigma_tcp.cpp"],
    depends = ["sigma-vfs", "sigma-caps", "sigma-crypto"],
    caps    = ["sigma.cap.net.tx", "sigma.cap.net.rx"],
    tests   = ["tests/net/test_tcp.cpp"],
)
```

| Task | File | Branch | Timeline | 
| ------ | ------ | -------- | --------- | 
| sigma-build DSL design | `docs/adr/adr-build-system.md` | `docs-update` | Document decision before Phase 9 | 
| Proof-of-concept build | `sigma-build/sigma_build.py` | Phase 9 | Build microkernel profile from DSL | 
| Keep CMake for v16.0–v17.0 | `CMakeLists.txt` | all | CMake acceptable until Phase 9 | 


---

## Part 2 — User-Defined System Extensions

### UD1 — User-Defined Profession Tools (sigma-custom-app)

Any user can create a custom profession tool that integrates fully with
SigmaOS — no C++ required, no rebuild required.

```bash
# Create a custom profession app in 5 steps:

# 1. Scaffold
sigma-contrib new-app my-custom-tool

# 2. Generated structure:
# userland/apps/my-custom-tool/
#   sigma_my_custom_tool.h         # auto-generated ISigmaApp header
#   sigma_my_custom_tool.cpp       # implement your logic here
#   manifest.sigma                 # app metadata + capabilities
#   sigma-my-custom-tool.1         # man page template
#   tests/test_my_custom_tool.cpp  # test template
#   CMakeLists.txt                 # auto-generated build

# 3. Implement (C++ or sigma-script):
# sigma-script (YAML + Bash-like, no C++ needed for simple tools):
# my-custom-tool.sigma-script:
#   name: land-records-checker
#   description: Check land records for my village
#   commands:
#     check:
#       run: sigma-gov dilrmp lookup --khatauni $1
#       help: Check land records by khatauni number

# 4. Build and install
sigma-contrib build my-custom-tool
sigma-contrib install my-custom-tool    # installs to /sigma/apps/

# 5. Use immediately
sigma-my-custom-tool check 1234
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `sigma-contrib new-app` scaffold | `userland/tools/sigma_contrib_cli.cpp` | `tools-dev` | Generate full app skeleton from template | 
| sigma-script DSL | `userland/tools/sigma_script.cpp` | `tools-dev` | YAML + shell commands, no C++ needed | 
| sigma-script interpreter | `userland/tools/sigma_script.cpp` | `tools-dev` | Parse `.sigma-script`, dispatch via sigma-sh | 
| User app auto-discovery | `userland/daemons/sigma_appd.cpp` | `release/standalone` | Scan `~/sigma-apps/` + `/sigma/apps/` at login | 
| User app sigma-bus integration | `userland/daemons/sigma_appd.cpp` | `release/standalone` | User app can subscribe/publish sigma-bus topics | 
| Sandboxed user app execution | `userland/daemons/sigma_appd.cpp` | `release/standalone` | User app in sigma-pod with declared capabilities only | 
| `sigma-app share <name>` | `userland/tools/sigma_app_cli.cpp` | `tools-dev` | Build `.spkg`, sign with user DID, share link | 

### UD2 — User-Defined CLI Commands

```bash
# Any user can add custom commands to sigma-sh:

# ~/.sigma/commands/gst-check.sh:
#!/sigma/bin/sigma-sh
# Usage: gst-check <gstin>
# Description: Quick GST compliance check
GSTIN="${1:?Usage: gst-check <GSTIN>}"
sigma-ca gst compute --gstin "$GSTIN" --period "$(date +%Y-%m)"
echo "---"
sigma-digilocker fetch --gstin "$GSTIN" --doc gst-certificate

# Register as command:
sigma-cli command register gst-check ~/.sigma/commands/gst-check.sh

# Now available everywhere:
gst-check 27ABCDE1234F1Z5
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| User command directory `~/sigma-apps/bin/` | `userland/shell/sigma_shell.cpp` | `tools-dev` | Add to PATH at shell init | 
| `sigma-cli command register <name> <script>` | `userland/tools/sigma_cli.cpp` | `tools-dev` | Symlink to `~/sigma-apps/bin/`, add to completions | 
| `sigma-cli command list` | `userland/tools/sigma_cli.cpp` | `tools-dev` | Show user-defined + system commands | 
| `sigma-cli command unregister <name>` | `userland/tools/sigma_cli.cpp` | `tools-dev` | Remove from PATH + completions | 
| User commands in tab completion | `userland/shell/sigma_shell.cpp` | `tools-dev` | Add `~/sigma-apps/bin/` to completion scan | 
| `sigma-sh` shebang support | `userland/shell/sigma_shell.cpp` | `tools-dev` | `#!/sigma/bin/sigma-sh` runs scripts directly | 

### UD3 — User-Defined Aliases and Profiles

**Current:** `sigma-cli alias add` works. VFS profile load partial.

```toml
# ~/.sigma_profile — user can define everything:

[identity]
name            = "Arjun Sharma"
profession      = "chartered_accountant"
preferred_lang  = "hi"
did             = "did:sigma:arjun123"

[custom_commands]
gst   = "sigma-ca gst compute --gstin 27ABCDE1234F1Z5"
itr   = "sigma-ca itr compute --pan ABCDE1234F --ay 2026-27"
tds   = "sigma-ca tds compute --form 26Q"
enam  = "sigma-agri enam prices --mandi Azadpur"
court = "sigma-legal ecourt status"

[custom_aliases]
ll    = "sigma-ls -lah"
gs    = "git status"
pods  = "sigma-pod ps"
audit = "sigma-audit log --last 50"

[startup]
apps  = ["sigma-ca", "sigma-accounts"]
commands = ["sigma-ca gst --remind"]

[notifications]
gst_reminder      = true
itr_deadline      = true
court_hearing     = true
pmkisan_credit    = false
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| VFS read `~/.sigma_profile` at login | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `kernel-exp` | `vfs_open()` after VFS init — currently blocked | 
| Apply `[custom_commands]` to PATH | `userland/shell/sigma_shell.cpp` | `tools-dev` | Each `key=value` → register as shell command | 
| Apply `[custom_aliases]` to sigma-sh | `userland/shell/sigma_shell.cpp` | `tools-dev` | Each `key=value` → shell alias | 
| Apply `[startup]` apps at login | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `release/standalone` | Fork + exec each app in list | 
| Notification subscription from profile | `userland/daemons/sigma_notify.cpp` | `release/standalone` | Register sigma-bus subscriptions per `[notifications]` | 
| Profile schema validation | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `tools-dev` | Reject unknown keys with helpful error message | 
| `sigma-profile validate` CLI | `userland/tools/sigma_profile_cli.cpp` | `tools-dev` | Check syntax + print warnings | 

### UD4 — User-Defined Zenith Layout Plugins

```bash
# Users can define custom tiling layout algorithms:

# ~/.sigma/layouts/my-layout.sigma-layout:
# name: "India Stack Focus"
# description: "Large terminal + sigma-ca sidebar + sigma-health panel"
# script:
#   window 0: x=0 y=0 w=60% h=100%    # terminal (left)
#   window 1: x=60% y=0 w=40% h=50%   # sigma-ca (top right)
#   window 2: x=60% y=50% w=40% h=50% # sigma-health (bottom right)

sigma-zenith layout install ~/.sigma/layouts/my-layout.sigma-layout
sigma-zenith layout use "India Stack Focus"
sigma-zenith layout list       # shows built-in + user-defined
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Layout definition format | `zenith_desktop/wm/sigma_tiling_wm.cpp` | `release/standalone` | TOML: named regions with x/y/w/h percent | 
| Layout script interpreter | `zenith_desktop/wm/sigma_layout_script.cpp` | `release/standalone` | Apply layout definition to current windows | 
| `sigma-zenith layout install` | `userland/tools/sigma_zenith_cli.cpp` | `release/standalone` | Copy to `~/.sigma/layouts/`, register in WM | 
| User layout persistence | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `release/standalone` | Save active layout to `~/.sigma_profile` | 
| Layout preview before apply | `userland/tools/sigma_zenith_cli.cpp` | `release/standalone` | `sigma-zenith layout preview <name>` → ASCII art | 

### UD5 — User-Defined Themes and Widgets

```bash
# Full theme customisation:
sigma-zenith theme create my-india-theme
# Opens ~/.sigma/themes/my-india-theme.sigma-theme in editor

# ~/.sigma/themes/my-india-theme.sigma-theme:
[palette]
base       = "#FF9933"   # Saffron (India flag)
surface    = "#FFFFFF"   # White
accent     = "#138808"   # India green
text       = "#000080"   # Navy blue
warning    = "#FFC107"
error      = "#F44336"

[typography]
body_font  = "Noto Sans Devanagari"
mono_font  = "JetBrains Mono"
size_base  = 13
size_mono  = 12

[geometry]
border_radius = 8
gap_inner     = 6
gap_outer     = 12
border_width  = 2

[effects]
blur_radius    = 10    # glassmorphism
shadow_offset  = 4
animation_ms   = 200   # 0 to disable for accessibility
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Theme TOML parser | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | Parse palette/typography/geometry/effects sections | 
| WCAG contrast auto-check | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | Warn user if base/text contrast < 4.5:1 | 
| Hot-reload on file change | `zenith_desktop/theme/sigma_theme_engine.cpp` | `release/standalone` | Watch `~/.sigma/themes/`, apply in < 200 ms | 
| Theme export as `.sigma-theme` bundle | `userland/tools/sigma_zenith_cli.cpp` | `release/standalone` | Pack + ML-DSA-sign for sharing | 
| Community theme registry | `sigma_pkg_registry/` | `prepare-sigmaos-launch` | `sigma-pkg install sigma-theme-diwali` | 
| User widget definition | `zenith_desktop/widgets/sigma_user_widget.cpp` | `release/standalone` | YAML widget: label, command, refresh interval | 

### UD6 — User-Defined Automation Scripts (sigma-script)

```yaml
# ~/sigma-scripts/morning-check.sigma-script
name: Morning GST & Legal Check
description: Run every weekday morning

steps:
  - name: Check GST filing due
    run: sigma-ca gst --remind --days 7
    if: profession == "chartered_accountant"

  - name: Check court hearings today
    run: sigma-legal cause-list --today
    if: profession == "advocate"

  - name: Check eNAM prices
    run: sigma-agri enam prices --mandi nearest
    if: profession == "farmer"

  - name: Remind if filing overdue
    notify: "{{ output }}"
    on_fail: sigma-cli health check

schedule: "0 9 * * 1-5"   # weekdays 9 AM
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| sigma-script YAML parser | `userland/tools/sigma_script.cpp` | `tools-dev` | Parse steps, conditions, notifications | 
| Conditional execution (`if:` field) | `userland/tools/sigma_script.cpp` | `tools-dev` | Evaluate `profession == "ca"` from `~/.sigma_profile` | 
| `{{ output }}` template interpolation | `userland/tools/sigma_script.cpp` | `tools-dev` | Replace `{{ output }}` with previous step's stdout | 
| `notify:` → sigma-bus notification | `userland/tools/sigma_script.cpp` | `release/standalone` | Send to Zenith notification daemon | 
| Schedule integration with sigma-cron | `userland/daemons/sigma_cron.cpp` | `tools-dev` | `schedule:` field → sigma-cron job | 
| Script marketplace | `sigma_pkg_registry/scripts/` | `tools-dev` | Community-contributed scripts via sigma-pkg | 
| `sigma-script run/list/validate` CLI | `userland/tools/sigma_script_cli.cpp` | `tools-dev` | Full management CLI | 


---

## Part 3 — Sovereignty in Practice

### SP1 — India-First Data Residency

No user data ever leaves India by default.

```
Data residency policy:
  All profession app data → SigmaFS on local device
  Cloud sync → SovereignCloudFS (self-hosted on Indian servers only)
  India Stack API calls → NIC/MeitY endpoints (domestic)
  sigma-ai inference → local llama.cpp (no cloud LLM)
  sigma-bhashini → offline models (no cloud ASR/TTS)
  Package downloads → packages.sigmaos.dev (India CDN)
  DNS → sigma-dns-cache (no default 8.8.8.8)
  NTP → time.nic.in (NIC time server — not Google/Cloudflare)
  Telemetry → ZERO (no phone-home, no analytics)
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| Default DNS: NIC recursive resolver | `net/dns/sigma_dns_cache.cpp` | `drivers-dev` | Default upstream: `164.100.130.2` (NIC) not `8.8.8.8` | 
| Default NTP: NIC time server | `userland/daemons/sigma_netd.cpp` | `drivers-dev` | `time.nic.in` as default NTP | 
| Package CDN: India-hosted | `sigmad/repo/main.go` | `prepare-sigmaos-launch` | `packages.sigmaos.dev` served from NIC/DigitalIndia CDN | 
| Zero telemetry policy | All daemons | all | No background HTTP calls; sigma-quality-check scans for outbound HTTP in CI | 
| Data residency declaration | `wiki_repo/Data-Residency-Policy.md` | `docs-update` | Document every data flow | 
| Offline-first as default | All profession apps | `release/standalone` | Cache API responses; work without internet | 

### SP2 — Sovereign Identity (no Google/Microsoft/Apple login)

```bash
# Every user has a self-sovereign DID — no foreign IdP needed:

# Create DID (first boot):
sigma-trust did create --name "Arjun Sharma" --profession ca

# DID is stored locally on device:
# /sigma/var/trust/arjun_did.json

# Login to any India government portal:
sigma-gov login --portal gstn.gov.in    # Uses DID, not Google OAuth
sigma-gov login --portal abdm.gov.in
sigma-gov login --portal mca21.gov.in

# No username/password. No Google account. No Microsoft account
# The DID IS the identity
```

| Task | File | Branch | Detail | 
| ------ | ------ | -------- | -------- | 
| `sigma-trust did create` full impl | `security/SovereignDID.cpp` | `release/standalone` | Generate DID document + ML-DSA-87 keypair | 
| Local DID storage (encrypted) | `security/SovereignDID.cpp` | `release/standalone` | Encrypt DID private key with Argon2id-derived key | 
| DID → GSTN login | `userland/indiastack/sigma_gstn_client.cpp` | `release/standalone` | GSTN OAuth2 PKCE with DID instead of password | 
| DID → ABDM login | `userland/indiastack/sigma_abdm_client.cpp` | `release/standalone` | ABDM OAuth2 with DID | 
| DID → NIC SSO | `userland/auth/sigma_nic_sso.cpp` | `release/cloud` | NIC eID → DID mapping | 
| Revocation without central authority | `security/SovereignDID.cpp` | `release/standalone` | Publish revocation to sigma-blockchain-lite | 

### SP3 — Open Standards Only

SigmaOS never implements proprietary protocols. Every protocol is documented.

| Protocol | Standard | Sovereign? | Notes | 
| ---------- | --------- | ----------- | ------- | 
| Boot: sigma-boot.efi | UEFI spec (open) | ✅ | EDK2-free implementation | 
| Network: TCP/IP | RFC 793, 8200 (open) | ✅ | — | 
| TLS: sigma-tls | RFC 8446 + hybrid KEM draft | ✅ | No OpenSSL | 
| PQC: ML-KEM/ML-DSA | NIST FIPS 203/204/205 | ✅ | US standard, fully published | 
| Identity: DID | W3C DID Core 1.0 (open) | ✅ | Decentralised | 
| Package: .spkg | sigma-defined, open spec | ✅ | Documented in RECIPE_FORMAT.md | 
| IPC: sigma-bus | sigma-defined, open spec | ✅ | Documented in sigma_bus_topics.h | 
| Display: sigma-display | sigma-defined, open spec | ✅ | Replaces Wayland/X11 | 
| Filesystem: SigmaFS | sigma-defined, open spec | ✅ | Documented | 
| GST: GSTN JSON schema | Government of India | ✅ India | Indian government standard | 
| Health: FHIR R4 | HL7 (open) | ✅ | International open standard | 
| Windows compat | PE/COFF (documented) | ✅ | Documented Microsoft spec | 

---

## Part 4 — Comprehensive Roadmap Updates

### QS1 — Quality: Additional Test Scenarios

| Scenario | Test file | Branch | Detail | 
| ---------- | ---------- | -------- | -------- | 
| Foreign dependency scan | `tests/sovereignty/test_no_foreign_dep.sh` | all | Verify no outbound HTTP to non-Indian IPs in tests | 
| sigma-pqc-native correctness | `tests/crypto/test_pqc_native.cpp` | `performance-optimized` | KAT (Known Answer Test) vs NIST test vectors | 
| sigma-tls vs GSTN TLS | `tests/net/test_sigma_tls.sh` | `drivers-dev` | Full handshake + HTTP GET to GSTN sandbox | 
| sigma-boot.efi CI | `.github/workflows/sigma_ci.yml` | `kernel-exp` | QEMU UEFI boot via sigma-boot.efi (not GRUB) | 
| User app sandbox escape | `tests/security/test_app_sandbox.sh` | `release/standalone` | App cannot access capabilities not declared | 
| User script injection | `tests/security/test_script_inject.sh` | `tools-dev` | sigma-script rejects shell metacharacter injection | 
| DID login without internet | `tests/security/test_did_offline.sh` | `release/standalone` | DID auth works offline (no cloud IdP) | 

### ST1 — Stability: Additional Crash Scenarios

| Scenario | Test file | Branch | Target | 
| ---------- | ---------- | -------- | -------- | 
| sigma-pqc-native OOM during keygen | `tests/chaos/test_pqc_oom.sh` | `performance-optimized` | Returns SIGMA_ERR_NOMEM, no hang | 
| sigma-tls connection reset mid-handshake | `tests/chaos/test_tls_reset.sh` | `drivers-dev` | Retry once, then return error | 
| User script infinite loop | `tests/chaos/test_script_loop.sh` | `tools-dev` | sigma-cron kills job after timeout | 
| DID key corruption | `tests/chaos/test_did_corrupt.sh` | `release/standalone` | Fallback to recovery PIN; no data loss | 
| sigma-boot.efi bad kernel sig | `tests/chaos/test_secboot_reject.sh` | `kernel-exp` | Boot halts with error; rollback to B slot | 
| 1,000 concurrent sigma-bus messages | `tests/chaos/test_bus_flood.sh` | `kernel-exp` | No message dropped; latency < 1 ms p99 | 

---

## Per-Branch Sovereignty Targets

| Branch | Sovereignty task | Target | 
| -------- | ----------------- | -------- | 
| `kernel-exp` | sigma-boot.efi replaces GRUB | Before v16.0 | 
| `kernel-exp` | Argon2id replaces fake derive_key() | Before v16.0 | 
| `performance-optimized` | sigma-pqc-native replaces liboqs | Before v16.0 | 
| `drivers-dev` | sigma-tls replaces OpenSSL | Before v16.0 | 
| `tools-dev` | sigma-script user-defined automation | Before v15.1 | 
| `tools-dev` | User command/alias registration | Before v15.1 | 
| `release/standalone` | User-defined theme/layout/widget | Before v16.0 | 
| `release/standalone` | DID login replaces all foreign OAuth | Before v17.0 | 
| `release/cloud` | NIC CDN as default package mirror | Before v17.0 | 
| `release/mobile` | NIC NTP, NIC DNS as defaults | Before v16.0 | 
| `prepare-sigmaos-launch` | Zero telemetry verified by CI | Before v15.1 | 
| `docs-update` | Data Residency Policy doc | Before v15.1 | 

---

## Master Sovereignty Checklist

```
[ ] sigma-boot.efi boots without GRUB
[ ] Argon2id key derivation (fixes Issue #44)
[ ] sigma-pqc-native: Kyber/Dilithium without liboqs
[ ] sigma-tls: TLS 1.3 without OpenSSL
[ ] BLAKE3/SHA-2/AES-GCM: sovereign crypto primitives
[ ] Default DNS: NIC (164.100.130.2), not Google
[ ] Default NTP: time.nic.in, not pool.ntp.org
[ ] Package CDN: packages.sigmaos.dev (India), not GitHub
[ ] Zero telemetry: CI scan finds no outbound HTTP to foreign IPs
[ ] DID identity: no Google/Microsoft OAuth anywhere
[ ] Data residency: all data in India by default
[ ] User-defined commands: sigma-cli command register
[ ] User-defined scripts: sigma-script DSL
[ ] User-defined themes: sigma-theme TOML
[ ] User-defined layouts: sigma-layout TOML
[ ] User-defined apps: sigma-contrib new-app + ISigmaApp
```

---

*See also: [Modularisation Architecture Roadmap](Modularisation-Architecture-Roadmap) · [Engineering Principles Roadmap](Engineering-Principles-Roadmap) · [Advanced Quality Roadmap](Advanced-Quality-Roadmap) · [Windows Compatibility Layer Roadmap](Windows-Compatibility-Layer-Roadmap) · [Development Roadmap](Development-Roadmap)*
