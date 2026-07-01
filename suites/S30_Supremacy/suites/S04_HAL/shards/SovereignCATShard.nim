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
  of* = object of RootObj
    initialized*: SigmaBool

proc newof*(): of =
  result = of(initialized: false)

proc sigma_hal_cat_set_mask*(self: var of) =
  self.initialized = true

proc SovereignCAT_Register*(self: var of) =
  self.initialized = true

var instance* = newof()

proc sigma_hal_cat_set_mask*() {.exportc.} =
  instance.initialized = true

proc SovereignCAT_Register*() {.exportc.} =
  instance.initialized = true

