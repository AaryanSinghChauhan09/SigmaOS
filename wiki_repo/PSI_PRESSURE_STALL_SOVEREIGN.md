# SigmaOS Sovereign Pressure Stall Information (PSI)

## Overview

SigmaOS implements a **100% Safe Rust sovereign PSI (Pressure Stall Information) subsystem** (`src/kernel/psi_sovereign.rs`), absorbing the resource contention metrics framework introduced in Linux 4.20 (`/proc/pressure/cpu`, `/proc/pressure/memory`, `/proc/pressure/io`).

Traditional load averages (1/5/15 minutes) only count runnable tasks and fail to differentiate between CPU saturation, memory thrashing, and storage I/O bottlenecks. PSI quantifies the exact percentage of CPU cycles lost due to resource shortages.

## Metrics & Windows

- **`some`**: Percentage of wall-clock time where at least one non-idle task was stalled waiting for the resource.
- **`full`**: Percentage of wall-clock time where **all** non-idle tasks were stalled simultaneously (complete system lockup / throughput zeroed).
- **Averages**:
  - `avg10`: 10-second exponential moving average.
  - `avg60`: 60-second exponential moving average.
  - `avg300`: 300-second exponential moving average.
  - `total`: Cumulative microsecond stall time.

## Event Triggers

Userspace resource managers (such as OOM daemons or container governors) register triggers with specific stall percentage thresholds over monitoring windows. When pressure spikes above the configured limit, the kernel fires real-time notification events.

## Test Verification

6 standalone unit tests verified in test runner suite `[19]`:
- `test_psi_initial_state`: Initial zeroed accounting.
- `test_psi_record_stall_accumulation`: Tracking some and full stall time.
- `test_psi_trigger_firing`: Threshold breach detection and trigger firing.
- `test_psi_trigger_threshold_isolation`: Selective trigger activation.
- `test_psi_format_proc_pressure`: Linux `/proc/pressure/*` output format parity.
- `test_psi_cpu_no_full_stall`: Respects Linux PSI invariant where CPU lacks full stall line.
