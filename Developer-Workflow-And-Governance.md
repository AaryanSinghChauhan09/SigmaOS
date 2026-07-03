# SigmaOS Developer Workflow & Governance

> Inspired by Linux kernel engineering practices + Claude-code AI-assisted tooling.
> Everything here is enforced by CI and git hooks — not just aspirational docs.

---

## Quick Setup (5 minutes)

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS
./scripts/setup_hooks.sh    # installs hooks, checks toolchain, configures git
```

This installs three git hooks:
- **commit-msg**: enforces Conventional Commits format
- **pre-commit**: runs rustfmt on staged `.rs` files
- **prepare-commit-msg**: auto-appends `Signed-off-by:`

---

## Commit Message Convention

SigmaOS uses [Conventional Commits](https://www.conventionalcommits.org/), extended with OS-specific types:

```
type(scope): short description (max 100 chars)

Optional longer body explaining WHY, not what.

Signed-off-by: Your Name <your@email.com>
```

### Valid types

| Type | Use for |
|------|---------|
| `feat` | New feature |
| `fix` | Bug fix |
| `impl` | Implements a roadmap/ideas item |
| `kernel` | Kernel subsystem work |
| `driver` | Hardware driver |
| `security` | Security fix or hardening |
| `boot` | Bootloader / UEFI |
| `perf` | Performance improvement |
| `pkg` | Package manager |
| `ai` | sigma-ai / GGUF / NL-CLI |
| `ux` | Desktop / compositor |
| `sdk` | SDK / driver DDK |
| `ci` | CI / workflow changes |
| `docs` | Documentation only |
| `test` | Tests only |
| `refactor` | Refactoring (no behaviour change) |
| `chore` | Maintenance |

### Examples

```
kernel(sched): add MLFQ priority boost to prevent starvation

Without periodic boosting, low-priority tasks can starve indefinitely
when the high-priority queue is always non-empty. This implements
the aging mechanism described in OSTEP Chapter 8.

Signed-off-by: Aaryan Singh Chauhan <aaryansinghchauhan09@github>

impl(wifi): Intel iwlwifi WPA3-SAE commit/confirm state machine

Resolves #86 from Ideas-999-Structured.md (Wi-Fi Driver DDK).
Tested in QEMU with simulated WPA3 AP.

Signed-off-by: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
```

### `Signed-off-by` requirement

Required on **all commits** touching `kernel/`, `drivers/`, `security/`, or `kabi/`.
This is the Developer Certificate of Origin (DCO) — you certify the code is your own
and you have the right to submit it.

---

## Pull Request Process

### Before opening a PR

1. Run locally:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ./kabi/check.py check         # verify no ABI breakage
   ```

2. Fill in the PR template (`.github/PULL_REQUEST_TEMPLATE.md`) — every section matters.

3. Reference the Ideas-999-Structured.md idea number if applicable.

### Review requirements

| Change type | Required approvals |
|-------------|-------------------|
| Docs / wiki | 1 maintainer |
| Userland tools | 1 maintainer + CI green |
| Kernel subsystem | 2 maintainers + CI green + CODEOWNERS |
| Security / crypto | 2 maintainers + Security Lead + CI green |
| ABI change | 2 maintainers + kabi/check.py snapshot update |

### AI PR Summarizer

The `sigma_dev_workflow.yml` CI job runs automatically on every PR and posts a comment with:
- Changed subsystems and file list
- Suggested review checklist
- Auto-detected test gaps
- Affected CODEOWNERS

---

## CI Pipeline

### Workflows that run on every PR

| Workflow | What it checks |
|----------|----------------|
| `sigma_dev_workflow.yml` | Commit lint, rustfmt, clippy, Nim check, SPDX headers, unsafe audit, cargo-audit CVE scan, SBOM, AI PR summary |
| `sigma_multiarch_ci.yml` | Cross-build x86_64/arm64/riscv64, QEMU smoke boot, OCI compat, reproducible builds |
| `pr_quality_gate.yml` | Commit lint, format, license, build matrix |

### Nightly-only jobs

| Job | Schedule |
|-----|----------|
| Syscall fuzz (cargo-fuzz) | `0 2 * * *` |
| cargo-audit security scan | `0 2 * * *` |
| ABI regression check | `0 2 * * *` |

### Running CI locally (act)

```bash
# Install: https://github.com/nektos/act
act pull_request -W .github/workflows/sigma_dev_workflow.yml
```

---

## Kernel ABI Stability

SigmaOS maintains a stable ABI for the **Driver DDK** and **C-ABI exports**:

```bash
# Check for ABI breakage before pushing
python kabi/check.py check

# Generate a new snapshot after an intentional ABI addition
python kabi/check.py snapshot --version v15.1.0

# View the full stable symbol list
python kabi/check.py report
```

See [RFC-0002: Kernel Stable ABI Policy](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/rfcs/RFC-0002-kernel-stable-abi.md).

---

## RFC Process

Significant changes require an RFC before implementation:

1. Copy `docs/rfcs/RFC-0001-template.md` → `docs/rfcs/RFC-NNNN-<topic>.md`
2. Open a GitHub Issue titled `RFC: <topic>` with label `rfc`
3. Discussion period: minimum 7 days
4. Maintainer signs off → RFC status becomes **Accepted**
5. Implementation PR links back to the RFC

**RFCs are required for:**
- New syscalls
- ABI-breaking changes
- New kernel subsystems
- Changes to `sigma_pledge` promise bits
- Changes to the sigpkg format

---

## Subsystem Maintainers

See the [MAINTAINERS](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/MAINTAINERS)
file for the complete subsystem → maintainer mapping (Linux kernel style).

### Key areas

| Subsystem | Maintainer file path |
|-----------|---------------------|
| Kernel core (sched/mm/syscalls) | `MAINTAINERS: KERNEL CORE` |
| Security (pledge/seccomp/PQC) | `MAINTAINERS: SECURITY SUBSYSTEM` |
| Wi-Fi drivers | `MAINTAINERS: NETWORK STACK — WI-FI` |
| sigma-pkg | `MAINTAINERS: PACKAGE MANAGER` |
| Zenith desktop | `MAINTAINERS: ZENITH DESKTOP` |

---

## Performance & Tracing

```bash
# Run the microbenchmark suite
sigma-perf bench --save

# Live kernel trace (requires kernel trace socket)
sigma-trace live

# Generate a flamegraph
sigma-trace flamegraph | inferno-flamegraph > sigma-flame.svg

# Hardware counter summary
sigma-perf stat
```

See: [sigma-perf source](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/tools/sigma_perf.rs) ·
[sigma-trace source](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/tools/sigma_tracing.rs)

---

## SBOM and Supply Chain Security

Every build on `main` generates a [CycloneDX](https://cyclonedx.org/) SBOM artifact,
downloadable from the GitHub Actions run. This lets you verify every dependency.

```bash
# Generate SBOM locally
cargo install cargo-cyclonedx
cargo cyclonedx --format json --output-file sbom/sigmaos.cdx.json
```

---

## Branching Model

| Branch | Purpose | Protection |
|--------|---------|------------|
| `main` | Single unified branch | PR required, CI must pass, CODEOWNERS enforced |
| `release/vX.Y.Z` | Release maintenance | Backports only, no force-push |
| `kernel-exp` | Experimental kernel | No protection (caveat emptor) |

**Never force-push to `main`.** Use `git revert` to undo mistakes.

---

*See also: [CONTRIBUTING.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md) ·
[GOVERNANCE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/GOVERNANCE.md) ·
[MAINTAINERS](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/MAINTAINERS)*
