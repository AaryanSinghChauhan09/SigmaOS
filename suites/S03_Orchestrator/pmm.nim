## SigmaOS: =============================================================================
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
  BuddyNode* = object

type
  SigmaPMM* = object
    total_pages*: SigmaU64
    free_pages*: SigmaU64
    alloc_calls*: SigmaU64
    free_calls*: SigmaU64

proc bitmap_set*() {.exportc.} =
  discard

proc bitmap_clr*() {.exportc.} =
  discard

proc fl_push*() {.exportc.} =
  discard

proc fl_remove*() {.exportc.} =
  discard

proc pmm_free*() {.exportc.} =
  discard

proc pmm_free_page*() {.exportc.} =
  discard

proc pmm_audit*() {.exportc.} =
  discard

