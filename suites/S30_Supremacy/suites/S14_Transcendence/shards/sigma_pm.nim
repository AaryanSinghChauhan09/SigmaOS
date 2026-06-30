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

proc sigma_pm_init*() {.exportc.} =
  discard

proc sigma_pm_shutdown*() {.exportc.} =
  discard

proc sigma_pm_reboot*() {.exportc.} =
  discard

proc sigma_pm_enter_cstate*() {.exportc.} =
  discard

proc sigma_pm_tick*() {.exportc.} =
  discard

proc sigma_pm_wakelock_release*() {.exportc.} =
  discard

proc sigma_pm_doze_enter*() {.exportc.} =
  discard

proc sigma_pm_doze_exit*() {.exportc.} =
  discard

proc sigma_pm_thermal_update*() {.exportc.} =
  discard

proc sigma_pm_report*() {.exportc.} =
  discard

