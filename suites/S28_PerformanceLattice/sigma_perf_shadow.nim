## SigmaOS: SIGMA_PERF_SHADOW_H */
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
  SigmaShadowEntry* = object
    size*: SigmaU64
    alloc_cycles*: SigmaU64
    free_cycles*: SigmaU64
    state*: SigmaU64

type
  SigmaShadowMem* = object
    count*: SigmaI32
    total_allocated*: SigmaU64
    total_freed*: SigmaU64
    peak_live*: SigmaU64
    current_live*: SigmaU64

proc shadow_init*() {.exportc.} =
  discard

