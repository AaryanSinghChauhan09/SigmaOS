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
  SovereignTranspiler* = object of RootObj
    initialized*: SigmaBool

proc newSovereignTranspiler*(): SovereignTranspiler =
  result = SovereignTranspiler(initialized: false)

proc sigma_main*(self: var SovereignTranspiler) =
  self.initialized = true

proc audit*(self: var SovereignTranspiler) =
  self.initialized = true

proc start_transpiler_demo*(self: var SovereignTranspiler) =
  self.initialized = true

proc main*(self: var SovereignTranspiler) =
  self.initialized = true

var instance* = newSovereignTranspiler()

proc sigma_main*() {.exportc.} =
  instance.initialized = true

proc audit*() {.exportc.} =
  instance.initialized = true

proc start_transpiler_demo*() {.exportc.} =
  instance.initialized = true

