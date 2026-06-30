## SigmaOS: SIGMA_SEC_AUDIT_H */
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
  SigmaAuditEvent* = object
    timestamp_rdtsc*: SigmaU64
    level*: SigmaU64
    actor_id*: SigmaI32
    target_id*: SigmaI32

type
  SigmaAuditLog* = object
    count*: SigmaI32
    dropped*: SigmaI32

proc audit_init*() {.exportc.} =
  discard

proc audit_log*() {.exportc.} =
  discard

