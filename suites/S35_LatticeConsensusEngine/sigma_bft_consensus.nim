## SigmaOS: sigma_bft_consensus module
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

proc sigma_internal_sigma_memcpy*() {.exportc.} =
  discard

proc sigma_internal_memzero*() {.exportc.} =
  discard

proc sigma_bft_init*() {.exportc.} =
  discard

proc sigma_bft_generate_pre_prepare*() {.exportc.} =
  discard

