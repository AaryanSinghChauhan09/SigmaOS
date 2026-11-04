# SigmaOS GUI Development Plan

## Overview
SigmaOS GUI Development Plan specifies the complete architecture and phased milestones for building a robust, accessible, and secure desktop GUI environment.

## Design and Acceptance Rules
- **Rule 1: Separation of Concerns**: Keep compositor, session, input, and widget toolkits decoupled.
- **Rule 2: Predictable Focus**: Keyboard navigation and accessibility focus must be deterministic.
- **Rule 3: Deterministic Damage**: Buffer damage and page flipping must avoid redundant compositing.
- **Rule 4: Client Isolation**: Unprivileged applications communicate only via validated typed portals.

## Proposed 19-PR Sequence Roadmap
1. `docs(desktop): define GUI scope, session architecture, and support status`
2. `test(desktop): add deterministic compositor and rendering fixtures`
3. `feat(display): establish framebuffer and output discovery contract`
4. `feat(compositor): implement buffer import, surface commit, and release lifecycle`
5. `feat(compositor): implement frame scheduling and software composition`
6. `feat(input): connect keyboard and pointer devices to session events`
7. `feat(compositor): add core window management and focus policy`
8. `feat(desktop): add session manager and graphical login handoff`
9. `feat(desktop): add installed-app discovery and typed launcher actions`
10. `feat(desktop): add keyboard shortcuts and command palette actions`
11. `feat(desktop): implement session notification service and surface`
12. `feat(desktop): add settings service for display input and appearance`
13. `feat(desktop): add file chooser and privileged-action confirmation portals`
14. `feat(accessibility): add accessibility tree and assistive technology bridge`
15. `feat(desktop): add text scaling high contrast reduced motion and themes`
16. `security(compositor): isolate clients and validate protocol resources`
17. `feat(display): add validated atomic modeset and page-flip path`
18. `test(desktop): add QEMU graphical-session and accessibility workflow`
19. `docs(desktop): publish support matrix and recovery guide`

## Periodic Maintenance PR Cadences
- Every PR: Automated compositor tests and headless smoke tests.
- Weekly: Session crash review and accessibility tree audit.
- Monthly: DRM/KMS hardware display matrix verification.

## Component Rules
- Compositor: Handle wayland-like buffer lifecycles without panics or memory leaks.
- Session: Provide clean login handoff with fail-safe console fallback.
- Portals: Enforce user consent for privileged hardware/file access.

## PR Review and Merge Gates
- Must pass deterministic test suite without graphical display dependencies.
- Must document backward compatibility and fallback behavior.

## Rollout and Exit Criteria
- Prototype: Software framebuffer and core input events functioning in tests.
- Integrated: Bootable desktop session in QEMU with basic application launcher.
- Supported: Atomic DRM/KMS hardware acceleration and accessibility conformance.
