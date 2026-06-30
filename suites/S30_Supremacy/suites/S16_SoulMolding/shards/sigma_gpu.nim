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

proc sigma_gpu_init*() {.exportc.} =
  discard

proc sigma_bo_destroy*() {.exportc.} =
  discard

proc sigma_bo_list*() {.exportc.} =
  discard

proc sigma_cmdq_wait*() {.exportc.} =
  discard

proc sigma_connector_list*() {.exportc.} =
  discard

proc sigma_surface_present*() {.exportc.} =
  discard

proc sigma_gpu_stats*() {.exportc.} =
  discard

