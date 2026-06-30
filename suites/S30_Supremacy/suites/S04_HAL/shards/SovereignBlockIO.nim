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

type
  SigmaBio* = object
    opcode*: SigmaU8
    sector*: SigmaU64
    size*: SigmaU32
    status*: SigmaU64
    end_io*: SigmaU64

type
  SigmaBlockDevice* = object
    capacity*: SigmaU64
    hardsect_size*: SigmaU32
    online*: SigmaU64
    queue*: SigmaU64
    read_sectors*: SigmaU64
    write_sectors*: SigmaU64

proc sigma_bio_complete*() {.exportc.} =
  discard

proc my_bio_end_io*() {.exportc.} =
  discard

proc sigma_blk_print_stats*() {.exportc.} =
  discard

proc SovereignBlockIO_Init*() {.exportc.} =
  discard

