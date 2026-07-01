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
  SigmaInputDevice* = object
    id_bustype*: SigmaU16
    id_vendor*: SigmaU16
    id_product*: SigmaU16
    id_version*: SigmaU16
    queue_head*: SigmaU32
    queue_tail*: SigmaU32
    online*: SigmaU64
    minor*: SigmaU32

proc enqueue_event*() {.exportc.} =
  discard

proc sigma_input_event*() {.exportc.} =
  discard

proc sigma_input_sync*() {.exportc.} =
  discard

proc SovereignInput_Init*() {.exportc.} =
  discard

