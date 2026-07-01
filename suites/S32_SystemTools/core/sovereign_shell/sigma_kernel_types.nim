## SigmaOS: sigma_kernel_types.h — Sovereign canonical shim */
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
  sigma_jail* = object
    id*: SigmaU32
    flags*: SigmaU32

type
  sigma_unit* = object
    state*: SigmaU32

proc cpu_halt*() {.exportc.} =
  discard

proc cpu_pause*() {.exportc.} =
  discard

