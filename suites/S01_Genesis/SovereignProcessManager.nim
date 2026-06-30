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
  IProcess* = object of RootObj
    initialized*: SigmaBool

proc newIProcess*(): IProcess =
  result = IProcess(initialized: false)

proc audit*(self: var IProcess) =
  self.initialized = true

proc sigma_kernel_entry*(self: var IProcess) =
  self.initialized = true

proc main*(self: var IProcess) =
  self.initialized = true

type
  SovereignPCB* = object
    pid*: SigmaU64
    cr3*: SigmaU64
    rsp*: SigmaU64
    state*: SigmaU32

var instance* = newIProcess()

proc audit*() {.exportc.} =
  instance.initialized = true

proc sigma_kernel_entry*() {.exportc.} =
  instance.initialized = true

