## SigmaOS: SIGMA_AIO_H */
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
  SigmaAIOReq* = object
    op*: uint8
    fd*: SigmaI32
    len*: SigmaI32
    offset*: SigmaU64
    user_data*: SigmaI32

type
  SigmaAIOCompletion* = object
    user_data*: SigmaI32
    result*: SigmaI32

type
  SigmaAIOSQRing* = object
    head*: SigmaI32
    tail*: SigmaI32

type
  SigmaAIOCQRing* = object
    head*: SigmaI32
    tail*: SigmaI32

type
  SigmaAIOContext* = object
    sq*: SigmaU64
    cq*: SigmaU64

proc aio_init*() {.exportc.} =
  discard

