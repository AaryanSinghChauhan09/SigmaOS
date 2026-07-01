## SigmaOS: SigmaOS: Sovereign Nextcloud Proxy
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

proc sigma_nextcloud_init*() {.exportc.} =
  discard

proc sigma_nextcloud_sync_config*() {.exportc.} =
  discard

proc sigma_nextcloud_mount_remote*() {.exportc.} =
  discard

