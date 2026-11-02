# SigmaOS Repo Governance Template

> **Canonical governance template for every SigmaOS repository.**
> Every repo must populate all 17 sections below. Sections may not be removed.
> Heading text must remain verbatim so automation can reference sections by name.

---

## 1. Scope and Responsibility

_What exactly does this repo own? State the single, bounded responsibility in 1–3 sentences. Include what runtime layer or subsystem it lives in (kernel, core, desktop, user-space app, tooling, documentation). If the scope is shared with another repo, name that repo and describe the boundary explicitly._

**Template fields:**
- **Primary responsibility:** <!-- one sentence -->
- **Runtime layer:** <!-- kernel / core-system / desktop-shell / user-app / tooling / docs -->
- **Scope boundary with adjacent repos:** <!-- list repo names and where your boundary ends -->

---

## 2. Public API / Contract

_List every stable interface this repo exposes to other SigmaOS repos or external consumers: D-Bus services, shared libraries, CLI entry points, REST/IPC endpoints, environment variables, config file formats, and kernel interfaces. Mark each as `stable`, `unstable`, or `internal-only`._

**Template fields:**
- **Exposed interfaces:** <!-- table: Interface | Type | Stability -->
- **Breaking-change policy:** <!-- how changes are communicated before a break -->
- **API documentation location:** <!-- path or URL to generated/maintained API docs -->

---

## 3. Dependencies and Upstream Interface

_List all hard and optional runtime dependencies by repo name or upstream package. For each upstream dependency, state the minimum version, the reason it is needed, and whether SigmaOS carries any patches against it._

**Template fields:**
- **Hard dependencies:** <!-- table: Name | Min Version | Reason | Patched? -->
- **Optional/soft dependencies:** <!-- same format -->
- **Upstream patch inventory:** <!-- list of patches and their status (upstreamed / local-only / planned) -->
- **Dependency update policy:** <!-- how often deps are reviewed and how version pins are bumped -->

---

## 4. Ownership and Maintainer

_Name the current primary maintainer(s) and any co-maintainers. Include contact information or GitHub handle. Define the review policy (who must approve PRs, what quorum is needed, vacation/absence fallback)._

**Template fields:**
- **Primary maintainer:** <!-- GitHub handle or name -->
- **Co-maintainers:** <!-- list -->
- **Review quorum:** <!-- e.g., "1 maintainer approval required before merge" -->
- **Escalation path:** <!-- who to contact if primary is unavailable -->
- **Last ownership review date:** <!-- ISO date -->

---

## 5. Test Status and CI Status

_Describe the current test coverage, CI pipeline status, and the minimum acceptable quality bar before a PR is merged. Link to the CI configuration file and the latest build badge._

**Template fields:**
- **Unit test coverage:** <!-- percentage or "none yet" -->
- **Integration tests:** <!-- present / absent / planned -->
- **CI pipeline file:** <!-- path, e.g., `.github/workflows/ci.yml` -->
- **Required passing checks before merge:** <!-- list check names -->
- **Known flaky tests:** <!-- list or "none" -->
- **CI build badge:** <!-- markdown badge snippet -->

---

## 6. Support Matrix

_Define which CPU architectures, operating-system targets, kernel versions, and hardware classes this repo supports. Use explicit `supported`, `best-effort`, or `unsupported` labels._

**Template fields:**
- **CPU architectures:** <!-- table: Arch | Status (supported/best-effort/unsupported) -->
- **OS/kernel targets:** <!-- table: Target | Min Version | Status -->
- **Hardware classes:** <!-- desktop / laptop / server / embedded / VM -->
- **Backend variants:** <!-- Linux / BSD / custom — state which are tested in CI -->

---

## 7. Release Maturity

_Classify the current release maturity using exactly one of the defined labels. Update this section on every release._

**Allowed labels:**
- `planned` — does not exist yet; planned in the roadmap
- `prototype` — proof-of-concept only; not for any real use
- `alpha` — core functionality present but APIs unstable; expect breaking changes
- `beta` — feature-complete for the target milestone; stabilizing APIs; known bugs acceptable
- `stable` — production-ready; breaking changes require a major version bump
- `archived` — no active development; kept for historical reference or compatibility shims only

**Template fields:**
- **Current maturity label:** <!-- one of the labels above -->
- **Last label change date:** <!-- ISO date -->
- **Planned next label:** <!-- e.g., "beta → stable on v1.0 release" -->
- **Criteria for promotion:** <!-- what must be true before the label advances -->

---

## 8. What This Repo Does NOT Do

_Explicitly list responsibilities that a reader might expect this repo to cover but that belong elsewhere. Link to the correct repo for each excluded concern. This prevents scope creep and mis-filed issues._

**Template fields:**
- **Excluded responsibilities:** <!-- bulleted list, each with "→ handled by [repo name]" -->

---

## 9. Config Schema and Runtime Contract

_Define every configuration key, environment variable, and runtime contract this component exposes or consumes. State the schema format (TOML, JSON Schema, D-Bus introspection XML, etc.) and where the schema file lives in the repo._

**Template fields:**
- **Config file path(s):** <!-- e.g., `/etc/sigmaos/<component>.toml` -->
- **Schema file location:** <!-- path within the repo -->
- **Schema format:** <!-- TOML / JSON Schema / INI / D-Bus XML / other -->
- **Runtime environment variables:** <!-- table: Variable | Default | Purpose -->
- **Breaking config change policy:** <!-- how old configs are migrated and when old keys are removed -->

---

## 10. Failure Modes and Recovery Path

_Document every known failure mode: what causes it, what observable symptoms it produces, and the recovery path for both end users and operators. Include fallback behavior when dependencies are unavailable._

**Template fields:**
- **Failure modes:** <!-- table: Failure | Cause | Symptom | Recovery step -->
- **Fallback behavior when dependency is unavailable:** <!-- per dependency -->
- **Operator escalation path:** <!-- where to file issues, what logs to collect -->
- **Crash dump / diagnostics artifacts:** <!-- paths to logs, core dumps, status files -->

---

## 11. Update / Rollback Behavior

_Describe how this component is updated (package, in-place binary swap, live-reload, restart-required), the rollback mechanism if an update fails, and the expected downtime (if any) during an update._

**Template fields:**
- **Update mechanism:** <!-- package manager / hot-reload / restart / reboot-required -->
- **Rollback mechanism:** <!-- previous-version package / snapshot / none -->
- **Expected downtime on update:** <!-- zero / brief restart / reboot -->
- **Pre-update state checks:** <!-- what must be true before update is applied -->
- **Post-update validation:** <!-- how to verify the update succeeded -->
- **Atomic update guarantee:** <!-- yes / no / partial — explain -->

---

## 12. Security Boundary

_Define what security guarantees this component makes, what trust level it requires to run, what privilege it holds, and what threat model it is designed against. List any known CVEs or mitigations._

**Template fields:**
- **Privilege level required:** <!-- root / user / sandboxed / none -->
- **Trust boundary:** <!-- what it trusts, what it does not trust -->
- **Attack surface:** <!-- network / local-IPC / filesystem / none -->
- **Hardening measures in place:** <!-- e.g., seccomp, capabilities, namespace isolation -->
- **Threat model reference:** <!-- link to threat model doc or "none yet" -->
- **Known CVEs / mitigations:** <!-- list or "none" -->
- **Security audit status:** <!-- last audit date or "not yet audited" -->

---

## 13. Compatibility Matrix

_List all other SigmaOS repos and external software versions that this repo is tested against. Include known incompatibilities._

**Template fields:**
- **Tested against (SigmaOS repos):** <!-- table: Repo | Version Range | Status -->
- **Tested against (external):** <!-- table: Package | Version Range | Status -->
- **Known incompatibilities:** <!-- table: Package/Repo | Broken Since | Workaround -->
- **Forward-compatibility pledge:** <!-- what version range will remain compatible -->

---

## 14. Documentation Parity

_State whether user-facing and developer-facing documentation is current, what the gap is if not, and who is responsible for closing it. Documentation must be updated in the same PR as any API or behavior change._

**Template fields:**
- **User documentation status:** <!-- up-to-date / stale / missing | link -->
- **Developer/contributor docs status:** <!-- up-to-date / stale / missing | link -->
- **Man page or in-app help status:** <!-- present / absent / planned -->
- **Documentation parity policy:** <!-- describe the rule (e.g., "doc PR required with every API change") -->
- **Last documentation review date:** <!-- ISO date -->

---

## 15. Public Issue Labels and Roadmap

_List the canonical issue labels used in this repo's tracker and link to the public roadmap or milestone plan. Labels must be consistent across all SigmaOS repos._

**Required labels (all repos must use these):**

| Label | Meaning |
|-------|---------|
| `type:bug` | Confirmed regression or defect |
| `type:feature` | New capability or enhancement |
| `type:docs` | Documentation-only change |
| `type:chore` | Maintenance, dependency update, CI change |
| `status:needs-triage` | Not yet assessed by a maintainer |
| `status:accepted` | Confirmed and scheduled |
| `status:blocked` | Waiting on upstream or another repo |
| `priority:critical` | Production-breaking; handled immediately |
| `priority:high` | Important but not blocking release |
| `priority:normal` | Standard work queue item |
| `priority:low` | Nice-to-have; no timeline commitment |
| `maturity:prototype` | Work toward prototype milestone |
| `maturity:alpha` | Work toward alpha milestone |
| `maturity:beta` | Work toward beta milestone |
| `maturity:stable` | Work toward stable milestone |

**Template fields:**
- **Repo-specific extra labels:** <!-- any additional labels and their meaning -->
- **Public roadmap / milestone link:** <!-- URL or "none yet" -->
- **Issue triage cadence:** <!-- e.g., "every Monday" -->

---

## 16. Versioning Policy

_Define the versioning scheme, how version numbers are assigned, when pre-release suffixes are used, and how version tags are created in git._

**Template fields:**
- **Versioning scheme:** <!-- Semantic Versioning (MAJOR.MINOR.PATCH) is required for all stable repos -->
- **Pre-release suffix rules:** <!-- e.g., `-alpha.N`, `-beta.N`, `-rc.N` -->
- **Version tagging convention:** <!-- e.g., `vMAJOR.MINOR.PATCH` signed tags -->
- **Changelog format:** <!-- Keep-a-Changelog / Conventional Commits auto-generated / manual -->
- **Breaking change signal:** <!-- MAJOR bump required; announce in CHANGELOG and release notes -->
- **Version pinning in dependents:** <!-- how downstream repos pin this repo's version -->

---

## 17. Interaction with SigmaOS Core Repo

_Describe how this repo interacts with `SigmaOS-core`: whether it is a runtime dependency of core, provides services consumed by core, or is entirely independent. State the IPC/API boundary and the protocol used._

**Template fields:**
- **Relationship type:** <!-- depends-on-core / depended-on-by-core / peer / independent -->
- **IPC / integration mechanism:** <!-- D-Bus / shared library / socket / exec / none -->
- **Core API version it targets:** <!-- e.g., `SigmaOS-core >= 0.4` -->
- **Integration tested in CI:** <!-- yes / no -->
- **Change coordination policy:** <!-- how breaking changes in core are communicated to this repo and vice versa -->
- **Core team review required for cross-boundary changes:** <!-- yes / no -->
