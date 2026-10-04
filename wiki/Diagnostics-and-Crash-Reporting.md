# Diagnostics and Crash Reporting

**Capability state: Proposed.** No verified boot-time crash capture, persistent dump writer, or userland diagnostics service is documented. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository includes diagnostics-related names and models, but no supported end-to-end path has been validated. Do not claim capture of kernel panics, process crashes, hardware faults, automatic diagnosis, or persistent crash reports unless the runtime implementation and tests demonstrate those behaviors.

## Design references

Linux kernel logs and crash dump facilities, FreeBSD DTrace/crash diagnostics, OpenBSD's concise security reporting, and Mint's understandable troubleshooting guidance are useful references. Crash data should be bounded, privacy-aware, and available when the main system cannot boot.

## Roadmap

1. Establish a reliable serial log and panic path in the minimal boot target.
2. Define a bounded, versioned crash record and test corrupt/truncated records.
3. Add persistent storage only after filesystem recovery is available.
4. Provide clear user-facing instructions to export diagnostic data and protect sensitive contents.

**Completion evidence:** injected kernel panic and process failure produce bounded records that survive the documented failure mode and can be inspected in recovery without exposing secrets.
