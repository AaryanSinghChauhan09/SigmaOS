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
  LinuxDeviceShim* = object
    id*: SigmaU32
    is_ready*: SigmaU64

type
  LinuxDriverShim* = object
    active*: SigmaU64

proc linux_shim_init*() {.exportc.} =
  discard

