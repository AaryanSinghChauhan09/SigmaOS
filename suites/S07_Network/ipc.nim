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
  SigmaPipe* = object
    head*: SigmaU32
    tail*: SigmaU32
    count*: SigmaU32
    valid*: SigmaU64
    write_closed*: SigmaU64
    read_closed*: SigmaU64

type
  SigmaMsg* = object
    mtype*: SigmaU32
    len*: SigmaU32

type
  SigmaMQ* = object
    head*: SigmaU32
    count*: SigmaU32
    valid*: SigmaU64

type
  SigmaSHM* = object
    paddr*: SigmaU64
    size*: SigmaU64
    key*: SigmaU32
    refs*: SigmaU32
    valid*: SigmaU64

type
  SigmaFutex* = object
    waiter_tid*: SigmaU64
    valid*: SigmaU64

proc ipc_init*() {.exportc.} =
  discard

