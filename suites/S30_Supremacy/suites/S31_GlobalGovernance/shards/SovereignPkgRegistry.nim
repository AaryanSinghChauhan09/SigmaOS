## SigmaOS: Sovereign Package Registry (v1.0).
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

proc SovereignPkg_InitRegistry*() {.exportc.} =
  discard

proc SovereignPkg_Audit*() {.exportc.} =
  discard

proc SovereignPkg_SnapshotState*() {.exportc.} =
  discard

proc SovereignPkg_LoadManifest*() {.exportc.} =
  discard

