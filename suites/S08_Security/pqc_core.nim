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
  PQCContext* = object
    active*: SigmaU64

proc pqc_init*() {.exportc.} =
  discard

proc pqc_generate_shard_key*() {.exportc.} =
  discard

proc pqc_encrypt_buffer*() {.exportc.} =
  discard

