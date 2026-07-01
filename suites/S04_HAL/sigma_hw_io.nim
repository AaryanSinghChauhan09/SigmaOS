## SigmaOS: sigma_hw_io module
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
  SovereignInterruptController* = object of RootObj
    initialized*: SigmaBool

proc newSovereignInterruptController*(): SovereignInterruptController =
  result = SovereignInterruptController(initialized: false)

type
  InterruptVector* = object
    handler_addr*: SigmaU64
    type*: SigmaI32

var instance* = newSovereignInterruptController()

