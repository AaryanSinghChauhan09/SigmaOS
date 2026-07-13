# SigmaOS Foundation — Establishment Charter

> **Version**: 1.0-draft | **Status**: Ratified by founding contributors | **Date**: 2026-07-13

---

## Mission Statement

The **SigmaOS Foundation** is a non-profit open governance body dedicated to the development, stewardship, and long-term sovereignty of the SigmaOS operating system. Its mission is to ensure SigmaOS remains:

- **Technically excellent** — shipping a high-quality, secure, and performant sovereign OS
- **Independently governed** — free from undue commercial influence
- **Community-driven** — with transparent decision-making at every level
- **Globally accessible** — supporting users, developers, and organizations worldwide

---

## Organizational Structure

```
┌─────────────────────────────────────────────────────────┐
│                  SigmaOS Foundation                     │
│                   (Legal Entity)                        │
├────────────────┬────────────────────────────────────────┤
│  Board of      │  Technical Steering                    │
│  Directors     │  Committee (TSC)                       │
│  (Governance)  │  (Technical Direction)                 │
├────────────────┴────────────────────────────────────────┤
│              Working Groups                             │
│  Kernel │ Security │ Desktop │ Cloud │ Docs │ Community │
└─────────────────────────────────────────────────────────┘
```

### Board of Directors

- **5 elected seats** — voted by active contributors (≥5 merged PRs in last 12 months)
- **2 founding seats** — held by founding contributors for first 3 years
- **1 community seat** — elected by forum/community vote
- Term: 2 years, staggered elections
- Responsibilities: Legal, financial, trademark, high-level strategy

### Technical Steering Committee (TSC)

- **7 members** — subsystem maintainers elected by contributors
- Seats: Kernel, Security, Networking, Desktop, Package Management, Cloud, Toolchain
- Responsibilities: Technical roadmap, RFC approval, release management, merge criteria
- Meeting cadence: Bi-weekly, public agenda + minutes

### Working Groups

| Working Group | Scope | Chair Rotation |
|---------------|-------|----------------|
| Kernel WG | Core kernel, drivers, HAL | 6 months |
| Security WG | PQC, MAC, audit, CVEs | 6 months |
| Desktop WG | Zenith, UI/UX, accessibility | 6 months |
| Cloud WG | Orchestration, containers | 6 months |
| Docs WG | Wiki, guides, tutorials | 6 months |
| Community WG | Events, hackathons, mentoring | 6 months |

---

## Membership & Contributions

### Contributor Tiers

| Tier | Criteria | Privileges |
|------|----------|------------|
| **Community Member** | Any participant | Forum access, issue tracking |
| **Contributor** | 1+ merged PR | Voting in community polls |
| **Committer** | 10+ merged PRs + review quality | Repository write access to their area |
| **Maintainer** | Sustained contribution + TSC nomination | Subsystem ownership, release authority |
| **TSC Member** | Elected by contributors | TSC voting, RFC veto power |

### Founding Contributors

Founding contributors who established the initial codebase receive:
- 3-year protected seat on Board of Directors
- `@founder` badge on all Foundation platforms
- Recognition in every SigmaOS release announcement

---

## Governance Processes

### RFC (Request for Comments) Process

```
1. DRAFT      → Author proposes RFC in GitHub Discussions
2. REVIEW     → 14-day public comment period
3. TSC VOTE   → Simple majority of TSC (5/7 to approve)
4. ACCEPTED   → RFC merged into /docs/rfcs/
5. IMPLEMENT  → Feature branch created, tracked in roadmap
```

### Release Decision Process

1. Release Manager (TSC-appointed) tags release candidate
2. 7-day community testing window
3. Security WG sign-off required
4. TSC majority vote (4/7) to promote RC → stable
5. Reproducible build verification (SLSA Level 3)

### Conflict Resolution

1. **Technical disagreements** → TSC arbiter decision (final)
2. **Conduct issues** → Community WG review + Code of Conduct enforcement
3. **Trademark disputes** → Board of Directors + legal counsel

---

## Financial Model

### Revenue Sources (Planned)

| Source | % Target | Purpose |
|--------|----------|---------|
| Corporate sponsorship | 50% | Infrastructure, developer grants |
| Individual donations | 20% | Community programs |
| Foundation membership | 20% | Organizational support |
| Merchandise/events | 10% | Community engagement |

### Expenditure Budget (Year 1 Target)

| Category | % Budget |
|----------|----------|
| Infrastructure (CI/CD, mirrors, hosting) | 35% |
| Developer grants & bounties | 30% |
| Documentation & tutorials | 15% |
| Community events & hackathons | 10% |
| Legal & administration | 10% |

### Transparency

All financial records published quarterly at `foundation.sigmaos.org/financials`.

---

## Programs

### Developer Grant Program

- **Mini Grants** ($500–$2,000): For individual contributors implementing specific roadmap items
- **Project Grants** ($2,000–$15,000): For sustained 3–6 month development projects
- **Research Grants** ($15,000–$50,000): For academic/industrial research aligned with SigmaOS

Application process: Open quarterly, reviewed by TSC + Board.

### Mentorship Program

- 3-month structured mentorship for new contributors
- Paired with an experienced maintainer
- Commitment: ~10 hrs/week, culminating in a merged feature

### Hackathon Program

- **SigmaOS Hackathon**: Annual 48-hour event, prizes pool $25,000
- **Security Sprint**: Bi-annual security-focused audit + bounty event
- **Accessibility Jam**: Annual accessibility improvements sprint

### Bounty Program

See [Bug-Bounty.md](Bug-Bounty.md) for the full security bounty policy.

---

## Trademark & IP Policy

- The name **"SigmaOS"**, the SigmaOS logo, and related marks are trademarks of the SigmaOS Foundation
- All code contributions are licensed under the license specified in `LICENSE.md` (MIT/Apache 2.0)
- The Foundation holds the trademark but licenses usage freely to community forks using the word "SigmaOS" with appropriate attribution
- Commercial use of trademarks requires a Foundation trademark license agreement

---

## Founding Timeline

| Milestone | Target Date |
|-----------|------------|
| Foundation charter published | 2026 Q3 |
| Legal entity established | 2026 Q4 |
| First Board of Directors elected | 2027 Q1 |
| TSC formally constituted | 2027 Q1 |
| Grant program launched | 2027 Q2 |
| First Hackathon | 2027 Q3 |
| Foundation website live | 2027 Q1 |

---

## Contact

- **Foundation email**: foundation@sigmaos.org *(pending establishment)*
- **GitHub**: https://github.com/AaryanSinghChauhan09/SigmaOS
- **Community forum**: forum.sigmaos.org *(pending)*
- **Matrix/IRC**: #sigmaos on matrix.org *(pending)*

---

*This charter is a living document. Amendments require a 2/3 majority vote of the Board of Directors and ratification by the TSC.*
