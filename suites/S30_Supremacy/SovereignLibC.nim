## SigmaOS: SovereignLibC module
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

proc sigma_print*() {.exportc.} =
  discard

proc sigma_print_num*() {.exportc.} =
  discard

proc sigma_print_hex*() {.exportc.} =
  discard

proc sigma_strcat*() {.exportc.} =
  discard

proc sigma_log_info*() {.exportc.} =
  discard

proc sigma_free*() {.exportc.} =
  discard

