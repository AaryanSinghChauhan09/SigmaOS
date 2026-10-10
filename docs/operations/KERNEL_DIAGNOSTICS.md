# Kernel Diagnostics and Operational Rehearsal Guide

## Overview
SigmaOS incorporates structured boot diagnostic reports and panic diagnostic formats to convert kernel panic events into actionable, debuggable artifacts.

## Panic Reporting Format
```text
--- SIGMAOS KERNEL PANIC REPORT ---
Component: scheduler
Operation: context_switch
Message: Invalid thread state
Fault Address: 0x00000000deadbeef
Registers:
  RAX: 0x0000000012345678
  RSP: 0x00007fff00000000
Backtrace:
  #00: 0xffffffff80001000
--- END KERNEL PANIC REPORT ---
```

## Greppable Boot Event Logging
Boot logs use the standard prefix `[BOOT_SUB]` and `[BOOT_REPORT]`:
```text
[BOOT_REPORT] os=SigmaOS 1.0 success=true
[BOOT_SUB] stage=memory_init name=Paging status=SUCCESS duration_us=150
[BOOT_SUB] stage=scheduler_init name=Scheduler status=SUCCESS duration_us=300
```

## Monthly Diagnostics Rehearsal
Every month, engineering conducts a panic rehearsal:
1. Inject a fault in a non-critical subsystem in a test build.
2. Verify that the panic report includes fault address, register state, and component details.
3. Confirm that crash logs are persisted to disk or serial output.
