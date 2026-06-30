## SigmaOS: SIGMA_NET_TCP_H */
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
  SigmaTCPConn* = object
    state*: SigmaU64
    local_ip*: SigmaI32
    remote_ip*: SigmaI32
    local_port*: SigmaU64
    remote_port*: SigmaU64
    seq_num*: SigmaI32
    ack_num*: SigmaI32
    lock*: SigmaU64

type
  SigmaTCPStack* = object
    count*: SigmaI32

proc tcp_stack_init*() {.exportc.} =
  discard

proc tcp_close*() {.exportc.} =
  discard

