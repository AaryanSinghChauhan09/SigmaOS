## SigmaOS: SIGMA_NETMESH_ROUTING_HPP */
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
  IMeshRouter* = object of RootObj
    initialized*: SigmaBool

proc newIMeshRouter*(): IMeshRouter =
  result = IMeshRouter(initialized: false)

type
  RouteEntry* = object
    destination_ip*: SigmaI32
    gateway_ip*: SigmaI32
    metric*: SigmaI32

var instance* = newIMeshRouter()

