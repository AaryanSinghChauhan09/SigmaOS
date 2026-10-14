# SigmaOS System Applications Plan & Architecture

## Overview

This document specifies the master architecture, rules, component boundaries, 18-PR implementation roadmap, periodic maintenance cadences, application family guidelines, PR review and merge gates, and rollout/exit criteria for SigmaOS system applications.

Here, “system applications” means the user-facing and administrative programs shipped with the OS, not every userspace binary. The sequence starts with core utilities and read-only management apps, then adds package, settings, update, file, and recovery workflows as their system backends become real.

Component presence and unit tests alone do not establish a usable OS. Application status is strictly gated on real runtime evidence on boot targets.

---

## Rules and Component Boundaries

- **Applications call typed services, not kernel internals.** Keep UI/CLI separate from privileged system services; enforce permissions in the service or kernel boundary, not just in a button’s enabled state.
- **Define a shared application lifecycle.** Specify launch, arguments, environment, working directory, service dependencies, exit status, logging, crash reporting, and shutdown behavior.
- **Use documented app metadata.** Adopt a supported subset of the freedesktop Desktop Entry specification for launch name, icon, categories, and execution metadata. Treat metadata as untrusted input, and launch with structured argument vectors—not by concatenating shell strings.
- **Keep app access scoped.** For sandboxed apps, use portal-like interfaces for file selection, notifications, screenshots, printing, and other mediated actions rather than broad ambient privileges.
- **Ship accurate, localized, accessible interfaces.** Core tasks must work from CLI where practical, provide keyboard navigation in GUI, expose semantic labels, and avoid relying on color alone. Keep human output and machine-readable output distinct.
- **Make system changes previewable and recoverable.** Package changes, updates, storage changes, and network changes should show the intended operation, validate before applying, and have rollback/recovery behavior where feasible.
- **Treat security as part of the lifecycle.** Use NIST SSDF practices for secure development, build protection, release integrity, vulnerability response, and prevention of recurring defects.
- **Prefer focused, reviewed PRs.** GitHub PR templates can collect the rationale, scope, tests, and reviewer routing for each change; branch protection can require reviews and passing checks.
- **Do not claim application availability before integration.** A UI model, CLI prototype, or hosted tool is not equivalent to a packaged, launchable SigmaOS application.

---

## Proposed PR Sequence (18-PR Gateway)

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(system-apps): define app catalog, lifecycle, and support status` | Audit init, shell/core utilities, settings, system information, package manager/store, updater, installer/recovery, file manager, and desktop launcher. Define app metadata, launch contract, service API boundaries, and status evidence. | Every app has an owner, status, dependencies, launch path, permissions, tests, and known gaps; hosted-only tools are explicitly labeled. |
| 2 | `feat(userspace): define system app manifest and typed service client` | Add versioned manifests for app ID, name, entry point, icon, permissions, required services, and update channel; add a safe client library for service calls. | Manifest parser rejects invalid/unknown privileged fields; apps cannot request unrecognized permissions; API errors are typed and documented. |
| 3 | `feat(init): connect service manager to real process lifecycle` | Complete process spawning, dependencies, restart policy, environment, logs, stop/timeout behavior, and service readiness; begin with a minimal boot service set. | QEMU starts and supervises real services; dependency failure is visible; restart loops are bounded; shutdown reaps children. |
| 4 | `feat(userspace): add foundational core utilities` | Implement or integrate a minimal set of essential utilities: `cat`, `mkdir`, `rm` with safeguards, `cp`, `mv`, `touch`, `stat`, `env`, `date`, `uname`, `ps`, and `kill` as supported by actual syscalls/VFS. | Each utility has documented options/status, unit and runtime tests, and does not claim unsupported POSIX behavior; destructive utilities require correct path validation. |
| 5 | `feat(system-apps): add app catalog and launch integration` | Connect installed-app metadata to the shell, launcher, and desktop session; support search, categories, and unavailable-dependency reporting. | Apps install/list/launch consistently; invalid metadata cannot execute arbitrary shell text; launch failures provide actionable errors. |
| 6 | `feat(system-info): add SigmaOS runtime-backed diagnostics` | Extend the hosted report module with providers for actual SigmaOS kernel, hardware, filesystem, network, services, and package state as those APIs become available. | Every value labels its source; host data is never misreported as SigmaOS data; missing providers report unavailable rather than fabricated success. |
| 7 | `feat(settings): add typed settings service and initial settings app` | Create schema-backed settings for display, input, locale/time, users, privacy, and accessibility; start with read-only status and low-risk settings. | Schema versions migrate safely; invalid settings leave the previous state; each write is permission-checked and reversible. |
| 8 | `feat(package): add package query UI and CLI workflows` | Add search/list/details/installed-state operations using package metadata, signatures, licenses, dependencies, and update channel information. | Query functions work against the actual repository metadata; search results are stable; signature state and package provenance are visible. |
| 9 | `feat(package): add transactional install/remove and update preview` | Integrate package manager with the kernel package subsystem, filesystem, signing, dependency resolver, and recovery mechanism. | Dry-run shows all changes; verification occurs before install; failed operations roll back or clearly enter recovery; removal protects essential packages. |
| 10 | `feat(updater): add signed update check and staged deployment UI` | Add update discovery, changelog, compatibility check, download verification, staged apply, reboot coordination, and rollback status. | Tampered/stale updates are rejected; users can inspect changes; interrupted updates recover; current and target versions are shown accurately. |
| 11 | `feat(security-center): add permissions and security status application` | Show users sandbox permissions, update/security status, firewall/services status, disk-encryption state, and audit findings only where those controls are actually enforced. | Read-only reports reflect live enforcement; unavailable controls are labeled; permission changes call authoritative APIs and emit audit events. |
| 12 | `feat(file-manager): add safe file browsing and scoped app access` | Build basic file browsing, file operations, mounts/devices view, and document opening using VFS APIs and per-app file access grants. | Path traversal and symlink attacks are tested; destructive operations confirm targets; sandboxed apps receive only selected file handles through a portal-like API. |
| 13 | `feat(notifications): add session notification center and preferences` | Connect the notification data model to a service and GUI; add per-app preferences, quiet hours, actions, and lock-screen privacy. | Service and UI work in a real session; dismissal and action handling are tested; private notification content is hidden on lock screen by default. |
| 14 | `feat(recovery): add system recovery and troubleshooting application` | Surface installer recovery, update rollback, boot failure diagnostics, safe mode, and recovery media guidance through CLI and GUI. | Recovery instructions are tested on QEMU; destructive recovery requires confirmation; a failed desktop/service can still reach recovery via console. |
| 15 | `security(system-apps): add app permission and sandbox enforcement` | Enforce per-app permissions at service/syscall boundaries; add resource limits, safe IPC, and auditing. | Unprivileged apps cannot bypass UI permissions; denied operations are tested; sandbox status reflects actual enforcement. |
| 16 | `docs(system-apps): generate app manuals and user workflows` | Add command references, app guides, task-based onboarding, troubleshooting, and localization structure. | Every shipped app has launch/use/permission/troubleshooting docs; docs match tested behavior and list limitations. |
| 17 | `test(system-apps): add fresh-install and upgrade integration matrix` | Automate boot, login/session, core utilities, app launch, package query/update, settings, file access, recovery, and shutdown. | Tests run on named QEMU configuration and clean state; upgrade/recovery paths have artifacts; hosted-only tests are reported separately. |
| 18 | `release(system-apps): publish supported application matrix` | Publish app versions, runtime prerequisites, permissions, backends, architectures, known issues, and release evidence. | “Supported” apps have repeatable install/launch/use/upgrade tests; no prototype or model is marketed as shipped functionality. |

---

## Periodic Maintenance PR Cadence

| Cadence | Proposed Recurring PR/Task | Required Output |
|---|---|---|
| Every PR | `ci(system-apps): run application contract checks` | Formatting/build/tests, manifest validation, permission checks, docs checks, and runtime tests relevant to the app. |
| Weekly | `maint(system-apps): triage launch and service failures` | Reproducers, affected app/service/version, owner, priority, and issue/PR; no silent removal of failing checks. |
| Weekly | `docs(system-apps): reconcile app catalog and component status` | Catalog, feature matrix, launcher metadata, and documentation reflect actual installable/runnable state. |
| Monthly | `security(system-apps): review app permissions and privileged actions` | Permission matrix, service API access, secrets in logs/config, path handling, and sandbox enforcement reviewed. |
| Monthly | `test(system-apps): run clean-boot application smoke suite` | Boot, service start, CLI diagnostics, app launch, package query, settings, graceful shutdown, and failure artifacts. |
| Monthly | `maint(package): review package metadata and signature behavior` | Dependency/license/provenance changes, signature verification tests, and install/update rollback evidence. |
| Quarterly | `accessibility(system-apps): audit keyboard, screen-reader, and scaling paths` | Core workflows audited for keyboard access, focus, semantics, text scaling, contrast, and localization gaps. |
| Quarterly | `docs(system-apps): review Linux/BSD interoperability claims` | App metadata, command behavior, service assumptions, package compatibility, and intentional deviations documented. |
| Each Release Candidate | `release(system-apps): certify shipped app set` | Exact build/target, clean installation tests, app versions, permissions, update/recovery results, and known limitations. |
| After Each Incident | `fix(system-apps): prevent recurrence of <failure>` | Root cause, impacted workflows, regression test, security/data-loss impact, and release/backport decision. |

---

## Rules by Application Family

- **Core utilities:** Follow POSIX utility syntax where compatibility is intended: clear synopsis, options/operands, `--` handling, stdin/stdout/stderr, and documented exit status.
- **Package store and updater:** Show package origin, version, signature status, dependencies, and transaction preview; do not imply package signing is production-ready when cryptographic integration is still pending.
- **Settings:** Use typed schemas, per-user storage, validation, migration, and rollback. High-impact settings need a recovery path and should not depend on a graphical confirmation alone.
- **System information and diagnostics:** Mark data as live, cached, host-derived, or unavailable; redact tokens and personal data; export stable machine-readable reports only under a versioned schema.
- **Security center:** Display only controls the OS actually enforces; a modeled policy, prototype, or unit test must not appear as active protection.
- **File manager and portals:** Scope file access to selected objects and mediate requests; do not grant the entire home directory because one app needs one file.
- **Notifications and app launch:** Use session-scoped services, privacy-aware lock-screen behavior, and structured app metadata; never run an untrusted metadata `Exec` line through a shell.
- **Recovery/installer:** Separate inspection from destructive actions; identify exact target device/partition; provide dry-run and restore guidance; test under interruption and failure.
- **Logging and crash reporting:** Bound storage, redact secrets, obtain consent before exporting user data, and offer local-only diagnostics when possible.
- **Localization and accessibility:** Keep strings translatable, avoid hard-coded text geometry, support keyboard operation and assistive technologies in core flows, and provide non-color status indicators.

---

## PR Template and Merge Gates

```text
## Summary
What user task or system-app gap does this PR address?

## App/component
App name, CLI command or GUI surface, service/API dependencies, and owner.

## Runtime status
Hosted tool | model/unit | QEMU SigmaOS | physical target
State whether this is a prototype, integrated app, or supported feature.

## User-visible behavior
Launch path, workflow, permissions, error states, and supported configurations.

## Security and data handling
Privilege needed, data read/written, secrets/logging, sandbox/portal use,
confirmation, dry-run, and rollback/recovery behavior.

## Compatibility
POSIX/freedesktop/Linux/BSD conventions followed; deviations and migration.

## Tests and documentation
Commands run, fresh-boot/app tests, permission tests, docs/man pages,
and anything not tested.

## Checklist
- [ ] Focused PR with explicit acceptance criteria
- [ ] Uses typed service APIs, not shell-string execution
- [ ] Unavailable backends do not report false success
- [ ] Permissions and destructive actions are tested
- [ ] App catalog/status/docs updated
- [ ] Required CI passes; review discussions resolved
```

Require one maintainer approval for ordinary app/docs changes and two reviewers for privileged services, package/update operations, recovery, security settings, or access to user data.

---

## Rollout and Exit Criteria

Mark an application **Integrated** only when it is packaged or otherwise installed through the documented path, launches on a real SigmaOS boot target, uses its intended runtime service, and handles errors without simulated success. Mark it **Supported** only when its install/launch/use/update/recovery paths are repeatable and documented for a named configuration.
