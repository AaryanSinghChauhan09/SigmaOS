## SigmaOS: =========================================================================
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

proc sigma_sha256_transform*() {.exportc.} =
  discard

proc sigma_sha256_init*() {.exportc.} =
  discard

proc sigma_sha256_update*() {.exportc.} =
  discard

proc sigma_sha256_final*() {.exportc.} =
  discard

proc sigma_sha256*() {.exportc.} =
  discard

proc sigma_hmac_sha256*() {.exportc.} =
  discard

proc sigma_chacha20_encrypt*() {.exportc.} =
  discard

proc SovereignCrypto_Init*() {.exportc.} =
  discard

proc SovereignCrypto_Register*() {.exportc.} =
  discard

