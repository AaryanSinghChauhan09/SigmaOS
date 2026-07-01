## SigmaOS: SIGMA_SYS_CRON_H */
## Migrated from C/C++ to Nim — no stdlib import, no external packages.
## All types hand-defined. OOP via object hierarchy + method dispatch.
{.push raises: [].}

type
  SigmaU8*  = uint8
  SigmaU16* = uint16
  SigmaU32* = uint32
  SigmaU64* = uint64
  SigmaI32* = int32
  SigmaI64* = int64
  SigmaBool* = bool
  SigmaUsize* = uint

type
  SigmaCronTask* = object
    fn*: SigmaU64
    period_cycles*: SigmaU64
    last_run*: SigmaU64
    run_count*: SigmaU64
    enabled*: uint8

type
  SigmaCron* = object
    count*: SigmaI32

proc cron_init*() {.exportc.} =
  discard

proc cron_disable*() {.exportc.} =
  discard

proc cron_enable*() {.exportc.} =
  discard

