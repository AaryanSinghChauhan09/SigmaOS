# GitHub Issues to Open

Here are the ready-to-paste templates for the top 10 deliverables in our Month 0–3 Foundation Sprint. 
You can copy and paste these directly into GitHub to track the work on your project board.

---

### Issue 1: Publish ABI specification and LTS policy
**Title:** Publish ABI specification and LTS policy
**Description:**
```markdown
As part of our Month 0-3 Foundation Sprint, we need to document the formal non-POSIX syscall ABI for SigmaOS and define our LTS (Long Term Support) policy to ensure a stable `main` branch.

**Acceptance Criteria:**
- [ ] Create `docs/ABI.md` detailing the versioned syscall interface.
- [ ] Create `docs/LTS-Policy.md` (or add to `Contributing.md`) defining the `main` branch stability guarantees.
- [ ] Review and merge via PR.

**Labels:** `documentation`, `good first issue`, `phase:0.2`
**Assignee:** @ReleaseManager
```

---

### Issue 2: Create reproducible build CI job for `main`
**Title:** Create reproducible build CI job for `main`
**Description:**
```markdown
To ensure supply chain security and stability, the `main` branch must have a CI pipeline that verifies the kernel and userland can be built reproducibly across multiple architectures (x86_64, aarch64, riscv64).

**Acceptance Criteria:**
- [ ] Create `.github/workflows/reproducible-build.yml`.
- [ ] Add matrix build targets: `x86_64-unknown-none`, `aarch64`, `riscv64`.
- [ ] Verify that building the same commit twice produces matching binary hashes.

**Labels:** `ci`, `phase:0.2`
**Assignee:** @ReleaseManager
```

---

### Issue 3: Add role-based Wiki pages and contribution templates
**Title:** Add role-based Wiki pages and contribution templates
**Description:**
```markdown
To lower the barrier to entry for new contributors and clarify responsibilities, we need to flesh out role-based guides and standardize our issue/PR templates.

**Acceptance Criteria:**
- [ ] Create `.github/ISSUE_TEMPLATE/` files (Bug Report, Feature Request, Security Report).
- [ ] Create `.github/PULL_REQUEST_TEMPLATE.md`.
- [ ] Add role-based guides in `docs/wiki/`: Developer, Maintainer, Security Researcher, Hardware Vendor.

**Labels:** `documentation`, `good first issue`, `phase:0.2`
**Assignee:** @ReleaseManager
```

---

### Issue 4: Implement `sigma-sh` minimal shell and 3 core utilities
**Title:** Implement `sigma-sh` minimal shell and 3 core utilities
**Description:**
```markdown
We are absorbing the functionality of Bash/Zsh and GNU Coreutils to make SigmaOS self-sufficient. This issue tracks the initial MVP in Rust/Zig.

**Acceptance Criteria:**
- [ ] Finalize `sigma-sh` v0.2 REPL and scripting engine.
- [ ] Implement `ls` equivalent in Rust (in `sigma-core-utils`).
- [ ] Implement `cat` equivalent in Rust.
- [ ] Implement `cp` equivalent in Rust.
- [ ] Ensure all compile under `cargo build` without external dependencies.

**Labels:** `component:sigma-sh`, `component:coreutils`, `absorption`, `phase:0.2`
**Assignee:** @PackagingLead
```

---

### Issue 5: Define Security Model and CVE reporting workflow
**Title:** Define Security Model and CVE reporting workflow
**Description:**
```markdown
SigmaOS needs a formalized security posture. While the Security Model Wiki page outlines the architecture, we need a formal CVE reporting process and a finalized `SECURITY.md` file in the repository root.

**Acceptance Criteria:**
- [ ] Create `.github/SECURITY.md` outlining the responsible disclosure policy.
- [ ] Finalize the CVE reporting workflow (e.g., using GitHub Security Advisories).
- [ ] Link `SECURITY.md` from the README and Wiki.

**Labels:** `security`, `documentation`, `phase:0.2`
**Assignee:** @SecurityLead
```

---

### Issue 6: Publish Driver API spec and sample USB driver
**Title:** Publish Driver API spec and sample USB driver
**Description:**
```markdown
To enable community driver development in the `drivers-dev` branch, we need a documented Driver API and a sample implementation (USB or simple fallback).

**Acceptance Criteria:**
- [ ] Create `docs/driver-api.md` outlining device enumeration, interrupts, and DMA.
- [ ] Create a sample USB host controller driver skeleton in `kernel/drivers/usb/`.
- [ ] Create QEMU tests for the driver.

**Labels:** `component:drivers`, `phase:0.2`
**Assignee:** @DriversLead
```

---

### Issue 7: Create `sigpkg` repo skeleton and package signing spec
**Title:** Create `sigpkg` repo skeleton and package signing spec
**Description:**
```markdown
We are building a sovereign package manager (`sigpkg`) inspired by Wolfi OS reproducible packaging. This issue tracks the completion of the package signing specification.

**Acceptance Criteria:**
- [ ] Document the package metadata schema (JSON/TOML).
- [ ] Document the Ed25519 + SHA-256 signing process for packages.
- [ ] Integrate signing logic into `sigpkg` v0.2.

**Labels:** `component:sigpkg`, `security`, `phase:0.2`
**Assignee:** @PackagingLead
```

---

### Issue 8: Publish FS design doc for journaling and encryption
**Title:** Publish FS design doc for journaling and encryption
**Description:**
```markdown
`SovereignFS` will be the default filesystem, replacing ext4/btrfs. We need a formal design document outlining the on-disk format, journaling mechanism, and native encryption before implementation begins.

**Acceptance Criteria:**
- [ ] Create `docs/fs-design.md`.
- [ ] Outline on-disk structures (superblocks, inodes, extents).
- [ ] Define the journaling strategy (data vs. metadata journaling).
- [ ] Define the encryption strategy (file-based or block-based).

**Labels:** `component:fs`, `documentation`, `phase:0.2`
**Assignee:** @FSLead
```

---

### Issue 9: Implement NVMe driver and add QEMU tests
**Title:** Implement NVMe driver and add QEMU tests
**Description:**
```markdown
Modern hardware relies heavily on NVMe. We need a robust, non-blocking NVMe driver implemented in Rust for the `drivers-dev` branch.

**Acceptance Criteria:**
- [ ] Implement NVMe PCIe enumeration and queue pairs.
- [ ] Implement block read/write operations.
- [ ] Add QEMU automation to test NVMe read/write workflows in CI.

**Labels:** `component:drivers`, `hardware`, `phase:0.3`
**Assignee:** @DriversLead
```

---

### Issue 10: Define `sigma-core` meta-manifest and alpha desktop image
**Title:** Define `sigma-core` meta-manifest and alpha desktop image
**Description:**
```markdown
To move towards our first release profiles, we need to define the exact package compositions of our target environments.

**Acceptance Criteria:**
- [ ] Create `config/profiles/sigma-core.toml` listing all base CLI packages.
- [ ] Create `config/profiles/sigma-desktop.toml` extending core with Zenith compositor.
- [ ] Set up a build pipeline to output a bootable ISO for `sigma-desktop-alpha`.

**Labels:** `release`, `component:desktop`, `phase:0.3`
**Assignee:** @ReleaseManager
```
