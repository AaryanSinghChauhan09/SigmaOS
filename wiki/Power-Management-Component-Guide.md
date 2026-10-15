# Power Management & ACPI Component Guide

## Component Overview
Governs ACPI hardware tables, CPU P-states/C-states, dynamic frequency scaling (DVFS), battery telemetry, thermal zones, and suspend-to-RAM (S3/S0ix).

## Linux Inspiration
Linux ACPI subsystem, cpufreq/intel_pstate governors, TLP power management daemon, thermald dynamic cooling.

## BSD Inspiration
FreeBSD acpi driver and powerd daemon, OpenBSD apm and sensor framework (sysctl hw.sensors).

## Current SigmaOS Status
ACPI RSDP/MADT parsing, basic CPU frequency throttling, and battery state querying functional. Full sleep state transitions (S3/S4) and thermald cooling need work.

## Critical Missing Features
Complete ACPI AML bytecode interpreter, S3 suspend-to-RAM and S0ix modern standby, per-core energy-aware frequency governors, and thermal throttling policies.

## Implementation Priority
MEDIUM (Phase 2)

## Key Files to Create/Improve
src/system/sigmapower.rs, src/power/acpi.rs, src/power/cpufreq.rs, src/power/thermal.rs

## Testing Strategy
Suspend/resume cycle endurance tests, thermal limit stress tests under sustained multi-threaded workloads.

## Dependencies
HARDWARE_HAL_DRIVERS_AGENT, SCHEDULER_AGENT

## Success Criteria
Sub-1-second resume from suspend, 25% longer battery life than Linux on matching laptop hardware, zero thermal runaway incidents.

## Open Source Competitors Analysis
Replaces complex userland daemons (TLP, auto-cpufreq, thermald) with an integrated kernel-level autonomous energy governor.

## Future Enhancements
Machine-learning battery life predictor optimizing background tasks based on user work schedules.
