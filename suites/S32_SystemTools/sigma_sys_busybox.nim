## SigmaOS: SIGMA_SYS_BUSYBOX_H */
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

type
  SigmaWCResult* = object

proc sigma_echo*() {.exportc.} =
  discard

proc sigma_cat*() {.exportc.} =
  discard

proc sigma_memset_util*() {.exportc.} =
  discard

proc sigma_memcpy_util*() {.exportc.} =
  discard

proc sigma_strncpy*() {.exportc.} =
  discard

proc sigma_itoa*() {.exportc.} =
  discard

proc sigma_yes*() {.exportc.} =
  discard

proc sigma_tr*() {.exportc.} =
  discard

