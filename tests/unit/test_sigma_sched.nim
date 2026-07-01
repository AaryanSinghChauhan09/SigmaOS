## SigmaOS: ms per level */
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
  Task* = object
    pid*: SigmaI32
    priority*: SigmaI32
    cpu_time_used*: SigmaI32
    total_runtime*: SigmaI32
    runnable*: SigmaBool

type
  MLFQ* = object

proc mlfq_enqueue*() {.exportc.} =
  discard

proc mlfq_tick*() {.exportc.} =
  discard

