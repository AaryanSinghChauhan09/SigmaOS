# Distro Absorption: Debian — Universal OS Stability Model

> **Status**: 🟢 Active & Tested | **Source Paradigm**: Debian GNU/Linux | **Target Shard**: `SigmaOS Stable Release Engineering`

---

## 1. Executive Summary

Debian is the gold standard for stability-first Linux distributions. Its legendary **three-tier release model** (stable, testing, unstable), rigorous **social contract**, and **DFSG (Debian Free Software Guidelines)** ensure long-term production reliability and ecosystem health.

SigmaOS adopts, implements, and expands Debian's **three-tier release model**, **Social Contract**, and **freeze-based stabilization** process as first-class citizens in our systems core to ensure production reliability alongside rolling-edge experimental channels.

---

## 2. Key Absorbed Features

### 2.1 Three-Tier Release Model

We model our package update channels directly on Debian's tiered model:

| Channel | SigmaOS Name | Stability | Update Frequency | Description |
| :-------- | :------------- | :--------- | :---------------- | :----------- |
| `unstable` | `sigma.next` | Experimental | Daily | Rolling development and experimental features |
| `testing` | `sigma.beta` | Mostly stable | Weekly | Pre-release and stabilization testing |
| `stable` | `sigma.stable` | Production | Quarterly LTS | Production LTS, quarterly security-only updates |

We expose an OOP-based `ThreeTierReleaseModel` structure allowing listing and switching between channels:

```bash
$ sigma-pkg channel list
Σ [PKG] Available channels:
  sigma.next   (Σ-next)   — Rolling, experimental, daily updates
  sigma.beta   (Σ-beta)   — Pre-release, weekly, mostly stable
  sigma.stable (Σ-stable) — Production LTS, quarterly security-only

$ sigma-pkg channel set sigma.stable
Σ [PKG] Channel set to sigma.stable (LTS). No experimental features.
```

### 2.2 Social Contract & DFSG Compliance

Our `DebianSocialContract` class validates system and package compliance against a formal Social Contract committing to:

1. **100% Free & Open Source**: Only software matching DFSG-compliant licenses (such as `MIT`, `Apache-2.0`, `GPL-2.0`, `GPL-3.0`, `BSD-2-Clause`, `BSD-3-Clause`) is enabled by default.
2. **Giving Back**: Contributing patches and tools back to upstream communities.
3. **Radical Transparency**: Keeping all bugs open and public.
4. **Prioritizing Users**: Designing capabilities and system updates strictly for user empowerment.

### 2.3 Freeze-Based Stabilization Lifecycle

Before every major stable release, a strict freeze-based stabilization window is enforced via `FreezeBasedStabilization`:

- **Unfrozen (Default)**: All updates (features, bugfixes, security, documentation) are allowed.
- **Frozen (Release Freeze)**: Only `security` and `critical-bugfix` update types are allowed to merge into the branch; standard features and cosmetic patches are blocked to guarantee absolute runtime regression resistance.

---

## 3. Class Specifications (OOP-Implementation)

The model is programmatically implemented in `src/distro/specialized.rs` with zero-allocation structures:

### `ThreeTierReleaseModel`
Manages the active channel, ensuring robust, validated switching.

### `DebianSocialContract`
Evaluates system compliance and DFSG license verification.

### `FreezeBasedStabilization`
Models update blocks and lifecycle stages of release engineering freezes.
