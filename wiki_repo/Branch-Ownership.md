# Branch Ownership & CI Gates

SigmaOS uses a federated branch management model inspired by large-scale Linux distributions. This structure ensures that `main` remains stable (Debian-style LTS) while allowing rapid innovation in feature branches (Fedora-style experimentation).

---

## 🗂️ Branch Mapping

| Branch | Primary Owner | Key Deliverable | Priority | CI Gate |
|---|---|---|---|---|
| `main` | **Release Manager** | Stable LTS builds and ABI | High | Reproducible build pass |
| `drivers-dev` | **Drivers Lead** | USB, NVMe, GPU drivers | High | Driver unit + QEMU tests |
| `drivers-experimental`| **Drivers Lead** | Bleeding edge drivers | Medium | Integration smoke tests |
| `fs-dev` | **FS Lead** | Journaling encrypted FS | High | FS integrity tests |
| `kernel-exp` | **Research Lead** | Microkernel/WASM experiments | Low | Research CI only |
| `sigpkg` | **Packaging Lead** | Package manager MVP | Medium | Reproducible package build |
| `release/desktop` | **UX Lead** | `sigma-desktop` alpha image | Medium | Image boot + smoke tests |
| `release/cloud` | **Cloud Lead** | Signed cloud image | Medium | Image signing + boot tests |

---

## 👤 Maintainer Roles

We are actively seeking contributors to step into these ownership roles. If you are interested, open a PR adding your GitHub handle to the `.github/CODEOWNERS` file.

- **Release Manager:** Enforces LTS policy, reviews cross-system impact, cuts releases.
- **Drivers Lead:** Architect of the device model, reviews PCI/USB/NVMe/GPU submissions.
- **FS Lead:** Architect of `SovereignFS`, manages journaling, encryption, and SPARK proofs.
- **Security Lead:** Owns the capability sandbox, syscall audit, and vulnerability reporting.
- **Packaging Lead:** Manages `sigpkg`, reproducible build tooling, and package signing.
- **UX Lead:** Owns the Zenith compositor, desktop apps, and SDK ergonomics.

---

## 🧪 Quality Gates for Merging

### Pull Request Requirements
To merge a PR into *any* protected branch, the following must pass:
1. **Unit Tests** (`just test`)
2. **Static Analysis** (`cargo clippy`, `cargo fmt`)
3. **Reproducible Build Check** (CI hashes match)

### Subsystem Merge to `main`
When merging from a development branch (e.g., `drivers-dev`) into `main`:
1. **Approval:** Requires approvals from at least **two maintainers**.
2. **Artifact:** Requires a cryptographically signed CI artifact demonstrating stability.
3. **Security:** If modifying `security/` or `kernel/`, requires formal verification (Ada/SPARK proofs) and Security Lead sign-off.

---

## 🛠️ Testing Infrastructure

- **Cross-compile Matrix:** `x86_64`, `aarch64`, `riscv64`
- **Emulation:** QEMU test harness runs on every PR for USB, NVMe, and network smoke tests.
- **Physical Hardware:** Scheduled nightly CI runners execute on bare-metal hardware for GPU, audio, and real NIC validation.
- **Formal Verification:** `gnatprove` runs on merges to `fs-dev` or bootloader paths. Proof reports must be attached to the PR.
