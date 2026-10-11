# AI Agent Maintenance Instructions

> **SigmaOS Master Directive** — All AI agents operating on SigmaOS MUST read and follow this document before making any changes. Stability > Security > Performance. Only Rust, Zig, or Nim.

---

## 1. Core Principles

SigmaOS aims to defeat Linux and BSD distributions in speed, stability, and performance. Every decision made by AI agents must align with:

1. **Stability First** — Never break existing functionality when adding new features.
2. **Security Second** — Audit all code, never bypass safety checks.
3. **Performance Third** — Optimize only after stability and security are assured.
4. **Modern Languages Only** — Rust, Zig, or Nim. No C, C++, Python, Go, Java, or JavaScript for OS components.

---

## 2. Language Policy

| Language | Role | Notes |
|----------|------|-------|
| **Rust** | Primary OS components | Use `no_std` for kernel space; zero-cost abstractions; memory safety |
| **Zig** | Systems-level, C interop | Use for comptime metaprogramming, explicit allocators, hardware init |
| **Nim** | Tooling, scripts, build helpers | Use `--gc:none` or `--mm:arc` for GC-free operation |
| C | **FORBIDDEN** | No exceptions for OS components |
| C++ | **FORBIDDEN** | No exceptions |
| Python | **FORBIDDEN** | Not even for build scripts |
| Go | **FORBIDDEN** | Runtime GC unacceptable in kernel |
| JavaScript | **FORBIDDEN** | Not even for tooling |

### `unsafe` Rust Policy
- `unsafe` blocks are permitted **only** when:
  - Interfacing with hardware registers
  - Implementing core memory primitives (allocators, page tables)
  - Wrapping Zig/C FFI boundaries (if unavoidable)
- Every `unsafe` block **must** have a `// SAFETY:` comment explaining why it is sound.
- Prefer safe abstractions; wrap unsafe in a safe API as soon as possible.

---

## 3. Git Workflow

### Branch Policy
- **Only one branch exists: `main`**
- All commits go directly to `main`
- No feature branches, no pull requests, no merge commits
- Rebase if needed: `git pull --rebase origin main`

### Commit Message Format (Conventional Commits)

```
<type>(<scope>): <short description>

[optional body — explain WHY, not WHAT]

[optional footer: breaking changes, issue refs]
```

**Types:**
| Type | When to Use |
|------|------------|
| `feat` | New feature or capability |
| `fix` | Bug fix |
| `docs` | Documentation-only changes (agent files, wiki, comments) |
| `chore` | Build system, dependencies, CI changes |
| `refactor` | Code restructuring without behavior change |
| `perf` | Performance improvement |
| `test` | Adding or fixing tests |
| `security` | Security fix or hardening |

**Scopes (one per component):**
`kernel`, `boot`, `scheduler`, `memory`, `fs`, `net`, `ipc`, `drivers`, `desktop`, `audio`, `graphics`, `input`, `display`, `storage`, `power`, `security`, `crypto`, `packages`, `init`, `monitoring`, `timekeeping`, `virt`, `containers`, `i18n`, `a11y`, `compat`, `agents`

**Examples:**
```
feat(scheduler): implement O(1) CFS-inspired runqueue with per-CPU migration
fix(memory): prevent UAF in slab allocator free path
security(crypto): replace MD5 with BLAKE3 in integrity checks
perf(fs): add readahead buffer for ext4-compatible driver
docs(agents): update NETWORKING_AGENT with QUIC transport notes
chore(agents): comprehensive update with Linux/BSD-inspired best practices
```

### Push Policy
- Always `git push origin main` immediately after committing
- Never leave unpushed commits for more than one session
- If push fails due to upstream divergence: `git pull --rebase origin main && git push origin main`

---

## 4. Running Cargo Check

Every AI agent **must** run `cargo check` before committing any Rust code changes:

```bash
# From the repo root
cd /home/aaryansinghchauhan/SigmaOS
cargo check --workspace 2>&1

# For a specific crate
cargo check -p sigma-kernel 2>&1

# With all features enabled
cargo check --workspace --all-features 2>&1
```

**Clippy (run before every commit):**
```bash
cargo clippy --workspace -- \
  -D clippy::unused_variables \
  -D clippy::unwrap_in_result \
  -W clippy::missing_safety_doc \
  -W clippy::undocumented_unsafe_blocks
```

**If `cargo check` fails:**
1. Fix ALL errors before committing — zero-error policy.
2. Warnings are allowed temporarily but must be tracked as GitHub Issues.
3. Never use `#[allow(unused)]` to silence warnings without a comment.

---

## 5. Code Quality Checklist

Before every commit, verify:

- [ ] `cargo check` passes with **0 errors**
- [ ] `cargo clippy` produces no new warnings vs. `main`
- [ ] No `unwrap()` or `expect()` in production paths — use `?` or explicit `match`
- [ ] No hard-coded credentials, secrets, or magic numbers without named constants
- [ ] Every `unsafe` block has a `// SAFETY:` justification comment
- [ ] Every public function, struct, and trait has a `///` doc comment
- [ ] New files include the SPDX license header: `// SPDX-License-Identifier: MIT OR Apache-2.0`
- [ ] Error types implement `core::fmt::Display` and `core::fmt::Debug`
- [ ] No `std::process::exit()` in library code — return `Result` up the stack
- [ ] Integration tests updated if behavior changed
- [ ] Relevant agent `.md` file updated to reflect new implementation status

---

## 6. Code Scanning Alert Response Procedures

When GitHub Code Scanning (or local `cargo audit`) raises alerts:

### Severity: Critical / High
1. **Stop all other work immediately.**
2. Reproduce the issue locally.
3. Create a fix in a single commit with type `security(<scope>)`.
4. Run `cargo audit fix` for dependency vulnerabilities where safe.
5. Push immediately: `git push origin main`.
6. Update the relevant `Agents/SECURITY_AGENT.md` with the post-mortem.
7. If the vulnerability is in a dependency, pin to a safe version in `Cargo.toml`.

### Severity: Medium
1. Track as a GitHub Issue labeled `security`.
2. Fix within the current session if possible, otherwise within 48 hours.
3. Commit with type `security(<scope>): fix <cve-or-description>`.

### Severity: Low / Informational
1. Track as a GitHub Issue labeled `security/low`.
2. Batch-fix with other low-severity items.
3. Commit with type `fix` or `chore`.

### Running Local Security Audit
```bash
cargo install cargo-audit  # one-time setup
cargo audit
cargo audit fix --dry-run  # preview dependency fixes
```

---

## 7. Security Vulnerability Response Procedures

If a vulnerability is discovered in SigmaOS itself (not a dependency):

1. **Assess scope** — kernel, userspace, or both?
2. **Classify** — privilege escalation, information disclosure, DoS, memory corruption?
3. **Patch immediately** — no disclosure delay for a pre-release OS.
4. **Commit with**: `security(<scope>): [VULN] <description>`
5. **Update** `Agents/SECURITY_AGENT.md` Section 9 with details.
6. **Add regression test** that reproduces the vulnerability and confirms the fix.
7. **Document in** `SECURITY.md` at repo root (create if missing).

---

## 8. Agent File (`.md`) Maintenance

### When to Update an Agent File
- When implementing a new feature in that component → update Section 5 (status table)
- When discovering a bug → add to Section 6 (missing features) if architectural
- When Linux/BSD releases a new major version → review their changelog for that subsystem
- After every commit that touches `src/<component>/` → update the agent file in the same commit

### Transfer to GitHub Wiki
Transfer agent content to the GitHub Wiki (`<Component>-Component-Guide`) when:
- The component is **≥ 50% implemented** (as judged by the status table)
- The implementation is stable enough for external contributors to read
- Format: Copy Sections 1, 4, 5, 7, 8, 11, 14 into the wiki page

**Wiki page naming:** `<Component>-Component-Guide` (e.g., `Scheduler-Component-Guide`)

### Agent File Naming Convention
`Agents/<COMPONENT_NAME_IN_CAPS>_AGENT.md`

---

## 9. Stability-First Conflict Resolution Policy

When a proposed change conflicts with an existing feature:

1. **Never remove existing functionality** without a deprecation period (minimum 2 commits / sessions).
2. If a new algorithm replaces an old one, keep the old one behind a feature flag: `#[cfg(feature = "legacy-<name>")]`.
3. If API changes are required, provide a compatibility shim in `src/<component>/compat.rs`.
4. Document the conflict in the commit message body (not just the subject line).
5. When in doubt, choose the more conservative option and open a GitHub Issue for discussion.

**Priority order when resolving conflicts:**
```
Stability > Security > Performance > Feature completeness > Code elegance
```

---

## 10. Component Ownership Map

Each AI agent session should identify which component it owns and operate only within that scope unless explicitly tasked with cross-component work.

| Component | Agent File | Primary Source Directory |
|-----------|-----------|--------------------------|
| Kernel core | `KERNEL_AGENT.md` | `src/kernel/` |
| Boot | `BOOTLOADERS_AGENT.md` | `src/boot/` |
| Scheduler | `SCHEDULER_AGENT.md` | `src/kernel/sched/` |
| Memory | `MEMORY_MANAGEMENT_AGENT.md` | `src/kernel/mm/` |
| Filesystems | `FILESYSTEMS_AGENT.md` | `src/fs/` |
| Networking | `NETWORKING_AGENT.md` | `src/net/` |
| IPC | `IPC_AGENT.md` | `src/ipc/` |
| Drivers | `DRIVERS_AGENT.md` | `src/drivers/` |
| Desktop | `DESKTOP_AGENT.md` | `src/desktop/` |
| Audio | `AUDIO_AGENT.md` | `src/audio/` |
| Graphics | `GRAPHICS_AGENT.md` | `src/graphics/` |
| Input | `INPUT_AGENT.md` | `src/input/` |
| Display | `DISPLAY_AGENT.md` | `src/display/` |
| Storage | `STORAGE_AGENT.md` | `src/storage/` |
| Power | `POWER_MANAGEMENT_AGENT.md` | `src/power/` |
| Security | `SECURITY_AGENT.md` | `src/security/` |
| Cryptography | `CRYPTOGRAPHY_AGENT.md` | `src/crypto/` |
| Packages | `PACKAGE_MANAGEMENT_AGENT.md` | `src/packages/` |
| Init/systemd | `SYSTEMD_INIT_AGENT.md` | `src/init/` |
| Monitoring | `SYSTEM_MONITORING_AGENT.md` | `src/monitoring/` |
| Timekeeping | `TIMEKEEPING_AGENT.md` | `src/time/` |
| Virtualization | `VIRTUALIZATION_AGENT.md` | `src/virt/` |
| Containers | `CONTAINERS_AGENT.md` | `src/containers/` |
| i18n | `INTERNATIONALIZATION_AGENT.md` | `src/i18n/` |
| Accessibility | `ACCESSIBILITY_AGENT.md` | `src/a11y/` |
| Compatibility | `COMPATIBILITY_AGENT.md` | `src/compat/` |
| Linux/BSD parity | `LINUX_BSD_DISTRO_PARITY_ROADMAP_AGENT.md` | (cross-cutting) |
| Omarchy parity | `OMARCHY_LINUX_PARITY_AGENT.md` | (cross-cutting) |

---

## 11. Session Startup Checklist

At the start of every AI agent session:

1. `git pull --rebase origin main` — sync with latest upstream
2. `cargo check --workspace` — verify clean state
3. Read the relevant `Agents/<COMPONENT>_AGENT.md` for your component
4. Check `WHAT_IS_WORKING_AND_NOT_WORKING.md` for current status
5. Check `FEATURE_STATUS.toml` for component priorities
6. Begin work on the highest-priority missing feature in your component's Section 6

---

## 12. Session End Checklist

Before ending every AI agent session:

1. `cargo check --workspace` — must pass with 0 errors
2. `cargo clippy --workspace` — no new warnings
3. Update the relevant `Agents/<COMPONENT>_AGENT.md` Section 5 status table
4. `git add -A && git commit -m '<type>(<scope>): <description>'`
5. `git push origin main`
6. Report summary back to the orchestrating agent

---

## 13. References

- Conventional Commits: https://www.conventionalcommits.org/
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- Rust `unsafe` guidelines: https://doc.rust-lang.org/nomicon/
- Zig documentation: https://ziglang.org/documentation/master/
- Linux kernel coding style (for inspiration): https://www.kernel.org/doc/html/latest/process/coding-style.html
- FreeBSD Developer Handbook: https://docs.freebsd.org/en/books/developers-handbook/
- OpenBSD coding style: https://man.openbsd.org/style.9
