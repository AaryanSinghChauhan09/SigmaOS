# SigmaOS Component Integration Strategy

> How to structure multiple GitHub projects as first-class SigmaOS components.

---

## The Problem

As SigmaOS grows, related projects (browser, editor, SDK, installer, RTOS variants)
naturally emerge. Without a clear integration strategy, they become scattered experiments
that users don't recognise as part of the same OS.

The goal: every SigmaOS project should feel like **one coherent platform**, not a
collection of separate repos.

---

## Architecture: Three Layers

```
┌────────────────────────────────────────────────────────────┐
│  LAYER 3 — Labs (experimental, separate repos)             │
│  SigmaOS-RTOS · SigmaOS-Distributed · SigmaOS-Mobile      │
├────────────────────────────────────────────────────────────┤
│  LAYER 2 — Ecosystem (apps + tools, separate repos)        │
│  SigmaOS-Browser · SigmaOS-Editor · SigmaOS-SDK            │
│  SigmaOS-Installer · SigmaOS-PKG-Registry                  │
├────────────────────────────────────────────────────────────┤
│  LAYER 1 — Core (one monorepo: AaryanSinghChauhan09/SigmaOS)│
│  kernel/ · drivers/ · fs/ · net/ · security/ · crypto/     │
│  userland/ · suites/ · build/ · docs/ · wiki_repo/         │
└────────────────────────────────────────────────────────────┘
```

---

## Option A: Monorepo (Recommended Now)

Keep everything in `AaryanSinghChauhan09/SigmaOS`. Use subdirectory namespacing:

```
SigmaOS/
├── kernel/           # OS core
├── drivers/          # SDF drivers
├── fs/               # Filesystems
├── apps/
│   ├── sigma-edit/   # Text editor
│   ├── sigma-browser/# Browser
│   ├── sigma-play/   # Media player
│   └── sigma-mail/   # Email client
├── tools/
│   ├── sigma-pkg/    # Package manager
│   ├── sigma-sdk/    # Developer SDK
│   ├── sigma-monitor/# Process monitor
│   └── sigma-disks/  # Disk manager
├── installer/        # GUI + CLI installer
├── build/            # Multi-format build pipeline
└── labs/
    ├── rtos/         # RTOS experiments
    ├── distributed/  # Distributed OS
    └── mobile/       # Mobile builds
```

**Pros**: one CI pipeline, one issue tracker, atomic commits across kernel+app.
**Cons**: large repo size (already ~20k files — manageable).

---

## Option B: GitHub Organization (Recommended for v1.0+)

Create `github.com/SigmaOS-Project` organization. Each repo is a component:

| Repo | Description |
|------|-------------|
| `SigmaOS-Project/core` | Kernel, drivers, fs, net, security, crypto |
| `SigmaOS-Project/apps` | All user-facing applications |
| `SigmaOS-Project/tools` | sigma-pkg, sigma-sdk, sigma-monitor, sigma-disks |
| `SigmaOS-Project/installer` | GUI + CLI installer, ISO builder |
| `SigmaOS-Project/browser` | Chromium fork + navigator.sigmaos.* |
| `SigmaOS-Project/docs` | Wiki, tutorials, developer docs |
| `SigmaOS-Project/labs-rtos` | RTOS experimental builds |
| `SigmaOS-Project/labs-distributed` | Distributed OS experiments |
| `SigmaOS-Project/labs-mobile` | Mobile APK/IPA builds |
| `SigmaOS-Project/pkg-registry` | Package registry server + index |
| `SigmaOS-Project/ci` | Shared GitHub Actions workflows |

Every repo follows the same:
- Branch policy: `main` only, PRs required.
- CI template: from `SigmaOS-Project/ci`.
- Package output: each repo produces `.sigpkg` artifacts.
- Branding: "SigmaOS Browser", "SigmaOS Editor", etc.

---

## Option C: Hybrid (Current Best Fit)

- `AaryanSinghChauhan09/SigmaOS` = monorepo for core + early apps.
- Create `SigmaOS-Project` GitHub org when the first external contributor joins.
- Move experimental labs to separate repos under the org immediately (they're low-risk to separate).
- Keep apps in the monorepo until they have their own maintainer.

---

## Integration Contract

Every SigmaOS component (whether in monorepo or separate repo) must:

### 1. Produce a .sigpkg

```bash
# Every component's CI must output a signed package
sigma-pkg build PKGBUILD
# Uploads sigma-<name>-<version>-<arch>.sigpkg to the registry
```

### 2. Use the SDF Interface (for kernel-touching components)

```rust
// drivers and kernel modules use the SDF lifecycle
impl SdfDriver for MyComponent {
    fn probe(dev: &DeviceId) -> bool { ... }
    fn init(&mut self) -> SdfResult<()> { ... }
    fn shutdown(&mut self) { ... }
}
sigma_sdf::register_driver!(MyComponent, "sigma-mycomponent");
```

### 3. Declare Platform Compatibility

```toml
# MANIFEST.toml
[package]
profile = ["standalone", "cloud", "mobile"]  # which OS profiles it supports
arch    = ["x86_64", "arm64"]
```

### 4. Follow the Privacy Contract

- No telemetry by default.
- No network calls not documented in MANIFEST.toml.
- Secrets only via `sigma-vault` API, never plain files.

### 5. Pass CI Gates

```yaml
# Required checks before merge
- sigma-pkg build && sigma-pkg verify   # package builds and verifies
- make test-<component>                 # component tests pass
- sigma-pkg lint PKGBUILD               # PKGBUILD is valid
```

---

## Component Discovery

Users and developers find components via:

1. `sigma-pkg search <keyword>` — command-line search.
2. `app_store.html` — the sovereign app store UI.
3. `download.html` — format-specific downloads.
4. GitHub org page (when created).
5. `docs.sigmaos.app` — developer documentation hub.

---

## Branding Rules

Every component must:
- Use the prefix `sigma-` for CLI tools and daemons.
- Use `SigmaOS <Name>` for GUI apps (e.g., "SigmaOS Browser", "SigmaOS Editor").
- Include the Σ logo in any GUI launcher icon.
- Reference `https://github.com/AaryanSinghChauhan09/SigmaOS` in package metadata.

---

## Current Component Status

| Component | Location | sigpkg | CI | Status |
|-----------|----------|--------|----|--------|
| Kernel core | `kernel/` | ⬜ | ✅ | 🔄 Boot in progress |
| SDF drivers | `drivers/` | ⬜ | ✅ | 🔄 e1000/NVMe/USB done |
| sigma-pkg | `userland/pkg/` | ⬜ | ✅ | 🔄 Local mode partial |
| sigma-sh | `userland/shell/` | ⬜ | ✅ | ⬜ REPL needed |
| Zenith Desktop | `zenith_desktop/` | ⬜ | ✅ | 🔄 Web prototype done |
| sigma-monitor | `userland/tools/` | ⬜ | ✅ | 🔄 Stub |
| sigma-vault | `userland/vault/` | ⬜ | ✅ | ✅ Implemented |
| sigma-browser | `browser/` | ⬜ | ✅ | 🔄 WASM demo done |
| App store UI | `app_store.html` | ⬜ | ✅ | ✅ UI complete |
| Installer UI | `installer.html` | ⬜ | ✅ | ✅ UI complete |

---

*See also: [Professional-Tools-And-Apps](Professional-Tools-And-Apps.md) · [sigpkg-Spec](sigpkg-Spec.md) · [SDK-Guide](SDK-Guide.md) · [ROADMAP](Roadmap.md)*
