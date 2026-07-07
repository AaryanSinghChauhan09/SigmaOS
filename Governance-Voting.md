# Governance & Voting

> SigmaOS v15.0 "Zenith" — Community Governance Model

## Overview

SigmaOS is a community-governed open-source project. Major technical and strategic decisions are made transparently through a structured voting system.

---

## Governance Structure

```
SigmaOS Foundation
├── Core Team       (5 elected members, 2-year terms)
├── Security Team   (3 members, security-clearance review)
├── Driver Council  (1 per hardware category)
└── Community       (all registered contributors)
```

### Core Team Responsibilities

- Merge freeze and release decisions
- Security policy and CVE response coordination
- Road-map approval
- Infrastructure budget

---

## Decision Categories

| Category | Required Quorum | Voting Period |
|---|---|---|
| RFC (major feature) | 60% Core + 30% Community | 14 days |
| Patch approval | 2 Core team members | 7 days |
| Security patch | 1 Security team member | 48 hours |
| Release approval | 4/5 Core team members | 3 days |
| Constitutional change | 80% Core + 50% Community | 30 days |

---

## RFC Process

1. **Draft**: Author opens an RFC issue on GitHub with template
2. **Discussion**: 7-day open comment period
3. **Revision**: Author incorporates feedback
4. **Vote**: 14-day voting window opens
5. **Decision**: Quorum reached → Accepted/Rejected
6. **Implementation**: Tracked in `task.md` and assigned milestone

### RFC Template

```markdown
# RFC-XXXX: Title

## Summary
One-paragraph description of the change.

## Motivation
Why is this needed? What problem does it solve?

## Proposal
Detailed technical design.

## Drawbacks
Known limitations or risks.

## Alternatives
What other approaches were considered?

## Unresolved Questions
Open questions for community input.
```

---

## Voting Mechanics

- Votes are cast via GitHub Reactions on the RFC issue
  - 👍 = Yes
  - 👎 = No
  - 🤷 = Abstain
- Each contributor has one vote
- Core team votes are weighted 3x
- Results are announced in the SigmaOS forum and mailing list

---

## Code of Conduct

All participants must adhere to the [SigmaOS Code of Conduct](CODE_OF_CONDUCT.md):

- Be respectful and inclusive
- Constructive criticism only — critique code, not people
- Zero tolerance for harassment, discrimination, or bad-faith participation
- Violations reported to `conduct@sigmaos.dev`

---

## Roadmap Submission

Community members can submit roadmap items:

```bash
# Open a roadmap proposal
sigma-contribute roadmap new --title "Add RISC-V support" --priority medium
```

Proposals appear in the `FUTURE_ROADMAP.md` after Core team triage.
