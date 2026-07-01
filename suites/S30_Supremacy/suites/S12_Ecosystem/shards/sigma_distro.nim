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

proc sigma_distro_init*() {.exportc.} =
  discard

proc sigma_repo_remove*() {.exportc.} =
  discard

proc sigma_repo_list*() {.exportc.} =
  discard

proc sigma_pkg_list_installed*() {.exportc.} =
  discard

proc sigma_dal_generation_snapshot*() {.exportc.} =
  discard

proc sigma_distro_report*() {.exportc.} =
  discard

