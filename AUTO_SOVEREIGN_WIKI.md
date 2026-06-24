# Σ SigmaOS — Sovereign Wiki

> Auto-generated reference document. For the live GitHub Wiki, run `./deploy-to-github.sh`.  
> Last updated: 2026-06-24

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Architecture](#2-architecture)
3. [Init & Service Manager](#3-init--service-manager)
4. [Networking & Security](#4-networking--security)
   - [VPN Stack (WireGuard-inspired)](#41-vpn-stack)
   - [Stateful Firewall](#42-stateful-firewall)
   - [Intrusion Detection System (IDS)](#43-intrusion-detection-system)
   - [Zero-Trust Enforcement](#44-zero-trust-enforcement)
5. [Package Ecosystem](#5-package-ecosystem)
   - [Sigma Package Registry (σ-repo)](#51-sigma-package-registry)
   - [Sandbox Packaging (.σpkg)](#52-sandbox-packaging)
   - [App Store](#53-app-store)
6. [Desktop & Accessibility](#6-desktop--accessibility)
   - [Zenith Desktop Environment](#61-zenith-desktop-environment)
   - [Accessibility Framework](#62-accessibility-framework)
   - [Recovery GUI](#63-recovery-gui)
7. [Future Innovations](#7-future-innovations)
   - [AI-Native Filesystem (AIFS)](#71-ai-native-filesystem)
   - [Self-Healing Kernel](#72-self-healing-kernel)
   - [AI-Assisted Debugger](#73-ai-assisted-debugger)
   - [Unikernel Target](#74-unikernel-target)
8. [Governance & Community](#8-governance--community)
9. [Enterprise & Compliance](#9-enterprise--compliance)
10. [Roadmap](#10-roadmap)
11. [API Reference (navigator.sigmaos)](#11-api-reference)

---

## 1. Project Overview

SigmaOS is the world's first **AI-native, post-quantum-secure, sovereign operating system**.

| Property | Value |
|---------|-------|
| **License** | MIT |
| **Language** | C11 / C++17 / Go / Rust (drivers) |
| **Cryptography** | Kyber-1024 + Dilithium-5 (post-quantum hybrid) |
| **Init system** | sigma-init (PID 1) — custom, systemd-free |
| **Package format** | `.σpkg` (OCI-compatible, Dilithium-5 signed) |
| **Desktop** | Zenith DE (Wayland-native, tiling) |
| **Target** | Desktop · Server · IoT · Cloud unikernel · WASM |

### Key Differentiators

1. **Post-Quantum by default** — every cryptographic surface uses hybrid classical+PQC
2. **Zero-trust process model** — per-process SPIFFE SVIDs, default-deny policy
3. **AI-native** — inference-aware scheduling, semantic filesystem, AI crash correlation
4. **Sovereign libc** — `klib/` replaces glibc; zero external libc dependency
5. **No mandatory telemetry** — opt-in only; all data stays on-device by default

---

## 2. Architecture

```
┌────────────────────────── User Layer ──────────────────────────────┐
│  Zenith DE · App Store · PWAs · Web Extensions · sigma_a11y        │
│  sigma_de_core.h · sigma_appstore.h · sigma_recovery_gui.h        │
├────────────────────────── Userland Layer ──────────────────────────┤
│  sigma-init (PID 1) · sigma_service_graph · sigma_socket_activate  │
│  sigmad-process (Go) · sigmad-sys · sigmad-ws · sigmad-hotplug     │
│  sigma_registry_client · sigma_sandbox_pkg · .σpkg format          │
├────────────────────────── Security Layer ──────────────────────────┤
│  sigma_zerotrust (SVID/mTLS) · sigma_firewall · sigma_ids          │
│  sigma_vpn_wireguard (Noise+Kyber-1024 hybrid PQ extension)        │
├────────────────────────── Kernel Layer ────────────────────────────┤
│  sigma_self_healing · sigma_aifs · sigma_ai_debug                   │
│  Drivers: linux_shim (compat) · NVMe · USB xHCI · Wi-Fi · BT       │
│  klib: sovereign libc · Kyber-1024 · Dilithium-5 · SHA-3           │
├────────────────────────── Boot Layer ──────────────────────────────┤
│  GRUB2/UEFI · Verified Boot (Dilithium-5 chain) · TPM 2.0          │
│  sigma_unikernel_target (cloud/edge/WASM deployment variant)        │
└────────────────────────────────────────────────────────────────────┘
```

### Compile Flags Note

- **C files** (`.h`, `.c`): compiled with `-std=c11`
- **C++ files** (`.cpp`, `.cxx`): compiled with `-std=c++17`
- Scoped via `.clangd` `If: PathMatch` conditions — no global flag collision

---

## 3. Init & Service Manager

**Files**: `userland/init/`

| File | Purpose |
|------|---------|
| `sigma_init.cpp` | PID 1: boot topology, watchdog, runlevel-ordered startup |
| `sigma_rc.h` | `sigma_rc_service_t` — service descriptor struct |
| `sigma_service_graph.h` | DAG for After/Wants/Requires/PartOf dependency ordering |
| `sigma_socket_activate.h` | systemd-style socket activation protocol |
| `sigma_unikernel_target.h` | Single-binary unikernel build target |

### Boot Topology (sigma_rc_service_t)

```c
static const sigma_rc_service_t boot_units[] = {
    { "sigma-journal",          "/sigma/bin/sigma-journal",
      SIGMA_RUNLEVEL_BOOT,    /* critical */ 1 },
    { "sigma-wifi",             "/sigma/bin/sigma-wifi",
      SIGMA_RUNLEVEL_DAEMONS, /* critical */ 1 },
    { "zenith-compositor",      "/sigma/bin/zenith-compositor",
      SIGMA_RUNLEVEL_GUI,     /* critical */ 1 },
};
```

### Runlevels

| # | Constant | Phase |
|---|---------|-------|
| 0 | `SIGMA_RUNLEVEL_BOOT` | Blocking — journal, FS indexer |
| 1 | `SIGMA_RUNLEVEL_DAEMONS` | Parallel — Wi-Fi, PipeWire, cluster |
| 2 | `SIGMA_RUNLEVEL_GUI` | Zenith compositor |
| 3 | `SIGMA_RUNLEVEL_SHUTDOWN` | Reverse-order graceful teardown |

### Dependency Graph API

```c
sigma_service_graph_t g;
sigma_sg_init(&g);
int compositor = sigma_sg_add_node(&g, "zenith-compositor");
sigma_sg_add_dep(&g, compositor, "sigma-pipewire", SIGMA_DEP_REQUIRES);
bool ok = sigma_sg_topo_sort(&g); // false = cycle detected
```

### Socket Activation

```c
sigma_sock_table_t table;
sigma_sock_table_init(&table);
sigma_sock_register(&table, "sigma-journal",
                    "unix:/run/sigma/journal.sock", SIGMA_SOCK_STREAM);
sigma_sock_bind_all(&table); // PID 1 pre-binds; service starts on first connection
```

---

## 4. Networking & Security

### 4.1 VPN Stack

**File**: `kernel/net/vpn/sigma_vpn_wireguard.h`

WireGuard-compatible Noise_IKpsk2 + **Kyber-1024 post-quantum hybrid**.

| Primitive | Classical | PQ Extension |
|-----------|-----------|-------------|
| Key Exchange | X25519 | Kyber-1024 |
| AEAD | ChaCha20-Poly1305 | — |
| Hash/KDF | BLAKE2s (HKDF) | — |

```c
sigma_vpn_interface_t iface;
sigma_vpn_init(&iface, /* pq_enabled */ true);
int peer = sigma_vpn_add_peer(&iface, peer_pubkey, "10.0.0.1", 51820);

uint8_t handshake[256];
sigma_vpn_handshake_initiate(&iface, peer, handshake, sizeof(handshake));

// Encrypt outbound packet
sigma_vpn_encrypt_packet(&iface.peers[peer], plaintext, len, ciphertext, cap);
```

Anti-replay: 64-bit counter window; replayed packets silently dropped.

---

### 4.2 Stateful Firewall

**File**: `kernel/net/firewall/sigma_firewall.h`

- **Chains**: INPUT · OUTPUT · FORWARD
- **Default policy**: DROP
- **Connection tracking**: SYN/ESTABLISHED/RELATED/CLOSE state machine
- **AI hook**: pluggable ML callback receives normalised packet feature vector

```c
sigma_firewall_t fw;
sigma_fw_init(&fw);  // default-deny

sigma_fw_rule_t allow_https = {
    .dst_port_lo = 443, .dst_port_hi = 443,
    .proto = SIGMA_PROTO_TCP,
    .action = SIGMA_FW_ACTION_ACCEPT,
};
sigma_fw_add_rule(&fw, SIGMA_FW_CHAIN_INPUT, &allow_https);

// Register AI anomaly detection model
sigma_fw_set_ai_hook(&fw, my_ml_hook, model_ctx);
```

---

### 4.3 Intrusion Detection System

**File**: `kernel/net/firewall/sigma_ids.h`

**Layer 1 — Signature**: pattern match at configurable byte offset (CVE patterns, C2 beacons)  
**Layer 2 — Behavioral**: per-flow rolling statistics (SYN flood, RST injection, entropy spikes)

Alert severity: `INFO` → `LOW` → `MEDIUM` → `HIGH` → `CRITICAL`

`auto_block = true` → HIGH/CRITICAL alerts automatically add DROP rule to sigma_firewall.

---

### 4.4 Zero-Trust Enforcement

**File**: `kernel/security/sigma_zerotrust.h`

Every process gets a SPIFFE-compatible SVID:
`spiffe://sigma.os/workload/<service-name>`

```c
sigma_zt_ctx_t zt;
sigma_zt_init(&zt);  // default_deny = true

// Attest a newly-spawned process
sigma_zt_attest_workload(&zt, pid, "/sigma/bin/sigma-wifi", shard_id);

// Allow a specific flow
sigma_zt_policy_t p = {
    .src_spiffe = "spiffe://sigma.os/workload/sigma-wifi",
    .dst_spiffe = "spiffe://sigma.os/workload/zenith-compositor",
    .action = SIGMA_ZT_POLICY_ALLOW,
};
sigma_zt_add_policy(&zt, &p);

// Check before every IPC call
sigma_zt_policy_action_t decision = sigma_zt_check_flow(&zt, src_pid, dst_pid, 4200, SIGMA_PROTO_TCP);
```

SVIDs auto-rotate every hour via `sigma_zt_rotate_svid()`. All decisions are audit-logged.

---

## 5. Package Ecosystem

### 5.1 Sigma Package Registry

**File**: `userland/pkg/sigma_registry_client.h`

- Signatures: **Dilithium-5** post-quantum
- Delta updates: binary diff patches
- Rollback: snapshot ref per install
- Dependency resolution: SAT-solver topological sort (DNF-inspired)

```c
sigma_registry_ctx_t ctx;
sigma_registry_init(&ctx, "/home/sigma/.sigmaos/cache");
sigma_registry_sync(&ctx);

sigma_pkg_meta_t* pkg = sigma_registry_find(&ctx, "ffmpeg");
const char* wants[] = { "ffmpeg" };
sigma_pkg_resolve_result_t result;
sigma_registry_resolve_deps(&ctx, wants, 1, &result);
sigma_registry_install(&ctx, result.install_order[0]);

// Rollback
sigma_registry_rollback(&ctx, pkg->snapshot_ref);
```

### 5.2 Sandbox Packaging

**File**: `userland/pkg/sigma_sandbox_pkg.h`

Flatpak-inspired `.σpkg` format:

| Feature | Detail |
|---------|--------|
| Filesystem isolation | Per-app `/app`, `/var`, `/home` overlay mounts via bubblewrap |
| Portal API | FileChooser · Camera · Location · Notifications · Print · Secret |
| Runtime Refs | Apps declare `sigma-runtime-core-1.0` — no bundled glibc |
| Permissions | Declared in `sigma.yaml` manifest; user-approved at install |
| Resource limits | `mem_limit_bytes`, `cpu_shares`, `pids_limit` via cgroup |

### 5.3 App Store

**File**: `userland/pkg/sigma_appstore.h`  
**API**: `https://store.sigma.os/api/v1/`

| Badge | Meaning |
|-------|---------|
| `SIGMA_STORE_BADGE_VERIFIED` | Publisher identity confirmed |
| `SIGMA_STORE_BADGE_AUDITED` | Security audit passed |
| `SIGMA_STORE_BADGE_SOVEREIGN` | No external telemetry |
| `SIGMA_STORE_BADGE_STAFF_PICK` | Editorial recommendation |
| `SIGMA_STORE_BADGE_ACCESSIBLE` | Full WCAG 2.2 + AT-SPI2 |

Collections: `gaming` · `enterprise` · `iot` · `staff-picks` · `new`

---

## 6. Desktop & Accessibility

### 6.1 Zenith Desktop Environment

**File**: `userland/gui/sigma_de_core.h`

- Wayland-native (xdg-shell + layer-shell + ext-session-lock)
- Window modes: Tiled · Floating · Fullscreen · Scratchpad · PiP
- 16 named workspaces with session save/restore
- Live theme hot-reload (32 ARGB8888 color slots)
- Notification system (Low/Normal/Critical urgency)

**Default color palette** (SigmaOS Dark):

| Slot | Constant | Default Value |
|------|---------|--------------|
| 0 | `SIGMA_COLOR_BG_PRIMARY` | `#0D0D12` |
| 2 | `SIGMA_COLOR_ACCENT` | `#6C63FF` |
| 4 | `SIGMA_COLOR_TEXT_PRIMARY` | `#E8E8F0` |
| 8 | `SIGMA_COLOR_DANGER` | `#FF4D6D` |

### 6.2 Accessibility Framework

**File**: `userland/a11y/sigma_a11y.h`

AT-SPI2-compatible accessibility tree targeting **WCAG 2.2**.

Features:
- 27 WAI-ARIA roles
- Live regions (polite + assertive announcements)
- Screen reader TTS integration
- Voice control command registry
- High-contrast, reduce-motion, text-scale system tokens
- Focus indicator customisation (color + border width)

### 6.3 Recovery GUI

**File**: `userland/gui/sigma_recovery_gui.h`

6 recovery modes, rendered on Linux framebuffer (`/dev/fb0`) — **no compositor needed**.

| Mode | Function |
|------|---------|
| `SIGMA_RECOVERY_READONLY` | Forensic read-only mount |
| `SIGMA_RECOVERY_FSCK` | Interactive filesystem check |
| `SIGMA_RECOVERY_ROLLBACK` | Snapshot browser + restore |
| `SIGMA_RECOVERY_SHELL` | Restricted emergency shell |
| `SIGMA_RECOVERY_NETWORK` | Pull updates over network |
| `SIGMA_RECOVERY_FACTORY` | Confirmed factory reset |

---

## 7. Future Innovations

### 7.1 AI-Native Filesystem

**File**: `kernel/fs/sigma_aifs.h`

- Content-addressed storage (SHA-256, CoW, automatic deduplication)
- ML-generated semantic tags via sigma-inference-engine
- Relational query: `aifs_query("topic:kernel AND date:>2026-01-01")`
- Predictive prefetching: access pattern model issues readahead 50-200ms early
- Provenance tracking: `(who, when, from-content-hash)` per write

### 7.2 Self-Healing Kernel

**File**: `kernel/core/sigma_self_healing.h`

- Hardware watchdog timer (`/dev/watchdog`) integration
- Subsystem heartbeat registry — missed beats trigger escalating recovery
- In-memory checkpoint/restore (`< 1ms` overhead at safe points)
- Live module hot-swap — quiesce/unload/reload without reboot
- ML fault classifier: "recoverable" vs "degraded" vs "must reboot"

### 7.3 AI-Assisted Debugger

**File**: `userland/devtools/sigma_ai_debug.h`

| Feature | Inspiration |
|---------|------------|
| Log anomaly detection | Learned entropy baseline |
| Crash grouping + CVE match | Pattern correlation |
| Memory leak tracking | Valgrind memcheck shadow allocator |
| CPU profiling + hot-function analysis | Brendan Gregg perf-tools / eBPF |

### 7.4 Unikernel Target

**File**: `userland/init/sigma_unikernel_target.h`

| Target | Platform |
|--------|---------|
| `SIGMA_UK_TARGET_CLOUD` | AWS Firecracker / GCP gVisor |
| `SIGMA_UK_TARGET_EDGE` | ARM Cortex-M33 / RISC-V MCU |
| `SIGMA_UK_TARGET_WASM` | WebAssembly WASI-preview2 |
| `SIGMA_UK_TARGET_QEMU` | KVM/QEMU for CI |

Single-binary: SigmaOS kernel + one application, statically linked, boots via GRUB2 Multiboot2.

---

## 8. Governance & Community

**Documents**: `GOVERNANCE.md` · `SECURITY.md` · `CODE_OF_CONDUCT.md`

### Steering Committee (5 seats)

Chair · Technical Lead · Community Lead · Ecosystem Lead · Release Manager

Term: 2 years; max 2 consecutive. Quorum: 3/5. Supermajority: 4/5 for charter amendments.

### Working Groups

`kernel-wg` · `security-wg` · `pkg-wg` · `de-wg` · `enterprise-wg` · `docs-wg`

### RFC Process

```
DRAFT → FEEDBACK (14 days public) → ACCEPTED (SC vote) → ACTIVE → FINAL
```

### Contributor Tiers

| Tier | Criteria |
|------|---------|
| Contributor | ≥1 merged PR — listed in CONTRIBUTORS.md |
| Committer | ≥10 PRs + 3-month activity — branch write access |
| Maintainer | WG nomination + SC approval — protected branch rights |
| Fellow | Exceptional sustained contribution — advisory role |

### Security Reporting

Report vulnerabilities to **security@sigma.os** or via GitHub Private Advisory.  
**Never** open a public issue. See `SECURITY.md` for full CVD policy.

---

## 9. Enterprise & Compliance

**Documents**: `docs/COMPLIANCE.md` · `docs/LTS_POLICY.md`

### Framework Alignment

| Framework | Status |
|-----------|--------|
| ISO/IEC 27001:2022 | ✅ Alignment documented |
| NIST SP 800-53 Rev. 5 | ✅ Control mapping complete |
| GDPR | ✅ Full on-premise alignment |
| India DPDPA 2023 | ✅ Architecture-level compliance |
| FIPS 140-3 | 📋 Planned Q2 2027 |
| Common Criteria EAL3+ | 📋 Planned 2028 |

### LTS Release Stream

| Release | Code Name | Date | EOL |
|---------|-----------|------|-----|
| v1.0-LTS | *Sigma Prime* | Q4 2026 | Q4 2033 |
| v2.0-LTS | TBD | Q2 2028 | Q2 2035 |

Support phases: **2yr Full** → **+3yr Security** → **+2yr Critical CVEs only** → EOL

### ABI Stability (LTS)

- `klib/` public headers: frozen for LTS window
- Kernel module ABI: binary-compatible throughout
- `.σpkg` format: backward-compatible

---

## 10. Roadmap

See [Roadmap.md](Roadmap.md) for the full phase breakdown.

| Phase | Status | Highlights |
|-------|--------|-----------|
| E — Core Subsystems | ✅ Q2 2026 | NVMe, USB, ACPI, Cgroups, Package Registry |
| F — Type Hardening | ✅ Q2 2026 | PQC headers, error codes, TCP fixes |
| G — Wireless Stacks | 🔄 Q3 2026 | Wi-Fi 802.11, BT HCI, WPA3, TLS 1.3 |
| H — Recovery GUI | 📋 Q3 2026 | Compositor wiring, recovery, tiling WM |
| I — First ISO | 📋 Q4 2026 | GRUB2 bootable ISO, QEMU tested |
| K — Governance | ✅ Q3 2026 | Charter, RFC, WGs, contributor tiers |
| L — Enterprise | ✅ Q4 2026 | Compliance maps, LTS policy |
| M — Future | 📋 2027+ | RISC-V, live patching, Lattice cluster |

---

## 11. API Reference

### navigator.sigmaos — Browser API

```typescript
// Package management
await navigator.sigmaos.pkg.ensure(["ffmpeg", "imagemagick"]);
const pkgs: string[] = await navigator.sigmaos.pkg.list();

// Shell execution (bubblewrap sandbox)
const result = await navigator.sigmaos.shell.exec({
  cmd: "ffmpeg",
  args: ["-i", "pipe:0", "-c:v", "libx264", "-f", "mp4", "pipe:1"],
  stdin: videoBytes,           // Uint8Array
  caps: ["bin:~/.sigmaos/bin/ffmpeg", "fs:/tmp"]
});
// result.stdout: Uint8Array, result.stderr: Uint8Array, result.code: number

// WebAssembly execution
const wasmResult = await navigator.sigmaos.wasm.run({
  wasm: wasmBytes,             // Uint8Array (.wasm binary)
  args: ["--input", "hello"],
  caps: []
});

// Window management
const win = await navigator.sigmaos.window.create({
  width: 400, height: 300,
  title: "My Floating Tool",
  alwaysOnTop: true,
  frameless: true
});

// Filesystem
await navigator.sigmaos.fs.writeFile("output.mp4", result.stdout);
const content = await navigator.sigmaos.fs.readFile("input.txt");

// Live process output (SSE)
const { pid, stdout } = await navigator.sigmaos.process.spawn("python3", ["-c", "..."]);
const reader = stdout.getReader();
while (true) {
  const { value, done } = await reader.read();
  if (done) break;
  console.log(new TextDecoder().decode(value));
}
```

### Capability Strings

| Cap String | Grants |
|-----------|--------|
| `"net"` | Outbound network access |
| `"fs:/path/to/dir"` | Read-write filesystem access to path |
| `"bin:/path/to/binary"` | Execute a specific binary |
| `"wayland"` | Access to Wayland/Zenith display socket |

### Security Model

The SigmaOS Chrome Extension acts as gatekeeper:
1. Apps request capabilities via `navigator.sigmaos.*`
2. Extension checks `~/.sigmaos/capabilities.json` for granted caps
3. If denied, a system consent dialog is shown to the user
4. All execution happens inside `bwrap` namespace containers
