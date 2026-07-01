## SigmaOS: SIGMA_AUTO_ROLLBACK_H */
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
  SigmaSnapshot* = object
    snap_id*: SigmaI32
    taken_at*: SigmaU64
    content_hash*: SigmaU64
    state*: SigmaU64
    is_boot_snapshot*: uint8

type
  SigmaRollbackManager* = object
    count*: SigmaI32
    active_snap*: SigmaI32
    next_id*: SigmaI32

proc rollback_init*() {.exportc.} =
  discard

