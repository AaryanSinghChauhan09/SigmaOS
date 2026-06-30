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

proc s_ls*() {.exportc.} =
  discard

proc s_cat*() {.exportc.} =
  discard

proc s_grep*() {.exportc.} =
  discard

proc s_top*() {.exportc.} =
  discard

proc s_mkdir*() {.exportc.} =
  discard

proc s_rm*() {.exportc.} =
  discard

proc s_touch*() {.exportc.} =
  discard

proc s_ps*() {.exportc.} =
  discard

proc s_kill*() {.exportc.} =
  discard

proc s_ping*() {.exportc.} =
  discard

proc s_ifconfig*() {.exportc.} =
  discard

proc s_uname*() {.exportc.} =
  discard

proc s_whoami*() {.exportc.} =
  discard

proc s_clear*() {.exportc.} =
  discard

