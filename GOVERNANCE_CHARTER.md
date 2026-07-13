# GOVERNANCE CHARTER

> **Status**: RATIFIED | **Version**: 1.0 | **Effective**: Session-3

To establish institutional trust and manage a scaling community of contributors, SigmaOS is adopting a transparent governance model inspired by the success of the Debian and Fedora projects.

---

## 1. Purpose and Scope

This charter defines the governance structure, decision-making processes, and community standards for the **SigmaOS Project**. It applies to:

- All code, documentation, and wiki contributions to the SigmaOS repository
- The SigmaOS Wiki and associated roadmaps
- Community communications (issues, PRs, discussions, forums)
- Release engineering and roadmap decisions

---

## 2. Governance Structure

```
┌─────────────────────────────────────────────┐
│            SIGMAOS GOVERNANCE               │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │         Sovereign Council           │   │
│  │    (3-7 elected maintainers)        │   │
│  └──────────────┬──────────────────────┘   │
│                 │                           │
│  ┌──────────────┴────────────────────┐     │
│  │         Technical Committee        │     │
│  │  (kernel, security, UX, packages)  │     │
│  └──────────────┬────────────────────┘     │
│                 │                           │
│  ┌──────────────┴────────────────────┐     │
│  │          Contributors              │     │
│  │  (anyone who submits quality PRs)  │     │
│  └────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
```

### 2.1 The Sovereign Council

The **Sovereign Council** is the steering committee responsible for the long-term vision, architectural integrity, and community health of SigmaOS.

**Responsibilities**:
- Set the annual roadmap and milestone priorities
- Break deadlocks in Technical Committee votes
- Approve or veto changes to this charter
- Manage release engineering and signing keys
- Represent SigmaOS in external partnerships

**Composition**:
- Minimum 3, maximum 7 members
- Initial members: Project Founder(s) and early core contributors
- New members elected by existing Council via simple majority
- Term: 2 years, renewable

**Decision Threshold**: Ordinary resolutions require simple majority (>50%). Charter amendments require supermajority (>66%).

### 2.2 Technical Committee

The **Technical Committee** (TC) handles day-to-day engineering decisions.

**Sub-committees**:

| Committee | Scope | CODEOWNERS Group |
|---|---|---|
| Kernel TC | `kernel/`, `kernel/security/` | `@sigma/kernel-maintainers` |
| Security TC | CVE response, PQC, MAC policies | `@sigma/security-team` |
| UX/Desktop TC | GUI, accessibility, theming | `@sigma/ux-team` |
| Package TC | `sigpkg`, shard manifests | `@sigma/package-maintainers` |
| Docs TC | Wiki, roadmaps, tutorials | `@sigma/docs-team` |

**Decision Process**: Decisions within a sub-committee scope require approval from 2+ members of that committee. Cross-cutting changes require approval from all affected committees.

### 2.3 Contributors

Anyone who submits a merged pull request is considered a **Contributor**. After 5 merged PRs, contributors may be nominated as **Committers** with direct push access to feature branches (not `main`).

---

## 3. Decision-Making Process

### 3.1 RFC (Request for Comments) Process

Significant changes MUST follow the RFC process:

```
1. Author opens an issue with [RFC] prefix
2. 14-day public comment period
3. Technical Committee reviews and responds
4. Author incorporates feedback → final RFC document
5. TC votes: approve / reject / defer
6. Approved RFCs merged into wiki/rfcs/
```

**RFC Template**: See [RFC-0001-template.md](RFC-0001-template.md)

**When RFC is Required**:
- New kernel subsystem or shard category
- Breaking changes to `sigma-bus` IPC ABI
- Changes to `sigpkg` package format
- Security policy changes
- Governance charter amendments

### 3.2 Ordinary PR Process

Standard contributions follow:

```
PR Submitted → CI passes → 1 reviewer approval (committer)
→ 1 maintainer approval → merge to main
```

For security-sensitive files (per `CODEOWNERS`), 2 security-team approvals are required.

### 3.3 Conflict Resolution

1. **Level 1**: Authors discuss in PR comments (48h window)
2. **Level 2**: Relevant TC sub-committee votes
3. **Level 3**: Full Sovereign Council vote
4. **Level 4**: Community vote (binding for charter issues only)

---

## 4. Release Engineering

### 4.1 Release Cadence

| Channel | Frequency | Stability | Audience |
|---|---|---|---|
| Nightly | Daily | Unstable | Developers |
| Beta | Monthly | Testing | Early adopters |
| Stable | Quarterly | Production | End users |
| LTS | Annual | Long-term | Enterprise |

### 4.2 Release Criteria

A release MUST pass:
- [ ] All CI pipelines green on `x86_64`, `arm64`, `riscv64`
- [ ] `sigma_quality_check.sh` returns 0 stubs
- [ ] Security audit by Security TC
- [ ] `sigpkg` package signing ceremony (Sovereign Council keys)
- [ ] Release notes approved by Docs TC

### 4.3 Signing Keys

Release artifacts are signed using:
- **Algorithm**: Dilithium5 (post-quantum)
- **Key ceremony**: Requires 3-of-5 Council members with hardware tokens
- **Key rotation**: Annual or upon member departure
- **Public key**: Published at `https://sigmaos.dev/keys/release.pub`

---

## 5. Intellectual Property

### 5.1 License Policy

- **Kernel and core**: GPLv2-only (SPDX: `GPL-2.0-only`)
- **Userland tools**: MIT or Apache-2.0 dual-license
- **Documentation**: CC-BY-SA-4.0
- **Shard contributions**: Must be GPL-2.0-compatible

All contributions require a Developer Certificate of Origin (DCO) sign-off:
```
Signed-off-by: Your Name <your@email.com>
```

### 5.2 FOSS Absorption Policy

When absorbing code from other projects:
- License must be GPL-2.0-compatible
- Attribution maintained in `THIRD_PARTY_LICENSES`
- Cleanroom reimplementation preferred for proprietary-inspired features
- Legal review required for any code from non-FOSS sources

---

## 6. Code of Conduct

SigmaOS enforces a strict [Code of Conduct](CODE_OF_CONDUCT.md). We prioritize:

- **Inclusivity**: All backgrounds welcome; zero tolerance for discrimination
- **Technical excellence**: Constructive, evidence-based technical discussion
- **Constructive feedback**: Critique code, not people
- **Transparency**: Decisions made in the open, reasoning documented

**Enforcement**: CoC violations reported to `conduct@sigmaos.dev`. The Sovereign Council acts as final arbiter.

---

## 7. Charter Amendments

To amend this charter:

1. Open a GitHub Discussion with tag `[charter-amendment]`
2. 30-day public comment period
3. Supermajority (>66%) Sovereign Council vote
4. Amendment takes effect 14 days after ratification

---

## 8. Dissolution

In the unlikely event of project dissolution:
- All code remains under its stated open-source license
- Wiki content migrates to a community-maintained archive
- Signing keys are revoked and publicly announced
- A 6-month maintenance window is provided for final releases

---

By adhering to this charter, SigmaOS aims to transition from a single-architect project to a **globally trusted, decentralized operating system ecosystem** that serves billions of users across all computing contexts.

*Sovereign Council Ratification: Active | Next Review: Annual*
