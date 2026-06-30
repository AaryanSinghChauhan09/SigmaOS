## SigmaOS: SIGMA_DEV_BUILD_H */
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
  SigmaBuildTarget* = object
    target_id*: SigmaI32
    content_hash*: SigmaU64
    prev_hash*: SigmaU64
    dep_count*: SigmaI32
    fn*: SigmaU64
    built*: uint8
    dirty*: uint8

type
  SigmaBuildGraph* = object
    count*: SigmaI32
    total_built*: SigmaU64
    total_skipped*: SigmaU64

proc build_graph_init*() {.exportc.} =
  discard

proc build_add_dep*() {.exportc.} =
  discard

