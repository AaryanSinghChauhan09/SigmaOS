## SigmaOS: SIGMA_IMMUTABLE_FS_H */
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
  SigmaFSSlot* = object
    magic*: SigmaI32
    slot_id*: uint8
    integrity_hash*: SigmaU64
    verified*: uint8
    active*: uint8
    boot_count*: SigmaI32
    max_boot_tries*: SigmaI32

type
  SigmaImmutableFS* = object
    current_slot*: uint8

proc ifs_init*() {.exportc.} =
  discard

proc ifs_swap_slot*() {.exportc.} =
  discard

proc ifs_maybe_rollback*() {.exportc.} =
  discard

