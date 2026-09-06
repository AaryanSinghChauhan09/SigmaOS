# 🇸🇴 AI Agents Chroot Management Architecture in SigmaOS

## Executive Overview

SigmaOS introduces an **autonomous, sovereign AI Agent Chroot Management Architecture** that automates, isolates, and governs root file system (`rootfs`) chroot environments across all Linux and BSD distribution modes. Built into SigmaOS's zero-dependency `#![no_std]` Rust microkernel and userland layer, AI Agents manage isolated build sandboxes, bind mount points, environment variables, network namespace suppression, and privilege separation boundaries.

By combining Linux container isolation mechanisms with BSD security paradigms (OpenBSD `pledge`/`unveil` and FreeBSD Jails), AI Agents ensure that software compilation, package building, and legacy rootfs execution take place inside ephemeral, hermetic, and verifiable chroot environments.

---

## 🌟 Architectural Principles & Linux/BSD Inspirations

SigmaOS absorbs and unifies key chroot sandboxing paradigms from classic and modern distributions:

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                            SigmaOS AI Agent Chroot Orchestrator                          │
│          (ACP / MCP Protocols, Dilithium-5 Attestation, OpenBSD Pledge/Unveil)           │
└───────────────────────────┬──────────────────────────────────────────────────────────────┘
                            │
         ┌──────────────────┼──────────────────┬──────────────────┐
         ▼                  ▼                  ▼                  ▼
┌─────────────────┐┌─────────────────┐┌─────────────────┐┌─────────────────┐
│ Alpine / Void   ││ Debian / Ubuntu ││ FreeBSD Jails   ││ OpenBSD Unveil  │
│ ApkChroot Sbx   ││ Sbuild Chroot   ││ Nested Jails    ││ Path Restriction│
└─────────────────┘└─────────────────┘└─────────────────┘└─────────────────┘
```

### 1. Linux Chroot & Build Sandboxing Paradigms
- **Alpine & Void Linux (`ApkChrootBuildSandboxEngine`):** Provides fast, minimal chroot compilation sandboxes with strict network isolation and custom bind mounts (`add_bind_mount`, `enter_chroot`, `exit_chroot`).
- **Debian & Ubuntu (`SbuildChrootSandboxEngine`):** Manages reproducible Debian `sbuild` chroots, dependency resolution, and hermetic build root creation.
- **OverlayFS & Ephemeral Storage:** AI Agents instantiate ephemeral read-only base layers with writable tmpfs/overlayfs scratchpads, guaranteeing zero residual artifacts after chroot exit.

### 2. BSD Security & Isolation Paradigms
- **FreeBSD Jails & Hierarchical Chroots:** Combines chroot directory containment with IPC, PID, and virtual VNET network stack isolation (`FreeBSDJail`).
- **OpenBSD `unveil(2)` Path Restriction:** Restricts chroot process access to explicitly unveiled directories (`/usr/include`, `/lib`), denying access to non-essential host paths.
- **OpenBSD `pledge(2)` & Descriptor Rights:** Enforces `OpenBsdFdPledgeGate` file descriptor rights masks within chroots, preventing permission escalation.

---

## 🤖 Core AI Chroot Governors & Engines

SigmaOS implements two primary native Rust chroot engines managed by autonomous AI agents:

### 1. Alpine / Void Chroot Sandbox Engine (`ApkChrootBuildSandboxEngine`)
Located in `src/distro/linux_bsd_inspirations.rs`:
- **Structure:**
  - `sandbox_id: String` - Unique identifier for the chroot session.
  - `root_path: String` - Path to the isolated rootfs directory.
  - `isolate_network: bool` - Enables network namespace isolation.
  - `allowed_bind_mounts: Vec<String>` - Host paths allowed as bind mounts inside the chroot.
  - `environment_vars: Vec<(String, String)>` - Governed environment variables (e.g., `CC`, `CFLAGS`, `PATH`).
  - `is_active: bool` - Tracks active chroot state.
- **Lifecycle Operations:**
  - `add_bind_mount(&mut self, source_path: &str)` - Mounts host directories before chroot activation.
  - `set_env(&mut self, key: &str, val: &str)` - Sets isolated build environment variables.
  - `enter_chroot(&mut self)` - Transitions execution into the isolated rootfs environment.
  - `compile_package(&mut self, pkg_name: &str, build_cmd: &str)` - Executes hermetic package compilation.
  - `exit_chroot(&mut self)` - Safely exits the chroot and triggers cleanup.

### 2. Debian Sbuild Chroot Engine (`SbuildChrootSandboxEngine`)
Located in `src/distro/developer.rs`:
- **Structure:**
  - Manages target distribution suites (`sid`, `stable`, `testing`).
  - Resolves Debian/Ubuntu build dependencies (`build-essential`, `devscripts`).
  - Integrates with `PoudriereBulkBuildEngine` for cross-distro bulk compilation.

---

## 📡 Agent Communication & Protocol Integration (ACP / MCP)

AI Agents interact with chroot sandboxes through standardized Agent Protocols:

### Agent Client Protocol (ACP)
- **JSON-RPC Chroot Control:**
  - `chroot_create`: Provisions a new isolated rootfs.
  - `chroot_mount`: Dynamically configures bind mounts.
  - `chroot_exec`: Executes builds or commands inside the chroot.
  - `chroot_destroy`: Purges ephemeral chroot scratchpads.

### Model Context Protocol (MCP)
- **Context Bridge:**
  - Exposes chroot build logs, compiler warnings, and dependency trees to LLMs (`LocalLlmDaemon`, `QwenPaw`, `KimiCodeAgent`).
  - Enforces OpenBSD `unveil` boundaries so LLM context collectors cannot read confidential host paths outside the chroot.

---

## 🔒 Security, Attestation & Audit Governance

1. **Post-Quantum Cryptographic Attestation:**
   - Binaries built inside AI-governed chroots receive Dilithium-5 post-quantum signature attestations and SBOM (Software Bill of Materials) provenance records.
2. **Path Unveil & Descriptor Rights Enforcement:**
   - AI Agents restrict process access within chroots via `OpenBSDUnveil` rules and `OpenBsdFdPledgeGate` descriptor rights masks.
3. **Network Namespace Isolation:**
   - By default, chroot sandboxes run with network isolation enabled (`isolate_network: true`), blocking unauthorized egress during package builds.
4. **Immutable Audit Logging:**
   - All chroot entry, exit, bind mount, and compilation events are recorded in the SigmaOS unified audit log (`UnifiedLogEntry`).

---

## 🛠️ Inspection & Manual Control Commands

Administrators can inspect and manage AI agent chroot sandboxes via `sigma-sh`:

```bash
# View active AI agent chroot sandboxes
sigma-sh> ai-agent status chroot

# Inspect a specific chroot sandbox session
sigma-sh> ai-agent inspect chroot --id=sbx_alpine_01

# Manually trigger a package build inside a chroot sandbox
sigma-sh> chroot-build --sandbox=sbx_alpine_01 --package=curl --cmd="make"

# Verify post-quantum attestation signatures for build artifacts
sigma-sh> ai-agent verify-build-provenance --file=curl.apk
```
