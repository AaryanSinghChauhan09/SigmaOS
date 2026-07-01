## SigmaOS: =========================================================================
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
  SigmaTraceBuffer* = object
    head*: SigmaU64
    tail*: SigmaU64
    size*: SigmaU64
    dropped*: SigmaU64

proc sigma_mcount_tracer*() {.exportc.} =
  discard

proc sigma_trace_event_commit*() {.exportc.} =
  discard

proc SovereignFTrace_Init*() {.exportc.} =
  discard

