## SigmaOS: =========================================================================
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
  SovereignBootMaster* = object of RootObj
    initialized*: SigmaBool

proc newSovereignBootMaster*(): SovereignBootMaster =
  result = SovereignBootMaster(initialized: false)

proc FastInit*(self: var SovereignBootMaster) =
  self.initialized = true

proc LaunchKernel*(self: var SovereignBootMaster) =
  self.initialized = true

proc _start*(self: var SovereignBootMaster) =
  self.initialized = true

var instance* = newSovereignBootMaster()

proc FastInit*() {.exportc.} =
  instance.initialized = true

proc LaunchKernel*() {.exportc.} =
  instance.initialized = true

proc _start*() {.exportc.} =
  instance.initialized = true

