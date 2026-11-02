# Update Flow Contract

> **Shared SigmaOS platform contract.** Define the update lifecycle — schedule, check, preview, stage, apply, rollback — shared across Linux and BSD.

---

## Invariants

- Updates are transactional: a snapshot is taken before apply; on failure, rolls back automatically.
- Update preview shows what will change (packages added/removed/upgraded) before applying.
- Post-update smoke test validates boot, login, desktop, network; failure triggers rollback.
- Update logs record every step for audit and debugging.
- Rollback restores the pre-update package set + config in one operation.

## Lifecycle

Schedule -> Check -> Preview (diff) -> Stage (download) -> Apply (transaction) -> Verify (smoke) -> Commit | Rollback

## Linux Adapter

systemd timers for schedule; pacman/apt/dnf for transactions; dracut for initramfs rebuild.

## BSD Adapter

cron for schedule; pkg for transactions; /usr/libexec for initramfs-equivalent.

## Failure Modes and Recovery

Download interrupted (resume), disk full (abort + notify), smoke test fails (auto-rollback), boot failure (boot from snapshot via boot-manager fallback).

## Open Questions

Atomic updates (A/B partition scheme like OSTree)? Current: transactional snapshot; A/B planned post-stable.
