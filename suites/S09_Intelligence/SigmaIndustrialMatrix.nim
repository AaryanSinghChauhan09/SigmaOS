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

proc sigma_macro_trigger*() {.exportc.} =
  discard

proc sigma_cli_training*() {.exportc.} =
  discard

proc sigma_provision_user*() {.exportc.} =
  discard

proc sigma_media_sync*() {.exportc.} =
  discard

proc sigma_remote_exec*() {.exportc.} =
  discard

proc sigma_matrix_init*() {.exportc.} =
  discard

