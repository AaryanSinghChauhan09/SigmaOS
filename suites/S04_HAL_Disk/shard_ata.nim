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
  ATADrive* = object
    channel*: SigmaU8
    drive*: SigmaU8
    type*: SigmaU8
    lba48*: SigmaU64
    present*: SigmaU64
    sectors*: SigmaU64
    base*: SigmaU16
    ctrl*: SigmaU16

proc ata_write8*() {.exportc.} =
  discard

proc ata_delay400ns*() {.exportc.} =
  discard

proc ata_init*() {.exportc.} =
  discard

