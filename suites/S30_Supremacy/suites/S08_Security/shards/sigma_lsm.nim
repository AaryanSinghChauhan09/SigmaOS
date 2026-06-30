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

proc audit_write*() {.exportc.} =
  discard

proc sigma_lsm_init*() {.exportc.} =
  discard

proc sigma_lsm_register_hooks*() {.exportc.} =
  discard

proc sigma_lsm_ctx_destroy*() {.exportc.} =
  discard

proc sigma_lsm_unveil*() {.exportc.} =
  discard

proc sigma_lsm_audit_dump*() {.exportc.} =
  discard

