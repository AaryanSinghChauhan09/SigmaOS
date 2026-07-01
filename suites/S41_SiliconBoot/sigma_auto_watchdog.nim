## SigmaOS: SIGMA_AUTO_WATCHDOG_H */
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
  SigmaWDService* = object
    state*: SigmaU64
    last_heartbeat*: SigmaU64
    timeout_cycles*: SigmaU64
    restart_count*: SigmaI32
    max_restarts*: SigmaI32
    restart_fn*: SigmaU64

type
  SigmaWatchdog* = object
    count*: SigmaI32

proc wd_init*() {.exportc.} =
  discard

proc wd_heartbeat*() {.exportc.} =
  discard

proc wd_tick*() {.exportc.} =
  discard

