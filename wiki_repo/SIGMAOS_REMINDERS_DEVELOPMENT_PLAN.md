# SigmaOS Reminders & Notification Daemon Subsystem - Master Development Plan

## 1. Executive Summary & Vision

`Sigma-Reminders` (`sigma-remind`) is the native, zero-dependency task, reminder, alarm, and desktop notification management subsystem for **SigmaOS**. Inspired by the expressive power of Linux `remind(1)`, the high-precision job scheduling of BSD `atd`/`cron`, the Desktop Notification Specification (`org.freedesktop.Notifications`), and cross-device companion synchronization (KDE Connect/Itinerary), `Sigma-Reminders` provides a unified, context-aware notification and reminder system for power users and enterprise desktops.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Subsystem Capability Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **Linux `remind(1)` & RFC 5545 iCalendar** | Complex recurrence rules (`RRULE:FREQ=WEEKLY;BYDAY=MO,WE,FR`), lead-time alerts (`AT 09:00 +15`), location triggers, `.ics` file parser. | `src/productivity/` & `src/tools/` |
| **BSD `atd(8)` / `cron(8)` & RTC Alarms** | High-resolution hierarchical timer wheel, hardware RTC wake-alarm integration, low-power doze mode traversal. | `src/kernel/` & `src/init/` |
| **Linux Desktop Notification Portal** | D-Bus `org.freedesktop.Notifications` daemon, banner toasts (Libadwaita style), urgency levels (*Low*, *Normal*, *Critical*), action callbacks. | `src/desktop/desktop_portal.rs` & `src/ui/gtk.rs` |
| **KDE Connect & KDE Itinerary** | Encrypted cross-device notification mirroring, remote snooze/dismiss, travel itinerary alerts, shared clipboard reminders. | `src/orchestration/cross_device.rs` |
| **Linux Focus Mode & Doze Governor** | Context-aware notification batching, Do Not Disturb (DND) profiles, AI urgency filtering during focus sessions. | `src/ai/` & `src/graphics/video.rs` |

---

## 3. 5-Layer Reminders Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Sovereign AI Context Filter & DND Focus Mode Governor         │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Cross-Device Companion Sync Bridge (KDE Connect Mirroring)     │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: D-Bus `org.freedesktop.Notifications` Desktop Toast Dispatcher │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: BSD `atd` / RTC Exact Alarm High-Resolution Timer Wheel       │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: iCalendar RFC 5545 & `remind(1)` Expression Engine            │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: iCalendar RFC 5545 & `remind(1)` Expression Engine
- **Recurrence Parser:** Full support for `RRULE` frequency expansions, day offsets, month-end rules, and exception dates (`EXDATE`).
- **Lead-Time Offsets:** Pre-alert warnings (e.g. notify 15 minutes before meeting).
- **Location Triggers:** Geofence / SSID network proximity reminder activation.

### Layer 2: BSD `atd` / RTC Exact Alarm Timer Wheel
- **Timer Wheel:** $O(1)$ insert and expiration tracking for millions of scheduled reminders.
- **RTC Wake Alarms:** Programming Linux `/dev/rtc0` / BSD RTC hardware timers to wake the system from ACPI S3 (Suspend-to-RAM) or S4 (Hibernate) for critical alarms.
- **Doze Mode Traversal:** Coalescing non-critical reminders to conserve battery while guaranteeing exact delivery for critical alarms.

### Layer 3: D-Bus Notification Portal & Desktop Toast Dispatcher
- **Freedesktop Spec Parity:** Full implementation of `org.freedesktop.Notifications` methods (`Notify`, `CloseNotification`, `GetCapabilities`).
- **Visual Polish:** Libadwaita-inspired toast overlays, banner widgets, and urgency-based color accents.
- **Audio Alerts:** Spatial audio cues mapped to sound event themes (`notification-message-IM`, `alarm-clock-elapsed`).

### Layer 4: Cross-Device Companion Sync Bridge
- **Notification Mirroring:** Real-time encrypted push notifications to companion mobile devices (Android/iOS).
- **Remote Actions:** Dismissing or snoozing a reminder on one device synchronizes state across all paired nodes instantly.

### Layer 5: Sovereign AI Context Filter & Focus Mode Governor
- **Intelligent Urgency Scoring:** AI evaluation of notification content to determine whether to interrupt the current workspace.
- **DND Focus Profiles:** Silencing background alerts during coding, gaming, or presentations while queueing summary digests.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | Expression Engine | Implement iCalendar RFC 5545 parser, `remind(1)` expression evaluator, and lead-time calculations. | Implemented |
| **Milestone 2** | Timer Wheel & RTC | Implement high-resolution timer wheel and RTC hardware wake-alarm scheduler in kernel/userspace. | Implemented |
| **Milestone 3** | Desktop Portal | Implement `org.freedesktop.Notifications` D-Bus daemon, GTK toast overlay, and audio cues. | Implemented |
| **Milestone 4** | Cross-Device Sync | Implement companion notification mirroring, remote snooze/dismiss, and state synchronization. | Implemented |
| **Milestone 5** | AI Focus Governor | Implement AI urgency scoring, DND focus profiles, and intelligent summary digests. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test suites in `src/orchestration/cross_device.rs`, `src/ui/gtk.rs`, and `src/desktop/desktop_portal.rs`.
2. **Timer Accuracy Tests:** Verifying sub-millisecond timer wheel expiration and RTC wake alarm correctness.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.
