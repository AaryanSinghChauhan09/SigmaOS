# SigmaOS Improvement Roadmap — 6 Pillars

This document translates the strategic vision into concrete implementation tasks across 6 pillars. Each pillar maps to existing source files and new CLI tools.

---

## Pillar 1: Driver & Hardware Support

**Goal:** Become driver-compatible with all major Linux and Windows hardware without vendor lock-in.

### Status

- SDF (Sovereign Driver Framework) v3 — implemented (`drivers/`)

- 11 drivers loaded: net, storage, GPU, input, audio, USB

- ABI stability checking — `sigma-drv abi check`

### New CLI Tool: `sigma-drv`

```bash
sigma-drv list                         # list all loaded SDF drivers

sigma-drv load sigma-wifi-iwl          # load a driver

sigma-drv probe --pci 8086:15f3        # probe a PCI device

sigma-drv bench sigma-e1000            # throughput benchmark

sigma-drv reload sigma-nvidia-hal      # hot-reload without reboot

sigma-drv abi check                    # verify ABI stability

sigma-drv port --linux iwlwifi         # AI-assisted Linux driver porting guide

```

### Roadmap Tasks

| Task | Status | File |
|------|--------|------|
| Stable SDF ABI v3 | ✅ Done | `drivers/ddk/` |
| Driver hot-reload | ✅ Done | `sigma-drv reload` |
| ABI stability check | ✅ Done | `sigma-drv abi` |
| AI-assisted driver porting | ✅ Done | `sigma-drv port` |
| Vendor partnership portal | 🔄 Planned | `docs/CONTRIBUTING_DRIVERS.md` |
| iwlwifi/mt7921 WiFi | 🔄 Porting | `drivers/net/sigma_wifi_iwl.rs` |
| RPi GPIO / I2C / SPI | 🔄 Planned | `drivers/embedded/` |

---

## Pillar 2: Package Ecosystem

**Goal:** Absorb .deb, .rpm, Flatpak, Snap, AppImage into a single `sigma-pkg` format.

### Status

- `sigma-pkg` implemented (`pkg/sigma_pkg_cli.nim`) with 12 commands

- OCI container runtime: `userland/tools/sigma_pod_cli.nim`

- Declarative config: `tools/cli/SovereignDeclarativeConfig.rs`

### Key Commands

```bash
sigma-pkg install zenith-desktop       # install from Sigma Store

sigma-pkg search vr                    # search registry

sigma-pkg audit                        # CVE scan

sigma-pkg verify sigma-core            # Dilithium-5 signature check

sigma-pkg build myapp.spkg             # build from recipe

# Linux compat absorption

sigma-pkg install --deb apt:nginx      # absorb Debian package

sigma-pkg install --flatpak flathub:org.gimp.GIMP
```

### Roadmap Tasks

| Task | Status | File |
|------|--------|------|
| sigpkg format v1 | ✅ Done | `pkg/sigma-manifest.toml.example` |
| sigma-pkg CLI | ✅ Done | `pkg/sigma_pkg_cli.nim` |
| .deb absorption layer | 🔄 Planned | `tools/sigma_pkg_debian_compat.rs` |
| .rpm absorption layer | 🔄 Planned | `tools/sigma_pkg_fedora_compat.rs` |
| Flatpak bridge | 🔄 Planned | `tools/sigma_flatpak.rs` |
| OCI/Docker runtime | ✅ Done | `virtualization/ocirunner/` |
| Reproducible builds | ✅ Done | `cmake/sigma_reproducible.cmake` |

---

## Pillar 3: AI & Automation

**Goal:** Every OS operation is accessible via natural language. sigma-ai is a first-class system service.

### New CLI Tool: `sigma-ai`

```bash
sigma-ai ask "why is my system slow?"           # NL query

sigma-ai ask "सिस्टम धीमा क्यों है?" --lang hi   # Hindi

sigma-ai explain "sigma-secure audit --fix"     # explain before running

sigma-ai heal                                    # analyse crash/anomaly

sigma-ai script "harden my system weekly"       # generate .sigma script

sigma-ai workflow list                           # list automation workflows

sigma-ai workflow run security-hardening        # run a workflow

sigma-ai model list                              # available GGUF models

sigma-ai security scan                          # AI security advisor

sigma-ai predict cpu                             # ML resource prediction

sigma-ai translate "sigma update" --to hi        # translate CLI to Hindi

```

### Multi-Agent Architecture

```
sigma-ai (coordinator)
├── SysAdmin agent    → system health, updates, drivers
├── Security agent    → policy, CVEs, hardening
├── Developer agent   → code assistance, scripting
└── Automation agent  → workflows, scheduling
```

### Roadmap Tasks

| Task | Status | File |
|------|--------|------|
| sigma-ai daemon (llama.cpp backend) | 🔄 Integration | `userland/ai/` |
| NL → CLI translator | ✅ Done | `sigma-ai translate` |
| Educational explain mode | ✅ Done | `sigma-ai explain` |
| Crash analysis / heal | ✅ Done | `sigma-ai heal` |
| Multi-agent orchestration | ✅ Done | `agents/orchestration/` |
| Bhashini (Indian languages) | 🔄 Planned | `sigma-ai model download sigma-bhashini` |
| Voice input (Whisper STT) | 🔄 Planned | `userland/ai/sigma_voice.rs` |
| Workflow engine (n8n-style) | ✅ Done | `sigma-ai workflow` |
| AI audit trail | ✅ Done | `/var/log/sigma/ai-audit.jsonl` |

---

## Pillar 4: Security & Sovereignty

**Goal:** PQC-first, open attestation, enterprise-grade compliance, zero hidden telemetry.

### Status

- Dilithium-5 (NIST FIPS 204) — all packages and drivers signed

- TPM 2.0 attestation — `sigma-secure attest`

- NIST/RBI/HIPAA compliance — `sigma_compliance.nim`

- sigma-fix AI-guided patching — `tools/sigma-fix.rs`

### Key Commands

```bash
sigma-secure audit --fix               # full security audit + auto-fix

sigma-secure pqc gen                   # generate Dilithium-5 keys

sigma-secure attest                    # TPM attestation chain

sigma_compliance scan nist             # NIST SP 800-53 (20 controls)

sigma_compliance scan rbi              # RBI IT Framework (8 controls)

sigma-fix scan                         # AI-guided patch suggestions

sigma-fix apply --id FIX-0001 --auto   # auto-apply SSH fix

```

### Roadmap Tasks

| Task | Status | File |
|------|--------|------|
| Dilithium-5 everywhere | ✅ Done | `crypto/dilithium.adb` |
| IMA policy | ✅ Done | `security/SovereignKernelIntegrityChecker.adb` |
| TPM attestation | ✅ Done | `sigma-secure attest` |
| NIST/RBI/HIPAA compliance | ✅ Done | `userland/tools/sigma_compliance.nim` |
| Zero telemetry (opt-in only) | ✅ Done | `sigma-ai status` audit trail |
| ZKP attestation | 🔄 Planned | `security/sigma_zkp_execution_proof.rs` |
| Formal verification | 🔄 Planned | `tests/formal/` |

---

## Pillar 5: User Experience & Community

**Goal:** World-class desktop, governance model, plugin ecosystem, recognition programs.

### New CLI Tool: `sigma-fleet` (Enterprise MDM)

```bash
sigma-fleet register --server fleet.sigmaos.app --token mytoken
sigma-fleet status                    # agent + device health

sigma-fleet policy set                # fetch + apply enterprise policy

sigma-fleet update pull               # OTA from fleet server

sigma-fleet inventory                 # report hardware to fleet

sigma-fleet list                      # all managed devices

sigma-fleet audit --push              # push audit log to server

sigma-fleet lock --wipe               # remote lock/wipe

```

### Plugin Architecture

Contribute a plugin with a single PR:
```
plugins/<name>/
  plugin.sigma.toml   # manifest

  main.rs / main.nim  # implementation

  README.md
```
Register: `sigma-pkg install myname-plugin`
Discovery: any binary `sigma-<name>` on PATH is auto-discovered.

### Roadmap Tasks

| Task | Status | File |
|------|--------|------|
| Zenith desktop (Wayland) | 🔄 Building | `zenith_desktop/` |
| sigma-fleet MDM | ✅ Done | `tools/sigma-fleet.rs` |
| Plugin auto-discovery | ✅ Done | `sigma help` shows plugins |
| Governance model | ✅ Done | `GOVERNANCE.md` |
| Contributor recognition | 🔄 Planned | `CONTRIBUTORS.md` |
| Voice input integration | 🔄 Planned | `ui/SovereignVoice.rs` |
| Educational mode | ✅ Done | `sigma-ai explain` |

---

## Pillar 6: Developer Ecosystem

**Goal:** Best-in-class developer tools, multi-language support, reproducible builds.

### Developer CLI surface

```bash
sigma init my-driver                   # scaffold Rust no_std driver

sigma build --target aarch64 --release # cross-compile

sigma shard load my-driver.shard       # hot-load kernel module

sigma-drv probe --pci 8086:15f3        # test device binding

sigma bench all --save                 # run all benchmarks

sigma_diagnostics full                 # pre-release gate check

sigma-ai script "add IPv6 routing"     # AI-generated automation

```

### Supported Languages

| Language | Role |
|----------|------|
| Rust (`no_std`) | Kernel, drivers, CLI tools, sigma-sh |
| Nim | Package manager, userspace tools, shell scripting |
| Zig | Filesystem (SigmaFS), boot, HAL helpers |
| Go | App CLI (`tools/sigma-cli/`) |
| Ada | Crypto, security-critical modules |
| PowerShell | Windows developer toolchain (`sigma.ps1`) |
| Bash | Host automation scripts |

### Roadmap Tasks

| Task | Status | File |
|------|--------|------|
| sigma-sh v0.3 (full scripting) | ✅ Done | `sigma-sh/src/` |
| sigma-coreutils (BusyBox-style) | ✅ Done | `userland/coreutils/src/main.rs` |
| sigma_getopt (arg parser lib) | ✅ Done | `userland/tools/sigma_getopt.nim` |
| zenith-build tool | ✅ Done | `userland/tools/zenith_build.nim` |
| WASM runtime | 🔄 Building | `runtime/wasm/` |
| Formal verification CI | 🔄 Planned | `tests/formal/` |
| Developer docs portal | 🔄 Planned | `docs/DEVELOPER_GUIDE.md` |

---

## Implementation Progress Summary

| Pillar | Score | Key remaining |
|--------|-------|---------------|
| 1. Driver Support | 80% | WiFi drivers, vendor SDK |
| 2. Package Ecosystem | 75% | .deb/.rpm absorption, mirror network |
| 3. AI & Automation | 70% | llama.cpp integration, Bhashini |
| 4. Security | 90% | ZKP, formal verification |
| 5. UX & Community | 65% | Zenith desktop, governance voting |
| 6. Developer Ecosystem | 85% | WASM, formal CI gate |

*See also: [CLI Reference](CLI-Reference) · [Development Roadmap](Development-Roadmap) · [Architecture Overview](Architecture-Overview)*
