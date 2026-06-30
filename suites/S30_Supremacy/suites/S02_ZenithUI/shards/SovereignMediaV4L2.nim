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
  SigmaV4L2Device* = object
    minor*: SigmaU32
    capabilities*: SigmaU32
    current_fmt*: SigmaU64
    num_buffers*: SigmaU32
    req_memory*: SigmaU32
    streaming*: SigmaU64
    sequence*: SigmaU32
    online*: SigmaU64

proc SovereignMediaV4L2_Init*() {.exportc.} =
  discard

