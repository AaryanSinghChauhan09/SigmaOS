# Thermal Management & Dynamic Cooling Component Guide

## Component Overview
Governs CPU/GPU thermal zone monitoring, fan curve PWM modulation, thermal trip points (passive, active, critical), and proactive throttling.

## Linux Inspiration
Linux thermal subsystem, thermald (Intel Thermal Daemon), and lm-sensors fan control.

## BSD Inspiration
OpenBSD sysctl hw.sensors thermal monitoring, FreeBSD acpi_thermal driver.

## Current SigmaOS Status
ACPI thermal zone discovery and CPU temperature polling functional. Dynamic fan curve PWM regulation is under active development.

## Critical Missing Features
Custom user fan curve profiles with hysteresis smoothing, emergency thermal shutdown trip wire, and passive CPU clock down-stepping.

## Implementation Priority
MEDIUM (Phase 2)

## Key Files to Create/Improve
src/power/thermal.rs, src/hardware/hal.rs, src/system/sigmapower.rs

## Testing Strategy
Simulated thermal load spikes, thermal trip-point throttling benchmarks under sustained 100% CPU/GPU loads.

## Dependencies
HARDWARE_HAL_DRIVERS_AGENT, POWER_MANAGEMENT_AGENT, SCHEDULER_AGENT

## Success Criteria
Zero thermal throttling jitter, whisper-quiet fan curves during desktop idle, instant trip-point protection.

## Open Source Competitors Analysis
Integrates thermal monitoring directly with the scheduler rather than polling from userspace daemons.

## Future Enhancements
Machine learning thermal prediction ramping fan speeds ahead of time when heavy compilation jobs are spawned.
