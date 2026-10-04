# Power Management

**Capability state: Prototype.** ACPI or power-management models do not establish tested suspend, battery, thermal, or power-saving behavior. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository includes power-related code, but no hardware support matrix or verified suspend/resume, battery reporting, thermal control, or power-button path is documented. No battery-life or idle-power comparison against Linux Mint or Omarchy has been measured.

## Design references

Linux ACPI and runtime power management, FreeBSD powerd, and Mint's user-facing power controls are references. Preserve safe defaults and show unavailable hardware features clearly.

## Roadmap

1. Validate firmware-table parsing and timer/interrupt prerequisites in QEMU.
2. Add one power event at a time and test error paths without risking persistent data.
3. Validate suspend/resume, thermal reporting, battery state, and lid/power-button events on named hardware.
4. Publish repeatable power measurements with device, firmware, workload, and baseline.

**Completion evidence:** named-model event logs, repeated suspend/resume and thermal tests, recovery behavior, and reproducible battery/power measurements.
