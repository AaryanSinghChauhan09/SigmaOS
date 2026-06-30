## SigmaOS: "" = all signals on interface */
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
  SigmaMessage* = object
    body_len*: SigmaU64
    sender_pid*: SigmaU64
    timestamp_ns*: SigmaU64
    requires_cap*: SigmaBool

type
  Subscriber* = object
    handler*: SigmaU64
    active*: SigmaBool

type
  SigmaBus* = object
    n_subs*: SigmaI32
    messages_delivered*: SigmaI32
    messages_dropped*: SigmaI32

type
  ReceivedMsg* = object
    count*: SigmaI32

proc bus_reset*() {.exportc.} =
  discard

proc capture_handler*() {.exportc.} =
  discard

