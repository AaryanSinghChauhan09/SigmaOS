## SigmaOS: simple replay protection */
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
  SigmaCapToken* = object
    owner_pid*: SigmaI32
    permissions*: uint8
    nonce*: SigmaI32
    revoked*: uint8

proc cap_revoke*() {.exportc.} =
  discard

