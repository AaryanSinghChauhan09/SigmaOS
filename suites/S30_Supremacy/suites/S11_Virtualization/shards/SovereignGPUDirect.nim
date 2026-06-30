## SigmaOS: SigmaOS Sovereign GPU Direct Purity Shard
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

proc gpu_direct_map_surface*() {.exportc.} =
  discard

proc gpu_direct_submit_batch*() {.exportc.} =
  discard

proc S11_Register_GPUDirect*() {.exportc.} =
  discard

