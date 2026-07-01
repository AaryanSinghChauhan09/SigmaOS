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
  SigmaCapability* = object
    module_id*: SigmaU64
    resource_mask*: SigmaU64
    expiry_tick*: SigmaU64
    nonce*: SigmaU64

proc compute_signature*() {.exportc.} =
  discard

proc zt_init*() {.exportc.} =
  discard

proc zt_revoke*() {.exportc.} =
  discard

