# Unified Device Management, Event Processing & IPC Parity Specification

This document details the Pull Request (PR) proposals, architectures, and sub-engines implemented in `src/device/sovereign_device_management_inspirations_pr.rs` and `src/device/udev_devd_rules.rs` to bring SigmaOS into full native parity with landmark Linux and BSD device management, hotplug, power daemons, user session managers, PolicyKit privilege authorization, and D-Bus IPC bus brokers.

---

## 1. Subsystem Architecture Overview

The Unified Device Management, Event Processing & IPC Engine is structured into 6 modular sub-engines coordinated by a master gateway `SovereignDeviceManagementParityPrGateway`:

```
+-------------------------------------------------------------------------------+
|             Sovereign Device Management Parity PR Gateway                     |
|           (Zero-Dependency #![no_std] / alloc, O(1) Zero-Copy Dispatch)        |
+-----------------------+---------------------------------------+---------------+
                        |                                       |
    +-------------------+-------------------+   +---------------+-------------------+
    | Device & Hotplug Event Processing     |   | Power, Session & Authorization    |
    | - udev / eudev rules & symlinks       |   | - acpid power button/lid/AC events|
    | - hwdb PCI/USB modalias database      |   | - UPower battery health & profile |
    | - Predictable net naming (enp3s0...)  |   | - logind/elogind seats & sessions |
    | - BusyBox mdev regex match rules      |   | - Wayland DRI/evdev FD passing    |
    | - FreeBSD devd & devfs Jail sandbox   |   | - PolicyKit action evaluation     |
    | - OpenBSD hotplugd & NetBSD drvctl    |   | - D-Bus System Bus Broker         |
    +---------------------------------------+   +-----------------------------------+
```

---

## 2. Core Sub-Engine Capabilities

### 2.1 udev / eudev / mdev / devd / devfs / hotplug Engine (`UdevEudevMdevDevdDevfsEngine`)
- **udev & eudev Rule Engine**: Kernel Netlink `KOBJ_UEVENT` ring buffer parser (`add`, `remove`, `change`, `bind`, `unbind`), evaluating `/etc/udev/rules.d/` rules for subsystem, sysname pattern, and environment matches. Creates dynamic `/dev/disk/by-id/` and `/dev/input/by-path/` symlinks.
- **Hardware Database (`hwdb`)**: Modalias lookup for PCI (`pci:v...`), USB (`usb:v...`), Bluetooth, and DMI hardware properties.
- **Predictable Network Interface Naming**: Formats interface names according to systemd/udev predictable naming schemes:
  - Ethernet: `enp3s0`, `enp3s0f1`
  - Wireless: `wlp2s0`, `wlp2s0f0`
  - WWAN: `wwan0`
  - Onboard: `eno1`
  - USB Ethernet: `enp0s20f0u1`
  - MAC Address Based: `enx112233445566`
- **BusyBox mdev Regex Rules**: Regex rule table matching device nodes (`sd[a-z][0-9]*`) with ownership UID:GID, octal permissions, and helper scripts (`@/script`, `$/script`, `*/script`).
- **FreeBSD devd & devfs Rules**: `/etc/devd.conf` socket event router for `ATTACH`, `DETACH`, `NOMATCH`, `NOTIFY` events. `/etc/devfs.rules` rule sets for node hiding, permissions, and FreeBSD Jail device sandbox isolation (restricting `/dev/bpf*` or raw disk nodes inside unprivileged containers).
- **OpenBSD hotplugd & NetBSD drvctl**: OpenBSD `/etc/hotplug/` device class attach/detach scripts and NetBSD `drvctl(8)` bus rescan, device detach, and resume commands.
- **Coldplug Replay Simulator**: `udevadm trigger` simulation replaying historical device states for initial system startup.

### 2.2 ACPI Daemon Engine (`AcpidPowerDaemonEngine`)
- **ACPI Event Ring Parser**: Parses `/proc/acpi/event` and ACPI Netlink socket event strings (`button/power PWRF 00000080 00000000`).
- **Rule Pattern Matcher**: Matches ACPI events against `/etc/acpi/events/` rules for power button (`powerbtn.sh`), lid switch (`lid.sh`), AC adapter plug/unplug (`power_profile_toggle.sh`), and audio headphone/jack sensing (`audio_jack.sh`).
- **Thermal Policy Governor**: Evaluates temperature thresholds in Celsius:
  - `< 70°C`: Normal
  - `70°C - 79°C`: PassiveCooling
  - `80°C - 94°C`: FanSpeedHigh
  - `>= 95°C`: EmergencyShutdown

### 2.3 UPower Battery & Power Daemon Engine (`UpowerBatteryPowerEngine`)
- **Power Supply Enumerator**: Enumerates `/sys/class/power_supply/` devices (`BAT0`, `AC0`, `UPS`, wireless peripherals).
- **Battery Health & Wear Degree**: Calculates degradation ratio (`energy_full / energy_full_design * 100`).
- **Time-to-Empty / Time-to-Full Algorithm**: Estimates remaining discharge/charge runtime in seconds based on instantaneous power rate in Watts (`energy_now_wh / power_rate_w`).
- **Multi-Battery Aggregation**: Calculates weighted energy capacity percentage across multiple installed batteries.
- **Charge Stop Thresholds**: Configurable percentage limit (e.g. 80%) to prolong battery lifespan.
- **Power Profile Policy Hint Governor**: Dynamically hints power profile (`PowerSaver`, `Balanced`, `Performance`, `LowBatterySaver`) based on AC online state and remaining battery percentage.

### 2.4 logind / elogind Session & Seat Engine (`LogindElogindSessionEngine`)
- **User Session Lifecycle**: Manages user session states (`Active`, `Online`, `Closing`), TTY VT allocation (`/dev/tty1`), and user UID mapping.
- **Multiseat Allocation**: Allocates seats (`seat0`, `seat-workstation1`), tracks active session on seat, and tags device nodes (`/dev/dri/card0`, `/dev/input/event0`).
- **Unprivileged DRI & Evdev FD Delegation**: Enables Wayland compositors and display servers to request privileged GPU/input file descriptors (`take_device_fd`), release them (`release_device_fd`), and handle VT switch pause/resume events (`pause_device_fd`, `resume_device_fd`).
- **Inhibitor Lock Registry**: Registers sleep, shutdown, and idle inhibitor locks with `delay` or `block` modes, blocking unauthorized power transitions during critical tasks (e.g. system upgrades).
- **System Power Transitions**: Validates power transition requests (`Suspend`, `Hibernate`, `HybridSleep`, `Reboot`, `PowerOff`) against active inhibitor locks.

### 2.5 PolicyKit Privilege Authorization Engine (`PolkitAuthorizationEngine`)
- **Action Policy Evaluator**: Evaluates authorization policies for actions like `org.freedesktop.udisks2.filesystem-mount`, `org.freedesktop.login1.power-off`, and `org.freedesktop.packagekit.system-update`.
- **Subject Verification**: Identifies callers by PID, UID, and Session ID.
- **Authorization Results**: Supports `Yes`, `No`, `AuthSelf`, `AuthAdmin`, `AuthSelfKeep`, and `AuthAdminKeep`.
- **Wildcard Matcher**: Supports wildcard action patterns (`org.freedesktop.udisks2.*`).
- **Authorization Caching**: Temporary retention of privilege tokens (`AuthSelfKeep`, `AuthAdminKeep`) with TTL expiration.

### 2.6 D-Bus System Bus Broker Engine (`DbusSystemBusBrokerEngine`)
- **Wire Protocol Message Headers**: Parses and serializes `MethodCall`, `MethodReturn`, `Error`, and `Signal` messages.
- **Bus Name Ownership Registry**: Tracks well-known bus names (`org.freedesktop.udev`, `org.freedesktop.UPower`, `org.freedesktop.login1`, `org.freedesktop.PolicyKit1`) and unique client IDs (`:1.1`, `:1.2`).
- **Method Routing & Signal Broadcast**: Routes `MethodCall` requests to target destination and broadcasts `Signal` events to active subscribers.
- **Introspection XML Generator**: Dynamically generates D-Bus introspection XML schemas for service interfaces.
- **Access Control Policy Filters**: Enforces minimum UID requirements and interface prefix rules before routing messages.

---

## 3. Pull Request Proposals (#1001..#1006)

The `SovereignDeviceManagementParityPrGateway` pre-populates 6 proposals covering all device management, power, session, and IPC categories:

| PR ID | Title | Target Subsystem | Original Distros & Systems |
|-------|-------|------------------|----------------------------|
| 1001  | `feat(udev_devd)`: Implement Universal KOBJ_UEVENT Processing, Predictable Net Names & FreeBSD Devfs Jail Sandbox | Device Management & Devfs | Linux systemd-udevd / eudev, BusyBox mdev, FreeBSD devd & devfs |
| 1002  | `feat(acpid)`: Implement ACPI Ring Parser & Thermal/Lid/Power/Jack Event Dispatcher | ACPI Power & Thermal | Linux acpid, FreeBSD powerd, OpenBSD apmd |
| 1003  | `feat(upower)`: Implement Power Supply Enumerator, Battery Health & Runtime Estimator | UPower Framework | FreeDesktop UPower, Linux sysfs power_supply, FreeBSD powerd |
| 1004  | `feat(logind)`: Implement User Session Manager, Seat0 Multiseat, DRI/Evdev FD Delegate & Inhibitors | Session & Seat Governor | Linux systemd-logind, elogind, FreeBSD seatd / ConsoleKit2 |
| 1005  | `feat(polkit)`: Implement PolicyKit Action Evaluator, Subject Verification & Privilege Policy Rules | Policy & Authorization | FreeDesktop PolicyKit (polkit), FreeBSD MAC framework |
| 1006  | `feat(dbus_broker)`: Implement Zero-Copy D-Bus Wire Protocol Router, Service Activation & Name Registry | D-Bus System Bus Broker | FreeDesktop D-Bus, dbus-broker, Linux/BSD IPC |

---

## 4. Verification & Testing

The implementation is verified via standalone unit tests using the repository standalone test runner:

```bash
./scripts/changed_files_rustc_tests.sh src/device/sovereign_device_management_inspirations_pr.rs
```

All 9 unit tests pass cleanly, validating device hotplug event processing, ACPI event dispatching, battery wear/runtime estimation, logind FD delegation & inhibitors, PolicyKit privilege authorization, D-Bus message routing, and master gateway end-to-end integration.
