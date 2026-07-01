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

proc sigma_cpufreq_register*() {.exportc.} =
  discard

proc sigma_turbo_set*() {.exportc.} =
  discard

proc sigma_thermal_update*() {.exportc.} =
  discard

proc sigma_battery_status*() {.exportc.} =
  discard

proc sigma_battery_set_threshold*() {.exportc.} =
  discard

proc sigma_pm_get*() {.exportc.} =
  discard

proc sigma_pm_put*() {.exportc.} =
  discard

proc SovereignPowerManagement_Init*() {.exportc.} =
  discard

