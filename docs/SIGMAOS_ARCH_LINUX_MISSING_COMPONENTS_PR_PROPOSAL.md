# 🚀 SIGMAOS PULL REQUEST PROPOSAL: ARCH LINUX COMPONENT PARITY ENGINE

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **PR Title:** `feat(sigpkg): absorb missing Arch Linux components (ALPM hooks, makepkg PKGBUILD runner, AUR RPC engine, pacman DB verifier)`
> **Status:** Proposed Pull Request Specification & Arch Parity Roadmap

---

## 📌 PULL REQUEST OVERVIEW

This Pull Request proposal specifies the implementation of key missing Arch Linux components within **SigmaOS**, bridging the gap between standard Arch Linux ALPM/pacman/AUR tools and SigmaOS's universal packaging subsystem.

---

## 🎯 ARCH LINUX MISSING COMPONENTS & ABSORPTION SCOPE

### 1. ALPM Transaction Hooks Dispatcher (`/etc/pacman.d/hooks`)
* **Concept:** Execute pre/post transaction shell or native hooks triggered by package installations/upgrades (e.g. `mkinitcpio`, `dkms`, `systemd-tmpfiles`).
* **SigmaOS Implementation:** `SovereignAlpmHookDispatcher` in `src/sigpkg/arch_pacman_engine.rs`.

### 2. `makepkg` & PKGBUILD Build Sandbox Engine
* **Concept:** Parse Arch Linux `PKGBUILD` scripts (`pkgname`, `pkgver`, `pkgrel`, `depends`, `makedepends`, `source`, `build()`, `package()`) and execute cleanroom builds.
* **SigmaOS Implementation:** `SovereignPkgbuildRunner` in `src/sigpkg/arch_pacman_engine.rs`.

### 3. ALPM `db.lck` Lock & Database Integrity Verifier
* **Concept:** Prevent concurrent database writes and verify pacman local database sync consistency (`/var/lib/pacman/local/`).
* **SigmaOS Implementation:** `SovereignPacmanDbVerifier` in `src/sigpkg/arch_pacman_engine.rs`.

### 4. Arch User Repository (AUR) RPC v5 API Query Engine
* **Concept:** Search, query, and download AUR tarballs (`aur.archlinux.org/rpc/v5/search`) with dependency graph resolution.
* **SigmaOS Implementation:** `SovereignAurRpcEngine` in `src/sigpkg/arch_pacman_engine.rs`.

---

## 🛠️ ARCHITECTURAL MAP

```
+-----------------------------------------------------------------------------------+
|               SIGMAOS ARCH LINUX PARITY SUBSYSTEM ENGINE                        |
+-----------------------------------------------------------------------------------+
| 1. ALPM Transaction Hooks Dispatcher (`SovereignAlpmHookDispatcher`)              |
| 2. `makepkg` PKGBUILD Script Sandbox Runner (`SovereignPkgbuildRunner`)           |
| 3. Pacman DB Sync Lock & Verifier (`SovereignPacmanDbVerifier`)                  |
| 4. AUR RPC v5 Query & Dependency Resolver (`SovereignAurRpcEngine`)               |
+-----------------------------------------------------------------------------------+
```

---

## 🧪 TESTING & VERIFICATION PLAN

1. **Standalone Unit Tests:** `rustc --test --edition=2021 src/sigpkg/arch_pacman_engine.rs`
2. **Integration Verification:** `pytest tests/`
3. **Documentation Parity Check:** Verify synchronization across `./`, `docs/`, `wiki/`, and `WIKI/`.

---

*End of Arch Linux Pull Request Specification.*
