## SigmaOS: SIGMA_FW_UPDATE_H */
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
  SigmaFWComponent* = object
    current_hash*: SigmaU64
    pending_hash*: SigmaU64
    state*: SigmaU64
    retry_count*: SigmaI32

type
  SigmaFWUpdater* = object
    count*: SigmaI32

proc fwup_init*() {.exportc.} =
  discard

proc fwup_rollback*() {.exportc.} =
  discard

