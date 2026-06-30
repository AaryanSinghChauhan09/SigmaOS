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
  SigmaSPITransfer* = object
    len*: SigmaU32
    speed_hz*: SigmaU32
    bits_per_word*: SigmaU8
    delay_usecs*: SigmaU16
    cs_change*: SigmaU64

type
  SigmaSPIMessage* = object
    actual_length*: SigmaU32
    status*: SigmaU64

type
  SigmaSPIDevice* = object
    max_speed_hz*: SigmaU32
    chip_select*: SigmaU8
    mode*: SigmaU8
    bits_per_word*: SigmaU8
    online*: SigmaU64

type
  SigmaSPIController* = object
    bus_num*: SigmaU32
    num_chipselect*: SigmaU16
    online*: SigmaU64

proc SovereignSPICore_Init*() {.exportc.} =
  discard

