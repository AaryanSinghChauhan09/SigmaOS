# SigmaOS Battery Management & Portable Device Power Optimization Specification

## 1. Overview

The Battery Management Subsystem oversees battery health tracking, smart charge threshold management, power consumption profiles, thermal safeguards, and automated low-battery emergency actions for laptops, tablets, and handheld devices running SigmaOS.

## 2. Smart Battery Subsystem Architecture

The battery subsystem communicates with embedded controllers (EC) via ACPI Smart Battery System (SBS) or SMBus drivers.

```
+-----------------------------------------------------------------+
|                    Embedded Controller (EC) / SBS               |
+--------------------------------+--------------------------------+
                                 |
                      sysfs Battery Class Node
                    `/sys/class/power_supply/BAT0/`
                                 |
+--------------------------------v--------------------------------+
|                   Sigma Power Daemon (`sigpowerd`)              |
|  +--------------------+  +-------------------+  +-------------+ |
|  | Charge Thresholds  |  | Health Telemetry  |  | Power Profile| |
|  | (80% Longevity Mode|  | (Cycle Count, SOH)|  | Governor    | |
|  +--------------------+  +-------------------+  +-------------+ |
+--------------------------------+--------------------------------+
                                 |
+--------------------------------v--------------------------------+
|                    Zenith Desktop Power Applet                  |
+-----------------------------------------------------------------+
```

## 3. Battery Health Metrics & Properties

Exposed via `/sys/class/power_supply/BAT0/`:
- **`capacity`**: Remaining battery percentage (`0` to `100%`).
- **`status`**: Current state (`Charging`, `Discharging`, `Full`, `Not charging`).
- **`energy_now` / `energy_full` / `energy_full_design`**: Energy levels measured in micro-Watt-hours (uWh).
- **`power_now`**: Current instant discharge rate in micro-Watts (uW).
- **`cycle_count`**: Total battery charge cycles completed.
- **State of Health (SOH)**: Calculated ratio `(energy_full / energy_full_design) * 100`.

## 4. Charge Thresholds & Longevity Modes

To prevent lithium-ion degradation when laptops are plugged into AC power continuously:
- **Longevity Charge Limit (80%)**: Embedded Controller stops charging when capacity reaches 80% (`charge_control_end_threshold = 80`).
- **Full Capacity Charge (100%)**: Temporarily enables full charging for travel mode.

```bash
# Set max charge threshold to 80% to protect battery longevity
echo 80 > /sys/class/power_supply/BAT0/charge_control_end_threshold
```

## 5. Automated Emergency Battery Protections

`sigpowerd` continuously monitors discharging rates and triggers configurable safeguards:
1. **Low Battery Warning (15%)**: Desktop notification popup requesting connection to AC power.
2. **Critical Low Battery (5%)**: Automatic dimming of display brightness, suspension of non-critical background services, switch CPU to `powersave` governor.
3. **Emergency Auto-Hibernate (2%)**: Automates state compression and triggers immediate S4 hibernate-to-disk or emergency graceful shutdown to prevent data corruption.
