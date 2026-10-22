# Pull Request Template

## Component
- Target component details.

## Upstream study
- Reference study: Study document of upstream mechanisms.
- What SigmaOS adopts: Details on upstream adaptations.

## Safety and rollback
- Risks and rollback plan defined.

## Kernel safety review
- Kernel safety review completed.

## Reviewer checklist
- [ ] Reviewer checklist completed.

## Summary
What risk or security requirement does this PR address?

## Scope
Affected components, files, release branches, and explicitly excluded work.

## Security classification
Public hardening | Security fix | Dependency update | Policy/docs

## Threat model
Attacker capability, affected assets, trust boundary, and assumptions.

## Impact and versions
Affected configurations/releases, exploitability rationale, and known mitigations.

## Implementation
Why this fix is appropriate; compatibility/API impact; backport considerations.

## Tests and evidence
Regression test, fuzz/property tests, static analysis, integration/runtime results, commands run, and anything not tested.

## Disclosure and release
Private report reference (if applicable), coordination status, advisory/release plan.

## Checklist
- [ ] Minimal, reviewable change
- [ ] Security tests or regression tests added
- [ ] No secrets or exploit details exposed publicly
- [ ] Docs and component status updated
- [ ] CI passes and review conversations resolved
- [ ] Backport/recovery plan recorded where relevant
