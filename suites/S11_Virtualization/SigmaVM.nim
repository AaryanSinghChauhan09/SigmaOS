## SigmaOS: SigmaVM module
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
  SigmaVM* = object of RootObj
    initialized*: SigmaBool

proc newSigmaVM*(): SigmaVM =
  result = SigmaVM(initialized: false)

proc start_virtual_machine*(self: var SigmaVM) =
  self.initialized = true

var instance* = newSigmaVM()

proc start_virtual_machine*() {.exportc.} =
  instance.initialized = true

