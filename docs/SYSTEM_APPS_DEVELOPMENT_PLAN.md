# SigmaOS System Applications Development Plan & App Catalog (PR 1)

This document defines the architecture, catalog, lifecycle, service boundaries, PR roadmap, periodic maintenance cadence, and rollout criteria for **SigmaOS System Applications**.

System applications are the user-facing and administrative programs shipped with the OS (e.g. init/services, core utilities, system diagnostics, settings, package management, updater, file manager, security center, desktop launcher, notifications, and recovery workflows).

---

## 1. System Applications Architecture & Lifecycle

### Architectural Principles & Boundaries
- **Typed Service Boundaries**: System applications communicate with system backends through versioned, typed RPC or IPC APIs—never by directly invoking internal kernel structures or concatenating unvalidated shell strings. Privilege checks are enforced at the service/kernel boundary rather than in UI controls.
- **Shared Application Lifecycle**: Every system application follows a defined lifecycle contract:
  - **Launch & Invocation**: Structured argument vectors (`argv`), sanitized environment variables (`envp`), explicit working directory.
  - **Service Dependencies**: Explicit declaration of required IPC/RPC system services. If a required service is offline, the app handles unavailable state gracefully without panic or fake success.
  - **Exit Status & Signal Handling**: Standardized POSIX exit codes, SIGTERM graceful shutdown handling, bounded timeout enforcement.
  - **Logging & Crash Reporting**: Redacted user secrets, bounded circular ring-buffer logs, structured error reporting.
- **App Metadata & Desktop Entry**: Applications supply metadata conforming to a supported subset of the freedesktop Desktop Entry Specification (`.desktop` format):
  - `Name`, `Exec` (parsed as structured argv), `Icon`, `Categories`, `Keywords`, `X-SigmaOS-Permissions`, `X-SigmaOS-RequiredServices`.
- **Scoped App Access & Portals**: Sandboxed applications interact with privileged desktop capabilities (file picker, notifications, screen capture, printing) through portal-like mediated IPC interfaces inspired by XDG Desktop Portals.

---

## 2. Comprehensive System Applications Audit Catalog

The table below provides a complete audit of shipped system applications and management programs in SigmaOS:

| App / Component ID | App Name & Surface | Owner | Support Status | Service Dependencies | Launch Path / Entry Point | Permissions / Scope | Test Evidence | Known Gaps / Next Milestones |
|---|---|---|---|---|---|---|---|---|
| `sys.init` | Systemd/FreeBSD Init Service Manager | Userspace Core | `working` | Kernel Syscall, VFS | PID 1 / `src/userspace/init.rs` | Unrestricted (PID 1) | Unit tests (`systemd_test`) | Process spawning via `execve` and real service dependency enforcement |
| `sys.shell_utils` | Sigma Core Utilities (`cat`, `mkdir`, `pwd`, `ls`, etc.) | Userspace Core | `prototype` | VFS, Libc Syscall Shim | CLI / `src/userland/system_utilities.rs` | User VFS Permissions | Unit & standalone tests | Add remaining POSIX utilities (`rm`, `cp`, `mv`, `touch`, `stat`, `uname`, `ps`, `kill`) |
| `sys.info` | SigmaOS System Information & Diagnostics | Diagnostics Team | `hosted-only` | Linux `/proc`, `/sys` (Host) | CLI / `src/tools/mint_system_report.rs` | Read-only System Access | Unit report formatting tests | Currently runs on host Linux; requires SigmaOS runtime providers |
| `app.launcher` | Desktop Command Palette & App Launcher | Desktop Team | `prototype` | Compositor, App Catalog | Wayland Surface / `src/desktop/launcher.rs` | Session Input / Render | Desktop model tests | Lacks installed-app discovery and running UI session integration |
| `app.package` | Universal Package Manager (`sigpkg`) | Package Subsystem | `working` | VFS, Signing Engine, SAT Resolver | CLI / `src/package/manager.rs` | Package Install Rights | Unit & V18–V34 tests | Needs integration with kernel package subsystem & physical installation |
| `app.updater` | Mint Update Manager & Policy Engine | Desktop Team | `working` | Package Manager, Keyring | GUI/CLI / `src/desktop/mint_update_manager.rs` | Update Admin Rights | Unit tests (`mint_update_manager_test`) | Staged deployment and signed update check verification |
| `sys.installer` | Safe Installer & Migration Wizard | Installer Team | `working` | GPT Partition, Storage Driver, LUKS | Boot ISO / `src/installer/safe_installer.rs` | Storage Raw Block Rights | Unit tests pass | QEMU end-to-end installation test execution |
| `sys.recovery` | System Recovery & Auto-Rollback | Recovery Team | `working` | Btrfs/ZFS Snapshot, Bootloader | Recovery Console / `src/installer/recovery.rs` | System Snapshot Rights | Unit tests pass | Real QEMU boot failure auto-rollback validation |
| `app.file_manager` | DiskTree & Batch Renamer | Desktop Team | `prototype` | VFS, Portal Service | Wayland Surface / `src/desktop/omarchy_disktree_inspector.rs` | Scoped VFS / Portals | Unit tests (`disktree_test`, `bulky_test`) | Mediated portal integration for sandboxed app file handles |
| `app.notification` | Session Notification Center | Desktop Team | `prototype` | Wayland Compositor | Session Service / `src/desktop/notification.rs` | Session Notification Ring | Unit model tests | Delivery, dismissal, and lock-screen privacy enforcement |
| `sys.security_center` | Landlock / Pledge Security Audit Application | Security Team | `working` | Landlock v4, Pledge, Audit Framework | GUI/CLI / `src/security/audit.rs` | Audit Read Access | Unit tests pass | Display only live-enforced kernel controls; report unavailable state |

---

## 3. Proposed 18-PR System Applications Sequence

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(system-apps): define app catalog, lifecycle, and support status` | Audit init, shell/core utilities, settings, system information, package manager/store, updater, installer/recovery, file manager, and desktop launcher. Define app metadata, launch contract, service API boundaries, and status evidence. | Every app has an owner, status, dependencies, launch path, permissions, tests, and known gaps; hosted-only tools are explicitly labeled. |
| 2 | `feat(userspace): define system app manifest and typed service client` | Add versioned manifests for app ID, name, entry point, icon, permissions, required services, and update channel; add a safe client library for service calls. | Manifest parser rejects invalid/unknown privileged fields; apps cannot request unrecognized permissions; API errors are typed and documented. |
| 3 | `feat(init): connect service manager to real process lifecycle` | Complete process spawning, dependencies, restart policy, environment, logs, stop/timeout behavior, and service readiness; begin with a minimal boot service set. | QEMU starts and supervises real services; dependency failure is visible; restart loops are bounded; shutdown reaps children. |
| 4 | `feat(userspace): add foundational core utilities` | Implement or integrate a minimal set of essential utilities: `cat`, `mkdir`, `rm` with safeguards, `cp`, `mv`, `touch`, `stat`, `env`, `date`, `uname`, `ps`, and `kill` as supported by actual syscalls/VFS. | Each utility has documented options/status, unit and runtime tests, and does not claim unsupported POSIX behavior; destructive utilities require path validation. |
| 5 | `feat(system-apps): add app catalog and launch integration` | Connect installed-app metadata to the shell, launcher, and desktop session; support search, categories, and unavailable-dependency reporting. | Apps install/list/launch consistently; invalid metadata cannot execute arbitrary shell text; launch failures provide actionable errors. |
| 6 | `feat(system-info): add SigmaOS runtime-backed diagnostics` | Extend the hosted report module with providers for actual SigmaOS kernel, hardware, filesystem, network, services, and package state as those APIs become available. | Every value labels its source; host data is never misreported as SigmaOS data; missing providers report unavailable rather than fabricated success. |
| 7 | `feat(settings): add typed settings service and initial settings app` | Create schema-backed settings for display, input, locale/time, users, privacy, and accessibility; start with read-only status and low-risk settings. | Schema versions migrate safely; invalid settings leave the previous state; each write is permission-checked and reversible. |
| 8 | `feat(package): add package query UI and CLI workflows` | Add search/list/details/installed-state operations using package metadata, signatures, licenses, dependencies, and update channel information. | Query functions work against actual repository metadata; search results are stable; signature state and package provenance are visible. |
| 9 | `feat(package): add transactional install/remove and update preview` | Integrate package manager with kernel package subsystem, filesystem, signing, dependency resolver, and recovery mechanism. | Dry-run shows all changes; verification occurs before install; failed operations roll back; removal protects essential packages. |
| 10 | `feat(updater): add signed update check and staged deployment UI` | Add update discovery, changelog, compatibility check, download verification, staged apply, reboot coordination, and rollback status. | Tampered/stale updates are rejected; users can inspect changes; interrupted updates recover; versions shown accurately. |
| 11 | `feat(security-center): add permissions and security status application` | Show users sandbox permissions, update/security status, firewall/services status, disk-encryption state, and audit findings only where enforced. | Read-only reports reflect live enforcement; unavailable controls labeled; permission changes emit audit events. |
| 12 | `feat(file-manager): add safe file browsing and scoped app access` | Build basic file browsing, file operations, mounts/devices view, and document opening using VFS APIs and per-app file access grants. | Path traversal and symlink attacks tested; destructive operations confirm targets; sandboxed apps receive only selected file handles through portals. |
| 13 | `feat(notifications): add session notification center and preferences` | Connect notification data model to service and GUI; add per-app preferences, quiet hours, actions, and lock-screen privacy. | Service and UI work in real session; dismissal and action handling tested; private notification content hidden on lock screen. |
| 14 | `feat(recovery): add system recovery and troubleshooting application` | Surface installer recovery, update rollback, boot failure diagnostics, safe mode, and recovery media guidance through CLI and GUI. | Recovery instructions tested on QEMU; destructive recovery requires confirmation; console recovery available on desktop failure. |
| 15 | `security(system-apps): add app permission and sandbox enforcement` | Enforce per-app permissions at service/syscall boundaries; add resource limits, safe IPC, and auditing. | Unprivileged apps cannot bypass UI permissions; denied operations tested; sandbox status reflects actual enforcement. |
| 16 | `docs(system-apps): generate app manuals and user workflows` | Add command references, app guides, task-based onboarding, troubleshooting, and localization structure. | Every shipped app has launch/use/permission/troubleshooting docs; docs match tested behavior and list limitations. |
| 17 | `test(system-apps): add fresh-install and upgrade integration matrix` | Automate boot, login/session, core utilities, app launch, package query/update, settings, file access, recovery, and shutdown. | Tests run on named QEMU configuration; upgrade/recovery paths have artifacts; hosted-only tests reported separately. |
| 18 | `release(system-apps): publish supported application matrix` | Publish app versions, runtime prerequisites, permissions, backends, architectures, known issues, and release evidence. | "Supported" apps have repeatable install/launch/use/upgrade tests; no prototype or model is marketed as shipped functionality. |

---

## 4. Rules by Application Family

- **Core Utilities**: Adhere strictly to POSIX utility argument conventions (synopsis, operands, `--` delimiter handling, standard stream separation, explicit non-zero exit codes on error). Destructive commands (`rm`) enforce path validation safety guards.
- **Package Store & Updater**: Display package origin, version, signature status, dependency trees, and transaction previews. Do not claim cryptographic verification is production-ready if signatures are simulated.
- **Settings**: Use typed schemas, per-user configuration storage, input validation, migration routines, and rollback on invalid writes. Critical settings (display resolution, security policy) require recovery fallback timers.
- **System Information & Diagnostics**: Label every metric source as `Live`, `Cached`, `Host-Derived`, or `Unavailable`. Redact credentials and private user paths.
- **Security Center**: Report active security state strictly according to kernel enforcement (Landlock v4, Pledge, Seccomp, Capabilities).
- **File Manager & Portals**: Scope application file access to explicitly selected handles via portal interfaces. Never grant unrestricted home directory access to sandboxed apps.
- **Notifications & Launcher**: Use session-scoped IPC channels. Protect sensitive notification text on lock screens. Parse launcher metadata safely without shell evaluation.
- **Recovery & Installer**: Maintain strict separation between inspection and destructive formatting/partitioning. Provide explicit target confirmation and dry-run previews.
- **Logging & Crash Reporting**: Enforce storage quotas, redact secrets, obtain explicit opt-in for remote diagnostic submissions, and preserve local crash dumps.
- **Localization & Accessibility**: Keep strings externalized for translation, support full keyboard navigation and high-contrast styling, and ensure screen-reader ARIA/semantic labels exist across core flows.

---

## 5. Periodic Maintenance PR Cadence

| Cadence | Proposed Recurring Task | Required Output / Artifact |
|---|---|---|
| Every PR | `ci(system-apps): run application contract checks` | Code formatting, build, manifest validation, unit/integration tests, and permission checks. |
| Weekly | `maint(system-apps): triage launch and service failures` | Reproducer scripts, affected app/service mapping, owner assignment, priority triage, and issue updates. |
| Weekly | `docs(system-apps): reconcile app catalog and component status` | Synchronized catalog matrix, desktop launcher metadata, and capability status reflecting actual runnable state. |
| Monthly | `security(system-apps): review app permissions and privileged actions` | Audit matrix of per-app permissions, service API boundaries, log redaction verification, and sandbox rule validation. |
| Monthly | `test(system-apps): run clean-boot application smoke suite` | Automated QEMU boot, service launch, CLI diagnostics, package query, settings, and graceful shutdown smoke run. |
| Monthly | `maint(package): review package metadata and signature behavior` | Dependency graph validation, license audit, PQC/Ed25519 signature checks, and transaction rollback verification. |
| Quarterly | `accessibility(system-apps): audit keyboard, screen-reader, and scaling paths` | Audit report for keyboard navigation, focus management, high-contrast modes, text scaling, and ARIA labels. |
| Quarterly | `docs(system-apps): review Linux/BSD interoperability claims` | Updated documentation on POSIX/freedesktop compliance, service API assumptions, and intentional OS deviations. |
| Each Release Candidate | `release(system-apps): certify shipped app set` | Release certification matrix for exact target architectures, clean boot tests, versioning, and verified permissions. |
| Post-Incident | `fix(system-apps): prevent recurrence of <failure>` | Root cause analysis, regression test addition, security impact review, and backport patch deployment. |

---

## 6. PR Template & Merge Gates

### PR Template (`.github/PULL_REQUEST_TEMPLATE.md`)
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

### Review & Merge Gate Rules
- **Ordinary App / Docs PRs**: Require approval from at least 1 maintainer.
- **Privileged Services & Core System Apps**: Require approval from at least 2 maintainers (covering package/update operations, recovery workflows, security center, disk partitioning, or user data portals).

---

## 7. Rollout & Exit Criteria

- **Integrated Milestone**:
  - The application is packaged/installed through the standard system path.
  - The application launches on a real boot target (QEMU / physical hardware).
  - The application uses its intended typed runtime service backend.
  - Error handling returns typed actionable diagnostics without simulating success.

- **Supported Milestone**:
  - The application passes repeatable automated clean-boot, installation, launch, use, update, and recovery tests.
  - Complete user manuals and troubleshooting documentation exist for named target configurations.
  - All permission requests and portal interfaces are verified against active kernel sandboxing.
