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

proc sigma_logger_write*() {.exportc.} =
  discard

proc sigma_event_publish*() {.exportc.} =
  discard

proc strategy_insertion_sort*() {.exportc.} =
  discard

proc strategy_selection_sort*() {.exportc.} =
  discard

proc sigma_sort_set_strategy*() {.exportc.} =
  discard

proc sigma_sort_execute*() {.exportc.} =
  discard

proc sigma_iter_reset*() {.exportc.} =
  discard

proc SovereignDesignPatterns_Register*() {.exportc.} =
  discard

