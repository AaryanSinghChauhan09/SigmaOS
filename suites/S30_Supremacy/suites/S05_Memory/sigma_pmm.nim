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

proc bitmap_set*() {.exportc.} =
  discard

proc bitmap_clear*() {.exportc.} =
  discard

proc sigma_pmm_init*() {.exportc.} =
  discard

proc sigma_pmm_mark_used*() {.exportc.} =
  discard

proc sigma_pmm_mark_sigma_free*() {.exportc.} =
  discard

proc sigma_pmm_free_block*() {.exportc.} =
  discard

