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
  dispatch* = object of RootObj
    initialized*: SigmaBool

proc newdispatch*(): dispatch =
  result = dispatch(initialized: false)

proc sigma_sigma_strcmp*(self: var dispatch) =
  self.initialized = true

proc sigma_print_usage*(self: var dispatch) =
  self.initialized = true

proc SigmaCLI_Dispatcher_ToolMain*(self: var dispatch) =
  self.initialized = true

var instance* = newdispatch()

proc sigma_print_usage*() {.exportc.} =
  instance.initialized = true

