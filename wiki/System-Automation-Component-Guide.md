# System Automation & Scheduled Tasks Component Guide

## Component Overview
Coordinates cron-style time-based triggers, hardware sensor event triggers, automated file backups, and proactive thermal/power policies.

## Linux Inspiration
systemd timers, cron daemon, anacron, and udev event triggers.

## BSD Inspiration
FreeBSD periodic maintenance scripts, OpenBSD cron privilege separation.

## Current SigmaOS Status
Rule evaluation engine and time-triggered task dispatch implemented in SystemAutomationManager. Hardware sensor triggers in progress.

## Critical Missing Features
High-precision calendar-based event scheduling (systemd OnCalendar syntax), miss-tolerant catchup execution, and CPU battery threshold triggers.

## Implementation Priority
MEDIUM (Phase 2)

## Key Files to Create/Improve
src/automation/rules.rs, src/automation/triggers.rs, src/time/clocksource.rs

## Testing Strategy
Time-warp simulation verifying schedule trigger accuracy, high-frequency event rule dispatch benchmarks.

## Dependencies
INIT_SUPERVISOR_AGENT, TIMEKEEPING_AGENT, POWER_MANAGEMENT_AGENT

## Success Criteria
Sub-millisecond trigger accuracy, zero CPU wake-lock battery drain from idle timer polling.

## Open Source Competitors Analysis
Replaces complex cron/anacron/systemd timer divergence with a unified, compile-time verified declarative automation engine.

## Future Enhancements
Self-tuning automation engine that adjusts maintenance window times based on user sleep habits.
