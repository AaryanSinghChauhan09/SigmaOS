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
  NetBuf* = object
    len*: SigmaU32
    used*: SigmaU64

type
  RouteEntry* = object
    dest*: SigmaU32
    mask*: SigmaU32
    gateway*: SigmaU32
    iface*: SigmaU8
    valid*: SigmaU64

type
  SigmaSocket* = object
    proto*: SigmaU8
    local_port*: SigmaU16
    remote_port*: SigmaU16
    local_ip*: SigmaU32
    remote_ip*: SigmaU32
    state*: SigmaU64
    seq*: SigmaU32
    ack*: SigmaU32
    window*: SigmaU16
    rx_head*: SigmaU32
    rx_count*: SigmaU32
    used*: SigmaU64

proc netbuf_free*() {.exportc.} =
  discard

proc route_add*() {.exportc.} =
  discard

proc net_build_tcp*() {.exportc.} =
  discard

proc net_init*() {.exportc.} =
  discard

proc net_audit*() {.exportc.} =
  discard

