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

proc sigma_mobile_dispatch_intent*() {.exportc.} =
  discard

proc sigma_mobile_sensor_sync*() {.exportc.} =
  discard

proc SovereignAndroidBridge_Register*() {.exportc.} =
  discard

