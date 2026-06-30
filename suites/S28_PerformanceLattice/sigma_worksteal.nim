## SigmaOS: SIGMA_WORKSTEAL_H */
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
  WSTask* = object
    fn*: SigmaU64

type
  WSDeque* = object
    top*: SigmaI32
    bot*: SigmaI32
    lock*: SigmaU64

type
  SigmaWorkStealPool* = object
    worker_count*: SigmaI32
    shutdown*: SigmaI32

proc ws_pool_init*() {.exportc.} =
  discard

