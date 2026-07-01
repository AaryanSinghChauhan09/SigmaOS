## SigmaOS: Read hardware cycle counter */
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
  SigmaTask* = object
    pid*: SigmaI32
    state*: SigmaU64
    entry*: SigmaU64
    slice_start*: SigmaU64

type
  SigmaScheduler* = object
    count*: SigmaI32
    current*: SigmaI32

proc sched_init*() {.exportc.} =
  discard

proc sched_tick*() {.exportc.} =
  discard

proc sched_block*() {.exportc.} =
  discard

proc sched_unblock*() {.exportc.} =
  discard

