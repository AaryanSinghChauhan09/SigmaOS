## SigmaOS: =============================================================================
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
  RCUCallback* = object

type
  SigmaRCU* = object
    grace_period_start*: SigmaU64
    quiescent_mask*: SigmaU64
    last_tick*: SigmaU64

proc rcu_read_lock*() {.exportc.} =
  discard

proc rcu_read_unlock*() {.exportc.} =
  discard

proc rcu_on_quiescent_state*() {.exportc.} =
  discard

proc rcu_init_core*() {.exportc.} =
  discard

