# Future Development: Power and Time

**Status:** Proposal. Power-management models and clock APIs do not establish hardware control, suspend/resume support, or accurate timekeeping.

## Scope

Plan power-state transitions, thermal and battery reporting, clock sources, timers, and clock synchronization. Current source areas include [`src/power`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/power), the [CPU frequency governor](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/power/governor.rs), [`src/time`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/time), and [`src/drivers/acpi.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/drivers/acpi.rs).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| Linux | cpufreq governors, runtime power management, and suspend states | Which transitions are supported by each discovered device and platform? |
| FreeBSD | powerd policy and ACPI integration | How should policy be separated from hardware-specific transitions? |
| OpenBSD | Conservative hardware defaults and clear device reporting | Can unsupported or unsafe transitions be refused without leaving devices inconsistent? |
| NetBSD | Machine-independent timecounter and clock framework | Can timekeeping use a stable abstraction with explicit source quality and fallback? |

## Proposed work sequence

1. Separate monotonic time, wall-clock time, deadlines, and calendar conversion; specify precision, wraparound, and invalid-source behavior.
2. Identify clock sources and timers during hardware discovery, with validated fallback selection and drift reporting.
3. Model power states as transitions with preconditions, device ordering, timeout, rollback, and user-visible failure status.
4. Read battery and thermal data through validated drivers; do not let policy act on synthetic or stale measurements.
5. Add platform-specific suspend/resume only after device quiescence, state preservation, and recovery paths are defined.
6. Keep CPU-frequency policy calculations separate from hardware drivers. Report a requested target separately from measured operating frequency and apply only limits discovered and validated for that platform.

## Completion criteria

- Monotonic timers meet documented bounds on named targets and behave correctly across counter wraparound.
- Clock-source loss or invalid hardware data selects a documented fallback or reports timekeeping unavailable.
- Power-state requests verify device support and return failure if any transition step cannot complete.
- Suspend/resume preserves or safely reinitializes device and timer state on each claimed platform.
- Thermal and battery reporting states its units, update rate, source, and stale-data behavior.
- Frequency-governor calculations remain within valid limits without overflow, and hardware requests pass through a platform provider.

## Maintenance

State tested platforms and clock or power providers. Keep policy, telemetry, and actual hardware transitions separate in both code links and support claims.
