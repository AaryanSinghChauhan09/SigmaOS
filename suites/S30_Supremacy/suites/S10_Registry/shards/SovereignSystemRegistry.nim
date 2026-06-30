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

proc SovereignRegistry_Init*() {.exportc.} =
  discard

proc SovereignRegistry_Audit*() {.exportc.} =
  discard

proc SovereignRegistry_Finalize*() {.exportc.} =
  discard

proc S10_Registry_Register*() {.exportc.} =
  discard

