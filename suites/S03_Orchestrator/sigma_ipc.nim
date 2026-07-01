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
  SigmaIPCMsg* = object
    sender_tid*: SigmaU64
    msg_type*: SigmaU64
    payload_len*: SigmaU64
    _reserved*: SigmaU64

type
  SigmaIPCChannel* = object
    id*: SigmaU64
    active*: SigmaU64
    owner_pool*: SigmaU64
    target_pool*: SigmaU64
    required_cap*: SigmaU64
    head*: SigmaU64
    tail*: SigmaU64
    count*: SigmaU64
    total_sent*: SigmaU64
    total_recv*: SigmaU64
    total_dropped*: SigmaU64

proc ipc_audit*() {.exportc.} =
  discard

