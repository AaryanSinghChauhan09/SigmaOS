# ⚡ SigmaOS System, Desktop, Security & Automation Actions (`actions`) Strategic Development Plan

## Executive Summary & Design Vision

In operating systems, **Actions** represent executable responses to system events, user interactions, security authorization requests, or automated rule triggers. Actions unify four essential system domains:
1. **System & Event Actions**: Automated responses to hardware events (`udev` rules, FreeBSD `devd` events, systemd timers, thermal throttling, power saving triggers).
2. **Desktop & File Manager Actions**: Contextual quick actions in desktop launchers (Rofi/KRunner actions, Thunar custom file actions, `OmarchyThunarFileActions`).
3. **Privileged Authorization Actions**: PolicyKit (`org.freedesktop.*`) and `doas.conf`/`sudoers` action authorization rules for administrative system operations.
4. **Interactive Notification Actions**: Actionable desktop notification buttons (`Reply`, `Snooze`, `Approve`, `Dismiss`) communicating via D-Bus desktop portals.

This strategic plan establishes the architectural design, multi-distro inspirations, core subsystems, phased development roadmap, and verification standards for **SigmaOS Action Infrastructure**.

---

## 1. Multi-Distro & Multi-OS Action Inspirations

### 1.1 Linux `udev` & FreeBSD `devd` Hardware Event Actions
- **Inspirations**:
  - **Linux `udev` Rules**: Hardware attachment (`ACTION=="add"`) triggering kernel module loading, devtmpfs permissions, and script execution (`RUN+="/usr/bin/script"`).
  - **FreeBSD `devd.conf` Actions**: Match-action rules mapping hardware state changes (AC adapter disconnect, thermal threshold exceed, network link state change) to system automation actions.
- **SigmaOS Integration**: `SystemAction` engine in `src/automation/system_level.rs` and `BsdDevdHardwareEventDispatcher` in `src/distro/bsd_linux_innovations.rs`.

### 1.2 Thunar Custom File Actions & Rofi / KRunner Quick Actions
- **Inspirations**:
  - **Thunar File Actions**: Context menu extension rules (`Open Terminal Here`, `Compress Archive`, `Shred File`, `Checksum MD5`).
  - **Desktop Quick Actions**: `.desktop` file `Actions=Gallery;NextTrack;` declarations allowing right-click taskbar or launcher quick invocation.
- **SigmaOS Integration**: `OmarchyThunarFileActions` and `OmarchyRofiAppLauncher` in `src/distro/omarchy_expanded_parity.rs`.

### 1.3 Linux PolicyKit (`polkit`) & OpenBSD `doas` Privileged Actions
- **Inspirations**:
  - **PolicyKit Action IDs**: Standardized privilege action identifiers (`org.freedesktop.packagekit.system-update`, `org.freedesktop.udisks2.filesystem-mount`).
  - **`doas.conf` Rule Actions**: Fine-grained command privilege escalation with or without password prompts (`permit nopass user as root cmd /sbin/reboot`).
- **SigmaOS Integration**: `PrivilegeEngine`, `PolicyKitAuthority`, and `DoasPolicy` in `src/security/privilege_engine.rs`.

### 1.4 Desktop Portal Notification Action Buttons
- **Inspirations**: FreeDesktop Desktop Notification Portal (`org.freedesktop.Notifications`) supporting interactive inline action buttons (`Action { id: "accept", label: "Accept Transfer" }`).
- **SigmaOS Integration**: `SmartNotificationManager` and `ZenithNotificationPortal` in `src/desktop/`.

---

## 2. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                     Userland System & Zenith Desktop                      │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│              Desktop Quick Action & File Association Router               │
│     - Thunar Custom Actions (`Open Terminal`, `Extract Here`, `Shred`)    │
│     - Rofi / KRunner Desktop File Quick Actions (`Actions=...`)           │
│     - Interactive Notification Action Buttons (`Reply`, `Approve`)        │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               PolicyKit & Doas Privileged Action Guard                    │
│     - Action ID Authorization (`org.freedesktop.policykit.*`)            │
│     - `doas.conf` & `/etc/sudoers` Rule Matching & Audit Logging         │
│     - Biometric / Security Key (FIDO2) Action Prompting                   │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│             SystemAction Automation & Event Trigger Engine                │
│     - Thermal Throttling: `AdjustCooling`, `ThrottleProcesses`             │
│     - Power State: `EnablePowerSaving`, `AdjustCpuFrequency`              │
│     - Maintenance: `ClearCache`, `OptimizeStorage`, `ScheduleUpdate`       │
│     - Cron / Timer: `TriggerBsdCronTask`, `TriggerSystemdTimer`           │
└───────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Action Priority & Conflict Resolution
- Actions triggered by rule engines are sorted by priority:
  1. **Emergency Safety Actions** (Thermal shutdown, OOM process kill, Power cutoff guard): Priority **15-20**.
  2. **Power & Hardware Adjustment Actions**: Priority **10-14**.
  3. **Background Maintenance Actions** (Storage trim, cache clear, update check): Priority **1-9**.

### 2.2 Execution Latency Guarantee
- Hardware event action dispatch runtime: **< 1 millisecond**.
- Desktop notification action button response latency: **< 5 milliseconds**.

---

## 3. Phased Development Roadmap

### Phase 1: Core `SystemAction` Engine & Priority Dispatcher (Q4 2026)
- Standardize `SystemAction` enum types and priority sorting in `src/automation/system_level.rs`.
- Connect thermal sensors and power state changes to automatic `SystemAction` execution.
- Implement error recovery and rollback logging for failed actions.

### Phase 2: Privileged PolicyKit & Doas Action Guards (Q1 2027)
- Integrate PolicyKit action IDs (`org.freedesktop.*`) into `src/security/privilege_engine.rs`.
- Implement `doas.conf` rule action matcher with audit logging.
- Support FIDO2 / Passkey biometric confirmation for high-risk administrative actions.

### Phase 3: Desktop Quick Actions & Notification Portal (Q2 2027)
- Deploy `OmarchyThunarFileActions` custom context menu actions in Zenith file manager.
- Implement `.desktop` file quick action parsing in `OmarchyRofiAppLauncher`.
- Wire notification action buttons to desktop portals (`org.freedesktop.Notifications`).

### Phase 4: Automated Event Action Benchmarks & CI Workflows (Q3 2027+)
- Connect hardware hotplug events (`BsdDevdHardwareEventDispatcher`) with dynamic driver load actions.
- Deploy automated action dispatch benchmarks in `scripts/tech_media_benchmark_suite.sh`.
- Conduct security fuzzing on action argument parsers (`cargo fuzz`).

---

## 4. Verification & Testing Standards

All action components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/automation/system_level.rs` (`SystemAction` rule engine execution & priority sorting)
- `src/security/privilege_engine.rs` (`PolicyKitAuthority` & `DoasPolicy` action authorization)
- `src/distro/omarchy_expanded_parity.rs` (`OmarchyThunarFileActions` & `OmarchyRofiAppLauncher`)
- `src/distro/bsd_linux_innovations.rs` (`BsdDevdHardwareEventDispatcher` hardware actions)
