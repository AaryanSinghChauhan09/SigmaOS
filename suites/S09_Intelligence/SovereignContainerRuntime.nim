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
  SovereignContainer* = object of RootObj
    initialized*: SigmaBool

proc newSovereignContainer*(): SovereignContainer =
  result = SovereignContainer(initialized: false)

proc InjectShard*(self: var SovereignContainer) =
  self.initialized = true

proc _start*(self: var SovereignContainer) =
  self.initialized = true

var instance* = newSovereignContainer()

proc InjectShard*() {.exportc.} =
  instance.initialized = true

proc _start*() {.exportc.} =
  instance.initialized = true

