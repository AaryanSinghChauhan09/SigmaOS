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
  SigmaSndPCMSubstream* = object
    stream*: SigmaU32
    dma_bytes*: SigmaU64
    hw_ptr*: SigmaU32
    appl_ptr*: SigmaU32
    hw_params*: SigmaU64
    active*: SigmaU64

type
  SigmaSndPCM* = object
    device*: SigmaU32
    playback*: SigmaU64
    capture*: SigmaU64

type
  SigmaSndKControl* = object
    type*: SigmaU32
    min_val*: SigmaU64
    max_val*: SigmaU64
    value*: SigmaU64

type
  SigmaSndCard* = object
    number*: SigmaI32
    pcm_count*: SigmaU32
    control_count*: SigmaU32
    online*: SigmaU64

proc SovereignSoundALSA_Init*() {.exportc.} =
  discard

