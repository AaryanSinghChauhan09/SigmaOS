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

proc sigma_util_ls*() {.exportc.} =
  discard

proc sigma_util_cat*() {.exportc.} =
  discard

proc sigma_util_mkdir*() {.exportc.} =
  discard

proc sigma_util_init*() {.exportc.} =
  discard

