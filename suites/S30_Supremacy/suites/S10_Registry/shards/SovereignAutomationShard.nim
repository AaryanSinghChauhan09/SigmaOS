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

proc sigma_cron_tick*() {.exportc.} =
  discard

proc sigma_automation_self_heal*() {.exportc.} =
  discard

proc handler_log_rotate*() {.exportc.} =
  discard

proc handler_slab_gc*() {.exportc.} =
  discard

proc handler_fs_defrag*() {.exportc.} =
  discard

proc SovereignAutomation_Audit*() {.exportc.} =
  discard

proc SovereignAutomation_Register*() {.exportc.} =
  discard

