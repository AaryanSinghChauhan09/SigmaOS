# Pull Request: Distro-Style Evidence-Gated Gap-Closing Roadmap

## Title
`feat(roadmap): Distro-Style Evidence-Gated Gap-Closing Roadmap`

## Summary
SigmaOS’s central gap is not a shortage of plans—it is the distance between documented ambition and reproducible, runtime-verified OS behavior. The wiki itself states that a listed source module is not proof of a working runtime feature, and its support matrix currently records 0% QEMU boot success, no tested hardware, and no evidence for the M1 x86_64 boot milestone.

This Pull Request establishes a **distro-style, evidence-gated gap-closing model** for SigmaOS: identify gaps, rank them by user-visible risk, close the smallest complete vertical slice, prove it in QEMU, then expand outward. This mirrors how practical Linux/BSD distributions prioritize a bootable installer, core userspace, package management, and recovery before broad feature parity.

---

## Distro & BSD Practice Mapping

| Distro/BSD practice | SigmaOS gap-closing application |
|---|---|
| Debian installer first | Prove QEMU x86_64 boot-to-shell before desktop ambitions |
| Arch reproducible build discipline | Every artifact must be rebuildable from a locked toolchain and commit |
| FreeBSD release engineering | Maintain a release checklist, known issues, and rollback path |
| OpenBSD least privilege | Security boundaries are reviewed separately and never bypassed for speed |
| Fedora CI/gating | CI must block regressions, not merely record them |
| NixOS declarative configuration | Make system configuration, tests, and build inputs explicit |
| Linux hardware enablement | Maintain a support matrix with logs, dates, and tested device models |

---

## Priority Gap Classes

| Gap class | Current signal | Closure outcome |
|---|---|---|
| Boot and installation | QEMU x86_64 boot success is 0%; UEFI/BIOS marked “not started” | Reproducible boot-to-shell and clean-install path |
| Runtime integration | Source modules are not proof of working runtime features | End-to-end subsystem tests connected to boot |
| Capability truth | Many plans, broad status vocabulary, historical documents | Evidence-backed component status |
| Hardware enablement | No real hardware tested; no graphics, Wi-Fi, audio, USB, or suspend support | Tiered device support with test logs |
| Packaging and updates | Packaging/update recovery remains a stated priority | Safe install, update, rollback, and recovery workflow |
| Security | Security is a core component but requires separate assurance | Threat model, capability tests, and review gates |
| Documentation sprawl | 165 wiki pages and many historical planning documents | Canonical component pages and archived plans |

---

## Gap-Closing Operating Loop

Every gap moves through a strict 9-stage lifecycle:

```text
IDENTIFY GAP
  → CLASSIFY severity and user impact
  → DEFINE MINIMUM PROOF
  → CREATE VERTICAL SLICE
  → IMPLEMENT
  → TEST UNIT + INTEGRATION + RUNTIME
  → UPDATE CANONICAL COMPONENT PAGE
  → RELEASE GATE
  → MONITOR REGRESSIONS
```

A gap is closed only when its implementation, tests, documentation, runtime evidence, and release status agree.

---

## Gap Severity Model

| Severity | Definition | Examples | Closure target |
|---|---|---|---|
| S0 — Blocking | Prevents boot, install, update, recovery, or basic use | Kernel panic, broken ISO, failed boot-to-shell | Fix immediately |
| S1 — Core parity | Required for a minimal usable OS | Shell, filesystem, process execution, basic networking | Current milestone |
| S2 — Distro usability | Expected by desktop Linux/BSD users | Installer recovery, package updates, desktop basics | Next release |
| S3 — Enhancement | Valuable but not required for core use | Gaming suite, streaming apps, advanced GPU acceleration | Backlog |

---

## 10-Point Minimum Proof Standard

A gap may be marked **closed** only when all 10 of the following criteria are satisfied:

1. **Implementation exists** in the canonical source path.
2. **Unit tests pass** for isolated logic.
3. **Integration tests pass** across module boundaries.
4. **Runtime evidence exists** from QEMU or named hardware.
5. **Documentation matches reality**, including limitations.
6. **Regression test exists** to prevent reversal.
7. **Rollback or recovery path is defined** for installer, update, and boot changes.
8. **Security review is complete** for privilege, memory, crypto, sandbox, package, or network changes.
9. **Status is updated** on the component’s canonical wiki page, not in an archived plan.
10. **Evidence is linked**, dated, and reproducible.

---

## Canonical Status Vocabulary

| Status | Allowed evidence | Prohibited claim |
|---|---|---|
| `Proposed` | Design document or issue | “Works” or “supported” |
| `Prototype` | Unit test or isolated demo | “Boot-ready” or “production-ready” |
| `Integrated` | Integration test plus runtime path | “Hardware supported” without hardware log |
| `Supported` | Reproducible QEMU/hardware test, release checklist, regression coverage | “Universal” compatibility without test corpus |

---

## Workstreams

### Workstream 1: Boot and installation
- Build a clean ISO artifact from a pinned commit and toolchain.
- Test UEFI and BIOS boot separately in QEMU.
- Capture serial console, kernel log, boot duration, and failure stage.
- Define the minimum boot contract: bootloader → kernel init → root filesystem → init/services → shell/login.
- Add a boot-to-shell smoke suite.
- Add a clean-install test using a disposable virtual disk.
- Add update and rollback tests before calling installation complete.
- **Exit criterion:** 100% boot success across supported QEMU x86_64 CI images, with retained logs and artifact hashes.

### Workstream 2: Kernel and core runtime
- Select the minimum viable kernel path: boot, memory allocation, scheduling, basic syscalls, VFS, process creation, and shell execution.
- Write one integration test per subsystem exercising the real runtime path.
- Add a runtime dependency map showing component initialization ordering.
- Treat non-reachable subsystems as `Prototype`.
- Add fault-injection tests for memory exhaustion, invalid syscalls, malformed paths, and driver init errors.
- **Exit criterion:** Boot-to-shell run exercises every claimed core subsystem and produces a trace showing initialization and use.

### Workstream 3: Security and isolation
- Create a threat model covering untrusted packages, downloaded binaries, network input, local users, compromised processes, and update channels.
- Define capability boundaries for filesystem, network, process, IPC, device, and package execution.
- Add negative denial tests confirming protected operations fail without capability.
- Separate security review from feature review.
- Fuzz parsers, syscall handlers, IPC, filesystem paths, and package metadata.
- **Exit criterion:** A documented threat model, capability matrix, and passing denial tests for every security boundary.

### Workstream 4: Packaging, updates, and recovery
- Phase 1: parse and validate package metadata.
- Phase 2: resolve dependencies and simulate installation.
- Phase 3: install into an isolated test image.
- Phase 4: execute package scripts only in a sandboxed environment.
- Phase 5: add atomic update, rollback, and recovery tests.
- **Exit criterion:** Failed updates complete atomically or restore prior known-good state proven by automated test.

### Workstream 5: Hardware enablement
- Keep QEMU x86_64 as Tier 1 until stable.
- Add ARM64 QEMU only after x86_64 reaches its boot-success target.
- Maintain a device matrix with model, driver version, test date, pass/fail result, and log link.
- Start with virtio devices (`virtio-blk`, `virtio-net`, VGA framebuffer, `virtio-gpu`).
- **Exit criterion:** Every support-tier claim has a current test result; unsupported features remain explicitly listed.

### Workstream 6: Desktop and userspace
- Define a minimal desktop contract: display output, input, window creation, application launch, and shutdown.
- Add automated UI smoke tests after the underlying runtime is stable.
- Treat the software store as a packaging frontend.
- Add crash reporting after reliable logging exists.
- **Exit criterion:** QEMU boot reaches desktop, launches basic application, accepts input, and shuts down cleanly.

### Workstream 7: Documentation and project governance
- Declare component wiki pages canonical.
- Move historical plans into an archive namespace.
- Add front-matter verification block to component pages.
- Automatically flag unverified pages.
- Replace duplicated roadmap documents with links to canonical component pages.
- **Exit criterion:** Every active component page has current status, evidence, and next action.

---

## 12 Industry-Standard Rules

1. **Single source of truth:** Each component has exactly one canonical status page.
2. **Evidence-gated promotion:** No promotion from `Prototype` → `Integrated` → `Supported` without linked, dated, reproducible evidence.
3. **Vertical slices over horizontal promises:** Complete one narrow end-to-end path over partial implementations.
4. **CI is a gate, not a dashboard:** Failing required checks blocks merge.
5. **Reproducibility:** Record commit hash, toolchain versions, container image, commands, timestamps, exit codes, and artifact hashes.
6. **Least privilege:** Agents and CI receive minimal necessary permissions.
7. **Security changes require separate review:** Security boundaries require dedicated review.
8. **Fail safely:** Installer, update, boot, and storage changes must define a rollback/recovery path.
9. **No unsupported claims:** Reference-project capabilities cannot be claimed as SigmaOS capabilities.
10. **Regression protection:** Every closed gap adds an automated regression test.
11. **Scope discipline:** One PR closes one gap or narrow sub-gap.
12. **Human accountability:** Humans approve security, release, hardware support, and destructive operations.

---

## Gap-Closing Backlog (P0–P3)

### P0 — Restore Credibility
1. Create a machine-readable gap register.
2. Audit all component pages and downgrade unevidenced claims.
3. Add CI gates for clean build, unit tests, formatting, and test scripts.
4. Create QEMU x86_64 boot-to-shell validator.
5. Publish a "Verified Today" report generated from actual evidence.

### P1 — Complete M1
6. Prove UEFI boot in QEMU.
7. Prove BIOS boot in QEMU.
8. Prove boot-to-shell with serial logs.
9. Add VGA framebuffer smoke test.
10. Add basic `virtio-net` networking test.
11. Add clean-install test on a disposable virtual disk.
12. Add update rollback and recovery test.

### P2 — Make the OS Usable
13. Connect filesystem/VFS, process creation, syscalls, and shell into one runtime trace.
14. Add package metadata parsing and dependency resolution.
15. Add sandboxed package installation.
16. Add minimal desktop launch and input smoke tests.
17. Add diagnostics, crash logs, and known-issue report.

### P3 — Expand Support
18. Add ARM64 QEMU after x86_64 target succeeds.
19. Add real-hardware validation program with named devices and logs.
20. Expand package compatibility after sandbox and rollback tests pass.

---

## Metrics and Release Gates

| Metric | M1 target | Current baseline | Gate |
|---|---:|---:|---|
| QEMU x86_64 boot success | 100% | 0% | Must pass before nightly |
| QEMU ARM64 boot success | 95% | 0% | Future milestone |
| Clean installation success | 95%+ | 0% | Required for validated release |
| Boot-to-shell duration | Defined and stable | Unknown | Regression threshold |
| Capability claims with evidence | 100% | Unknown | Required for status promotion |
| Critical security regression response | Within 1 week | Not established | Release policy |
| Package update rollback success | 100% | Unknown | Required before update channel |

---

## First 30 Days Execution Calendar

- **Day 1–3:** Create gap register, severity model, evidence policy, canonical template.
- **Day 4–7:** Run full audit and downgrade unsupported claims.
- **Day 8–14:** Implement QEMU x86_64 boot validator and retain serial logs.
- **Day 15–21:** Fix first boot blocker; hold scope until boot-to-shell is reproducible.
- **Day 22–27:** Add clean-install and rollback tests.
- **Day 28–30:** Publish first evidence-backed M1 status report and known-issues list.
