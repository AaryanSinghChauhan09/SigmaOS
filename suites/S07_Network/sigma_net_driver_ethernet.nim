## SigmaOS: SIGMA_NET_DRIVER_ETHERNET_HPP */
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
  IEthernetNIC* = object of RootObj
    initialized*: SigmaBool

proc newIEthernetNIC*(): IEthernetNIC =
  result = IEthernetNIC(initialized: false)

proc kick_dma_doorbell*(self: var IEthernetNIC) =
  self.initialized = true

type
  EthPacketDescriptor* = object
    phys_addr*: SigmaU64
    length*: SigmaI32
    flags*: SigmaI32

var instance* = newIEthernetNIC()

proc kick_dma_doorbell*() {.exportc.} =
  instance.initialized = true

