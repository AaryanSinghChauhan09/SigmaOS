# SigmaOS: Release Channel Specification

This document specifies the release management structure, channel promotions, test gates, and backporting automations of SigmaOS.

---

## ⚡ Release Channels

SigmaOS splits development into three distinct channels to balance bleeding-edge software access with enterprise-grade stability:

```
    +--------------------------------------------------------------+
    |                     SIGMA-EXPERIMENTAL                       |
    |  - Bleeding-edge features, WIP kernel modules, new drivers   |
    +--------------------------------------------------------------+
                                   |
                     Promotion Gate: Passes Unit &
                     Integration Test Matrix
                                   v
    +--------------------------------------------------------------+
    |                        SIGMA-ROLLING                         |
    |  - Daily builds, fully tested driver trees, package updates  |
    +--------------------------------------------------------------+
                                   |
                     Promotion Gate: Passes CI Gating Checklist,
                     Reproducibility, and 30-Day Stability Burn
                                   v
    +--------------------------------------------------------------+
    |                         SIGMA-STABLE                         |
    |  - Enterprise-grade releases, strict LTS guarantees,         |
    |    Dilithium-5 signed SBOMs.                                 |
    +--------------------------------------------------------------+
```

### 1. `sigma-experimental`
* **Target Audience:** Internal developers and core testers.
* **Release Frequency:** Continuous integration on every commit.
* **Test Gate:** Successful compilation and local shard-level unit tests.

### 2. `sigma-rolling`
* **Target Audience:** General developers, power users, and advanced workstations (comparable to Arch Linux or Fedora).
* **Release Frequency:** Daily or weekly rolls.
* **Test Gate:** Core virtual memory page controllers, completely fair schedulers, and network adapters pass integration test matrices.

### 3. `sigma-stable`
* **Target Audience:** Enterprise servers, cloud deployments, and production workstations.
* **Release Frequency:** Every 6 months with 2-year Long Term Support (LTS) lifecycle paths.
* **Test Gate:** Dual-build binary reproducibility checks pass, zero memory pool leaks are recorded, and the build passes a 30-day stability burn-in test.

---

## 🛠️ Automated Backport & Mitigation Workflow

To prevent fragmentation between `rolling` and `stable` channels, SigmaOS automates critical security fixes and driver compatibility mitigations:

1. **Backport Gating:** Security patches labeled with critical CVE tags merged into the main branch are automatically cherry-picked by a webhook action.
2. **Stable Regression Matrix:** Automated test runner VMs execute the patched code against older `stable` kernels to ensure no API compatibility regressions are introduced.
3. **PQC Re-signing:** Following a successful merge, the update manager compiles a patch block, signs it with the root Dilithium-5 private key, and pushes it directly to the `stable` channel.
