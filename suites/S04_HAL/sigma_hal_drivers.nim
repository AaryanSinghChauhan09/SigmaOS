## SigmaOS: sigma_hal_drivers module
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
  NVMeDriver* = object of RootObj
    initialized*: SigmaBool

proc newNVMeDriver*(): NVMeDriver =
  result = NVMeDriver(initialized: false)

proc hal_run_all_drivers*(self: var NVMeDriver) =
  self.initialized = true

var instance* = newNVMeDriver()

proc hal_run_all_drivers*() {.exportc.} =
  instance.initialized = true

