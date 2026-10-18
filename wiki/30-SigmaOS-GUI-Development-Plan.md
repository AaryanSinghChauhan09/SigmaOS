# SigmaOS GUI Development Plan & Architecture

## Overview

This document specifies the master architecture, design and acceptance rules, 19-PR implementation sequence, periodic maintenance cadences, component rules, PR review and merge gates, and rollout/exit criteria for the SigmaOS Graphical User Interface (GUI) and compositor stack.

The GUI subsystem transitions SigmaOS from model-tested display infrastructure and standalone prototypes into a fully verified, interactive graphical session. It uses Wayland's client/compositor protocol model, Linux DRM/KMS atomic modesetting, established freedesktop interfaces, and WCAG 2.2 accessibility guidance as design and engineering references.

---

## Design and Acceptance Rules

1. **Complete Vertical Slice First:** Build a complete vertical slice before broad feature work: boot to a session, draw a basic desktop, receive keyboard/pointer input, launch one application, and shut down cleanly. The first slice is software-rendered QEMU desktop + one input device + one app launch.
2. **Evidence-Based Status:** Keep implementation status evidence-based: distinguish model tests, hosted UI tests, QEMU graphical-session tests, and physical GPU tests. Promote the GUI to `Integrated` only after a real session works end-to-end.
3. **Separate Policy from Rendering:** The compositor owns focus, input routing, window placement, output policy, and access to display resources; clients render their own surfaces and cannot inspect other clients' buffers. This follows the core Wayland model.
4. **Atomic Display State Validation:** Validate every display change before commit: reject unsupported modes, formats, dimensions, buffer layouts, and resource combinations rather than partially programming hardware. Linux DRM/KMS requires atomic state validation before applying a modeset.
5. **Keyboard Operation First-Class:** Essential tasks—login, launching, window switching, settings, notifications, locking, and recovery—must work cleanly without a pointer.
6. **WCAG 2.2 Accessibility Reference:** Treat WCAG 2.2 as a design and test reference for user-facing surfaces: include keyboard access, visible focus, contrast, text scaling, reduced motion, understandable errors, and non-color status indicators.
7. **Open Interoperable Interfaces:** Adopt freedesktop/Wayland protocols selectively when practical; version SigmaOS-specific extensions and document deviations. Notifications, application desktop entries, icons, and sandbox portals follow established specifications.
8. **No Unauthenticated UI Action:** Settings, file access, screen capture, power actions, and app launch must call typed services with permission checks—never shell strings or raw privileged commands.

---

## Proposed PR Sequence (19-PR Gateway)

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(desktop): define GUI scope, session architecture, and support status` | Audit `src/desktop/`, `src/compositor/`, `src/drivers/drm_kms.rs`, `src/input/`, `zenith_desktop/`, `web_ui/`, `docs`, and feature inventory. Define session lifecycle, display/input boundaries, supported target, protocol profile, and limitations. | Each component has a status and evidence level; no “desktop environment” or hardware-acceleration claim without an end-to-end test; one primary QEMU target is selected. |
| 2 | `test(desktop): add deterministic compositor and rendering fixtures` | Add reproducible geometry, damage, z-order, focus, scale, color, and event fixtures; define screenshot/pixel-test format and golden-update review rules. | Tests run without a physical display; snapshots are deterministic; fixtures cover empty, overlapping, resized, and removed surfaces. |
| 3 | `feat(display): establish framebuffer and output discovery contract` | Connect the existing DRM/KMS model to one display output and a software-rendered framebuffer path before GPU acceleration. Enumerate modes and validate dimensions/format/stride. | QEMU displays a known test pattern; invalid modes/buffers are rejected; output dimensions and mode can be queried by the session. |
| 4 | `feat(compositor): implement buffer import, surface commit, and release lifecycle` | Connect Wayland-like surface state to actual buffers; define commit, damage, release, and client-disconnect behavior. | A test client draws a surface; compositor presents it and releases the buffer only when safe; malformed or oversized requests cannot panic or access another client’s buffer. |
| 5 | `feat(compositor): implement frame scheduling and software composition` | Add frame callbacks/timing, damage tracking, basic alpha composition, and a software fallback. | Multiple surfaces compose deterministically; no busy loop when idle; frame timing and dropped frames are observable; software path works without a GPU driver. |
| 6 | `feat(input): connect keyboard and pointer devices to session events` | Integrate the prototype input layer with device discovery, keymap translation, pointer coordinates, focus, and compositor event routing. | QEMU keyboard and pointer events reach the focused surface; unplug/reset and invalid events are handled; raw input is not broadcast to unrelated clients. Input event codes are hardware-independent. |
| 7 | `feat(compositor): add core window management and focus policy` | Implement window map/unmap, stacking, focus transfer, move/resize, close requests, workspace state, and multi-output basics. | Focus and stacking rules are deterministic; keyboard-only operation works; clients cannot force global focus or escape their surface bounds. |
| 8 | `feat(desktop): add session manager and graphical login handoff` | Define session startup/shutdown, environment, user session, compositor supervision, and recovery to a text console if the GUI fails. | A QEMU boot reaches a graphical session and returns to a recoverable console on compositor failure; session services start in dependency order and stop cleanly. |
| 9 | `feat(desktop): add installed-app discovery and typed launcher actions` | Replace the launcher model with installed-app discovery, metadata parsing, search, favorites, and typed launch service. Use desktop-entry concepts where appropriate. | Search is responsive and deterministic; malformed app metadata is ignored safely; app launch does not execute a shell command string; unavailable actions are visibly disabled. |
| 10 | `feat(desktop): add keyboard shortcuts and command palette actions` | Connect translated key events to documented actions; integrate the command palette with installed apps and typed system services. | Shortcuts are configurable and discoverable; no character-only shortcut is the sole path to an action; conflicts are surfaced; actions require appropriate authorization. |
| 11 | `feat(desktop): implement session notification service and surface` | Connect the notification model to a session-scoped service and GUI surface; support actions, dismissal, priority, expiry, and per-app controls. | Notifications render in the session; dismissal and action behavior are tested; no secret content is exposed on the lock screen by default. Implements a compatible subset of freedesktop notifications. |
| 12 | `feat(desktop): add settings service for display input and appearance` | Provide user-facing display, scale, keyboard, pointer, theme, and accessibility settings backed by validated typed APIs. | Changes validate before application and can be reverted; settings persist per user; invalid display modes and keymaps do not strand the session. |
| 13 | `feat(desktop): add file chooser and privileged-action confirmation portals` | Define minimal portal APIs for sandboxed app access to files, screenshots, screen capture, and selected privileged actions. Start with file chooser and screenshot if the app sandbox exists. | Access is explicit and per-request; file access uses authorized handles rather than granting broad path access; denial/cancellation is handled; no portal grants permissions silently. |
| 14 | `feat(accessibility): add accessibility tree and assistive technology bridge` | Define semantic roles, names, values, focus, state-change events, and an IPC interface for assistive technologies; add screen-reader-friendly labels to core UI. | Launcher, settings, notifications, and login expose semantic information; focus changes are announced; protocol is versioned and permission-scoped. Studied against AT-SPI standards. |
| 15 | `feat(desktop): add text scaling high contrast reduced motion and themes` | Add scalable UI layout, high-contrast mode, reduced-motion setting, theme tokens, and user-selectable cursor/text sizes. | No content is clipped at supported scales; focus is visible; color is not the sole status cue; contrast is measured for supported themes. |
| 16 | `security(compositor): isolate clients and validate protocol resources` | Add per-client resource ownership, quotas, request validation, surface/buffer bounds, clipboard policy, and teardown. | A client cannot access another client’s buffer or input; malformed messages and resource exhaustion disconnect or reject safely; clipboard and capture access are policy-controlled. |
| 17 | `feat(display): add validated atomic modeset and page-flip path` | Connect display state changes to a hardware page-flip/atomic commit path once the driver supports it; retain software fallback. | Unsupported combinations fail before hardware state changes; successful flips produce completion events; display remains usable after failed commits. |
| 18 | `test(desktop): add QEMU graphical-session and accessibility workflow` | Automate boot-to-session, input, launch, window focus, notification, settings, and shutdown; add accessibility and screenshot regression checks. | CI or documented local automation verifies the path on a named QEMU configuration; failures include logs/screenshots; no external network access is required. |
| 19 | `docs(desktop): publish support matrix and recovery guide` | Document supported QEMU/GPU/output/input configurations, session controls, accessibility features, known gaps, and recovery steps. | Each “Supported” claim links to repeatable tests and exact configuration; hardware acceleration, multi-monitor, suspend/resume, and accessibility claims are separately scoped. |

---

## Periodic Maintenance Cadence

| Cadence | Proposed Recurring PR/Task | Required Evidence |
|---|---|---|
| Every PR | `ci(desktop): run UI and compositor checks for changed components` | Format/build, unit/model tests, protocol validation, and affected QEMU/screenshot tests; skipped checks explained. |
| Weekly | `test(desktop): triage graphical-session failures and flaky tests` | Reproducer, logs/screenshot, affected configuration, owner, and fix/tracking decision. |
| Weekly | `docs(desktop): reconcile GUI claims with test evidence` | Update desktop status and capability matrix only when evidence changes; remove stale hardware/accessibility claims. |
| Monthly | `accessibility(desktop): audit keyboard path and focus behavior` | Test login, launcher, settings, dialogs, notifications, and recovery without a pointer; verify visible and unobscured focus. |
| Monthly | `security(desktop): review client isolation and portal permissions` | Review compositor protocol parsing, buffer lifetime, clipboard, screen capture, file access, and privileged actions. |
| Monthly | `test(desktop): run display/input/session compatibility matrix` | QEMU modes, scaling, keyboard layouts, pointer input, session startup/exit, and graphics fallback results. |
| Quarterly | `docs(desktop): review protocols and Linux/BSD interoperability` | Reassess Wayland/freedesktop subset, deliberate deviations, and application compatibility boundaries. |
| Quarterly | `accessibility(desktop): review assistive technology and localization gaps` | Screen-reader bridge, language direction, text expansion, shortcut discoverability, and user testing gaps. |
| Each Release Candidate | `release(desktop): certify supported GUI configuration` | Boot-to-session results, device/mode list, input and app checks, recovery, known limitations, and artifacts. |
| After Each Incident | `fix(desktop): prevent recurrence of <session/display/input issue>` | Root cause, minimal reproducer, regression test, user impact, and release/backport decision. |

---

## Component Rules

- **Compositor and protocol:** Version interfaces; validate message sizes, object IDs, file descriptors, and ownership; bound queues and per-client resources; never trust client-supplied dimensions or formats.
- **DRM/KMS and framebuffer:** Validate mode, plane, stride, pixel format, and buffer dimensions before commit; keep an accessible software renderer for unsupported hardware; handle hotplug and display loss without terminating the session.
- **Window manager:** Define focus, stacking, tiling/floating behavior, workspaces, dialogs, and shortcuts in a stable policy document; provide keyboard equivalents and settings.
- **Input:** Separate hardware event decoding from keymap translation and compositor actions; do not leak input to unfocused clients; document keyboard-layout support.
- **Launcher and application metadata:** Parse metadata as untrusted input; never launch through concatenated shell strings; show provenance and unavailable dependencies; support keyboard navigation and clear errors.
- **Notifications:** Use a session-scoped service, documented priorities/expiry/actions, and privacy-aware lock-screen policy.
- **Accessibility:** Preserve semantic roles and names independently of visual styling; support keyboard access, visible focus, text scaling, high contrast, reduced motion, and screen-reader announcements.
- **Portals and sandboxing:** Mediate file, screenshot, screencast, and remote-input access; ask consent at the point of use; pass authorized file descriptors or scoped references, not broad filesystem access.
- **Settings and session services:** Use typed IPC, validate before applying changes, persist per-user preferences safely, and provide rollback for changes that can make the session unusable.
- **Crash and recovery:** A compositor crash must not make the machine unrecoverable; collect minimal diagnostics, restart or return to a console, and avoid recording sensitive window contents by default.

---

## PR Review Template and Merge Gates

```text
## Summary
What user-visible GUI behavior or integration gap does this PR address?

## Scope
Compositor | display | input | launcher | window manager | accessibility | service
State what is not implemented.

## Runtime evidence
Model/unit | hosted UI | QEMU graphical session | physical hardware
List the tested configuration and exact user flow.

## Design and compatibility
Protocol/API changes, session lifecycle, app compatibility, settings migration,
and intentional Linux/BSD/freedesktop deviations.

## Accessibility
Keyboard path, focus behavior, labels/roles, contrast, scaling, reduced motion,
and screen-reader impact.

## Security and failure behavior
Client isolation, permissions, resource limits, malformed inputs, fallback,
rollback, and recovery path.

## Tests and artifacts
Commands, screenshots/logs, automated checks, manual testing, and limitations.

## Status and documentation
Proposed | Prototype | Integrated | Supported
Link updated capability matrix, desktop docs, and support matrix.

## Checklist
- [ ] One focused change with a clear acceptance criterion
- [ ] Keyboard-only path considered
- [ ] Invalid client/UI input handled safely
- [ ] No shell-string execution or implicit privilege bypass
- [ ] QEMU/session evidence included where applicable
- [ ] Required CI passes; review conversations resolved
```

Require at least one maintainer review for ordinary UI changes and two relevant reviewers for compositor protocol, display modesetting, client isolation, portals, or authentication/lock-screen behavior.

---

## Rollout and Exit Criteria

1. **Audit and test baseline:** PRs 1–2.
2. **Bootable desktop slice:** PRs 3–8, using software composition and QEMU.
3. **Usable core UX:** PRs 9–12 for app launch, shortcuts, notifications, and settings.
4. **Security and accessibility:** PRs 13–16.
5. **Hardware and release readiness:** PRs 17–19.

Mark the GUI **Integrated** only when a fresh boot reaches a session, renders a desktop, accepts keyboard/pointer input, launches an app, and recovers from session failure on the documented QEMU target. Mark it **Supported** only after repeated session tests, supported device/mode documentation, input and accessibility checks, client-isolation tests, and a tested recovery path.
