## SigmaOS: SigmaOS Sovereign Parallel Dispatcher
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

proc parallelism_sync_all*() {.exportc.} =
  discard

proc S12_Register_ParallelDispatcher*() {.exportc.} =
  discard

