# 32. System Applications Plan & Architecture

This document serves as the GitHub Wiki specification for Idea #32 / System Applications Plan in SigmaOS.

For full details, see the canonical documentation at [`docs/SYSTEM_APPS_DEVELOPMENT_PLAN.md`](../docs/SYSTEM_APPS_DEVELOPMENT_PLAN.md).

## Summary
- Defines user-facing and administrative system applications scope in SigmaOS.
- Enforces strict runtime evidence gates (component presence/unit tests alone do not constitute OS readiness).
- Establishes typed service calls over direct kernel internal manipulation.
- Implements freedesktop Desktop Entry subset for application metadata and argument vectors.
- Scopes sandboxed application permissions via XDG Desktop Portal-like interfaces.
- Details the 18-PR implementation roadmap, recurring maintenance cadences, application family rules, PR template and merge gates, and rollout/exit criteria.
